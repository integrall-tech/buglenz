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

Verificação: `git diff --name-only <tag-base> main` deve ser um subconjunto da coluna
"Arquivo" (expandindo `governance/**`).

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
| `THIRD-PARTY-LICENSES.md` | G | 0003 | Novo. Inventário de licenças das dependências Rust e JavaScript; gerado por `governance/tools/third-party-licenses.py` |
| `apps/server/src/telemetry/posthog.rs`, `reporter.rs`, `report.rs`, `identity.rs`, `volume.rs`, `resources.rs` | G | 0004 | Removidos. Repórter de telemetria anônima: sink PostHog, agendador, relatório, identidade da instância, volume, amostragem de recursos |
| `apps/server/src/telemetry/mod.rs` | G | 0004 | Alterado. Só declara `counters` e `metrics`; mantém `install_panic_hook`, `own_location`, `major_minor` |
| `apps/server/src/routes/telemetry.rs` | G | 0004 | Removido. `GET /api/telemetry/preview` |
| `apps/server/src/routes/mod.rs`, `src/openapi.rs`, `src/main.rs` | G | 0004 | Alterados. Pontos de ligação do repórter e da rota removidos |
| `apps/server/src/config.rs` | G | 0004 | Alterado. `TelemetryConfig` (`RUSTRAK_TELEMETRY`, `DO_NOT_TRACK`) e `ConfigError::InvalidTelemetrySwitch` removidos |
| `apps/server/src/routes/projects.rs` | G | 0004 | Alterado. Só o módulo `#[cfg(test)]`: fixture de `Config` sem o campo `telemetry` |
| `apps/server/openapi.json` | G | 0004 | Regenerado com `gen_openapi`; perde `/api/telemetry/preview` e `TelemetryPreview` |
| `apps/server/tests/common/telemetry.rs` | G | 0004 | Removido. Fixture do relatório |
| `apps/server/tests/common/mod.rs`, `tests/unit/telemetry_test.rs`, `tests/unit/config_test.rs`, `tests/integration/telemetry_test.rs` | G | 0004 | Alterados. Testes do repórter, do sink, da identidade, do volume, da prévia e das variáveis removidos; testes de `Counters` mantidos |
| `apps/server/tests/integration/*.rs` (18 arquivos), `tests/e2e/sentry_sdk_test.rs` | G | 0004 | Alterados. Fixture de `Config` sem o campo `telemetry` (4 linhas cada) |
| `governance/**` | G | 0001, 0002 | Novo. Corpus de governança: CONSTITUTION, GAP-ANALYSIS, `adr/`, `rfc/`, `openspec/`, `baseline/`, `tools/` |

## Histórico de bases

| Data | Base do upstream | Pacote |
|---|---|---|
| 2026-10-07 | `v0.15.2` (`ff75852c`) | 001 bootstrap |
| 2026-10-07 | `v0.16.0` (`4dbe5ce7`) | sync/2026-10-07, primeiro ciclo do ADR-0005; merge sem conflito |
| 2026-10-07 | `v0.16.0` (`4dbe5ce7`) | 002 remoção de egress |
