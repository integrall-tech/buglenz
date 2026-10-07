# 002 — Design

**Base:** Rustrak `v0.16.0` (`4dbe5ce75d16b62ef507474f8a30582438aa332d`), após o merge de
`sync/2026-10-07` em `main`. Branch de trabalho: `pkg/002-remocao-de-egress`.

Tudo marcado [confirmado] foi lido no código da `v0.16.0` em 2026-10-07. Números de linha são da
tag e servem de referência, não de contrato.

## 1. Inventário de egress [confirmado]

Toda conexão que o código inicia, levantada por `reqwest::Client`, `lettre`, `fetch` e URLs
literais em `apps/server/src`, `apps/dashboard/src`, `packages/*/src`:

| Origem | Destino | Dispara quando | I3 | Destino neste pacote |
|---|---|---|---|---|
| `telemetry/posthog.rs` → `reporter.rs` | `https://us.i.posthog.com/i/v0/e/` | 10 min após boot e a cada 6 h, se `RUSTRAK_TELEMETRY_KEY` foi compilada e `RUSTRAK_TELEMETRY`/`DO_NOT_TRACK` não desligam | viola | **remover** |
| `apps/dashboard/src/shared/api/version-check.ts` | `https://rustrak.github.io/rustrak/versions.json` | em toda sessão autenticada, do navegador, salvo build com `VITE_RUSTRAK_VERSION_CHECK_ENABLED=false` | viola | **remover** |
| `auth/oidc.rs` | issuer configurado em `OIDC_*` | login SSO | permitido | fica |
| `services/notification/email.rs` (`lettre`) | `SMTP_HOST` | alerta | permitido | fica |
| `services/notification/slack.rs` | `https://slack.com/api/chat.postMessage` | alerta Slack configurado | permitido | fica |
| `services/notification/webhook.rs`, `custom_webhook.rs` | URL configurada na regra | alerta | permitido | fica |
| UI: links para `docs.sentry.io`, `github.com` | — | clique do usuário | não é egress da instância | fica |

Não há fonte externa, CDN ou analytics no `index.html` nem no bundle do dashboard. O site de
documentação (`apps/docs`) não é servido pela instância.

## 2. Servidor

### 2.1 Módulo `telemetry` (v0.16.0: 11 arquivos)

| Arquivo | Conteúdo | Ação |
|---|---|---|
| `posthog.rs` | `PostHogSink`, `ENDPOINT`, `SINK_NAME`, `compiled_key()` (`option_env!("RUSTRAK_TELEMETRY_KEY")`) | remover |
| `reporter.rs` | `Reporter`, `Schedule`, `Context`; laço de envio; `preview()` | remover |
| `report.rs` | `Report`, `Sink`, `SinkError`, `ConfigFacts`, `Volume`, `SCHEMA` | remover |
| `identity.rs` | `instance_id()`: UUID em `installation.telemetry_id` | remover (coluna fica) |
| `volume.rs` | contagens borradas para o relatório; `alert_providers()` | remover |
| `resources.rs` | `probe`, `Rss`, `Sampler` (memória do processo) | remover |
| `mod.rs` | declara os módulos acima; `decide()`, `Disabled`, `TelemetryStatus`, `blur_count()`, `sqlite_path_from_url()`; **e também** `install_panic_hook()`, `own_location()`, `major_minor()` | editar: remover os seis primeiros itens e as declarações/re-exports dos módulos removidos; manter `counters`, `metrics`, `install_panic_hook`, `own_location`, `major_minor` (usado por `db/mod.rs:192`) |
| `counters.rs` | `Counters` (global, embute `MetricsCounters`), `Health`, `Rejection`, `LATENCY_BOUNDS_MS` | **não tocar** |
| `metrics.rs` | `MetricsCounters`, `Spool`, `render()` do Prometheus | **não tocar** |
| `../middleware/telemetry.rs` | `TelemetryMiddleware`: conta respostas em `Counters` | **não tocar** |
| `../routes/metrics.rs` | `GET /metrics`, lê `Counters::metrics()` | **não tocar** |

`Health` e `snapshot_and_reset()` em `counters.rs` ficam sem consumidor depois da remoção. É
código morto de 30 linhas; fica, porque editar `counters.rs` cria conflito com a refatoração que o
upstream anunciou na issue #375.

### 2.2 Pontos de ligação

| Arquivo | O que muda |
|---|---|
| `src/main.rs` | Some o bloco "Anonymous telemetry" (linhas 158–250 da tag) **exceto** `install_panic_hook` (160) e a construção de `metrics_data` (251–258); somem `.app_data(telemetry_reporter_data)`, `.app_data(telemetry_status_data)` (315–316) e `.configure(routes::telemetry::configure)` (392–393) |
| `src/routes/mod.rs` | some `pub mod telemetry;` |
| `src/routes/telemetry.rs` | removido (rota `GET /api/telemetry/preview`, 73 linhas) |
| `src/openapi.rs` | somem `crate::routes::telemetry::preview` (132) e `crate::routes::telemetry::TelemetryPreview` (210) |
| `openapi.json` | regenerado com `cargo run --bin gen_openapi --features openapi`; o job `rust-test` falha se houver drift |
| `src/config.rs` | somem `TelemetryConfig` (struct, `from_env`), o campo `Config::telemetry`, a variante `ConfigError::InvalidTelemetrySwitch` e seu `Display` |
| `Dockerfile` | o `RUN --mount=type=secret,id=rustrak_telemetry_key ... RUSTRAK_TELEMETRY_KEY=...` (linhas 35–41) vira `RUN cargo build --release --locked --no-default-features --features ${FEATURES}` |
| `.env.example` (85–88), `docker-compose.yml` (23–24), `docker-compose.postgres.yml` (38–39) | somem as linhas de telemetria |

