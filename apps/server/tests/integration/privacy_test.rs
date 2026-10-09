//! Erasure for one data subject (ADR-0009): the user's events and
//! transactions go, everyone else's stay, and the counters agree.

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, test, web, App};
use chrono::Utc;
use serde_json::{json, Value};
use tempfile::TempDir;
use uuid::Uuid;

use crate::common::TestDb;
use rustrak::config::RateLimitConfig;
use rustrak::db::DbPool;
use rustrak::digest::processors::{Processor, ProcessorCtx, Processors, TransactionProcessor};
use rustrak::ingest::{store_event, EventMetadata};
use rustrak::models::{CreateAuthToken, CreateProject, CreateUserRequest, UserRole};
use rustrak::routes;
use rustrak::routes::ingest::digest_stored_event;
use rustrak::services::privacy::PrivacyService;
use rustrak::services::{AuthTokenService, ProjectService, UsersService};

async fn project(pool: &DbPool, name: &str) -> i32 {
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
    .id
}

async fn digest_event(pool: &DbPool, dir: &TempDir, project_id: i32, user: &str, kind: &str) {
    let processors = Processors::new(
        dir.path().to_path_buf(),
        RateLimitConfig::default(),
        crate::common::null_sourcemap_provider(),
        None,
    );
    let event_id = Uuid::new_v4().simple().to_string();
    let event = json!({
        "event_id": event_id,
        "timestamp": Utc::now().timestamp() as f64,
        "platform": "javascript",
        "user": { "id": user },
        "exception": { "values": [{ "type": kind, "value": "boom" }] }
    });
    store_event(dir.path(), &event_id, &serde_json::to_vec(&event).unwrap())
        .await
        .unwrap();
    let metadata = EventMetadata {
        event_id,
        project_id,
        ingested_at: Utc::now(),
        remote_addr: None,
    };
    digest_stored_event(&processors, pool, &metadata).await;
}

async fn store_transaction(pool: &DbPool, project_id: i32, user: &str) {
    let now = Utc::now().timestamp();
    let payload = json!({
        "event_id": Uuid::new_v4().simple().to_string(),
        "type": "transaction",
        "transaction": "GET /x",
        "timestamp": now,
        "start_timestamp": now - 1,
        "platform": "javascript",
        "user": { "id": user },
        "contexts": { "trace": { "trace_id": "a".repeat(32), "span_id": "b".repeat(16), "op": "http.server" } },
        "spans": [{ "span_id": "c".repeat(16), "trace_id": "a".repeat(32), "parent_span_id": "b".repeat(16),
                    "op": "db", "start_timestamp": now - 1, "timestamp": now }]
    });
    TransactionProcessor
        .process(
            bytes::Bytes::from(serde_json::to_vec(&payload).unwrap()),
            &ProcessorCtx {
                pool: pool.clone(),
                project_id,
                event_id: Uuid::new_v4(),
                ingested_at: Utc::now(),
                remote_addr: None,
            },
        )
        .await
        .expect("transaction");
}

#[cfg(feature = "postgres")]
const COUNTS: &str = "SELECT \
    (SELECT COUNT(*) FROM events WHERE project_id = $1), \
    (SELECT COALESCE(SUM(stored_event_count), 0) FROM issues WHERE project_id = $1), \
    (SELECT CAST(stored_event_count AS BIGINT) FROM projects WHERE id = $1), \
    (SELECT COUNT(*) FROM transactions WHERE project_id = $1), \
    (SELECT COUNT(*) FROM spans WHERE project_id = $1)";
#[cfg(not(feature = "postgres"))]
const COUNTS: &str = "SELECT \
    (SELECT COUNT(*) FROM events WHERE project_id = ?1), \
    (SELECT COALESCE(SUM(stored_event_count), 0) FROM issues WHERE project_id = ?1), \
    (SELECT CAST(stored_event_count AS INTEGER) FROM projects WHERE id = ?1), \
    (SELECT COUNT(*) FROM transactions WHERE project_id = ?1), \
    (SELECT COUNT(*) FROM spans WHERE project_id = ?1)";

