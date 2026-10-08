---
"@rustrak/server": patch
---

A session that an SDK reports more than once is now counted once. The Java SDK resends the final state of a session, which made the release health count two crashes for one session (`crashed 2`, `healthy -1`). The aggregator remembers, per session id, what it already counted: the first `init` and the first terminal outcome count, repeats do not, and a worse outcome reported later moves the session to that counter. The memory is bounded (100,000 ids, 24 hours).
