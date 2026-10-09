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
| `.github/workflows/manifest.yml` | G | 0002, 0021 | Novo (auditoria de 2026-10-09). Job `manifest`: `governance/tools/check-manifest.py` falha se um arquivo diverge do upstream sem linha neste manifesto, ou se uma linha cita arquivo que já não diverge (CONSTITUTION I1) |
| `.github/workflows/licenses.yml` | G | 0003, 0005 | Novo (003). Regenera `THIRD-PARTY-LICENSES.md` em Linux e falha se divergir do commitado |
| `.github/workflows/release-image.yml` | G | 0012, 0005, 0006 | Novo (003); a partir do 007 constrói a imagem da cópia marcada, depois de verificada. Publica `ghcr.io/integrall-tech/buglenz-server:<tag>` por tag `v*-itl.*`; build sem push em PR |
| `deploy/swarm/buglenz.stack.yml`, `deploy/swarm/README.md`, `deploy/swarm/provision.sh`, `deploy/swarm/backup.sh` | B | 0012, 0009, 0006 | Novos (003; 005 acrescenta `RUSTRAK_SCRUB_EXTRA_KEYS` e a seção de dados pessoais; 007 acrescenta `SMTP_FROM` no domínio `buglenz.dev`). Stack Swarm parametrizada, provisionamento via API (sem `RUSTRAK_BOOTSTRAP_TOKEN`, #356), backup e restauração |
| `apps/server/src/workers/session_aggregator.rs`, `apps/server/src/models/session.rs`, `apps/server/tests/integration/session_dedupe_test.rs`, `apps/server/tests/integration/mod.rs`, `.changeset/session-dedupe.md` | G | 0011, 0002 | Alterados / novos (G24). Uma sessão reportada mais de uma vez (o SDK Java repete o estado final) é contada uma vez, por `sid`; memória limitada. **Temporário**: proposto ao upstream em [rustrak/rustrak#388](https://github.com/rustrak/rustrak/issues/388); sai do manifesto quando entrar por sync |
| `apps/server/src/models/session.rs`, `apps/server/src/workers/session_aggregator.rs` | G | 0011, 0002 | Alterados (006). Status de sessão `unhandled` (protocolo 1.6.0) aceito e contado como errored; campo `unhandled` dos agregados. **Temporário**: proposto ao upstream em [rustrak/rustrak#383](https://github.com/rustrak/rustrak/pull/383); sai do manifesto quando entrar por sync |
| `apps/server/tests/unit/envelope_parser_test.rs` | G | 0011 | Alterado (006). Teste do parser para o status `unhandled`. Temporário, idem |
| `.changeset/sessions-unhandled-status.md` | G | 0002 | Novo (006). Changeset `patch` de `@rustrak/server` que acompanha o PR ao upstream. Temporário, idem |
| `e2e/react-app/**` | G | 0005, 0011 | Novo (006). App React 19 + `@sentry/react` 11.5.0 fora do workspace pnpm (lockfile próprio), para o teste de ponta a ponta |
| `scripts/e2e-react-assert.sh`, `.github/workflows/e2e-react.yml` | G | 0005, 0011 | Novos (006). Job `e2e-react`: servidor do PR, source maps, Chromium, asserções pela API; passo 6 do ADR-0005 |
| `apps/server/src/scrub/mod.rs`, `apps/server/src/scrub/keys.rs`, `apps/server/src/scrub/text.rs`, `apps/server/src/scrub/pseudonym.rs` | G | 0009 | Novos (005; `pseudonym.rs` na auditoria de 2026-10-09: o `did` das sessões é gravado como pseudônimo com chave); proposta ao upstream em [rustrak/rustrak#384](https://github.com/rustrak/rustrak/issues/384). Scrubbing de dados pessoais: chaves negadas → `[Filtered]`, máscaras `[cpf]`/`[cnpj]`/`[cartao]`/`[email]` |
| `apps/server/src/lib.rs` | G | 0009 | Alterado (005). `pub mod scrub;` |
| `apps/server/tests/unit/scrub_test.rs`, `apps/server/tests/unit/mod.rs` | G | 0009 | Novo / registro (005) |
| `apps/server/src/routes/ingest.rs` | G | 0009 | Alterado (005). O IP do cliente não é lido na ingestão |
| `apps/server/src/digest/processors/event.rs`, `apps/server/src/digest/processors/transaction.rs`, `apps/server/src/digest/processors/logs.rs`, `apps/server/src/digest/processors/span.rs`, `apps/server/src/digest/processors/span_v2.rs` | G | 0009 | Alterados (005). Chamada ao `scrub` antes de agrupar e persistir; `events.remote_addr` nunca gravado |
| `apps/server/tests/integration/scrub_test.rs`, `apps/server/tests/integration/mod.rs` | G | 0009 | Novo / registro (005) |
| `apps/server/src/models/user.rs`, `apps/server/src/routes/auth.rs`, `apps/server/src/services/invitation.rs`, `apps/server/src/services/users.rs` | G | 0018 | Alterados (023). Limite de 1024 bytes na senha (login, convite, troca, vínculo SSO); verificação Argon2 fictícia quando o e-mail não existe; sessão limpa e renovada no login e no aceite de convite. **Temporário**: corrige H-1, H-2/M-3 e M-2 do PR [rustrak/rustrak#57](https://github.com/rustrak/rustrak/pull/57), parado e em conflito; sai do manifesto se o upstream aceitar |
| `apps/server/src/services/notification/destination.rs`, `apps/server/src/services/notification/mod.rs`, `apps/server/src/services/notification/webhook.rs`, `apps/server/src/services/notification/custom_webhook.rs` | G | 0018 | Novo / alterados (023). Webhooks não podem apontar para loopback, redes privadas, link-local, CGNAT ou nomes internos, na configuração e no envio; exceção por instância em `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`; o cliente HTTP dos notificadores não segue redirecionamentos. Corrige H-4 do PR #57 |
| `.github/rulesets/protect-main.json`, `.github/rulesets/protect-release-tags.json`, `.github/rulesets/README.md` | G | 0021 | Novos. Rulesets para `main` (PR e os oito checks obrigatórios) e para as tags `v*-itl.*`; **ainda não aplicados** |
| `apps/server/tests/integration/bootstrap_test.rs` | G | 0002 | Alterado. Os testes de bootstrap passam a rodar um de cada vez (trava assíncrona): todos mexiam na mesma `CREATE_SUPERUSER` e um apagava a variável de outro (falha esporádica, 1 em 12 localmente). **Temporário**: a propor ao upstream |
| `apps/server/tests/unit/pseudonym_test.rs`, `.changeset/session-did-pseudonym.md` | G | 0009 | Novos (auditoria I4, 2026-10-09) |
| `.changeset/scrub-numbers-and-keys.md` | G | 0009 | Novo (auditoria I4, segunda rodada) |
| `.changeset/scrub-email-in-id.md` | G | 0009 | Novo (auditoria I4, segunda rodada) |
| `.changeset/retention-standalone-spans.md` | G | 0009 | Novo (auditoria I5, segunda rodada) |
| `.changeset/retention-sessions.md` | G | 0009 | Novo (auditoria I5, 2026-10-09) |
| `apps/dashboard/src/styles.css`, `apps/dashboard/src/main.tsx`, `apps/dashboard/index.html` | A | 0022, 0023 | Alterados (0023: título serifado, cartões e títulos de cartão em frase normal, fonte Instrument Serif). Paleta quente (papel, tinta e um laranja), barra lateral de tinta nos dois temas e tema claro como padrão; o verde-limão sai |
| `apps/dashboard/package.json`, `pnpm-lock.yaml` | A | 0023 | Alterados. Dependência `@fontsource/instrument-serif` 5.3.0 (OFL-1.1), a fonte dos títulos de página |
| `apps/dashboard/src/shared/ui/components/sparkline.tsx`, `apps/dashboard/src/shared/lib/sparkline.ts`, `apps/dashboard/src/shared/lib/sparkline.test.ts`, `apps/dashboard/src/features/project/model/overview-summary.ts`, `apps/dashboard/src/features/project/model/overview-summary.test.ts`, `apps/dashboard/src/routes/_authenticated/projects/$id/-components/overview-summary.tsx`, `.changeset/overview-summary.md` | A | 0023 | Novos. Frase de resumo da Visão geral e mini gráfico do cartão de eventos |
| `apps/dashboard/src/shared/ui/components/stat-tile.tsx`, `apps/dashboard/src/routes/_authenticated/projects/$id/-components/overview-tiles.tsx`, `apps/dashboard/src/routes/_authenticated/projects/$id/index.tsx` | A | 0023 | Alterados. O cartão de estatística aceita uma tendência opcional; o cartão de eventos a recebe; a página mostra a frase de resumo |
| `scripts/seed-demo.py`, `.changeset/display-type-cards.md` | A | 0023 | Novos. Dados de demonstração para uma instância local (recusa instância não local) e o changeset da tipografia |
| `apps/dashboard/src/shared/lib/palette.test.ts`, `.changeset/warm-palette.md` | A | 0022 | Novos. Teste de contraste AA da paleta nos dois temas e da barra lateral |
| `.changeset/server-hardening.md` | G | 0018 | Novo (023) |
| `.changeset/automatic-retention.md` | G | 0009 | Novo (004) |
| `.changeset/dashboard-pt-br.md` | G | 0002 | Novo (021) |
| `apps/server/migrations/sqlite/20261008000000_project_retention.up.sql`, `apps/server/migrations/sqlite/20261008000000_project_retention.down.sql`, `apps/server/migrations/postgres/20261008000000_project_retention.up.sql`, `apps/server/migrations/postgres/20261008000000_project_retention.down.sql` | G | 0009 | Novos (004). Tabela `project_retention`, prazos por projeto |
| `apps/server/src/services/retention.rs`, `apps/server/src/workers/retention.rs`, `apps/server/src/routes/retention.rs` | G | 0009 | Novos (004). Prazos por projeto e tipo, worker de retenção, API de administração |
| `apps/server/src/services/mod.rs`, `apps/server/src/workers/mod.rs`, `apps/server/src/routes/mod.rs`, `apps/server/src/main.rs`, `apps/server/src/openapi.rs`, `apps/server/openapi.json` | G | 0009 | Alterados (004). Registro dos módulos, worker iniciado no `main`, rotas e documento OpenAPI |
| `apps/server/tests/unit/retention_test.rs`, `apps/server/tests/integration/retention_test.rs` | G | 0009 | Novos (004) |
| `packages/client/src/schemas/retention.ts`, `packages/client/src/types/retention.ts`, `packages/client/src/resources/retention.ts`, `packages/client/src/schemas/index.ts`, `packages/client/src/types/index.ts`, `packages/client/src/resources/index.ts`, `packages/client/src/client.ts`, `packages/client/src/index.ts`, `packages/client/tests/integration/retention.test.ts`, `packages/client/tests/mocks/handlers.ts` | G | 0009 | Novos / alterados (004/T8). Recurso `retention` do cliente |
| `apps/dashboard/src/features/retention/**`, `apps/dashboard/src/routes/_authenticated/settings/retention.tsx`, `apps/dashboard/src/routes/_authenticated/settings/-components/settings-nav.tsx`, `apps/dashboard/src/routeTree.gen.ts` | G | 0009 | Novos / alterados (004/T8). Tela de retenção e item do menu; `routeTree.gen.ts` é gerado |
| `apps/dashboard/src/shared/i18n/messages/{en,es,fr,ro,zh}.json` | G | 0009 | Alterados (004/T8). Chaves `settings.nav.retention` e `settings.retention.*` (acréscimo); uma ocorrência de "Rustrak" a mais por catálogo, no título da página |
| `apps/dashboard/src/features/release/ui/components/releases-list.tsx`, `.changeset/releases-header-width.md` | G | 0002, 0010 | Alterado (021). A coluna de usuários da tabela de releases passa de `w-40` a `w-60`: o rótulo em português ("Usuários sem travamentos") não cabia e se sobrepunha à coluna seguinte. Mesma mudança serve a qualquer idioma de rótulo longo |
| `apps/dashboard/src/shared/i18n/messages/pt.json`, `apps/dashboard/src/shared/i18n/routing.ts`, `apps/dashboard/src/shared/i18n/intl.ts`, `apps/dashboard/src/shared/i18n/config.test.ts`, `apps/dashboard/src/__tests__/architecture/message-keys.test.ts` | G | 0002, 0010 | Novo / alterados (021). Dashboard em português do Brasil (idioma `pt`); os cinco catálogos existentes ganham só `locale.pt`. **Temporário**: a propor ao upstream; sai do manifesto quando entrar por sync |
| `brand/buglenz/rules.json` | G | 0006 | Alterado (004/T8, 021). Em 021, `pt` entra nos textos de atribuição e de link do código-fonte. `catalog-brand-name` passa de 68 para 69 ocorrências por catálogo |
| `apps/server/src/routes/ingest.rs` | G | 0018 | Alterado (023). O corpo do `/envelope/` é lido no handler com o limite de 100 MB e excedê-lo responde 413 em JSON (M-1) |
| `apps/server/tests/integration/security_test.rs`, `apps/server/tests/unit/destination_test.rs`, `apps/server/tests/common/db.rs`, `apps/server/tests/integration/mod.rs`, `apps/server/tests/unit/mod.rs` | G | 0018 | Novos / alterados (023). Testes escritos antes das correções; `common/db.rs` isenta o loopback nos testes, que entregam webhooks a ouvintes locais |
| `apps/server/tests/integration/digest_test.rs` | G | 0009 | Alterado (005). Expectativa de `test_list_stats_counts_by_email_when_id_is_absent`: com e-mails mascarados, eventos sem `user.id` contam como um usuário |
| `apps/server/src/services/privacy.rs`, `apps/server/src/routes/privacy.rs` | G | 0009 | Novos (005). Exclusão por titular: `DELETE /api/projects/{id}/privacy/users/{user_id}` (admin), contadores de issue e projeto ajustados |
| `apps/server/src/services/mod.rs`, `apps/server/src/routes/mod.rs`, `apps/server/src/main.rs`, `apps/server/src/openapi.rs`, `apps/server/openapi.json` | G | 0009 | Registro da rota e do schema (005); `openapi.json` regenerado |
| `apps/server/tests/integration/privacy_test.rs` | G | 0009 | Novo (005) |
| `brand/lib.mjs`, `brand/apply.mjs`, `brand/verify.mjs`, `brand/test/brand.test.mjs` | A | 0006, 0002 | Novos (007); nome de produto configurável proposto ao upstream em [rustrak/rustrak#387](https://github.com/rustrak/rustrak/issues/387). Motor da sobreposição de marca: aplica regras declarativas com contagem esperada a uma cópia da árvore; `verify` guarda a zona C e a atribuição; 17 testes |
| `brand/buglenz/rules.json`, `brand/buglenz/verify.json` | A | 0006 | Novos (007). 26 regras: catálogos (68 ocorrências por idioma), título, rodapés, modelos de webhook, `actor` e rodapé dos alertas, título da API, mensagem de SSO, tela "Sobre" (atribuição, link de código-fonte, sem link ao rastreador do upstream) |
| `brand/buglenz/overrides/apps/dashboard/src/shared/ui/components/rustrak-wordmark.tsx`, `brand/buglenz/overrides/packages/ui/src/components/brand/wordmark.tsx` | A | 0006 | Novos (007). Logotipo tipográfico provisório (D9 aberta); substituem por inteiro, em tempo de build, os dois componentes do upstream, com o mesmo nome e a mesma assinatura |
| `brand/buglenz/assets/icon.png`, `brand/buglenz/assets/apple-icon.png` | A | 0006 | Novos (007). Ícones provisórios gerados (D9 aberta) |
| `.github/workflows/brand.yml` | A | 0006, 0005 | Novo (007). A cada PR e sync: motor, aplicação das regras, verificação, build do dashboard e `cargo check` do servidor marcados, busca de vazamentos no bundle |
| `BUGLENZ.md` | A | 0003, 0006 | Novo (007). Apresentação do fork: origem, licença, o que muda, como a marca é aplicada; o `README.md` da raiz continua o do upstream |
| `governance/baseline/007.md`, `governance/baseline/007-screens/*.png` | G | 0006 | Novos (007). Baseline e capturas de tela (login e "Sobre") do servidor marcado |
| `governance/baseline/009-t1.md` | G | 0011 | Novo (009/T1). Tráfego real do `sentry-spring-boot` 8.60.0 contra a instância; origem do gap G24 |
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
