//! Gap G24: a session reported more than once is counted once, through the aggregator and the
//! database. `sentry-spring-boot` 8.60.0 sends its final state twice.

use crate::common::TestDb;
use rustrak::models::session::SessionUpdate;
use rustrak::models::CreateProject;
use rustrak::services::ProjectService;
use rustrak::workers::session_aggregator::SessionAggregator;
use serde_json::json;

fn update(init: bool, status: &str, errors: i64) -> SessionUpdate {
    serde_json::from_value(json!({
        "sid": "5ad6c89cc0874427b57feb17e61f9330",
        "init": init,
        "started": "2026-10-08T14:35:00.984Z",
        "status": status,
        "errors": errors,
        "attrs": { "release": "probe-api@1.0.0", "environment": "homolog" }
    }))
    .unwrap()
}

async fn totals(pool: &rustrak::db::DbPool, project_id: i32) -> (i64, i64, i64, i64) {
    #[cfg(feature = "postgres")]
    const Q: &str = "SELECT COALESCE(SUM(total),0)::BIGINT, COALESCE(SUM(errored),0)::BIGINT, COALESCE(SUM(crashed),0)::BIGINT, COALESCE(SUM(abnormal),0)::BIGINT FROM session_counts WHERE project_id = $1";
    #[cfg(not(feature = "postgres"))]
    const Q: &str = "SELECT COALESCE(SUM(total),0), COALESCE(SUM(errored),0), COALESCE(SUM(crashed),0), COALESCE(SUM(abnormal),0) FROM session_counts WHERE project_id = $1";
    sqlx::query_as(Q)
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[actix_web::test]
async fn the_java_sequence_is_one_crashed_session_not_two() {
    let db = TestDb::new().await;
    let project = ProjectService::create(
        &db.pool,
        CreateProject {
            name: "dedupe".into(),
            slug: None,
            platform: None,
        },
    )
    .await
    .unwrap();
    let aggregator = SessionAggregator::new(db.pool.clone(), 30, 10_000);

    // init; the first error; the crash; the same crash again when the session ends.
    for u in [
        update(true, "ok", 0),
        update(false, "ok", 1),
        update(false, "crashed", 2),
        update(false, "crashed", 2),
    ] {
        aggregator.ingest_session(project.id, &u).await;
    }
    aggregator.flush().await.unwrap();

    assert_eq!(
        totals(&db.pool, project.id).await,
        (1, 0, 1, 0),
        "total 1, crashed 1 (was 2, healthy -1)"
    );
}

#[actix_web::test]
async fn a_correction_after_a_flush_moves_the_session_between_counters() {
    let db = TestDb::new().await;
    let project = ProjectService::create(
        &db.pool,
        CreateProject {
            name: "correction".into(),
            slug: None,
            platform: None,
        },
    )
    .await
    .unwrap();
    let aggregator = SessionAggregator::new(db.pool.clone(), 30, 10_000);

    aggregator
        .ingest_session(project.id, &update(true, "ok", 0))
        .await;
    aggregator
        .ingest_session(project.id, &update(false, "errored", 1))
        .await;
    aggregator.flush().await.unwrap();
    assert_eq!(totals(&db.pool, project.id).await, (1, 1, 0, 0));

    aggregator
        .ingest_session(project.id, &update(false, "crashed", 1))
        .await;
    aggregator.flush().await.unwrap();
    assert_eq!(
        totals(&db.pool, project.id).await,
        (1, 0, 1, 0),
        "errored became crashed, the session was not counted twice"
    );
}
