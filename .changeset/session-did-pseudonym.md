---
"@rustrak/server": patch
---

The distinct-user id of a session (`did`, which an SDK may build from the user's e-mail, username or IP address when there is no user id) is no longer stored as sent: the aggregator keeps an HMAC-SHA256 pseudonym keyed with the instance's `SESSION_SECRET_KEY`. Distinct-user counts are unchanged; rows written by earlier versions keep the original value.
