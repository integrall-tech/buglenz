# 006 — Especificação

> **Emenda de 2026-10-09 (auditoria).** Os cenários "o diff de migrações, `pnpm-lock.yaml` e `pnpm-workspace.yaml` é vazio" e "o diff de `apps/*/src` são 16 arquivos mais 2" valiam **no fim do pacote 006**. Hoje o delta é maior e está todo no `DELTA-MANIFEST.md`. Os cenários sobre abrir o PR no upstream e sobre o remoto `integrall-tech/rustrak` dependem de conferência no GitHub.

## Status de sessão

**WHEN** um envelope traz um item `session` com `"status":"unhandled"` e `"errors":1`
**THEN** o servidor aceita o item (nenhum aviso "session item: bad JSON" no log) e as estatísticas de sessão do projeto registram uma sessão **errored** e zero **crashed**

**WHEN** um envelope traz um item `sessions` (agregado) com `"unhandled": 2`
**THEN** as estatísticas registram duas sessões errored a mais

**WHEN** um item `session` traz `"status":"crashed"`
**THEN** continua contado como crashed, como na `v0.16.0`

**WHEN** `classify(Unhandled, errors)` é chamado com qualquer `errors`
**THEN** o resultado é `SessionOutcome::Errored`

## Teste de ponta a ponta

**WHEN** o job `e2e-react` roda com o servidor do PR, um app React 19 minificado com `@sentry/react` 11.5.0 e source maps enviados pelo `@sentry/vite-plugin`
**THEN** quatro erros disparados em Chromium geram **3 issues**, e a issue do erro de clique tem `event_count` 2

**WHEN** o evento mais recente de cada issue é lido pela API
**THEN** o frame da aplicação tem `filename` terminando no arquivo-fonte original (não no bundle minificado), a linha original e `context_line` preenchido

**WHEN** as estatísticas de sessão do projeto `e2e-react` são lidas após o teste
**THEN** há pelo menos uma sessão errored, nenhuma crashed, e o crash-free de sessões é 100% — porque o processo não terminou — enquanto o errored é maior que zero

**WHEN** o job termina
**THEN** o resumo registra, como medição e não como falha, o nome de função do frame da aplicação (G7) e o valor de `in_app` dos frames de `node_modules` (G8)

**WHEN** o status `unhandled` deixar de ser aceito (regressão em um sync)
**THEN** o job falha na verificação de release health

## Upstream

**WHEN** o pacote fecha
**THEN** existe um PR aberto em `rustrak/rustrak` a partir de `integrall-tech/rustrak:fix/session-status-unhandled`, só com a mudança de servidor, testes e changeset, e o link está no `DELTA-MANIFEST.md`

**WHEN** `integrall-tech/rustrak` é inspecionado
**THEN** não contém nenhum arquivo de `governance/`, `deploy/`, `e2e/` nem referência a BugLenz

## Delta controlado

**WHEN** `git diff --name-only v0.16.0 main` é filtrado por `apps/server/migrations`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`
**THEN** o resultado é vazio

**WHEN** `git diff --name-only v0.16.0 main` é filtrado por `apps/*/src`
**THEN** aparecem, além dos 16 arquivos do 002, exatamente `models/session.rs` e `workers/session_aggregator.rs`, ambos no `DELTA-MANIFEST.md` como temporários

**WHEN** `ci.yml` roda
**THEN** `web`, `rust-lint`, `rust-test` e `postgres-e2e` seguem verdes, e `rust-test` conta os testes novos
