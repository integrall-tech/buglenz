//! Erasure for one data subject (BugLenz, ADR-0009): every event and
//! transaction a project holds for a `user.id`, with the issue and project
//! counters kept in step the way the storage cleanup keeps them.
//!
//! Issues left with no events are not deleted: they stay as history that
//! carries no personal data.

use serde::Serialize;

use crate::db::DbPool;
use crate::error::AppResult;
use crate::services::ProjectService;

/// How many rows one transaction takes, so ingestion keeps flowing in between.
const BATCH: i64 = 500;

#[cfg(feature = "postgres")]
const USER_ID: &str = "data->'user'->>'id'";
#[cfg(not(feature = "postgres"))]
const USER_ID: &str = "json_extract(data, '$.user.id')";

/// What one erasure removed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Erasure {
    pub events: i64,
    pub transactions: i64,
}

pub struct PrivacyService;

impl PrivacyService {
    /// Deletes everything the project stores for `user_id`. A project that
    /// does not exist is `NotFound`; a user with nothing stored is a zero.
    pub async fn erase_user(pool: &DbPool, project_id: i32, user_id: &str) -> AppResult<Erasure> {
        ProjectService::get_by_id(pool, project_id).await?;
        let mut erasure = Erasure::default();
        while let Some(n) = Self::delete_event_batch(pool, project_id, user_id).await? {
            erasure.events += n;
        }
        erasure.transactions = Self::delete_transactions(pool, project_id, user_id).await?;
        log::info!(
            "privacy: erased {} events and {} transactions of one user in project {}",
            erasure.events,
            erasure.transactions,
            project_id
        );
        Ok(erasure)
    }

    /// One batch of the user's events, with the issues' and the project's
    /// counters reduced in the same commit. `None` once nothing is left.
    async fn delete_event_batch(
        pool: &DbPool,
        project_id: i32,
        user_id: &str,
    ) -> AppResult<Option<i64>> {
        let mut tx = crate::db::begin_write(pool).await?;
        sqlx::query(
            "CREATE TEMP TABLE privacy_batch AS SELECT id, issue_id FROM events WHERE 1 = 0",
        )
        .execute(&mut *tx)
        .await?;
        let pick = format!(
            "INSERT INTO privacy_batch \
             SELECT id, issue_id FROM events WHERE project_id = $1 AND {USER_ID} = $2 LIMIT $3"
        );
        let picked = sqlx::query(sqlx::AssertSqlSafe(&*pick))
            .bind(project_id)
            .bind(user_id)
            .bind(BATCH)
            .execute(&mut *tx)
            .await?
            .rows_affected();
        if picked == 0 {
            tx.rollback().await?;
            return Ok(None);
        }

        sqlx::query(
            r#"
            UPDATE issues SET
                stored_event_count = stored_event_count - b.n,
                digested_event_count = digested_event_count - b.n
            FROM (
                SELECT issue_id, COUNT(*) AS n FROM privacy_batch
                WHERE issue_id IS NOT NULL GROUP BY issue_id
            ) AS b
            WHERE issues.id = b.issue_id
            "#,
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            r#"
            UPDATE projects SET
                stored_event_count = stored_event_count - b.n,
                digested_event_count = digested_event_count - b.n
            FROM (
                SELECT COUNT(*) AS n FROM privacy_batch c
                JOIN issues i ON i.id = c.issue_id
            ) AS b
            WHERE projects.id = $1
            "#,
        )
        .bind(project_id)
        .execute(&mut *tx)
        .await?;
        let deleted = sqlx::query("DELETE FROM events WHERE id IN (SELECT id FROM privacy_batch)")
            .execute(&mut *tx)
            .await?
            .rows_affected() as i64;
        sqlx::query("DROP TABLE privacy_batch")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(Some(deleted))
    }

    /// The user's transactions and their spans. Transactions keep no
    /// per-project counter, so this is a plain delete.
    async fn delete_transactions(pool: &DbPool, project_id: i32, user_id: &str) -> AppResult<i64> {
        let mut tx = crate::db::begin_write(pool).await?;
        sqlx::query("CREATE TEMP TABLE privacy_txn AS SELECT id FROM transactions WHERE 1 = 0")
            .execute(&mut *tx)
            .await?;
        let pick = format!(
            "INSERT INTO privacy_txn \
             SELECT id FROM transactions WHERE project_id = $1 AND {USER_ID} = $2"
        );
        sqlx::query(sqlx::AssertSqlSafe(&*pick))
            .bind(project_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM spans WHERE transaction_id IN (SELECT id FROM privacy_txn)")
            .execute(&mut *tx)
            .await?;
        let deleted =
            sqlx::query("DELETE FROM transactions WHERE id IN (SELECT id FROM privacy_txn)")
                .execute(&mut *tx)
                .await?
                .rows_affected() as i64;
        sqlx::query("DROP TABLE privacy_txn")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(deleted)
    }
}
