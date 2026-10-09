use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, NaiveDate, Timelike, Utc};
use tokio::sync::Mutex;

use crate::db::DbPool;
use crate::error::AppResult;
use crate::models::session::{
    classify, parse_ts, SessionAggregateItem, SessionAggregates, SessionOutcome, SessionStatus,
    SessionUpdate,
};

const OVERFLOW_RELEASE: &str = "<overflow>";

/// Key identifying a minute-bucketed session count row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BucketKey {
    pub project_id: i32,
    pub release: String,
    pub environment: String,
    pub bucket: DateTime<Utc>,
}

/// Aggregated counters for a bucket key.
#[derive(Debug, Default, Clone)]
pub struct Counters {
    pub total: i64,
    pub errored: i64,
    pub crashed: i64,
    pub abnormal: i64,
}

/// Key identifying a day-bucketed distinct-user row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserKey {
    pub project_id: i32,
    pub release: String,
    pub environment: String,
    pub day: NaiveDate,
    pub did: String,
}

/// State held under the mutex.
#[derive(Debug, Default)]
pub struct AggregatorState {
    pub counts: HashMap<BucketKey, Counters>,
    /// Map of UserKey → crashed; TRUE means this user had a crash this flush cycle.
    pub users: HashMap<UserKey, bool>,
    /// What was already counted for each session id, so that a session reported more than once
    /// (the Java SDK sends its final state twice) is counted once. Survives flushes; bounded.
    pub seen: HashMap<(i32, String), SidSeen>,
}

/// What the aggregator already counted for one session id (BugLenz, gap G24).
#[derive(Debug, Clone, Copy)]
pub struct SidSeen {
    /// The `init` update was counted.
    pub init: bool,
    /// The worst terminal outcome counted so far.
    pub outcome: Option<SessionOutcome>,
    /// When this session id was last reported.
    pub at: DateTime<Utc>,
}

/// How many session ids are remembered, and for how long. Past either, the oldest go first; a
/// session whose id was forgotten and is then reported again is counted again.
pub const SEEN_CAP: usize = 100_000;
const SEEN_TTL_HOURS: i64 = 24;

/// Shared handle to the session aggregator — cheaply cloneable across handlers.
#[derive(Clone)]
pub struct SessionAggregatorHandle(Arc<SessionAggregator>);

impl SessionAggregatorHandle {
    pub async fn ingest_session(&self, project_id: i32, update: &SessionUpdate) {
        self.0.ingest_session(project_id, update).await;
    }

    pub async fn ingest_aggregates(&self, project_id: i32, agg: &SessionAggregates) {
        self.0.ingest_aggregates(project_id, agg).await;
    }

    /// Flush all in-memory state to DB.  Called on shutdown.
    pub async fn flush(&self) -> AppResult<()> {
        self.0.flush().await
    }
}

pub struct SessionAggregator {
    pool: DbPool,
    state: Mutex<AggregatorState>,
    flush_interval_secs: u64,
    cardinality_cap: usize,
}

