---
"@rustrak/server": minor
---

Automatic retention (BugLenz ADR-0009, invariant I5). A worker applies, every `RUSTRAK_RETENTION_INTERVAL_HOURS` (default 24), a retention period per project and data type (events, transactions with their spans, logs) using the existing batched cleanup. Instance defaults come from `RUSTRAK_RETENTION_EVENTS_DAYS`, `RUSTRAK_RETENTION_TRANSACTIONS_DAYS` and `RUSTRAK_RETENTION_LOGS_DAYS` (no built-in value); a project overrides them through `PUT /api/projects/{id}/retention`. A project with no period for a type loses nothing of that type and is listed as unprotected by `GET /api/retention`.
