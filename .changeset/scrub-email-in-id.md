---
"@rustrak/server": patch
---

Personal-data scrubbing now masks an e-mail address found under a key ending in `id` (such as `user.id`, which the JavaScript SDK builds from the e-mail when it has no id). CPF, CNPJ and card masks stay off for those keys, as an id made of digits is far more likely to be an id than a document.
