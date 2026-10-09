---
"@rustrak/server": patch
"@rustrak/client": patch
---

User feedback reports now go through the personal-data masks when they are stored (an e-mail becomes `[email]`, a CPF `[cpf]`) and are removed by the project's events period. The retention report gains `user_reports_removed`.
