---
"@rustrak/server": "minor"
---

Add an opt-in Prometheus `/metrics` endpoint (`RUSTRAK_METRICS=on`) (@WahidinAji). Bulk-delete issues by state filter (`open`, `resolved`, `muted`, `all`) through the API, the client and MCP (@WahidinAji). Storage cleanups now run in the background in small batches: `POST /api/storage/cleanup` answers `202` with a job, `GET /api/storage/cleanup/status` follows it, and `client.storage.executeCleanup()` returns a `CleanupStatus` instead of counts. The storage page estimates per-project size from a sample so it stays fast on large instances. The integrations page links back to the alert settings you came from (@WahidinAji).
