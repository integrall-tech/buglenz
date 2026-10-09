//! Package 004: the retention worker and the admin API (ADR-0009, invariant I5).

use crate::common::TestDb;
use actix_web::{test, web, App};
use chrono::{DateTime, Duration, Utc};
use rustrak::db::DbPool;
use rustrak::models::{CreateAuthToken, CreateProject, CreateUserRequest, UserRole};
use rustrak::routes;
use rustrak::services::retention::{RetentionDefaults, RetentionService, RetentionUpdate};
use rustrak::services::{AuthTokenService, ProjectService, UsersService};
use rustrak::workers::retention::{run_once, RetentionState};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicI32, Ordering};
use uuid::Uuid;

static ORDER: AtomicI32 = AtomicI32::new(1_000_000);

async fn project(pool: &DbPool, name: &str) -> i32 {
    ProjectService::create(
        pool,
        CreateProject {
            name: name.into(),
            slug: None,
            platform: None,
        },
    )
    .await
    .unwrap()
    .id
}

async fn event_at(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    let issue_id = Uuid::new_v4();
    let order = ORDER.fetch_add(1, Ordering::Relaxed);
    sqlx::query("INSERT INTO issues (id, project_id, digest_order, first_seen, last_seen) VALUES ($1, $2, $3, $4, $5)")
        .bind(issue_id).bind(project_id).bind(order).bind(at).bind(at).execute(pool).await.unwrap();
    let grouping_id: i32 = sqlx::query_scalar(
        "INSERT INTO groupings (project_id, issue_id, grouping_key, grouping_key_hash) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(project_id).bind(issue_id).bind(format!("key-{order}")).bind(format!("{order:0>64}"))
    .fetch_one(pool).await.unwrap();
    sqlx::query(
        "INSERT INTO events (id, event_id, project_id, issue_id, grouping_id, data, timestamp, ingested_at, digested_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(Uuid::new_v4()).bind(Uuid::new_v4()).bind(project_id).bind(issue_id).bind(grouping_id)
    .bind(json!({})).bind(at).bind(at).bind(at).execute(pool).await.unwrap();
}

async fn transaction_at(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    sqlx::query("INSERT INTO transactions (id, event_id, project_id, timestamp, ingested_at, data) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(Uuid::new_v4()).bind(Uuid::new_v4()).bind(project_id).bind(at).bind(at).bind(json!({}))
        .execute(pool).await.unwrap();
}

async fn log_at(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    sqlx::query(
        "INSERT INTO logs (id, project_id, trace_id, span_id, level, severity_number, body, attributes, timestamp, ingested_at) \
         VALUES ($1, $2, $3, NULL, $4, $5, $6, $7, $8, $9)",
    )
    .bind(Uuid::new_v4()).bind(project_id).bind("trace").bind("info").bind(9_i16).bind("hello")
    .bind(json!({})).bind(at).bind(at).execute(pool).await.unwrap();
}

async fn count(pool: &DbPool, table: &str, project_id: i32) -> i64 {
    let sql = match table {
        "events" => "SELECT COUNT(*) FROM events WHERE project_id = $1",
        "transactions" => "SELECT COUNT(*) FROM transactions WHERE project_id = $1",
        "logs" => "SELECT COUNT(*) FROM logs WHERE project_id = $1",
        other => panic!("unknown table {other}"),
    };
    sqlx::query_scalar(sql)
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

fn state(events: Option<&str>, transactions: Option<&str>, logs: Option<&str>) -> RetentionState {
    RetentionState::new(
        RetentionDefaults::parse(events, transactions, logs),
        std::time::Duration::from_secs(86_400),
    )
}

#[actix_web::test]
async fn the_pass_removes_what_is_older_than_the_period_and_keeps_the_rest() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "own-period").await;
    let (old, recent) = (
        Utc::now() - Duration::days(40),
        Utc::now() - Duration::days(5),
    );
    event_at(&db.pool, p, old).await;
    event_at(&db.pool, p, recent).await;
    transaction_at(&db.pool, p, old).await;
    log_at(&db.pool, p, old).await;

    RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(Some(30)),
            transactions_days: Some(Some(30)),
            logs_days: Some(Some(30)),
        },
    )
    .await
    .unwrap();
    let report = run_once(&db.pool, &state(None, None, None)).await;

    assert_eq!(
        count(&db.pool, "events", p).await,
        1,
        "the 5-day-old event stays"
    );
    assert_eq!(count(&db.pool, "transactions", p).await, 0);
    assert_eq!(count(&db.pool, "logs", p).await, 0);
    assert_eq!(
        (
            report.removed.events,
            report.removed.transactions,
            report.removed.logs
        ),
        (1, 1, 1)
    );
    assert!(report.unprotected.is_empty());
}