impl SessionAggregator {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        pool: DbPool,
        flush_interval_secs: u64,
        cardinality_cap: usize,
    ) -> SessionAggregatorHandle {
        let agg = Arc::new(Self {
            pool,
            state: Mutex::new(AggregatorState::default()),
            flush_interval_secs,
            cardinality_cap,
        });
        SessionAggregatorHandle(agg)
    }

    /// Ingest a single `session` envelope item.
    pub async fn ingest_session(&self, project_id: i32, update: &SessionUpdate) {
        let mut state = self.state.lock().await;
        apply_session(&mut state, project_id, update, self.cardinality_cap);
    }

    /// Ingest a pre-aggregated `sessions` envelope item.
    pub async fn ingest_aggregates(&self, project_id: i32, agg: &SessionAggregates) {
        let attrs = match &agg.attrs {
            Some(a) => a,
            None => {
                log::warn!("sessions item missing attrs, dropping");
                return;
            }
        };
        let release = match &attrs.release {
            Some(r) if !r.is_empty() => r.clone(),
            _ => {
                log::warn!("sessions item missing release, dropping");
                return;
            }
        };
        let environment = attrs.environment.clone().unwrap_or_default();

        let mut state = self.state.lock().await;
        let release = apply_cardinality_cap(&state, project_id, release, self.cardinality_cap);

        for item in &agg.aggregates {
            apply_aggregate_item(&mut state, project_id, &release, &environment, item);
        }
    }

    /// Flush all in-memory counters to the DB via batched UPSERT.
    pub async fn flush(&self) -> AppResult<()> {
        let (counts, users) = {
            let mut state = self.state.lock().await;
            let counts = std::mem::take(&mut state.counts);
            let users = std::mem::take(&mut state.users);
            (counts, users)
        };

        if counts.is_empty() && users.is_empty() {
            return Ok(());
        }

        let mut commit_started = false;
        let result = async {
            let mut tx = self.pool.begin().await?;
            for (key, c) in &counts {
                self.upsert_count(key, c, &mut *tx).await?;
            }
            for (key, &crashed) in &users {
                self.upsert_user(key, crashed, &mut *tx).await?;
            }
            commit_started = true;
            tx.commit().await
        }
        .await;

        if let Err(error) = result {
            if !commit_started {
                let mut state = self.state.lock().await;
                merge_state(&mut state, counts, users);
            } else {
                log::error!(
                    "session_aggregator: commit outcome unknown; counters will not be replayed"
                );
            }
            return Err(error.into());
        }

        Ok(())
    }

    async fn upsert_count<'e, E>(
        &self,
        key: &BucketKey,
        c: &Counters,
        executor: E,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = crate::db::Db>,
    {
        #[cfg(feature = "postgres")]
        sqlx::query(
            r#"
            INSERT INTO session_counts (project_id, release, environment, bucket, total, errored, crashed, abnormal)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (project_id, release, environment, bucket)
            DO UPDATE SET
                total    = session_counts.total    + EXCLUDED.total,
                errored  = session_counts.errored  + EXCLUDED.errored,
                crashed  = session_counts.crashed  + EXCLUDED.crashed,
                abnormal = session_counts.abnormal + EXCLUDED.abnormal
            "#,
        )
        .bind(key.project_id)
        .bind(&key.release)
        .bind(&key.environment)
        .bind(key.bucket)
        .bind(c.total)
        .bind(c.errored)
        .bind(c.crashed)
        .bind(c.abnormal)
        .execute(executor)
        .await?;

        #[cfg(not(feature = "postgres"))]
        sqlx::query(
            r#"
            INSERT INTO session_counts (project_id, release, environment, bucket, total, errored, crashed, abnormal)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT (project_id, release, environment, bucket)
            DO UPDATE SET
                total    = session_counts.total    + excluded.total,
                errored  = session_counts.errored  + excluded.errored,
                crashed  = session_counts.crashed  + excluded.crashed,
                abnormal = session_counts.abnormal + excluded.abnormal
            "#,
        )
        .bind(key.project_id)
        .bind(&key.release)
        .bind(&key.environment)
        .bind(key.bucket.naive_utc().to_string())
        .bind(c.total)
        .bind(c.errored)
        .bind(c.crashed)
        .bind(c.abnormal)
        .execute(executor)
        .await?;

        Ok(())
    }

    async fn upsert_user<'e, E>(
        &self,
        key: &UserKey,
        crashed: bool,
        executor: E,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = crate::db::Db>,
    {
        #[cfg(feature = "postgres")]
        sqlx::query(
            r#"
            INSERT INTO session_users (project_id, release, environment, day, did, crashed)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (project_id, release, environment, day, did)
            DO UPDATE SET crashed = session_users.crashed OR EXCLUDED.crashed
            "#,
        )
        .bind(key.project_id)
        .bind(&key.release)
        .bind(&key.environment)
        .bind(key.day)
        .bind(&key.did)
        .bind(crashed)
        .execute(executor)
        .await?;

        #[cfg(not(feature = "postgres"))]
        sqlx::query(
            r#"
            INSERT INTO session_users (project_id, release, environment, day, did, crashed)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (project_id, release, environment, day, did)
            DO UPDATE SET crashed = MAX(session_users.crashed, excluded.crashed)
            "#,
        )
        .bind(key.project_id)
        .bind(&key.release)
        .bind(&key.environment)
        .bind(key.day.to_string())
        .bind(&key.did)
        .bind(if crashed { 1i64 } else { 0i64 })
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Run the aggregator interval loop.  Pass the handle here; it owns the Arc.
    pub async fn run(handle: SessionAggregatorHandle) {
        log::info!("session_aggregator: started");
        let interval_secs = handle.0.flush_interval_secs;
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        interval.tick().await; // consume the immediate first tick
        loop {
            interval.tick().await;
            if let Err(e) = handle.0.flush().await {
                log::error!("session_aggregator: flush error: {:?}", e);
            }
        }
    }
}

