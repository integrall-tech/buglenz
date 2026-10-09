//! The retention worker (BugLenz, ADR-0009, invariant I5): applies each project's effective
//! period with the same batched cleanup the Storage screen uses.
//!
//! A pass runs shortly after the server starts and then every `RUSTRAK_RETENTION_INTERVAL_HOURS`
//! (default 24). A project with no period for a type loses nothing of that type; the pass says so
//! at `WARN` and the admin API lists it as unprotected.

use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::db::DbPool;
use crate::models::{CleanupCounts, CleanupFilter};
use crate::services::retention::{Effective, RetentionDefaults, RetentionService, INTERVAL_VAR};
use crate::services::StorageService;

/// How long after startup the first pass waits, so a boot loop does not hammer the database.
const FIRST_PASS_DELAY: Duration = Duration::from_secs(60);

/// A project and the types it has no period for.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Unprotected {
    pub project_id: i32,
    pub missing: Vec<String>,
}

/// What one pass did.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RetentionReport {
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub projects: usize,
    pub removed: CleanupCounts,
    /// Release-health rows removed (`session_counts` and `session_users`), by the `events` period.
    pub sessions_removed: i64,
    /// Alert-history rows removed, by the `events` period.
    pub alerts_removed: i64,
    /// User feedback reports removed, by the `events` period.
    pub user_reports_removed: i64,
    pub unprotected: Vec<Unprotected>,
    /// Projects where a cleanup failed; the pass went on to the next one.
    pub failed: Vec<i32>,
}

/// What the worker and the admin API share.
pub struct RetentionState {
    pub defaults: RetentionDefaults,
    pub interval: Duration,
    last: Mutex<Option<RetentionReport>>,
}

impl RetentionState {
    pub fn new(defaults: RetentionDefaults, interval: Duration) -> Self {
        Self {
            defaults,
            interval,
            last: Mutex::new(None),
        }
    }

    /// Defaults and interval from the environment.
    pub fn from_env() -> Self {
        let hours = std::env::var(INTERVAL_VAR)
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|h| (1..=168).contains(h))
            .unwrap_or(24);
        Self::new(
            RetentionDefaults::from_env(),
            Duration::from_secs(hours * 3600),
        )
    }

    pub fn last_run(&self) -> Option<RetentionReport> {
        self.last
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    fn record(&self, report: &RetentionReport) {
        *self.last.lock().unwrap_or_else(PoisonError::into_inner) = Some(report.clone());
    }
}

fn add(total: &mut CleanupCounts, part: &CleanupCounts) {
    total.events += part.events;
    total.transactions += part.transactions;
    total.spans += part.spans;
    total.logs += part.logs;
    total.issues_removed += part.issues_removed;
}

/// One pass over every project.
pub async fn run_once(pool: &DbPool, state: &RetentionState) -> RetentionReport {
    let started_at = Utc::now();
    let mut removed = CleanupCounts::default();
    let mut sessions_removed = 0_i64;
    let mut alerts_removed = 0_i64;
    let mut user_reports_removed = 0_i64;
    let mut unprotected = Vec::new();
    let mut failed = Vec::new();

    let policies = match RetentionService::all(pool).await {
        Ok(policies) => policies,
        Err(e) => {
            log::error!("Retention: could not read the projects: {e}");
            Vec::new()
        }
    };

    for policy in &policies {
        let effective = Effective::resolve(&state.defaults, &policy.own());
        let missing = effective.missing();
        if !missing.is_empty() {
            log::warn!(
                "Retention: project {} ({}) has no period for {}; that data is not being removed",
                policy.project_id,
                policy.name,
                missing.join(", ")
            );
            unprotected.push(Unprotected {
                project_id: policy.project_id,
                missing: missing.iter().map(|m| (*m).to_string()).collect(),
            });
        }

        let steps = [
            (
                effective.events_days,
                CleanupFilter {
                    include_events: true,
                    include_transactions: false,
                    include_logs: false,
                },
            ),
            (
                effective.transactions_days,
                CleanupFilter {
                    include_events: false,
                    include_transactions: true,
                    include_logs: false,
                },
            ),
            (
                effective.logs_days,
                CleanupFilter {
                    include_events: false,
                    include_transactions: false,
                    include_logs: true,
                },
            ),
        ];
        for (days, filter) in steps {
            let Some(days) = days else { continue };
            match StorageService::execute_cleanup(
                pool,
                i64::from(days),
                Some(policy.project_id),
                filter,
            )
            .await
            {
                Ok(counts) => add(&mut removed, &counts),
                Err(e) => {
                    log::error!("Retention: project {} failed: {e}", policy.project_id);
                    if !failed.contains(&policy.project_id) {
                        failed.push(policy.project_id);
                    }
                }
            }
        }

        // Standalone spans have no transaction to cascade from: they take the transactions period.
        if let Some(days) = effective.transactions_days {
            match RetentionService::purge_standalone_spans(pool, policy.project_id, days).await {
                Ok(n) => removed.spans += n,
                Err(e) => {
                    log::error!(
                        "Retention: project {} standalone spans failed: {e}",
                        policy.project_id
                    );
                    if !failed.contains(&policy.project_id) {
                        failed.push(policy.project_id);
                    }
                }
            }
        }

        // Release health and alert history follow the events period.
        if let Some(days) = effective.events_days {
            match RetentionService::purge_session_data(pool, policy.project_id, days).await {
                Ok(purge) => {
                    sessions_removed += purge.session_counts + purge.session_users;
                    alerts_removed += purge.alert_history;
                    user_reports_removed += purge.user_reports;
                }
                Err(e) => {
                    log::error!(
                        "Retention: project {} sessions failed: {e}",
                        policy.project_id
                    );
                    if !failed.contains(&policy.project_id) {
                        failed.push(policy.project_id);
                    }
                }
            }
        }
    }

    let report = RetentionReport {
        started_at,
        finished_at: Utc::now(),
        projects: policies.len(),
        removed,
        sessions_removed,
        alerts_removed,
        user_reports_removed,
        unprotected,
        failed,
    };
    log::info!(
        "Retention pass: {} projects, removed {} events, {} transactions, {} logs",
        report.projects,
        report.removed.events,
        report.removed.transactions,
        report.removed.logs
    );
    state.record(&report);
    report
}

/// The worker loop.
pub async fn run(pool: DbPool, state: Arc<RetentionState>) {
    tokio::time::sleep(FIRST_PASS_DELAY).await;
    loop {
        run_once(&pool, &state).await;
        tokio::time::sleep(state.interval).await;
    }
}
