//! Personal data never reaches the database (ADR-0009, invariant I4): what
//! each digest path stores after the scrub, and that the source address is
//! not recorded by the ingest route.

use std::sync::Arc;

use actix_web::{test, web, App};
use chrono::Utc;
use serde_json::{json, Value};
use tempfile::TempDir;
use uuid::Uuid;

use crate::common::TestDb;
use rustrak::config::RateLimitConfig;
use rustrak::digest::processors::{
    LogsProcessor, Processor, ProcessorCtx, Processors, SpanProcessor, TransactionProcessor,
};
use rustrak::ingest::{store_event, EventMetadata};
use rustrak::models::CreateProject;
use rustrak::routes;
use rustrak::routes::ingest::digest_stored_event;
use rustrak::scrub::FILTERED;
use rustrak::services::log::{LogFilters, LogService};
use rustrak::services::{
    DbSourceMapProvider, LocalSourceMapStore, ProjectService, SourceMapProvider, SourceMapStore,
};

const SECRETS: &[&str] = &[
    "hunter2",
    "sid=1",
    "Bearer x",
    "ana@example.com",
    "529.982.247-25",
    "52998224725",
    "10.0.0.1",
    "4111 1111 1111 1111",
];

fn assert_clean(stored: &Value, what: &str) {
    let text = stored.to_string();
    for secret in SECRETS {
        assert!(
            !text.contains(secret),
            "{what} still carries {secret:?}: {text}"
        );
    }
}

async fn project(pool: &rustrak::db::DbPool, name: &str) -> rustrak::models::Project {
    ProjectService::create(
        pool,
        CreateProject {
            name: name.to_string(),
            slug: None,
            platform: None,
        },
    )
    .await
    .expect("project")
}

fn ctx(pool: &rustrak::db::DbPool, project_id: i32) -> ProcessorCtx {
    ProcessorCtx {
        pool: pool.clone(),
        project_id,
        event_id: Uuid::new_v4(),
        ingested_at: Utc::now(),
        remote_addr: None,
    }
}

#[cfg(feature = "postgres")]
const EVENT_ROW: &str =
    "SELECT data, remote_addr, issue_id FROM events WHERE project_id = $1 ORDER BY ingested_at";
#[cfg(not(feature = "postgres"))]
const EVENT_ROW: &str =
    "SELECT data, remote_addr, issue_id FROM events WHERE project_id = ? ORDER BY ingested_at";

#[cfg(feature = "postgres")]
const TRANSACTION_ROW: &str = "SELECT data, remote_addr FROM transactions WHERE project_id = $1";
#[cfg(not(feature = "postgres"))]
const TRANSACTION_ROW: &str = "SELECT data, remote_addr FROM transactions WHERE project_id = ?";

#[cfg(feature = "postgres")]
const SPAN_DATA: &str = "SELECT data FROM spans WHERE project_id = $1";
#[cfg(not(feature = "postgres"))]
const SPAN_DATA: &str = "SELECT data FROM spans WHERE project_id = ?";

// =============================================================================
// Events
// =============================================================================

fn dirty_event(event_id: &str, cpf: &str) -> Value {
    json!({
        "event_id": event_id,
        "timestamp": Utc::now().timestamp() as f64,
        "platform": "javascript",
        "message": format!("login de ana@example.com com cpf {cpf} falhou"),
        "user": { "id": "u-1", "email": "ana@example.com", "ip_address": "10.0.0.1" },
        "request": {
            "url": "https://app.example.com/pedidos",
            "headers": { "Cookie": "sid=1", "Authorization": "Bearer x", "Accept": "*/*" },
            "data": { "password": "hunter2", "nome": "Ana", "cartao": "4111 1111 1111 1111" }
        },
        "exception": { "values": [{ "type": "TypeError", "value": format!("cpf {cpf} invalido") }] }
    })
}

async fn digest(pool: &rustrak::db::DbPool, dir: &TempDir, project_id: i32, event: &Value) {
    let processors = Processors::new(
        dir.path().to_path_buf(),
        RateLimitConfig::default(),
        crate::common::null_sourcemap_provider(),
        None,
    );
    let event_id = event["event_id"].as_str().unwrap().to_string();
    store_event(dir.path(), &event_id, &serde_json::to_vec(event).unwrap())
        .await
        .expect("store");
    let metadata = EventMetadata {
        event_id,
        project_id,
        ingested_at: Utc::now(),
        remote_addr: Some("203.0.113.9".to_string()),
    };
    digest_stored_event(&processors, pool, &metadata).await;
}