fn merge_state(
    state: &mut AggregatorState,
    counts: HashMap<BucketKey, Counters>,
    users: HashMap<UserKey, bool>,
) {
    for (key, counters) in counts {
        let current = state.counts.entry(key).or_default();
        current.total += counters.total;
        current.errored += counters.errored;
        current.crashed += counters.crashed;
        current.abnormal += counters.abnormal;
    }
    for (key, crashed) in users {
        state
            .users
            .entry(key)
            .and_modify(|current| *current |= crashed)
            .or_insert(crashed);
    }
}

/// Counts one `session` update into the state. Pure: no database, no clock beyond `Utc::now()` for
/// the memory of session ids.
///
/// A session id (`sid`) is counted once however many times it is reported: SDKs resend the final
/// state (BugLenz, gap G24). The first `init` adds to `total`; the first terminal outcome adds to
/// its counter; a later, worse outcome moves the session to that counter (errored becomes crashed);
/// the same or a milder one changes nothing. An update without a `sid` cannot be told apart and is
/// counted as it arrives.
pub fn apply_session(
    state: &mut AggregatorState,
    project_id: i32,
    update: &SessionUpdate,
    cardinality_cap: usize,
) {
    let attrs = match &update.attrs {
        Some(a) => a,
        None => {
            log::warn!("session item missing attrs, dropping");
            return;
        }
    };
    let release = match &attrs.release {
        Some(r) if !r.is_empty() => r.clone(),
        _ => {
            log::warn!("session item missing release, dropping");
            return;
        }
    };
    let environment = attrs.environment.clone().unwrap_or_default();

    let started_str = update.started.as_deref().unwrap_or("");
    let bucket = match parse_ts(started_str) {
        Some(dt) => truncate_to_minute(dt),
        None => truncate_to_minute(Utc::now()),
    };

    let release = apply_cardinality_cap(state, project_id, release, cardinality_cap);
    let key = BucketKey {
        project_id,
        release: release.clone(),
        environment: environment.clone(),
        bucket,
    };
    let user_key = |did: &str| UserKey {
        project_id,
        release: release.clone(),
        environment: environment.clone(),
        day: bucket.date_naive(),
        did: did.to_string(),
    };
    // The SDK's distinct-user id may be an e-mail, a username or an IP address; only the keyed
    // pseudonym is counted and stored (ADR-0009, invariant I4).
    let pseudonymous = update
        .did
        .as_deref()
        .filter(|d| !d.is_empty())
        .map(crate::scrub::pseudonym::pseudonym);
    let did = pseudonymous.as_deref();

    let sid = update
        .sid
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| (project_id, s.to_string()));
    let before = sid
        .as_ref()
        .and_then(|k| state.seen.get(k))
        .copied()
        .unwrap_or(SidSeen {
            init: false,
            outcome: None,
            at: Utc::now(),
        });

    if update.init && !before.init {
        state.counts.entry(key.clone()).or_default().total += 1;
        if let Some(did) = did {
            state.users.entry(user_key(did)).or_insert(false);
        }
    }

    let mut outcome_now = before.outcome;
    let status = update.status.as_ref().cloned().unwrap_or(SessionStatus::Ok);
    if status.is_terminal() {
        let outcome = classify(&status, update.errors);
        let counts = match before.outcome {
            None => Some((None, outcome)),
            Some(prev) if outcome.severity() > prev.severity() => Some((Some(prev), outcome)),
            Some(_) => None,
        };
        if let Some((replaced, outcome)) = counts {
            let entry = state.counts.entry(key).or_default();
            match replaced {
                Some(SessionOutcome::Errored) => entry.errored -= 1,
                Some(SessionOutcome::Abnormal) => entry.abnormal -= 1,
                Some(SessionOutcome::Crashed) | Some(SessionOutcome::Healthy) | None => {}
            }
            match outcome {
                SessionOutcome::Crashed => {
                    entry.crashed += 1;
                    if let Some(did) = did {
                        state.users.insert(user_key(did), true);
                    }
                }
                SessionOutcome::Abnormal => entry.abnormal += 1,
                SessionOutcome::Errored => entry.errored += 1,
                SessionOutcome::Healthy => {}
            }
            outcome_now = Some(outcome);
        }
    }

    if let Some(sid) = sid {
        let now = Utc::now();
        state.seen.insert(
            sid,
            SidSeen {
                init: before.init || update.init,
                outcome: outcome_now,
                at: now,
            },
        );
        evict_seen(&mut state.seen, now);
    }
}