#[actix_web::test]
async fn the_instance_default_applies_to_a_project_without_its_own_period() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "uses-default").await;
    transaction_at(&db.pool, p, Utc::now() - Duration::days(20)).await;
    transaction_at(&db.pool, p, Utc::now() - Duration::days(2)).await;

    run_once(&db.pool, &state(Some("90"), Some("14"), Some("90"))).await;
    assert_eq!(count(&db.pool, "transactions", p).await, 1);
}

#[actix_web::test]
async fn a_project_without_an_effective_period_loses_nothing_and_is_reported() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "unprotected").await;
    let ancient = Utc::now() - Duration::days(400);
    event_at(&db.pool, p, ancient).await;
    log_at(&db.pool, p, ancient).await;

    // Only events have a period; logs and transactions have none.
    let report = run_once(&db.pool, &state(Some("30"), None, None)).await;
    assert_eq!(count(&db.pool, "events", p).await, 0, "events had a period");
    assert_eq!(
        count(&db.pool, "logs", p).await,
        1,
        "logs had none: untouched"
    );
    let entry = report
        .unprotected
        .iter()
        .find(|u| u.project_id == p)
        .expect("reported");
    assert_eq!(
        entry.missing,
        vec!["transactions".to_string(), "logs".to_string()]
    );

    // With nothing configured at all nothing is deleted.
    event_at(&db.pool, p, ancient).await;
    let report = run_once(&db.pool, &state(None, None, None)).await;
    assert_eq!(count(&db.pool, "events", p).await, 1);
    assert_eq!(report.removed.events, 0);
}

#[actix_web::test]
async fn setting_and_clearing_a_period_validates_and_persists() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "edit").await;

    let r = RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(Some(30)),
            transactions_days: None,
            logs_days: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        (r.events_days, r.transactions_days, r.logs_days),
        (Some(30), None, None)
    );
    let r = RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: None,
            transactions_days: Some(Some(7)),
            logs_days: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        (r.events_days, r.transactions_days),
        (Some(30), Some(7)),
        "absent fields stay"
    );
    let r = RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(None),
            transactions_days: None,
            logs_days: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        (r.events_days, r.transactions_days),
        (None, Some(7)),
        "null clears"
    );

    assert!(RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(Some(0)),
            transactions_days: None,
            logs_days: None
        }
    )
    .await
    .is_err());
    assert_eq!(
        RetentionService::get(&db.pool, p)
            .await
            .unwrap()
            .transactions_days,
        Some(7),
        "a refused update changes nothing"
    );
}

// ── API ──────────────────────────────────────────────────────────────────────

async fn user_token(pool: &DbPool, email: &str, role: UserRole) -> String {
    let u = UsersService::create_user(
        pool,
        &CreateUserRequest {
            email: email.into(),
            password: "password123".into(),
        },
        role,
    )
    .await
    .unwrap();
    AuthTokenService::create_for_user(pool, CreateAuthToken { description: None }, Some(u.id))
        .await
        .unwrap()
        .token
}

macro_rules! api {
    ($db:expr, $state:expr) => {
        test::init_service(
            App::new()
                .app_data(web::Data::new($db.pool.clone()))
                .app_data(web::Data::new(std::sync::Arc::new($state)))
                .configure(routes::retention::configure),
        )
        .await
    };
}

