# ADR-0004 — Remoção de telemetria e de egress herdado

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

[confirmado] O servidor envia um relatório anônimo a cada 6 h para `https://us.i.posthog.com`
(`src/telemetry/posthog.rs`). O envio só ocorre se `RUSTRAK_TELEMETRY_KEY` foi embutida na
compilação; um build do fork sem essa chave registra "Telemetry is off" no boot. O dashboard busca
`https://rustrak.github.io/rustrak/versions.json` do navegador de cada usuário
(`shared/api/version-check.ts`).

O módulo `telemetry` mistura duas coisas: o repórter externo e os contadores que alimentam
`/metrics`. `Counters` é usado pelo digest e pelo middleware.

## Decisão

Remover o caminho de envio, não desligá-lo:

- Sai: `telemetry/posthog.rs`, `reporter.rs`, `report.rs`, `identity.rs`, `volume.rs`, `resources.rs`, a rota
  `/api/telemetry/preview`, as variáveis `RUSTRAK_TELEMETRY` e `DO_NOT_TRACK`, o segredo
  `rustrak_telemetry_key` do `Dockerfile` e o trecho de `main.rs` que monta o `Reporter`.
- Fica: `counters.rs`, `metrics.rs` e o middleware, que são o que `/metrics` importa, e o
  utilitário `major_minor` de `telemetry/mod.rs`, usado por `db/mod.rs`. [confirmado]
- Sai do dashboard: `version-check.ts` e o aviso de nova versão.
- Links para `docs.sentry.io` e `rustrak.github.io` na UI permanecem: são navegação iniciada pelo
  usuário, não conexão iniciada pela instância.

## Verificação

Teste de conformidade de rede no CI: a instância sobe em rede sem rota externa, recebe eventos e
permanece 15 minutos; qualquer tentativa de conexão fora da lista do invariante I3 falha o build.

## Consequências

`main.rs` e `telemetry/mod.rs` entram no `DELTA-MANIFEST.md` e vão conflitar quando o upstream
mexer em telemetria. É um custo aceito: I3 não admite chave de desligamento.
