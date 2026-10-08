---
"@rustrak/server": patch
---

Security hardening of the server (BugLenz ADR-0018, from the open upstream review #57): passwords above 1024 bytes are refused before any database or Argon2 work; an unknown login email now costs the same Argon2 as a wrong password; login and invitation acceptance drop and renew the session; webhook URLs may not target loopback, private, link-local or internal hosts (opt-in exceptions through `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`) and notifier requests no longer follow redirects; an oversized ingest body answers a JSON 413.