#[actix_web::test]
async fn the_api_is_for_administrators_only() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "api").await;
    let member = user_token(&db.pool, "member@x.com", UserRole::Member).await;
    let app = api!(db, state(Some("90"), Some("30"), Some("90")));

    let get = test::TestRequest::get()
        .uri("/api/retention")
        .insert_header(("Authorization", format!("Bearer {member}")))
        .to_request();
    assert_eq!(test::call_service(&app, get).await.status(), 403);
    let put = test::TestRequest::put()
        .uri(&format!("/api/projects/{p}/retention"))
        .insert_header(("Authorization", format!("Bearer {member}")))
        .set_json(json!({ "events_days": 5 }))
        .to_request();
    assert_eq!(test::call_service(&app, put).await.status(), 403);
    assert_eq!(count(&db.pool, "events", p).await, 0);
    assert_eq!(
        RetentionService::get(&db.pool, p)
            .await
            .unwrap()
            .events_days,
        None
    );
}

#[actix_web::test]
async fn an_administrator_reads_the_policy_and_edits_a_project() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "api-admin").await;
    let q = project(&db.pool, "api-other").await;
    let admin = user_token(&db.pool, "admin@x.com", UserRole::Admin).await;
    let app = api!(db, state(Some("90"), Some("30"), None));
    let auth = ("Authorization", format!("Bearer {admin}"));

    let put = test::TestRequest::put()
        .uri(&format!("/api/projects/{p}/retention"))
        .insert_header(auth.clone())
        .set_json(json!({ "events_days": 14, "logs_days": 60 }))
        .to_request();
    let resp = test::call_service(&app, put).await;
    assert_eq!(resp.status(), 200);

    let bad = test::TestRequest::put()
        .uri(&format!("/api/projects/{p}/retention"))
        .insert_header(auth.clone())
        .set_json(json!({ "events_days": 0 }))
        .to_request();
    assert_eq!(test::call_service(&app, bad).await.status(), 400);

    let missing = test::TestRequest::put()
        .uri("/api/projects/999999/retention")
        .insert_header(auth.clone())
        .set_json(json!({ "events_days": 5 }))
        .to_request();
    assert_eq!(test::call_service(&app, missing).await.status(), 404);

    let get = test::TestRequest::get()
        .uri("/api/retention")
        .insert_header(auth)
        .to_request();
    let body: Value = test::call_and_read_body_json(&app, get).await;
    assert_eq!(body["defaults"]["events_days"], 90);
    assert_eq!(body["defaults"]["logs_days"], Value::Null);
    let row = |id: i32| {
        body["projects"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["project_id"] == id)
            .unwrap()
            .clone()
    };
    assert_eq!(row(p)["own"]["events_days"], 14);
    assert_eq!(row(p)["effective"]["events_days"], 14);
    assert_eq!(
        row(p)["effective"]["transactions_days"],
        30,
        "default shows through"
    );
    assert_eq!(row(p)["protected"], true, "logs have their own period");
    assert_eq!(row(q)["protected"], false, "no period for logs");
    assert_eq!(row(q)["missing"], json!(["logs"]));
    assert_eq!(body["last_run"], Value::Null);
}

// ── sessions and alert history follow the events period (audit of 2026-10-09, invariant I5) ──

async fn seed_session_count(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    #[cfg(feature = "postgres")]
    let (bucket,) = (at,);
    #[cfg(not(feature = "postgres"))]
    let (bucket,) = (at.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),);
    sqlx::query("INSERT INTO session_counts (project_id, release, environment, bucket, total) VALUES ($1, 'r@1', 'e', $2, 1)")
        .bind(project_id).bind(bucket).execute(pool).await.unwrap();
}

async fn seed_session_user(pool: &DbPool, project_id: i32, at: DateTime<Utc>, did: &str) {
    #[cfg(feature = "postgres")]
    let day = at.date_naive();
    #[cfg(not(feature = "postgres"))]
    let day = at.date_naive().to_string();
    sqlx::query("INSERT INTO session_users (project_id, release, environment, day, did) VALUES ($1, 'r@1', 'e', $2, $3)")
        .bind(project_id).bind(day).bind(did).execute(pool).await.unwrap();
}

