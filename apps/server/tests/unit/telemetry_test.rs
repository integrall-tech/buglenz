//! Unit tests for the process counters and the panic hook. The upstream's
//! anonymous telemetry (the on/off decision, the report, the PostHog adapter)
//! was removed in the BugLenz fork (ADR-0004); its tests went with it.

// =============================================================================
// Health counters
// =============================================================================

mod counters {
    use rustrak::telemetry::{Counters, Rejection};
    use std::time::Duration;

    #[test]
    fn a_fresh_counter_set_reports_zero_everywhere() {
        let health = Counters::new().snapshot_and_reset();
        assert_eq!(health.ingest.accepted, 0);
        assert_eq!(health.ingest.rejected.rate_limit, 0);
        assert_eq!(health.ingest.latency_ms.p50, None);
        assert_eq!(health.ingest.latency_ms.p99, None);
        assert_eq!(health.digest.ok, 0);
        assert_eq!(health.digest.failed, 0);
        assert!(health.http_5xx_by_route.is_empty());
        assert!(health.alerts_failed_by_provider.is_empty());
        assert!(health.panics.is_empty());
    }

    #[test]
    fn accepted_ingests_are_counted_with_their_latency_percentiles() {
        let counters = Counters::new();
        for ms in [3, 4, 4, 5, 6, 7, 8, 9, 40, 400] {
            counters.ingest_accepted(Duration::from_millis(ms));
        }
        let health = counters.snapshot_and_reset();
        assert_eq!(health.ingest.accepted, 10);
        // Bucketed on a fixed log scale: the percentile is the bucket's upper bound.
        assert_eq!(health.ingest.latency_ms.p50, Some(10));
        assert_eq!(health.ingest.latency_ms.p99, Some(500));
    }

    #[test]
    fn rejections_are_counted_by_reason_and_nothing_else() {
        let counters = Counters::new();
        counters.ingest_rejected(Rejection::RateLimit);
        counters.ingest_rejected(Rejection::RateLimit);
        counters.ingest_rejected(Rejection::Auth);
        counters.ingest_rejected(Rejection::TooLarge);
        counters.ingest_rejected(Rejection::Malformed);
        counters.ingest_rejected(Rejection::Other);
        let r = counters.snapshot_and_reset().ingest.rejected;
        assert_eq!(
            (r.rate_limit, r.auth, r.too_large, r.malformed, r.other),
            (2, 1, 1, 1, 1)
        );
    }

    #[test]
    fn a_snapshot_starts_the_next_window_from_zero() {
        let counters = Counters::new();
        counters.ingest_accepted(Duration::from_millis(1));
        counters.digest_ok();
        counters.digest_failed();
        counters.http_5xx("/api/projects/{id}");
        counters.alert_failed("slack");
        counters.panic("src/digest/mod.rs:212");

        let first = counters.snapshot_and_reset();
        assert_eq!(first.ingest.accepted, 1);
        assert_eq!((first.digest.ok, first.digest.failed), (1, 1));
        assert_eq!(first.http_5xx_by_route["/api/projects/{id}"], 1);
        assert_eq!(first.alerts_failed_by_provider["slack"], 1);
        assert_eq!(first.panics["src/digest/mod.rs:212"], 1);

        let second = counters.snapshot_and_reset();
        assert_eq!(second.ingest.accepted, 0);
        assert_eq!(second.ingest.latency_ms.p50, None);
        assert_eq!((second.digest.ok, second.digest.failed), (0, 0));
        assert!(second.http_5xx_by_route.is_empty());
        assert!(second.alerts_failed_by_provider.is_empty());
        assert!(second.panics.is_empty());
    }

    #[test]
    fn repeated_keys_accumulate() {
        let counters = Counters::new();
        counters.http_5xx("/api/issues/{id}");
        counters.http_5xx("/api/issues/{id}");
        counters.http_5xx("/api/projects");
        let health = counters.snapshot_and_reset();
        assert_eq!(health.http_5xx_by_route["/api/issues/{id}"], 2);
        assert_eq!(health.http_5xx_by_route["/api/projects"], 1);
    }

    /// A snapshot looks without taking, so a later drain still carries the
    /// whole window.
    #[test]
    fn a_peek_leaves_the_window_intact() {
        let counters = Counters::new();
        counters.ingest_accepted(Duration::from_millis(3));
        counters.http_5xx("/api/projects");
        let peeked = counters.snapshot();
        assert_eq!(peeked.ingest.accepted, 1);
        assert_eq!(peeked.http_5xx_by_route["/api/projects"], 1);
        assert_eq!(counters.snapshot_and_reset(), peeked);
    }

    /// Every hook in the request path bumps the same process-wide set.
    #[test]
    fn the_global_set_is_one_instance() {
        assert!(std::ptr::eq(Counters::global(), Counters::global()));
    }
}

// =============================================================================
// Panic hook
// =============================================================================

mod panics {
    use rustrak::telemetry::{install_panic_hook, own_location, Counters};

    /// The hook records where our code panicked and nothing about why: the
    /// message may carry a path, a query or a DSN.
    #[test]
    fn a_panic_in_our_own_tree_is_counted_by_location_only() {
        install_panic_hook(Counters::global());
        let line = line!() + 1;
        let _ = std::panic::catch_unwind(|| panic!("secret /home/alice/app.db"));
        let health = Counters::global().snapshot_and_reset();
        let key = format!("{}:{line}", file!());
        assert_eq!(health.panics.get(&key), Some(&1), "{:?}", health.panics);
        assert!(health.panics.keys().all(|k| !k.contains("secret")));
    }

    /// Dependencies and the standard library report absolute paths
    /// (`/rustc/…`, `~/.cargo/registry/…`); those are not ours to report.
    #[test]
    fn only_relative_paths_count_as_our_own_code() {
        assert_eq!(
            own_location("src/digest/mod.rs", 212).as_deref(),
            Some("src/digest/mod.rs:212")
        );
        assert_eq!(
            own_location("/rustc/abc/library/core/src/option.rs", 1),
            None
        );
        assert_eq!(
            own_location("/Users/x/.cargo/registry/src/foo/lib.rs", 1),
            None
        );
    }
}

// =============================================================================
// Engine version
// =============================================================================

/// Only the first two components are kept: enough to spot an engine that
/// misbehaves, not enough to fingerprint a build.
#[test]
fn the_engine_version_is_cut_to_major_and_minor() {
    use rustrak::telemetry::major_minor;
    assert_eq!(major_minor("3.46.1"), "3.46");
    assert_eq!(major_minor("16.4 (Debian 16.4-1.pgdg120+1)"), "16.4");
    assert_eq!(major_minor("17"), "17");
    assert_eq!(major_minor(""), "");
}
