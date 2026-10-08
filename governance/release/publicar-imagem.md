# Roteiro: publicar uma imagem do BugLenz (`vX.Y.Z-itl.N`)

Próxima tag prevista: **`v0.16.0-itl.6`** (a `itl.5` é de antes do 023, da retenção, do G24 e do pt-BR).
A publicação roda no GitHub Actions (`release-image.yml`); **enquanto a cota gratuita estiver
esgotada, a tag não constrói** (ver ADR-0019). Não crie a tag antes de a cobrança ou o ciclo voltarem.

## 1. Antes da tag

- [ ] `main` com os PRs desejados mesclados e o PR da ADR-0019 (#19) também.
- [ ] Verificação local de `main` (a CI pode não rodar):
  ```bash
  cd apps/server
  cargo fmt --check && cargo clippy --locked --all-targets --features openapi -- -D warnings
  cargo test --locked --features openapi                       # SQLite
  cargo test --locked --no-default-features --features postgres --test unit_tests --test e2e_tests   # Docker
  cargo run --bin gen_openapi --features openapi --locked && git diff --exit-code -- openapi.json
  cd ../.. && pnpm install --frozen-lockfile && pnpm --filter=@rustrak/client test && (cd apps/dashboard && pnpm test && pnpm build)
  node --test "brand/test/*.test.mjs"
  ```
  (O teste `test_trigger_alert_does_not_block_on_slow_webhook_delivery` falha no PostgreSQL por defeito do
  upstream, usa `datetime()` do SQLite; não rode `integration_tests` no PostgreSQL esperando verde.)
- [ ] **`Network conformance` por `workflow_dispatch`, janela de 15 minutos** (ADR-0019: deixou de rodar
  a cada push em `main`). Verde ou não há tag.
- [ ] `DELTA-MANIFEST.md` cobre `git diff --name-status v0.16.0 HEAD` (cada arquivo, com ADR).
- [ ] As três variáveis de retenção decididas (D6) para a instância que vai receber a imagem.

## 2. A tag

```bash
git checkout main && git pull --ff-only
git tag -a v0.16.0-itl.6 -m "BugLenz v0.16.0-itl.6: security hardening, retention, session dedupe, pt-BR"
git push origin v0.16.0-itl.6        # dispara release-image.yml
```

A imagem sai privada em `ghcr.io/integrall-tech/buglenz-server:<tag>` (e `:latest`), só `linux/amd64`;
o workflow mantém as 10 versões mais recentes.

## 3. Depois da publicação

Do Mac (`arm64`): `docker pull --platform linux/amd64 ghcr.io/integrall-tech/buglenz-server:v0.16.0-itl.6`.

- [ ] Pacote **privado** e vinculado a `integrall-tech/buglenz`; digest anotado.
- [ ] Rótulos OCI: `version`, `revision` = commit da tag, `licenses=GPL-3.0-only`, `source`.
- [ ] Sem `posthog`/`versions.json` no binário; dashboard embutido com `<title>BugLenz …`.
- [ ] **Contrato dos wrappers contra a imagem** (a verificação mais forte):
  ```bash
  cd ../buglenz-sdk && ./contract/run.sh ghcr.io/integrall-tech/buglenz-server:v0.16.0-itl.6 all
  ```
  Atualiza `docs/matriz.md` se passar.
- [ ] Rota de exclusão por titular **responde** (200, não 404) e `GET /api/retention` responde (200).
- [ ] Registrar digest e resultados em `governance/baseline/` (como a baseline 007 fez para a `itl.5`).

## 4. O que muda para quem opera (avisos do 023, 004 e G24)

| Mudança | Efeito |
|---|---|
| **Retenção** (004) | A stack Swarm **exige** `RUSTRAK_RETENTION_EVENTS_DAYS`, `_TRANSACTIONS_DAYS` e `_LOGS_DAYS` (7 a 3650); sem elas `docker stack deploy` falha. A primeira passada apaga dados 60 s depois da partida. |
| **Migration nova** | `project_retention` (aditiva). Sobe sozinha no primeiro boot; o `down` é `DROP TABLE`. |
| **Webhooks** (023) | Destino interno (loopback, rede privada, `*.internal`) é recusado ao salvar **e ao enviar**; canais já salvos assim passam a falhar. Exceção: `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`. Redirecionamentos não são seguidos. |
| **Senha** (023) | Acima de 1024 bytes é recusada (login, convite, troca, vínculo SSO). Não há mínimo. |
| **Exclusão por titular** (005, corrigida no 004) | Passa a funcionar; na `itl.5` respondia 404. |
| **Sessões** (G24) | Uma sessão reportada duas vezes conta uma. |
| **Dashboard** (021) | Novo idioma "Português (Brasil)". |

## 5. Reverter

Uma imagem anterior sobe com o mesmo banco: a tabela `project_retention` fica sem uso (a `itl.5` não a
conhece). Para reverter de fato: `docker service update --image …:v0.16.0-itl.5` e, se quiser, o `down`
da migration. Os dados apagados pela retenção **não voltam**: restaure do backup (`backup.sh`).