/// Keeps the memory of session ids inside [`SEEN_CAP`] and [`SEEN_TTL_HOURS`].
fn evict_seen(seen: &mut HashMap<(i32, String), SidSeen>, now: DateTime<Utc>) {
    if seen.len() <= SEEN_CAP {
        return;
    }
    let horizon = now - chrono::Duration::hours(SEEN_TTL_HOURS);
    seen.retain(|_, s| s.at >= horizon);
    if seen.len() > SEEN_CAP {
        let mut times: Vec<DateTime<Utc>> = seen.values().map(|s| s.at).collect();
        times.sort_unstable();
        let cut = times[times.len() / 10];
        seen.retain(|_, s| s.at > cut);
    }
}

pub fn truncate_to_minute(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_second(0)
        .and_then(|d| d.with_nanosecond(0))
        .unwrap_or(dt)
}

/// Apply cardinality cap: if the new release would exceed the per-project cap, use OVERFLOW_RELEASE.
pub fn apply_cardinality_cap(
    state: &AggregatorState,
    project_id: i32,
    release: String,
    cap: usize,
) -> String {
    let project_releases: std::collections::HashSet<&str> = state
        .counts
        .keys()
        .filter(|k| k.project_id == project_id)
        .map(|k| k.release.as_str())
        .collect();

    if project_releases.contains(release.as_str()) {
        return release;
    }
    if project_releases.len() >= cap {
        return OVERFLOW_RELEASE.to_string();
    }
    release
}

