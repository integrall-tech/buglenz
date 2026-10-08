//! Process counters.
//!
//! What the server did since it started, counted in memory for the Prometheus
//! endpoint (`/metrics`) and nothing else. The anonymous usage report the
//! upstream sends from this module was removed in the BugLenz fork (ADR-0004,
//! `governance/adr/0004-remocao-de-telemetria-e-egress.md`): nothing here
//! opens a connection.

pub mod counters;
pub mod metrics;

pub use counters::{Counters, Health, Rejection};

/// `file:line` when `file` is a path inside this crate, which rustc records
/// relative to the crate root. Dependencies and the standard library come
/// through as absolute paths and are not ours to report.
pub fn own_location(file: &str, line: u32) -> Option<String> {
    let relative = !file.starts_with('/') && !file.starts_with('\\') && !file.contains(":\\");
    relative.then(|| format!("{file}:{line}"))
}

/// Counts panics by location on top of whatever hook was already installed,
/// so the default stderr report still happens. Installing twice chains twice,
/// so call it once at startup.
pub fn install_panic_hook(counters: &'static Counters) {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Some(key) = info
            .location()
            .and_then(|l| own_location(l.file(), l.line()))
        {
            counters.panic(&key);
        }
        previous(info);
    }));
}

/// `3.46.1` reads as `3.46`, `16.4 (Debian 16.4-1)` as `16.4`.
pub fn major_minor(raw: &str) -> String {
    raw.split(|c: char| !c.is_ascii_digit() && c != '.')
        .next()
        .unwrap_or("")
        .split('.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
}
