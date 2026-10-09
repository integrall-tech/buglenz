# Repository rulesets

Proposed in ADR-0021. **Not applied yet**: they take effect only when someone runs the commands below.
GitHub does not validate a ruleset without creating it, so the first application is the test; the
rollback is one command.

| File | Protects | Rules |
|---|---|---|
| `protect-main.json` | `main` | no deletion, no force-push, every change through a pull request (0 required approvals), and these checks passing: `web`, `rust-lint`, `rust-test`, `postgres-e2e`, `brand`, `e2e-react`, `network-conformance`, `licenses` |
| `protect-release-tags.json` | tags `v*-itl.*` | a published release tag cannot be deleted or moved (creating one stays free) |

There are **no bypass actors**. The required checks are **job names** (from GitHub Actions, app id 15368):
renaming one of those jobs in a workflow makes every pull request wait for a check that never reports,
so a rename must update `protect-main.json` and the ruleset in the same change. Why these checks, and
why the workflows run on every pull request, is in `governance/adr/0021-protecao-de-main.md`.

## Apply

```bash
gh api -X POST repos/integrall-tech/buglenz/rulesets --input .github/rulesets/protect-main.json
gh api -X POST repos/integrall-tech/buglenz/rulesets --input .github/rulesets/protect-release-tags.json
gh api repos/integrall-tech/buglenz/rulesets --jq '.[]|"\(.id) \(.name) \(.enforcement)"'
```

## Check that it works

```bash
git checkout main && git commit --allow-empty -m "probe" && git push origin main   # must be refused
git push origin :refs/tags/v0.16.0-itl.6                                          # must be refused
```

## Roll back

```bash
gh api repos/integrall-tech/buglenz/rulesets --jq '.[]|"\(.id) \(.name)"'
gh api -X DELETE repos/integrall-tech/buglenz/rulesets/<id>
```

(Or Settings → Rules → Rulesets, set the ruleset to *Disabled*.)

## Check `manifest` (a aplicar)

O workflow `manifest.yml` cria o job `manifest`. Depois de mesclado e verde em `main`, acrescente
`{ "context": "manifest" }` aos checks obrigatórios de `protect-main.json` e reaplique o ruleset
(`gh api -X PUT repos/integrall-tech/buglenz/rulesets/<id> --input .github/rulesets/protect-main.json`).
Antes disso o check roda mas não bloqueia.