#[actix_web::test]
async fn a_digested_event_keeps_no_personal_data_and_no_address() {
    let db = TestDb::new().await;
    let dir = TempDir::new().unwrap();
    let project = project(&db.pool, "Scrub events").await;
    let event_id = Uuid::new_v4().simple().to_string();

    digest(
        &db.pool,
        &dir,
        project.id,
        &dirty_event(&event_id, "529.982.247-25"),
    )
    .await;

    let rows: Vec<(Value, Option<String>, Option<Uuid>)> = sqlx::query_as(EVENT_ROW)
        .bind(project.id)
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1, "one stored event");
    let (data, remote_addr, _) = &rows[0];
    assert_clean(data, "events.data");
    assert_eq!(data["user"]["id"], "u-1", "the user id is kept");
    assert_eq!(data["user"]["email"], "[email]");
    assert_eq!(data["user"]["ip_address"], FILTERED);
    assert_eq!(data["request"]["headers"]["Cookie"], FILTERED);
    assert_eq!(data["request"]["headers"]["Accept"], "*/*");
    assert_eq!(data["request"]["data"]["password"], FILTERED);
    assert_eq!(data["request"]["data"]["cartao"], FILTERED);
    assert_eq!(data["message"], "login de [email] com cpf [cpf] falhou");
    assert_eq!(
        data["exception"]["values"][0]["value"],
        "cpf [cpf] invalido"
    );
    assert!(
        remote_addr.is_none(),
        "the address handed to the digest is still not stored"
    );
}

#[actix_web::test]
async fn two_events_with_different_cpfs_share_one_issue() {
    let db = TestDb::new().await;
    let dir = TempDir::new().unwrap();
    let project = project(&db.pool, "Scrub grouping").await;

    for cpf in ["529.982.247-25", "111.444.777-35"] {
        let id = Uuid::new_v4().simple().to_string();
        digest(&db.pool, &dir, project.id, &dirty_event(&id, cpf)).await;
    }

    let rows: Vec<(Value, Option<String>, Option<Uuid>)> = sqlx::query_as(EVENT_ROW)
        .bind(project.id)
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0].2, rows[1].2,
        "the fingerprint is computed on the masked value, so both land in one issue"
    );
}

// =============================================================================
// Transactions, through the ingest route, with a peer address
// =============================================================================

fn transaction_payload() -> Value {
    let now = Utc::now().timestamp();
    json!({
        "event_id": Uuid::new_v4().simple().to_string(),
        "type": "transaction",
        "transaction": "GET /pedidos",
        "timestamp": now,
        "start_timestamp": now - 1,
        "platform": "javascript",
        "request": { "headers": { "Authorization": "Bearer x" }, "url": "https://app.example.com/pedidos" },
        "user": { "id": "u-1", "email": "ana@example.com" },
        "contexts": { "trace": { "trace_id": "a".repeat(32), "span_id": "b".repeat(16), "op": "http.server" } },
        "spans": [{
            "span_id": "c".repeat(16), "trace_id": "a".repeat(32), "parent_span_id": "b".repeat(16),
            "op": "db.query", "description": "SELECT * FROM users WHERE email = 'ana@example.com'",
            "start_timestamp": now - 1, "timestamp": now,
            "data": { "password": "hunter2", "db.system": "postgresql" }
        }]
    })
}

