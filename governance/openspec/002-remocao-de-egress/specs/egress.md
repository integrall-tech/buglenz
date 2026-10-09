# 002 — Especificação

> **Emenda de 2026-10-09 (auditoria).** Os cenários "a diferença nas migrações é vazia" e "só os arquivos do 002 e do 001 diferem" valiam **no fim do pacote 002**. Depois entraram as migrações de retenção (004) e muitos arquivos de outros pacotes, todos no `DELTA-MANIFEST.md`. A varredura do binário agora roda também sobre a imagem que sai (`release-image.yml`), com a lista inteira de tokens e sem distinguir maiúsculas.

## Código

**WHEN** `grep -rE 'posthog|versions\.json|RUSTRAK_TELEMETRY|DO_NOT_TRACK|VITE_RUSTRAK_VERSION_CHECK_ENABLED|/api/telemetry'` é executado sobre `apps/server/src`, `apps/server/tests`, `apps/dashboard/src`, `packages/*/src`, `apps/server/Dockerfile`, `apps/server/.env.example` e `docker-compose*.yml`
**THEN** o resultado é vazio

**WHEN** `apps/server/src/telemetry/counters.rs`, `apps/server/src/telemetry/metrics.rs`, `apps/server/src/middleware/telemetry.rs`, `apps/server/src/routes/metrics.rs` e `apps/server/tests/integration/metrics_test.rs` são comparados com a tag `v0.16.0`
**THEN** os arquivos são idênticos

**WHEN** `git diff --name-only v0.16.0 main` é filtrado por `apps/server/migrations`
**THEN** o resultado é vazio

## Binário e bundle

**WHEN** `strings` é aplicado ao binário `rustrak` compilado (debug ou release, SQLite ou PostgreSQL)
**THEN** não contém `posthog` nem `versions.json`

**WHEN** o dashboard é construído (`apps/dashboard/dist`)
**THEN** nenhum arquivo contém `rustrak.github.io/rustrak/versions.json`

## Comportamento do servidor

**WHEN** o servidor sobe com `RUSTRAK_TELEMETRY=off` ou `DO_NOT_TRACK=1` no ambiente
**THEN** ele inicia normalmente e o log de boot não contém a palavra `telemetry` em nenhuma grafia

**WHEN** um administrador autenticado chama `GET /api/telemetry/preview`
**THEN** a resposta é `404`, como qualquer rota inexistente: com corpo JSON de erro quando o dashboard está montado (fallback de `API_PREFIXES`), vazia na API pura

**WHEN** o servidor sobe com `RUSTRAK_METRICS=on` e recebe um envelope de evento válido
**THEN** `GET /metrics` responde `200` e contém `rustrak_ingest_accepted_total` com valor ≥ 1

**WHEN** o servidor sobe sem `RUSTRAK_METRICS`
**THEN** `GET /metrics` responde `404`, como na `v0.16.0`

## Conformidade de rede

**WHEN** o job `network-conformance` executa o autoteste (`curl https://example.com` como o usuário do servidor, sob as regras de bloqueio)
**THEN** a conexão falha e exatamente uma linha `EGRESS` é registrada

**WHEN** o servidor roda como esse usuário pela janela configurada (15 min em `push`/`workflow_dispatch`, 3 min em `pull_request`), recebe login, criação de projeto, envelopes de evento e sessão e uma leitura de `/metrics`
**THEN** nenhuma linha `EGRESS` é registrada após o marcador, o processo continua vivo e todos os pedidos responderam `2xx`

**WHEN** uma linha com `posthog`, `versions.json` ou outro token de `scripts/egress-denylist.txt` é introduzida em `apps/server/src` ou `apps/dashboard/src`
**THEN** o job `network-conformance` falha na camada estática antes de compilar

## Verificação herdada

**WHEN** `ci.yml` roda no branch do pacote
**THEN** `web`, `rust-lint`, `rust-test` e `postgres-e2e` terminam com sucesso, e `rust-test` não acusa drift em `openapi.json`

**WHEN** `cargo test` roda em `apps/server`
**THEN** zero falhas; a contagem por suíte está em `governance/baseline/002.md` e os seis testes de `Counters` listados na seção 2.3 do `design.md` continuam presentes

## Delta controlado

**WHEN** `git diff --name-status v0.16.0 main` é executado ao fim do pacote
**THEN** todo arquivo modificado ou removido consta em `DELTA-MANIFEST.md` com o ADR-0004, e nenhum arquivo além dos listados na seção 5 do `design.md` e dos do pacote 001 aparece

**WHEN** `README.md` e `apps/docs/**` são comparados com a tag `v0.16.0`
**THEN** são idênticos, e `CHANGES-FROM-UPSTREAM.md` diz que a seção de telemetria da documentação não se aplica ao fork