O que **não** muda e por quê: `README.md` §Telemetry e `apps/docs/content/configuration/telemetry.mdx`
continuam descrevendo um recurso que o fork não tem. São documentação do upstream; editá-las é
delta de zona A sem ganho funcional. `CHANGES-FROM-UPSTREAM.md` diz que a seção não se aplica.

### 2.3 Testes do servidor

| Arquivo | Ação |
|---|---|
| `tests/common/telemetry.rs` | removido (fixture `sample_report`); `tests/common/mod.rs` perde `pub mod telemetry;` |
| `tests/unit/telemetry_test.rs` (28 fns) | ficam os testes de `Counters`, do panic hook (`own_location`) e `the_engine_version_is_cut_to_major_and_minor`; saem os de `decide`, `blur_count`, envelope PostHog, `Sampler`/`probe`, `sqlite_path_from_url` (~12) |
| `tests/integration/telemetry_test.rs` (21 fns) | ficam os testes de middleware e digest que contam em `Counters` (`ingest_outcomes_are_bucketed_by_status`, `server_errors_are_counted_by_route_pattern_not_path`, `ordinary_api_traffic_is_not_counted_at_all`, `a_digested_event_counts_as_ok`, `an_event_that_cannot_be_digested_counts_as_failed`, `a_failed_delivery_is_counted_under_its_provider_kind`); saem sink, identidade, volume, laço do repórter e os três de `/api/telemetry/preview` (~15) |
| `tests/integration/metrics_test.rs` | **não tocar**; precisa continuar passando |
| `tests/unit/config_test.rs` | somem `with_telemetry_env` e os 4 testes de `RUSTRAK_TELEMETRY`/`DO_NOT_TRACK` |
| `tests/integration/auth_test.rs:53`, `tests/e2e/sentry_sdk_test.rs:61` | somem o campo `telemetry: TelemetryConfig {...}` da construção de `Config` |
| `tests/integration/quota_enforcement_test.rs` | usa só `Counters`; não muda |

Nenhuma dependência do `Cargo.toml` fica órfã: `reqwest` segue nas notificações e no OIDC;
`async-trait`, `serde_json` e `uuid` são usados em todo o crate. [confirmado por grep]

## 3. Dashboard

| Arquivo | Ação |
|---|---|
| `src/shared/api/version-check.ts` | removido |
| `src/shared/ui/components/update-banner-slot.tsx` | removido |
| `src/routes/_authenticated.tsx` | somem o `import { UpdateBannerSlot }` (linha 12) e `<UpdateBannerSlot />` (72) |
| `src/shared/ui/components/update-banner.tsx`, `src/shared/lib/version.ts` | **ficam**, sem consumidor. `biome check` não acusa arquivo não importado; apagar cria dois arquivos a mais no delta por nada |
| catálogos `shared/i18n/messages/*.json` | não mudam (zona A, pacote 007); as chaves do aviso ficam sem uso |

`VITE_RUSTRAK_VERSION_CHECK_ENABLED` deixa de existir. Nenhum arquivo do repositório a define
além de `version-check.ts`. [confirmado]

Não há teste existente para esses arquivos. O build (`vite build`), `biome check` e `tsc` do job
`web` são a verificação.

## 4. Teste de conformidade de rede

Duas camadas, em um workflow novo `.github/workflows/network-conformance.yml` e um script
`scripts/network-conformance.sh` (arquivos novos; zona G).

### 4.1 Estática (toda execução da CI, ~1 min)

Falha se qualquer destes tokens aparecer fora de `apps/docs`, `CHANGELOG.md`, `README.md` e `governance/`:

```
us.i.posthog.com   posthog   rustrak.github.io/rustrak/versions.json   versions.json
RUSTRAK_TELEMETRY   DO_NOT_TRACK   VITE_RUSTRAK_VERSION_CHECK_ENABLED   /api/telemetry
```

E, sobre os artefatos construídos no mesmo job: `strings target/debug/rustrak | grep -E
'posthog|versions\.json'` vazio; `grep -rE 'posthog|versions\.json' apps/dashboard/dist` vazio.

A lista fica em `scripts/egress-denylist.txt` para que o sync a estenda quando o upstream criar
nova saída.

### 4.2 Em execução (`ubuntu-latest`, SQLite, binário debug)