async fn seed_alert(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    sqlx::query(
        "INSERT INTO alert_history (project_id, alert_type, channel_type, channel_name, status, idempotency_key, created_at) \
         VALUES ($1, 'new_issue', 'webhook', 'w', 'sent', $2, $3)",
    )
    .bind(project_id).bind(Uuid::new_v4().to_string()).bind(at).execute(pool).await.unwrap();
}

async fn rows(pool: &DbPool, table: &str, project_id: i32) -> i64 {
    let sql = match table {
        "session_counts" => "SELECT COUNT(*) FROM session_counts WHERE project_id = $1",
        "session_users" => "SELECT COUNT(*) FROM session_users WHERE project_id = $1",
        "alert_history" => "SELECT COUNT(*) FROM alert_history WHERE project_id = $1",
        other => panic!("unknown table {other}"),
    };
    sqlx::query_scalar(sql)
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[actix_web::test]
async fn the_pass_also_removes_old_session_data_and_alert_history() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "sessions-retention").await;
    let (old, recent) = (
        Utc::now() - Duration::days(40),
        Utc::now() - Duration::days(5),
    );
    for at in [old, recent] {
        seed_session_count(&db.pool, p, at).await;
        seed_session_user(&db.pool, p, at, &format!("p1:{}", at.timestamp())).await;
        seed_alert(&db.pool, p, at).await;
    }

    RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(Some(30)),
            transactions_days: Some(Some(30)),
            logs_days: Some(Some(30)),
        },
    )
    .await
    .unwrap();
    let report = run_once(&db.pool, &state(None, None, None)).await;

    for table in ["session_counts", "session_users", "alert_history"] {
        assert_eq!(
            rows(&db.pool, table, p).await,
            1,
            "{table}: the old row goes, the recent one stays"
        );
    }
    assert_eq!((report.sessions_removed, report.alerts_removed), (2, 1));
}

#[actix_web::test]
async fn a_project_without_an_events_period_keeps_its_session_data() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "sessions-unprotected").await;
    let ancient = Utc::now() - Duration::days(400);
    seed_session_count(&db.pool, p, ancient).await;
    seed_session_user(&db.pool, p, ancient, "p1:x").await;
    seed_alert(&db.pool, p, ancient).await;

    run_once(&db.pool, &state(None, Some("30"), Some("30"))).await;

    for table in ["session_counts", "session_users", "alert_history"] {
        assert_eq!(
            rows(&db.pool, table, p).await,
            1,
            "{table}: no period for events, nothing is removed"
        );
    }
}

// ── user reports follow the events period (audit of 2026-10-09, invariant I5) ──

async fn seed_user_report(pool: &DbPool, project_id: i32, at: DateTime<Utc>) {
    sqlx::query(
        "INSERT INTO user_reports (id, project_id, name, email, comments, created_at) \
         VALUES ($1, $2, '', '[email]', 'c', $3)",
    )
    .bind(Uuid::new_v4())
    .bind(project_id)
    .bind(at)
    .execute(pool)
    .await
    .unwrap();
}

async fn user_reports(pool: &DbPool, project_id: i32) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM user_reports WHERE project_id = $1")
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[actix_web::test]
async fn the_pass_removes_old_user_reports() {
    let db = TestDb::new().await;
    let p = project(&db.pool, "reports-retention").await;
    seed_user_report(&db.pool, p, Utc::now() - Duration::days(40)).await;
    seed_user_report(&db.pool, p, Utc::now() - Duration::days(2)).await;

    RetentionService::set(
        &db.pool,
        p,
        &RetentionUpdate {
            events_days: Some(Some(30)),
            transactions_days: Some(Some(30)),
            logs_days: Some(Some(30)),
        },
    )
    .await
    .unwrap();
    let report = run_once(&db.pool, &state(None, None, None)).await;

    assert_eq!(user_reports(&db.pool, p).await, 1);
    assert_eq!(report.user_reports_removed, 1);
}
