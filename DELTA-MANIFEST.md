# DELTA-MANIFEST

Relação de **toda** divergência deste repositório em relação ao upstream
[rustrak/rustrak](https://github.com/rustrak/rustrak). Base atual: tag `v0.16.0`,
commit `4dbe5ce7` (sincronizada em 2026-10-07; base inicial `v0.15.2`, `ff75852c`).

Regras (CONSTITUTION I1, ADR-0002, ADR-0005):

- Todo arquivo do upstream alterado ou removido consta aqui, com a ADR que o justifica.
  Divergência sem ADR não entra no repositório.
- Arquivo novo também consta, para que a sincronização saiba o que é do fork.
- A entrada entra **no mesmo commit** que a divergência.
- Em cada sincronização com o upstream, conflito fora dos arquivos listados aqui é erro do
  manifesto e é corrigido no mesmo PR.

Verificação: cada arquivo de `git diff --name-only <tag-base> main` deve aparecer, com caminho
completo ou pelo diretório pai nomeado na linha, na coluna "Arquivo" (expandindo `governance/**`).
Arquivos de um mesmo diretório podem ser agrupados em uma linha desde que o diretório esteja escrito.

## Zonas

As zonas A, B e C são as do ADR-0006 (marca). A zona G não existe no ADR-0006 e é definida aqui:

| Zona | O que é |
|---|---|
| A | Visível ao usuário: textos, logotipo, e-mails |
| B | Operação: nome de imagem, serviço, rótulos de compose |
| C | Identificadores: crate, escopo npm, variáveis, métricas, tabelas, API. **Não muda** |
| G | Governança e CI: arquivos fora do produto (workflows, documentos, manifestos). Não afetam o binário |

## Entradas

| Arquivo | Zona | ADR | Natureza da divergência |
|---|---|---|---|
| `NOTICE.md` | G | 0003 | Novo. Atribuição ao upstream, copyright das modificações, oferta de código-fonte |
| `DELTA-MANIFEST.md` | G | 0002 | Novo. Este arquivo |
| `CHANGES-FROM-UPSTREAM.md` | G | 0002 | Novo. Resumo legível deste manifesto |
| `CLAUDE.md` | G | 0002 | Alterado. Seção "BugLenz fork governance" acrescentada ao final; texto do upstream intacto acima |
| `.github/workflows/release.yml` | G | 0002, 0003 | Removido. Versionava e publicava os pacotes `@rustrak/*` no npm público; o fork não publica pacotes (CONSTITUTION I11) |
| `.github/workflows/docker-publish.yml` | G | 0002, 0012 | Removido. Publicava `rustrak/rustrak-server` e `rustrak/rustrak-ui` no Docker Hub; o build para registry privado é do pacote 003 (I11) |
| `.github/workflows/deploy-docs.yml` | G | 0002 | Removido. Publicava o site de documentação do upstream no GitHub Pages |
| `.github/FUNDING.yml` | G | 0002 | Removido. Patrocínio do autor original (GitHub Sponsors) não se aplica ao repositório privado |
| `THIRD-PARTY-LICENSES.md` | G | 0003 | Novo. Inventário de licenças das dependências Rust e JavaScript; gerado por `governance/tools/third-party-licenses.py` em Linux e conferido por `licenses.yml` (003) |
| `apps/server/src/telemetry/posthog.rs`, `apps/server/src/telemetry/reporter.rs`, `apps/server/src/telemetry/report.rs`, `apps/server/src/telemetry/identity.rs`, `apps/server/src/telemetry/volume.rs`, `apps/server/src/telemetry/resources.rs` | G | 0004 | Removidos. Repórter de telemetria anônima: sink PostHog, agendador, relatório, identidade da instância, volume, amostragem de recursos |
| `apps/server/src/telemetry/mod.rs` | G | 0004 | Alterado. Só declara `counters` e `metrics`; mantém `install_panic_hook`, `own_location`, `major_minor` |
| `apps/server/src/routes/telemetry.rs` | G | 0004 | Removido. `GET /api/telemetry/preview` |
| `apps/server/src/routes/mod.rs`, `apps/server/src/openapi.rs`, `apps/server/src/main.rs` | G | 0004 | Alterados. Pontos de ligação do repórter e da rota removidos |
| `apps/server/src/config.rs` | G | 0004 | Alterado. `TelemetryConfig` (`RUSTRAK_TELEMETRY`, `DO_NOT_TRACK`) e `ConfigError::InvalidTelemetrySwitch` removidos |
| `apps/server/src/routes/projects.rs` | G | 0004 | Alterado. Só o módulo `#[cfg(test)]`: fixture de `Config` sem o campo `telemetry` |
| `apps/server/openapi.json` | G | 0004 | Regenerado com `gen_openapi`; perde `/api/telemetry/preview` e `TelemetryPreview` |
| `apps/server/Dockerfile` | B | 0004 | Alterado. Sem o segredo de build `rustrak_telemetry_key` |
| `apps/server/.env.example`, `docker-compose.yml`, `docker-compose.postgres.yml` | B | 0004 | Alterados. Linhas de `RUSTRAK_TELEMETRY`/`DO_NOT_TRACK` removidas |
| `apps/server/tests/common/telemetry.rs` | G | 0004 | Removido. Fixture do relatório |
| `apps/server/tests/common/mod.rs`, `apps/server/tests/unit/telemetry_test.rs`, `apps/server/tests/unit/config_test.rs`, `apps/server/tests/integration/telemetry_test.rs` | G | 0004 | Alterados. Testes do repórter, do sink, da identidade, do volume, da prévia e das variáveis removidos; testes de `Counters` mantidos |
| `apps/server/tests/e2e/sentry_sdk_test.rs` e, em `apps/server/tests/integration/`: `agents_api_test.rs`, `alerts_api_test.rs`, `auth_test.rs`, `envelope_v2_test.rs`, `events_api_test.rs`, `field_errors_test.rs`, `ingest_test.rs`, `issues_api_test.rs`, `logs_api_test.rs`, `projects_api_test.rs`, `rate_limit_test.rs`, `releases_api_test.rs`, `sourcemaps_api_test.rs`, `span_v2_ingest_test.rs`, `spans_api_test.rs`, `storage_api_test.rs`, `team_rbac_test.rs`, `tokens_api_test.rs`, `transactions_api_test.rs` | G | 0004 | Alterados. Fixture de `Config` sem o campo `telemetry` (4 linhas cada) |
| `apps/dashboard/src/shared/api/version-check.ts`, `apps/dashboard/src/shared/ui/components/update-banner-slot.tsx` | A | 0004 | Removidos. Checagem de versão em `rustrak.github.io` e o aviso de atualização |
| `apps/dashboard/src/routes/_authenticated.tsx` | A | 0004 | Alterado. Sem o `<UpdateBannerSlot />` (2 linhas) |
| `.github/workflows/network-conformance.yml`, `scripts/network-conformance.sh`, `scripts/egress-denylist.txt` | G | 0004 | Novos. Teste de conformidade de rede: camada estática e servidor sob bloqueio de saída |
| `apps/server/Dockerfile` | B | 0012 | Alterado (003). `ENV INGEST_DIR=/data/ingest`: spool de ingestão dentro do volume (issue #359 do upstream) |
| `.github/workflows/licenses.yml` | G | 0003, 0005 | Novo (003). Regenera `THIRD-PARTY-LICENSES.md` em Linux e falha se divergir do commitado |
| `.github/workflows/release-image.yml` | G | 0012, 0005 | Novo (003). Publica `ghcr.io/integrall-tech/buglenz-server:<tag>` por tag `v*-itl.*`; build sem push em PR |
| `deploy/swarm/buglenz.stack.yml`, `deploy/swarm/README.md`, `deploy/swarm/provision.sh`, `deploy/swarm/backup.sh` | B | 0012, 0009 | Novos (003; 005 acrescenta `RUSTRAK_SCRUB_EXTRA_KEYS` e a seção de dados pessoais). Stack Swarm parametrizada, provisionamento via API (sem `RUSTRAK_BOOTSTRAP_TOKEN`, #356), backup e restauração |
| `apps/server/src/models/session.rs`, `apps/server/src/workers/session_aggregator.rs` | G | 0011, 0002 | Alterados (006). Status de sessão `unhandled` (protocolo 1.6.0) aceito e contado como errored; campo `unhandled` dos agregados. **Temporário**: proposto ao upstream em [rustrak/rustrak#383](https://github.com/rustrak/rustrak/pull/383); sai do manifesto quando entrar por sync |
| `apps/server/tests/unit/envelope_parser_test.rs` | G | 0011 | Alterado (006). Teste do parser para o status `unhandled`. Temporário, idem |
| `.changeset/sessions-unhandled-status.md` | G | 0002 | Novo (006). Changeset `patch` de `@rustrak/server` que acompanha o PR ao upstream. Temporário, idem |
| `e2e/react-app/**` | G | 0005, 0011 | Novo (006). App React 19 + `@sentry/react` 11.5.0 fora do workspace pnpm (lockfile próprio), para o teste de ponta a ponta |
| `scripts/e2e-react-assert.sh`, `.github/workflows/e2e-react.yml` | G | 0005, 0011 | Novos (006). Job `e2e-react`: servidor do PR, source maps, Chromium, asserções pela API; passo 6 do ADR-0005 |
| `apps/server/src/scrub/mod.rs`, `apps/server/src/scrub/keys.rs`, `apps/server/src/scrub/text.rs` | G | 0009 | Novos (005). Scrubbing de dados pessoais: chaves negadas → `[Filtered]`, máscaras `[cpf]`/`[cnpj]`/`[cartao]`/`[email]` |
| `apps/server/src/lib.rs` | G | 0009 | Alterado (005). `pub mod scrub;` |
| `apps/server/tests/unit/scrub_test.rs`, `apps/server/tests/unit/mod.rs` | G | 0009 | Novo / registro (005) |
| `apps/server/src/routes/ingest.rs` | G | 0009 | Alterado (005). O IP do cliente não é lido na ingestão |
| `apps/server/src/digest/processors/event.rs`, `apps/server/src/digest/processors/transaction.rs`, `apps/server/src/digest/processors/logs.rs`, `apps/server/src/digest/processors/span.rs`, `apps/server/src/digest/processors/span_v2.rs` | G | 0009 | Alterados (005). Chamada ao `scrub` antes de agrupar e persistir; `events.remote_addr` nunca gravado |
| `apps/server/tests/integration/scrub_test.rs`, `apps/server/tests/integration/mod.rs` | G | 0009 | Novo / registro (005) |
| `apps/server/tests/integration/digest_test.rs` | G | 0009 | Alterado (005). Expectativa de `test_list_stats_counts_by_email_when_id_is_absent`: com e-mails mascarados, eventos sem `user.id` contam como um usuário |
| `apps/server/src/services/privacy.rs`, `apps/server/src/routes/privacy.rs` | G | 0009 | Novos (005). Exclusão por titular: `DELETE /api/projects/{id}/privacy/users/{user_id}` (admin), contadores de issue e projeto ajustados |
| `apps/server/src/services/mod.rs`, `apps/server/src/routes/mod.rs`, `apps/server/src/main.rs`, `apps/server/src/openapi.rs`, `apps/server/openapi.json` | G | 0009 | Registro da rota e do schema (005); `openapi.json` regenerado |
| `apps/server/tests/integration/privacy_test.rs` | G | 0009 | Novo (005) |
| `governance/**` | G | 0001, 0002 | Novo. Corpus de governança: CONSTITUTION, GAP-ANALYSIS, `adr/`, `rfc/`, `openspec/`, `baseline/`, `tools/` |

## Histórico de bases

| Data | Base do upstream | Pacote |
|---|---|---|
| 2026-10-07 | `v0.15.2` (`ff75852c`) | 001 bootstrap |
| 2026-10-07 | `v0.16.0` (`4dbe5ce7`) | sync/2026-10-07, primeiro ciclo do ADR-0005; merge sem conflito |
| 2026-10-07 | `v0.16.0` (`4dbe5ce7`) | 002 remoção de egress |
| 2026-10-08 | `v0.16.0` (`4dbe5ce7`) | 003 build e implantação |
| 2026-10-08 | `v0.16.0` (`4dbe5ce7`) | 006 compatibilidade com o SDK atual |
| 2026-10-08 | `v0.16.0` (`4dbe5ce7`) | 005 scrubbing de dados pessoais |