#[actix_web::test]
async fn a_transaction_through_the_route_stores_no_address_and_no_personal_data() {
    let db = TestDb::new().await;
    let project = project(&db.pool, "Scrub transactions").await;
    let dir = TempDir::new().unwrap();
    let config = {
        let mut c = test_config();
        c.ingest_dir = Some(dir.path().to_string_lossy().to_string());
        c
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data({
                let store: Arc<dyn SourceMapStore> =
                    Arc::new(LocalSourceMapStore::new(dir.path().join("sm")));
                let provider: Arc<dyn SourceMapProvider> =
                    Arc::new(DbSourceMapProvider::new(db.pool.clone(), store));
                web::Data::new(provider)
            })
            .app_data(web::Data::new(Processors::new(
                dir.path().to_path_buf(),
                config.rate_limit.clone(),
                crate::common::null_sourcemap_provider(),
                None,
            )))
            .configure(routes::ingest::configure),
    )
    .await;

    let payload = serde_json::to_vec(&transaction_payload()).unwrap();
    let item = format!(r#"{{"type":"transaction","length":{}}}"#, payload.len());
    let mut envelope = format!("{{}}\n{item}\n").into_bytes();
    envelope.extend_from_slice(&payload);
    envelope.push(b'\n');
    let req = test::TestRequest::post()
        .uri(&format!("/api/{}/envelope/", project.id))
        .peer_addr("203.0.113.9:4000".parse().unwrap())
        .insert_header((
            "X-Sentry-Auth",
            format!(
                "Sentry sentry_key={}, sentry_version=7",
                project.sentry_key.simple()
            ),
        ))
        .insert_header(("Content-Type", "application/x-sentry-envelope"))
        .set_payload(envelope)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let rows: Vec<(Value, Option<String>)> = sqlx::query_as(TRANSACTION_ROW)
        .bind(project.id)
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    let (data, remote_addr) = &rows[0];
    assert_clean(data, "transactions.data");
    assert_eq!(data["request"]["headers"]["Authorization"], FILTERED);
    assert_eq!(data["user"]["email"], "[email]");
    assert_eq!(data["user"]["id"], "u-1");
    assert!(
        remote_addr.is_none(),
        "the peer address of the request is never stored"
    );

    let spans: Vec<(Value,)> = sqlx::query_as(SPAN_DATA)
        .bind(project.id)
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert!(!spans.is_empty(), "the child span was stored");
    for (span,) in &spans {
        assert_clean(span, "spans.data (child of a transaction)");
    }
}

// =============================================================================
// Logs and standalone spans, through their processors
// =============================================================================

#[actix_web::test]
async fn logs_are_masked_in_body_and_filtered_in_attributes() {
    let db = TestDb::new().await;
    let project = project(&db.pool, "Scrub logs").await;
    let container = json!({ "items": [{
        "timestamp": Utc::now().timestamp() as f64,
        "trace_id": "a".repeat(32),
        "level": "info",
        "body": "usuario ana@example.com autenticado, cpf 52998224725",
        "attributes": {
            "token": { "value": "Bearer x", "type": "string" },
            "user.id": { "value": "u-1", "type": "string" }
        }
    }]});
    LogsProcessor
        .process(
            bytes::Bytes::from(serde_json::to_vec(&container).unwrap()),
            &ctx(&db.pool, project.id),
        )
        .await
        .expect("logs stored");

    let (logs, total) =
        LogService::list_offset(&db.pool, project.id, 1, 10, &LogFilters::default())
            .await
            .unwrap();
    assert_eq!(total, 1);
    assert_eq!(logs[0].body, "usuario [email] autenticado, cpf [cpf]");
    assert_eq!(logs[0].attributes["token"], FILTERED);
    assert_eq!(logs[0].attributes["user.id"]["value"], "u-1");
}

#[actix_web::test]
async fn a_standalone_span_is_scrubbed() {
    let db = TestDb::new().await;
    let project = project(&db.pool, "Scrub spans").await;
    let now = Utc::now().timestamp();
    let span = json!({
        "span_id": "c".repeat(16), "trace_id": "a".repeat(32),
        "op": "http.client", "description": "POST /login user=ana@example.com",
        "start_timestamp": now - 1, "timestamp": now,
        "data": { "senha": "hunter2", "http.method": "POST" }
    });
    SpanProcessor
        .process(
            bytes::Bytes::from(serde_json::to_vec(&span).unwrap()),
            &ctx(&db.pool, project.id),
        )
        .await
        .expect("span stored");

    let spans: Vec<(Value,)> = sqlx::query_as(SPAN_DATA)
        .bind(project.id)
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(spans.len(), 1);
    let (data,) = &spans[0];
    assert_clean(data, "spans.data");
    // The row keeps the whole span JSON; the SDK's `data` bag is nested in it.
    assert_eq!(data["data"]["senha"], FILTERED);
    assert_eq!(data["data"]["http.method"], "POST");
    assert_eq!(data["description"], "POST /login user=[email]");
}

fn test_config() -> rustrak::config::Config {
    use rustrak::config::{Config, DashboardConfig, DatabaseConfig, SecurityConfig};
    use std::time::Duration;
    Config {
        host: "127.0.0.1".to_string(),
        port: 0,
        database: DatabaseConfig {
            url: "postgres://test:test@localhost/test".to_string(),
            max_connections: 5,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(60),
            max_lifetime: Duration::from_secs(300),
        },
        rate_limit: RateLimitConfig::default(),
        security: SecurityConfig {
            ssl_proxy: false,
            session_secret_key: None,
        },
        ingest_dir: None,
        public_url: None,
        sourcemap_storage_path: "/tmp/test_sourcemaps".to_string(),
        sourcemap_cache_bytes: 64 * 1024 * 1024,
        max_chunk_size_bytes: 10 * 1024 * 1024,
        session_flush_interval_secs: 30,
        session_cardinality_cap: 10_000,
        dashboard: DashboardConfig {
            dir: "./static".to_string(),
            enabled: true,
            url: None,
        },
    }
}

#[allow(dead_code)]
fn unused_processor_import_guard(_: TransactionProcessor) {}