/// Apply a pre-aggregated item into state (pure, no I/O).
pub fn apply_aggregate_item(
    state: &mut AggregatorState,
    project_id: i32,
    release: &str,
    environment: &str,
    item: &SessionAggregateItem,
) {
    let started_str = item.started.as_deref().unwrap_or("");
    let bucket = match parse_ts(started_str) {
        Some(dt) => truncate_to_minute(dt),
        None => truncate_to_minute(Utc::now()),
    };

    let key = BucketKey {
        project_id,
        release: release.to_string(),
        environment: environment.to_string(),
        bucket,
    };
    let entry = state.counts.entry(key).or_default();
    entry.total += item.exited + item.errored + item.crashed + item.abnormal + item.unhandled;
    // Protocol 1.6.0 `unhandled`: the process survived, so it is an errored
    // session, not a crash.
    entry.errored += item.errored + item.unhandled;
    entry.crashed += item.crashed;
    entry.abnormal += item.abnormal;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::session::{SessionAttributes, SessionStatus};

    fn make_update(
        init: bool,
        status: SessionStatus,
        errors: i64,
        did: Option<&str>,
    ) -> SessionUpdate {
        SessionUpdate {
            sid: Some("sid-1".to_string()),
            did: did.map(|s| s.to_string()),
            seq: Some(0),
            init,
            started: Some("2026-06-10T10:00:00.000Z".to_string()),
            timestamp: None,
            duration: None,
            status: Some(status),
            errors,
            attrs: Some(SessionAttributes {
                release: Some("1.0.0".to_string()),
                environment: Some("production".to_string()),
            }),
        }
    }

    /// Pure bucketing helper for tests — no DB pool needed.
    fn apply_update(state: &mut AggregatorState, project_id: i32, update: &SessionUpdate) {
        apply_session(state, project_id, update, 10_000);
    }

    #[test]
    fn truncate_removes_seconds() {
        use chrono::TimeZone;
        let dt = Utc.with_ymd_and_hms(2026, 6, 10, 10, 5, 42).unwrap();
        let t = truncate_to_minute(dt);
        assert_eq!(t.second(), 0);
        assert_eq!(t.minute(), 5);
    }

    #[test]
    fn init_increments_total() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, None),
        );
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.total, 1);
        assert_eq!(c.crashed, 0);
    }

    #[test]
    fn heartbeat_no_double_count() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, None),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Ok, 0, None),
        );
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.total, 1, "heartbeat must not increment total");
    }

    #[test]
    fn crash_increments_crashed_and_marks_user() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, Some("u1")),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Crashed, 0, Some("u1")),
        );
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.crashed, 1);
        assert_eq!(c.total, 1);
        let crashed = *state.users.values().next().unwrap();
        assert!(crashed);
    }

    #[test]
    fn errored_by_errors_count() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, None),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Exited, 3, None),
        );
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.errored, 1);
        assert_eq!(c.crashed, 0);
    }

    #[test]
    fn pre_aggregated_sums_correctly() {
        let mut state = AggregatorState::default();
        let item = SessionAggregateItem {
            started: Some("2026-06-10T10:00:00.000Z".to_string()),
            exited: 5,
            errored: 2,
            crashed: 1,
            abnormal: 0,
            unhandled: 0,
        };
        apply_aggregate_item(&mut state, 1, "2.0.0", "staging", &item);
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.total, 8); // 5+2+1+0
        assert_eq!(c.errored, 2);
        assert_eq!(c.crashed, 1);
    }

    #[test]
    fn unhandled_update_counts_as_errored() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, None),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Unhandled, 1, None),
        );
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.errored, 1);
        assert_eq!(c.crashed, 0);
    }

    #[test]
    fn pre_aggregated_unhandled_counts_as_errored() {
        let mut state = AggregatorState::default();
        let item = SessionAggregateItem {
            started: Some("2026-06-10T10:00:00.000Z".to_string()),
            exited: 5,
            errored: 1,
            crashed: 0,
            abnormal: 0,
            unhandled: 2,
        };
        apply_aggregate_item(&mut state, 1, "2.0.0", "staging", &item);
        let c = state.counts.values().next().unwrap();
        assert_eq!(c.total, 8); // 5+1+0+0+2
        assert_eq!(c.errored, 3);
        assert_eq!(c.crashed, 0);
    }

    #[test]
    fn cardinality_cap_overflows() {
        let mut state = AggregatorState::default();
        // Fill up to cap
        for i in 0..3usize {
            let key = BucketKey {
                project_id: 1,
                release: format!("v{i}"),
                environment: "prod".to_string(),
                bucket: Utc::now(),
            };
            state.counts.insert(key, Counters::default());
        }
        // Cap = 3; next release should overflow
        let result = apply_cardinality_cap(&state, 1, "v99".to_string(), 3);
        assert_eq!(result, OVERFLOW_RELEASE);
    }

    #[test]
    fn cardinality_cap_allows_existing_release() {
        let mut state = AggregatorState::default();
        let key = BucketKey {
            project_id: 1,
            release: "v1.0.0".to_string(),
            environment: "prod".to_string(),
            bucket: Utc::now(),
        };
        state.counts.insert(key, Counters::default());
        let result = apply_cardinality_cap(&state, 1, "v1.0.0".to_string(), 1);
        assert_eq!(result, "v1.0.0");
    }

    #[test]
    fn failed_flush_merges_counts_and_crash_flags_back_into_state() {
        let key = BucketKey {
            project_id: 1,
            release: "v1".to_string(),
            environment: "prod".to_string(),
            bucket: Utc::now(),
        };
        let user = UserKey {
            project_id: 1,
            release: "v1".to_string(),
            environment: "prod".to_string(),
            day: key.bucket.date_naive(),
            did: "did".to_string(),
        };
        let mut state = AggregatorState {
            counts: HashMap::from([(
                key.clone(),
                Counters {
                    total: 1,
                    ..Default::default()
                },
            )]),
            users: HashMap::from([(user.clone(), false)]),
            ..Default::default()
        };
        merge_state(
            &mut state,
            HashMap::from([(
                key.clone(),
                Counters {
                    total: 2,
                    ..Default::default()
                },
            )]),
            HashMap::from([(user.clone(), true)]),
        );
        assert_eq!(state.counts[&key].total, 3);
        assert!(state.users[&user]);
    }

    // --- G24: the same session id reported more than once -------------------

    fn with_sid(mut update: SessionUpdate, sid: Option<&str>) -> SessionUpdate {
        update.sid = sid.map(str::to_string);
        update
    }

    fn totals(state: &AggregatorState) -> (i64, i64, i64, i64) {
        state.counts.values().fold((0, 0, 0, 0), |a, c| {
            (
                a.0 + c.total,
                a.1 + c.errored,
                a.2 + c.crashed,
                a.3 + c.abnormal,
            )
        })
    }

    /// What `sentry-spring-boot` 8.60.0 sends for one session: init, an update
    /// with an error, the crash, and the same crash again when the session ends.
    #[test]
    fn a_terminal_state_repeated_for_one_session_counts_once() {
        let mut state = AggregatorState::default();
        for (init, status, errors) in [
            (true, SessionStatus::Ok, 0),
            (false, SessionStatus::Ok, 1),
            (false, SessionStatus::Crashed, 2),
            (false, SessionStatus::Crashed, 2),
        ] {
            apply_update(
                &mut state,
                1,
                &make_update(init, status, errors, Some("u1")),
            );
        }
        assert_eq!(totals(&state), (1, 0, 1, 0), "total 1, crashed 1, not 2");
    }

    #[test]
    fn an_init_repeated_for_one_session_counts_once() {
        let mut state = AggregatorState::default();
        for _ in 0..3 {
            apply_update(
                &mut state,
                1,
                &make_update(true, SessionStatus::Ok, 0, None),
            );
        }
        assert_eq!(totals(&state).0, 1);
    }

    #[test]
    fn different_sessions_are_counted_separately() {
        let mut state = AggregatorState::default();
        for sid in ["a", "b", "c"] {
            apply_update(
                &mut state,
                1,
                &with_sid(make_update(true, SessionStatus::Ok, 0, None), Some(sid)),
            );
            apply_update(
                &mut state,
                1,
                &with_sid(
                    make_update(false, SessionStatus::Crashed, 1, None),
                    Some(sid),
                ),
            );
        }
        assert_eq!(totals(&state), (3, 0, 3, 0));
    }

    #[test]
    fn the_same_sid_in_two_projects_is_two_sessions() {
        let mut state = AggregatorState::default();
        for project in [1, 2] {
            apply_update(
                &mut state,
                project,
                &make_update(true, SessionStatus::Ok, 0, None),
            );
            apply_update(
                &mut state,
                project,
                &make_update(false, SessionStatus::Crashed, 1, None),
            );
        }
        assert_eq!(totals(&state), (2, 0, 2, 0));
    }

    #[test]
    fn a_session_without_an_id_is_counted_as_before() {
        let mut state = AggregatorState::default();
        for _ in 0..2 {
            apply_update(
                &mut state,
                1,
                &with_sid(make_update(false, SessionStatus::Crashed, 1, None), None),
            );
        }
        assert_eq!(totals(&state).2, 2, "nothing to tell the updates apart");
    }

    #[test]
    fn a_worse_final_state_replaces_the_one_already_counted() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, Some("u1")),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Errored, 1, Some("u1")),
        );
        assert_eq!(totals(&state), (1, 1, 0, 0));
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Crashed, 1, Some("u1")),
        );
        assert_eq!(
            totals(&state),
            (1, 0, 1, 0),
            "errored becomes crashed, not both"
        );
        // A milder state afterwards changes nothing.
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Exited, 1, Some("u1")),
        );
        assert_eq!(totals(&state), (1, 0, 1, 0));
    }

    #[test]
    fn a_replacement_after_a_flush_is_a_correction_not_a_second_session() {
        let mut state = AggregatorState::default();
        apply_update(
            &mut state,
            1,
            &make_update(true, SessionStatus::Ok, 0, None),
        );
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Errored, 1, None),
        );
        // What the flush wrote is gone from memory; the memory of the session is not.
        state.counts.clear();
        apply_update(
            &mut state,
            1,
            &make_update(false, SessionStatus::Crashed, 1, None),
        );
        assert_eq!(
            totals(&state),
            (0, -1, 1, 0),
            "the delta that moves one session from errored to crashed"
        );
    }

    #[test]
    fn the_memory_of_sessions_is_bounded() {
        let mut state = AggregatorState::default();
        for i in 0..(SEEN_CAP + 10) {
            let sid = format!("sid-{i}");
            apply_update(
                &mut state,
                1,
                &with_sid(make_update(true, SessionStatus::Ok, 0, None), Some(&sid)),
            );
        }
        assert!(state.seen.len() <= SEEN_CAP, "{} entries", state.seen.len());
    }

    // --- I4: the distinct-user id never reaches the database as the SDK sent it -------------

    #[test]
    fn the_distinct_user_id_is_stored_as_a_pseudonym() {
        let mut state = AggregatorState::default();
        for did in ["ana@example.com", "bia@example.com", "ana@example.com"] {
            apply_update(
                &mut state,
                1,
                &make_update(true, SessionStatus::Ok, 0, Some(did)),
            );
        }
        // Different sessions would be needed to count two users; the point here is the key.
        let dids: Vec<&str> = state.users.keys().map(|k| k.did.as_str()).collect();
        assert!(
            dids.iter()
                .all(|d| d.starts_with("p1:") && !d.contains('@')),
            "{dids:?}"
        );
    }

    #[test]
    fn two_users_stay_two_and_one_user_stays_one() {
        let mut state = AggregatorState::default();
        for (sid, did) in [
            ("s1", "ana@example.com"),
            ("s2", "bia@example.com"),
            ("s3", "ana@example.com"),
        ] {
            let mut u = make_update(true, SessionStatus::Ok, 0, Some(did));
            u.sid = Some(sid.to_string());
            apply_update(&mut state, 1, &u);
        }
        assert_eq!(state.users.len(), 2, "ana counted once, bia once");
    }
}
