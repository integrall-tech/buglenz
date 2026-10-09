---
"@rustrak/server": patch
---

The retention pass also removes standalone spans (spans that arrived without a parent transaction), which had no period at all because the cascade from a transaction never reached them. They follow the project's transactions period and are counted in `removed.spans`.
