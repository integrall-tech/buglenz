//! Retention periods per project and data type (BugLenz, ADR-0009, invariant I5).
//!
//! A period is a number of days, 7 to 3650. The floor is deliberate: the first pass runs a minute
//! after every start, so a period typed too small (`1`) would delete almost everything at once;
//! the manual cleanup keeps its own, lower, minimum. The instance sets defaults through three variables
//! with no built-in value (the numbers are a decision for whoever answers for LGPD); a project may
//! override each type. Spans follow their transactions.

use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::FromRow;

use crate::db::DbPool;
use crate::error::{AppError, AppResult, FieldErrorCode};

/// The smallest period the automatic retention accepts, in the variables and per project.
pub const MIN_DAYS: i32 = 7;
pub const MAX_DAYS: i32 = 3650;

pub const EVENTS_VAR: &str = "RUSTRAK_RETENTION_EVENTS_DAYS";
pub const TRANSACTIONS_VAR: &str = "RUSTRAK_RETENTION_TRANSACTIONS_DAYS";
pub const LOGS_VAR: &str = "RUSTRAK_RETENTION_LOGS_DAYS";
pub const INTERVAL_VAR: &str = "RUSTRAK_RETENTION_INTERVAL_HOURS";

/// The instance's default period for each type; `None` means the instance has none.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RetentionDefaults {
    pub events_days: Option<i32>,
    pub transactions_days: Option<i32>,
    pub logs_days: Option<i32>,
}

/// A value that is not a whole number of days in range is not a default: a typo must not become
/// "delete everything" or a silently huge window.
fn parse_days(raw: Option<&str>) -> Option<i32> {
    let n: i32 = raw?.trim().parse().ok()?;
    (MIN_DAYS..=MAX_DAYS).contains(&n).then_some(n)
}

impl RetentionDefaults {
    pub fn parse(events: Option<&str>, transactions: Option<&str>, logs: Option<&str>) -> Self {
        Self {
            events_days: parse_days(events),
            transactions_days: parse_days(transactions),
            logs_days: parse_days(logs),
        }
    }

    pub fn from_env() -> Self {
        let read = |name: &str| {
            let raw = std::env::var(name).ok();
            if let Some(value) = raw.as_deref().filter(|v| !v.trim().is_empty()) {
                if parse_days(Some(value)).is_none() {
                    log::error!(
                        "{name}={value:?} is not a number of days between {MIN_DAYS} and {MAX_DAYS}; ignored"
                    );
                }
            }
            raw
        };
        Self::parse(
            read(EVENTS_VAR).as_deref(),
            read(TRANSACTIONS_VAR).as_deref(),
            read(LOGS_VAR).as_deref(),
        )
    }
}

/// A project's own periods; `None` falls back to the instance default.
#[derive(Debug, Clone, Copy, Default, Serialize, FromRow, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ProjectRetention {
    pub events_days: Option<i32>,
    pub transactions_days: Option<i32>,
    pub logs_days: Option<i32>,
}

/// What actually applies to a project.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Effective {
    pub events_days: Option<i32>,
    pub transactions_days: Option<i32>,
    pub logs_days: Option<i32>,
}

impl Effective {
    pub fn resolve(defaults: &RetentionDefaults, own: &ProjectRetention) -> Self {
        Self {
            events_days: own.events_days.or(defaults.events_days),
            transactions_days: own.transactions_days.or(defaults.transactions_days),
            logs_days: own.logs_days.or(defaults.logs_days),
        }
    }

    /// The types that have no period at all.
    pub fn missing(&self) -> Vec<&'static str> {
        [
            ("events", self.events_days),
            ("transactions", self.transactions_days),
            ("logs", self.logs_days),
        ]
        .into_iter()
        .filter(|(_, days)| days.is_none())
        .map(|(name, _)| name)
        .collect()
    }

    /// Every type has a period (invariant I5).
    pub fn is_protected(&self) -> bool {
        self.missing().is_empty()
    }
}

/// `Some(Some(n))` sets a period, `Some(None)` clears it, `None` leaves it as it is.
fn double_option<'de, D>(d: D) -> Result<Option<Option<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(d)?))
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RetentionUpdate {
    #[serde(default, deserialize_with = "double_option")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<i32>, minimum = 7, maximum = 3650))]
    pub events_days: Option<Option<i32>>,
    #[serde(default, deserialize_with = "double_option")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<i32>, minimum = 7, maximum = 3650))]
    pub transactions_days: Option<Option<i32>>,
    #[serde(default, deserialize_with = "double_option")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<i32>, minimum = 7, maximum = 3650))]
    pub logs_days: Option<Option<i32>>,
}

pub fn validate_days(field: &str, days: i32) -> AppResult<()> {
    if (MIN_DAYS..=MAX_DAYS).contains(&days) {
        return Ok(());
    }
    Err(
        AppError::Validation(format!("{field} must be between {MIN_DAYS} and {MAX_DAYS}"))
            .with_field(field, FieldErrorCode::Invalid),
    )
}

