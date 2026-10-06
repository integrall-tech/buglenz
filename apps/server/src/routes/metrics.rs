//! Opt-in, unauthenticated Prometheus scrape. Keep it behind a private network
//! or a proxy allowlist: when enabled, anyone who can reach this port can read it.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use actix_web::{http::header, web, HttpResponse};

use crate::db::DbPool;
use crate::services::StorageService;
use crate::telemetry::metrics::Spool;
use crate::telemetry::Counters;

pub struct MetricsEndpoint {
    pub enabled: bool,
    pub ingest_dir: PathBuf,
    pub counters: &'static Counters,
}

/// Off unless explicitly enabled; a typo must not silently change exposure.
pub fn enabled(value: Option<&str>) -> Result<bool, &'static str> {
    match value {
        None | Some("off") => Ok(false),
        Some("on") => Ok(true),
        _ => Err("RUSTRAK_METRICS must be 'on' or 'off'"),
    }
}

pub async fn scrape(state: web::Data<MetricsEndpoint>, pool: web::Data<DbPool>) -> HttpResponse {
    if !state.enabled {
        return HttpResponse::NotFound().finish();
    }

    let dir = state.ingest_dir.clone();
    let spool = match tokio::task::spawn_blocking(move || pending_spool(&dir)).await {
        Ok(Ok(spool)) => spool,
        _ => return HttpResponse::ServiceUnavailable().finish(),
    };

    // A failed size query must not be reported as a zero-byte database.
    let db_bytes = StorageService::db_size_bytes(pool.get_ref())
        .await
        .ok()
        .and_then(|n| u64::try_from(n).ok());

    HttpResponse::Ok()
        .insert_header((header::CACHE_CONTROL, "no-store"))
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(state.counters.metrics().render(spool, db_bytes))
}

/// Only pending records belong to the digest backlog, not other files in the
/// ingest directory. A vanished entry during a scrape is a normal race.
fn pending_spool(dir: &Path) -> std::io::Result<Spool> {
    let mut spool = Spool::default();
    let now = SystemTime::now();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if !entry
            .file_name()
            .to_string_lossy()
            .ends_with(".pending.json")
        {
            continue;
        }
        let metadata = match entry.metadata() {
            Ok(metadata) if metadata.is_file() => metadata,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Ok(_) => continue,
            Err(e) => return Err(e),
        };
        spool.pending += 1;
        spool.bytes += metadata.len();
        if let Ok(age) = metadata
            .modified()
            .and_then(|mtime| now.duration_since(mtime).map_err(std::io::Error::other))
        {
            spool.oldest_seconds = spool.oldest_seconds.max(age.as_secs());
        }
    }
    Ok(spool)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/metrics", web::get().to(scrape));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_are_opt_in() {
        assert_eq!(enabled(None), Ok(false));
        assert_eq!(enabled(Some("off")), Ok(false));
        assert_eq!(enabled(Some("on")), Ok(true));
        assert!(enabled(Some("true")).is_err());
    }

    #[test]
    fn only_pending_records_count_toward_the_spool() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("project-1-a.pending.json"), "event").unwrap();
        std::fs::write(dir.path().join("project-1-b.json"), "already digested").unwrap();
        let spool = pending_spool(dir.path()).unwrap();
        assert_eq!(spool.pending, 1);
        assert_eq!(spool.bytes, 5);
    }
}
