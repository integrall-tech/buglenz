//! Package 004: how the retention periods are read and resolved (ADR-0009, invariant I5).

use rustrak::services::retention::{
    validate_days, Effective, ProjectRetention, RetentionDefaults, RetentionUpdate,
};
use serde_json::json;

#[test]
fn defaults_come_from_the_three_variables_and_are_empty_when_unset() {
    let none = RetentionDefaults::parse(None, None, None);
    assert_eq!(
        (none.events_days, none.transactions_days, none.logs_days),
        (None, None, None)
    );

    let set = RetentionDefaults::parse(Some("90"), Some(" 30 "), Some("45"));
    assert_eq!(
        (set.events_days, set.transactions_days, set.logs_days),
        (Some(90), Some(30), Some(45))
    );
}

#[test]
fn a_value_that_is_not_a_valid_period_is_not_a_default() {
    // A typo must not become "delete everything" (0) or a silent huge window.
    for bad in ["0", "-5", "abc", "", "3651", "1.5"] {
        let d = RetentionDefaults::parse(Some(bad), None, None);
        assert_eq!(d.events_days, None, "{bad:?}");
    }
    assert_eq!(
        RetentionDefaults::parse(Some("3650"), None, None).events_days,
        Some(3650)
    );
    assert_eq!(
        RetentionDefaults::parse(Some("1"), None, None).events_days,
        Some(1)
    );
}

#[test]
fn the_effective_period_is_the_projects_own_then_the_default_then_none() {
    let defaults = RetentionDefaults::parse(Some("90"), Some("30"), None);
    let own = ProjectRetention {
        events_days: Some(7),
        transactions_days: None,
        logs_days: None,
    };
    let e = Effective::resolve(&defaults, &own);
    assert_eq!(
        (e.events_days, e.transactions_days, e.logs_days),
        (Some(7), Some(30), None)
    );
    assert_eq!(e.missing(), vec!["logs"]);
    assert!(!e.is_protected());

    let full = Effective::resolve(
        &RetentionDefaults::parse(Some("1"), Some("1"), Some("1")),
        &ProjectRetention::default(),
    );
    assert!(full.is_protected());
    assert!(full.missing().is_empty());
}

#[test]
fn days_outside_one_to_3650_are_refused() {
    assert!(validate_days("events_days", 1).is_ok());
    assert!(validate_days("events_days", 3650).is_ok());
    for bad in [0, -1, 3651, i32::MAX] {
        assert!(validate_days("events_days", bad).is_err(), "{bad}");
    }
}

#[test]
fn an_update_tells_absent_from_null() {
    let u: RetentionUpdate =
        serde_json::from_value(json!({ "events_days": 30, "logs_days": null })).unwrap();
    assert_eq!(u.events_days, Some(Some(30)));
    assert_eq!(u.logs_days, Some(None), "null clears the period");
    assert_eq!(u.transactions_days, None, "absent leaves it alone");
}
