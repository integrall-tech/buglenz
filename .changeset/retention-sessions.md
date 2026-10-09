---
"@rustrak/server": patch
---

The retention pass also removes a project's release-health rows (`session_counts`, `session_users`) and alert history older than its `events` period, and the erasure of a data subject also removes their `session_users` rows (the keyed pseudonym of the id and, for rows from before the pseudonym, the raw id). The erasure response gains a `sessions` count and the retention report gains `sessions_removed` and `alerts_removed`.
