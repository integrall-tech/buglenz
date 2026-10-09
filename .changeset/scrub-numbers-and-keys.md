---
"@rustrak/server": patch
---

Personal-data scrubbing now also covers a CPF or CNPJ sent as a JSON number (it becomes the string `[cpf]` or `[cnpj]`) and an e-mail used as an object key (renamed to `[email]`, numbered when two collide). Numbers under identifier keys, card-sized numbers and ordinary numbers are left alone.
