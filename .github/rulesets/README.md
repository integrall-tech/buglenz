# Repository rulesets

Proposed in ADR-0021. **Not applied yet**: they take effect only when someone runs the commands below.
GitHub does not validate a ruleset without creating it, so the first application is the test; the
rollback is one command.

| File | Protects | Rules |
|---|---|---|
| `protect-main.json` | `main` | no deletion, no force-push, every change through a pull request (0 required approvals) |
| `protect-release-tags.json` | tags `v*-itl.*` | a published release tag cannot be deleted or moved (creating one stays free) |

There are deliberately **no required status checks** and **no bypass actors**. Why, and what changes
that, is in `governance/adr/0021-protecao-de-main.md`.

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
