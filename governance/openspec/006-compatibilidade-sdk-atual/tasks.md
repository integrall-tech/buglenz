# 006 — Tarefas

Executado em 2026-10-08 no branch `pkg/006-compatibilidade-sdk-atual`, PR integrall-tech/buglenz#6; PR no
upstream rustrak/rustrak#383. Achados em `design.md`, seção 7.

Um commit por tarefa, conventional commits em inglês citando a tarefa. Alteração de arquivo do
upstream entra em `DELTA-MANIFEST.md` no mesmo commit. Os commits de servidor (T2, T3) são os que
vão ao upstream: sem referência a BugLenz, governance ou e2e no código e na mensagem.

- [x] T0. Branch `pkg/006-compatibilidade-sdk-atual` a partir de `main`; corpus atualizado em `governance/`
- [x] T1. Confirmar na base: enum e `classify` em `models/session.rs`; `ingest_aggregates`; rotas de eventos e de estatísticas de sessão que o e2e vai consultar; como o `sentry-cli` do plugin faz upload (`/api/0/projects/{org}/{project}/files/...`) contra a instância
- [x] T2. Servidor: `SessionStatus::Unhandled`, `classify`, `SessionAggregateItem.unhandled`, soma em `ingest_aggregates`; `cargo test` local
- [x] T3. Testes: unitários (classificação, desserialização) e de integração (sessão e agregado `unhandled` refletidos nas estatísticas); `.changeset` patch para `@rustrak/server`
- [x] T4. `e2e/react-app/`: app, `vite.config.ts` com o plugin, `run.mjs` (Playwright); build e execução local contra o servidor local; os quatro erros chegam
- [x] T5. `scripts/e2e-react-assert.sh` e `.github/workflows/e2e-react.yml`; verde no PR; medições de G7/G8 no resumo do job
- [x] T6. Fork público `integrall-tech/rustrak`; branch `fix/session-status-unhandled` sobre `upstream/main` com T2+T3; PR aberto em `rustrak/rustrak`; link no `DELTA-MANIFEST.md`
- [x] T7. `DELTA-MANIFEST.md`, `CHANGES-FROM-UPSTREAM.md`; ADR-0005 passo 6 aponta para o job `e2e-react`; `GAP-ANALYSIS.md` G3 marcado como corrigido no fork e proposto ao upstream
- [x] T8. `governance/baseline/006.md`: resultado do e2e (issues, frames, release health), medições de G7/G8, tempo do job
- [x] T9. Conferir cada cenário de `specs/sdk.md` e fechar o pacote