1. Compilar `apps/server` (`cargo build --locked`; cache do rust-cache lido da `main`).
2. Criar o usuário de sistema `rustrak-ct` e regras `iptables` por dono do processo
   (`-m owner --uid-owner rustrak-ct`): aceita loopback **exceto porta 53**; tudo o mais
   `LOG --log-prefix "EGRESS "` e `REJECT`. DNS entra no bloqueio porque qualquer resolução de nome
   é a primeira evidência de uma conexão não permitida; a `systemd-resolved` em `127.0.0.53` seria
   um furo se o loopback fosse aceito inteiro.
3. **Autoteste das regras:** `sudo -u rustrak-ct curl -m 5 https://example.com` deve falhar e
   produzir exatamente uma linha `EGRESS` no `dmesg`. Sem isso o teste pode passar por regra que
   não funciona.
4. Zerar o marcador de log e subir o servidor como `rustrak-ct` com `CREATE_SUPERUSER`,
   `SESSION_SECRET_KEY`, `RUSTRAK_METRICS=on`, `DATABASE_URL` SQLite em diretório temporário.
5. Exercitar: `POST /auth/login` (cookie), `POST /api/projects` (recebe `dsn` e `sentry_key`),
   `POST /api/{project_id}/envelope/` com um envelope de evento e um de sessão, `GET /metrics`
   (deve conter `rustrak_ingest_accepted_total`), `GET /api/telemetry/preview` (deve ser 404 JSON).
6. Esperar a janela: **15 minutos** em `push` para `main`, `next`, `sync/**` e em
   `workflow_dispatch`; **3 minutos** em `pull_request`. A janela de 15 min é a do ADR-0004 e
   cobre o antigo primeiro envio aos 10 min; 3 min no PR mantém o ciclo de revisão curto. [inferência:
   proposta; o Edson confirma]
7. Falha se: houver qualquer linha `EGRESS` após o marcador; o processo tiver morrido; o log do
   servidor contiver `Telemetry` ou `telemetry`.

PostgreSQL não entra: o egress não depende do banco, e o job `postgres-e2e` já cobre esse backend.
A imagem Docker não entra: o build release leva ~10 min e o pacote 003 é quem a produz.

[inferência] `iptables` com `-m owner` e `sudo` estão disponíveis nos runners `ubuntu-24.04`
hospedados; T7 confirma na primeira execução. Alternativa se faltar: `nft` com `meta skuid`.

## 5. Delta deste pacote (entradas do `DELTA-MANIFEST.md`)

| Arquivo | Zona | ADR | Natureza |
|---|---|---|---|
| `apps/server/src/telemetry/{posthog,reporter,report,identity,volume,resources}.rs` | G | 0004 | removidos |
| `apps/server/src/telemetry/mod.rs` | G | 0004 | alterado |
| `apps/server/src/routes/telemetry.rs` | G | 0004 | removido |
| `apps/server/src/routes/mod.rs`, `src/openapi.rs`, `src/config.rs`, `src/main.rs` | G | 0004 | alterados |
| `apps/server/openapi.json` | G | 0004 | regenerado |
| `apps/server/Dockerfile`, `apps/server/.env.example`, `docker-compose.yml`, `docker-compose.postgres.yml` | B | 0004 | alterados |
| `apps/server/tests/common/telemetry.rs` | G | 0004 | removido |
| `apps/server/tests/common/mod.rs`, `tests/unit/{telemetry,config}_test.rs`, `tests/integration/{telemetry,auth}_test.rs`, `tests/e2e/sentry_sdk_test.rs` | G | 0004 | alterados |
| `apps/dashboard/src/shared/api/version-check.ts`, `src/shared/ui/components/update-banner-slot.tsx` | A | 0004 | removidos |
| `apps/dashboard/src/routes/_authenticated.tsx` | A | 0004 | alterado |
| `.github/workflows/network-conformance.yml`, `scripts/network-conformance.sh`, `scripts/egress-denylist.txt` | G | 0004 | novos |

Zona: código do servidor removido não é marca (A), nem operação (B), nem identificador (C). Vai
como G por ser delta de governança do I3; se o manifesto ganhar uma zona "S — servidor" no
ADR-0006, estas entradas migram.

## 6. Conflitos esperados em sync

`main.rs`, `telemetry/mod.rs`, `config.rs`, `openapi.rs`, `openapi.json`, `Dockerfile` e os
três arquivos de teste alterados. A issue #375 do upstream (refatorar `Counters`/`MetricsCounters`)
vai tocar `telemetry/mod.rs` e `counters.rs`. Procedimento do ADR-0005: resolver mantendo a
remoção; regenerar `openapi.json` em vez de mesclá-lo à mão.

## 7. Baseline

`governance/baseline/002.md`: testes por suíte após a remoção (esperado: ~1.320 Rust aprovados
contra 1.351 na 001, ajustado pelo que a `v0.16.0` acrescentou), resultado e tempo do job
`network-conformance`, saída do `strings` sobre o binário, tamanho do binário e do bundle antes e
depois.

## 8. Pendências que este pacote não resolve

- Rebrand dos textos e da documentação que citam telemetria (007).
- Proteção de `main` com o novo check obrigatório `network-conformance` (D10).
- Teste de ponta a ponta com app React minificado (GAP §3) como job da CI: pertence ao 006.