async fn counts(pool: &DbPool, project_id: i32) -> (i64, i64, i64, i64, i64) {
    sqlx::query_as(COUNTS)
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[actix_web::test]
async fn erasing_a_user_removes_only_theirs_and_keeps_the_counters_right() {
    let db = TestDb::new().await;
    let dir = TempDir::new().unwrap();
    let project_id = project(&db.pool, "Privacy").await;
    // u-1: two events in issue A, one in issue B; u-2: one in issue A.
    digest_event(&db.pool, &dir, project_id, "u-1", "TypeError").await;
    digest_event(&db.pool, &dir, project_id, "u-1", "TypeError").await;
    digest_event(&db.pool, &dir, project_id, "u-1", "RangeError").await;
    digest_event(&db.pool, &dir, project_id, "u-2", "TypeError").await;
    store_transaction(&db.pool, project_id, "u-1").await;
    store_transaction(&db.pool, project_id, "u-2").await;
    assert_eq!(counts(&db.pool, project_id).await, (4, 4, 4, 2, 2));

    let erased = PrivacyService::erase_user(&db.pool, project_id, "u-1")
        .await
        .expect("erase");
    assert_eq!((erased.events, erased.transactions), (3, 1));

    let (events, issue_sum, project_count, transactions, spans) =
        counts(&db.pool, project_id).await;
    assert_eq!(events, 1, "u-2's event stays");
    assert_eq!(issue_sum, 1, "issue counters follow the events");
    assert_eq!(project_count, 1, "the project counter follows too");
    assert_eq!(transactions, 1, "u-2's transaction stays");
    assert_eq!(spans, 1, "u-1's child span went with its transaction");

    let again = PrivacyService::erase_user(&db.pool, project_id, "u-1")
        .await
        .unwrap();
    assert_eq!((again.events, again.transactions), (0, 0), "idempotent");
}

#[actix_web::test]
async fn a_missing_project_is_not_found() {
    let db = TestDb::new().await;
    let err = PrivacyService::erase_user(&db.pool, 999_999, "u-1")
        .await
        .expect_err("no project");
    assert!(
        matches!(err, rustrak::error::AppError::NotFound(_)),
        "{err:?}"
    );
}

async fn token_for(pool: &DbPool, role: UserRole, email: &str) -> String {
    let user = UsersService::create_user(
        pool,
        &CreateUserRequest {
            email: email.to_string(),
            password: "password123".to_string(),
        },
        role,
    )
    .await
    .expect("user");
    AuthTokenService::create_for_user(pool, CreateAuthToken { description: None }, Some(user.id))
        .await
        .expect("token")
        .token
}

#[actix_web::test]
async fn the_route_is_for_admins_and_answers_the_erasure() {
    let db = TestDb::new().await;
    let dir = TempDir::new().unwrap();
    let project_id = project(&db.pool, "Privacy route").await;
    digest_event(&db.pool, &dir, project_id, "u-1", "TypeError").await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.pool.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0u8; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .configure(routes::privacy::configure),
    )
    .await;

    let member = token_for(&db.pool, UserRole::Member, "member@x.com").await;
    let req = test::TestRequest::delete()
        .uri(&format!("/api/projects/{project_id}/privacy/users/u-1"))
        .insert_header(("Authorization", format!("Bearer {member}")))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), 403);

    let admin = token_for(&db.pool, UserRole::Admin, "admin@x.com").await;
    let req = test::TestRequest::delete()
        .uri(&format!("/api/projects/{project_id}/privacy/users/u-1"))
        .insert_header(("Authorization", format!("Bearer {admin}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({ "events": 1, "transactions": 0, "sessions": 0 })
    );

    let req = test::TestRequest::delete()
        .uri("/api/projects/999999/privacy/users/u-1")
        .insert_header(("Authorization", format!("Bearer {admin}")))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), 404);
}

// `day` is a `NaiveDate` (Copy) on PostgreSQL and a `String` on SQLite: the clone is needed on one.
#[allow(clippy::clone_on_copy)]
#[actix_web::test]
async fn erasing_a_subject_also_removes_their_session_rows() {
    use rustrak::scrub::pseudonym::pseudonym;

    let db = TestDb::new().await;
    let p = project(&db.pool, "erase-sessions").await;
    let q = project(&db.pool, "other-project").await;
    let today = Utc::now();
    #[cfg(feature = "postgres")]
    let day = today.date_naive();
    #[cfg(not(feature = "postgres"))]
    let day = today.date_naive().to_string();
    // The subject as a pseudonym (what the server writes now), as the raw id (rows from before
    // the pseudonym), someone else, and the same subject in another project.
    for (project_id, did) in [
        (p, pseudonym("u-42")),
        (p, "u-42".to_string()),
        (p, pseudonym("u-43")),
        (q, pseudonym("u-42")),
    ] {
        sqlx::query("INSERT INTO session_users (project_id, release, environment, day, did) VALUES ($1, 'r@1', 'e', $2, $3)")
            .bind(project_id).bind(day.clone()).bind(did).execute(&db.pool).await.unwrap();
    }

    let erasure = PrivacyService::erase_user(&db.pool, p, "u-42")
        .await
        .unwrap();

    assert_eq!(
        erasure.sessions, 2,
        "the pseudonym and the raw id of the subject"
    );
    let left: Vec<String> =
        sqlx::query_scalar("SELECT did FROM session_users WHERE project_id = $1")
            .bind(p)
            .fetch_all(&db.pool)
            .await
            .unwrap();
    assert_eq!(
        left,
        vec![pseudonym("u-43")],
        "only the other user remains in the project"
    );
    let other: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM session_users WHERE project_id = $1")
        .bind(q)
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(other, 1, "the other project is untouched");
}