/// One project with its own periods.
#[derive(Debug, FromRow)]
pub struct ProjectPolicy {
    pub project_id: i32,
    pub name: String,
    pub events_days: Option<i32>,
    pub transactions_days: Option<i32>,
    pub logs_days: Option<i32>,
}

impl ProjectPolicy {
    pub fn own(&self) -> ProjectRetention {
        ProjectRetention {
            events_days: self.events_days,
            transactions_days: self.transactions_days,
            logs_days: self.logs_days,
        }
    }
}

/// What a pass removed besides the three data types: the release-health rows and the alert history
/// of a project, which follow its `events` period (audit of 2026-10-09, invariant I5).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SessionPurge {
    pub session_counts: i64,
    pub session_users: i64,
    pub alert_history: i64,
}

pub struct RetentionService;

impl RetentionService {
    pub async fn get(pool: &DbPool, project_id: i32) -> AppResult<ProjectRetention> {
        let row: Option<ProjectRetention> = sqlx::query_as(
            "SELECT events_days, transactions_days, logs_days FROM project_retention WHERE project_id = $1",
        )
        .bind(project_id)
        .fetch_optional(pool)
        .await?;
        Ok(row.unwrap_or_default())
    }

    /// Applies an update. Everything is validated before anything is written.
    pub async fn set(
        pool: &DbPool,
        project_id: i32,
        update: &RetentionUpdate,
    ) -> AppResult<ProjectRetention> {
        for (field, value) in [
            ("events_days", update.events_days),
            ("transactions_days", update.transactions_days),
            ("logs_days", update.logs_days),
        ] {
            if let Some(Some(days)) = value {
                validate_days(field, days)?;
            }
        }

        let current = Self::get(pool, project_id).await?;
        let merged = ProjectRetention {
            events_days: update.events_days.unwrap_or(current.events_days),
            transactions_days: update
                .transactions_days
                .unwrap_or(current.transactions_days),
            logs_days: update.logs_days.unwrap_or(current.logs_days),
        };

        if merged == ProjectRetention::default() {
            sqlx::query("DELETE FROM project_retention WHERE project_id = $1")
                .bind(project_id)
                .execute(pool)
                .await?;
            return Ok(merged);
        }

        sqlx::query(
            "INSERT INTO project_retention (project_id, events_days, transactions_days, logs_days, updated_at) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (project_id) DO UPDATE SET events_days = excluded.events_days, \
             transactions_days = excluded.transactions_days, logs_days = excluded.logs_days, \
             updated_at = excluded.updated_at",
        )
        .bind(project_id)
        .bind(merged.events_days)
        .bind(merged.transactions_days)
        .bind(merged.logs_days)
        .bind(Utc::now())
        .execute(pool)
        .await?;
        Ok(merged)
    }

    /// Every project with its own periods (none where it has no row).
    pub async fn all(pool: &DbPool) -> AppResult<Vec<ProjectPolicy>> {
        Ok(sqlx::query_as(
            "SELECT p.id AS project_id, p.name AS name, r.events_days AS events_days, \
             r.transactions_days AS transactions_days, r.logs_days AS logs_days \
             FROM projects p LEFT JOIN project_retention r ON r.project_id = p.id ORDER BY p.id",
        )
        .fetch_all(pool)
        .await?)
    }

    /// Deletes a project's release-health rows (`session_counts`, `session_users`) and alert history
    /// older than `days`. They carry no period of their own: they follow `events_days`.
    pub async fn purge_session_data(
        pool: &DbPool,
        project_id: i32,
        days: i32,
    ) -> AppResult<SessionPurge> {
        let cutoff = Utc::now() - chrono::Duration::days(i64::from(days));

        #[cfg(feature = "postgres")]
        let (bucket, day, created) = (cutoff, cutoff.date_naive(), cutoff);
        // SQLite keeps these as text, in the formats the writers use.
        #[cfg(not(feature = "postgres"))]
        let (bucket, day, created) = (
            cutoff.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
            cutoff.date_naive().to_string(),
            cutoff.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
        );

        let session_counts =
            sqlx::query("DELETE FROM session_counts WHERE project_id = $1 AND bucket < $2")
                .bind(project_id)
                .bind(bucket)
                .execute(pool)
                .await?
                .rows_affected() as i64;
        let session_users =
            sqlx::query("DELETE FROM session_users WHERE project_id = $1 AND day < $2")
                .bind(project_id)
                .bind(day)
                .execute(pool)
                .await?
                .rows_affected() as i64;
        #[cfg(feature = "postgres")]
        const ALERTS: &str = "DELETE FROM alert_history WHERE project_id = $1 AND created_at < $2";
        #[cfg(not(feature = "postgres"))]
        const ALERTS: &str =
            "DELETE FROM alert_history WHERE project_id = $1 AND datetime(created_at) < datetime($2)";
        let alert_history = sqlx::query(ALERTS)
            .bind(project_id)
            .bind(created)
            .execute(pool)
            .await?
            .rows_affected() as i64;
        Ok(SessionPurge {
            session_counts,
            session_users,
            alert_history,
        })
    }
}
