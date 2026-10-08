# 006 — Tarefas

Um commit por tarefa, conventional commits em inglês citando a tarefa. Alteração de arquivo do
upstream entra em `DELTA-MANIFEST.md` no mesmo commit. Os commits de servidor (T2, T3) são os que
vão ao upstream: sem referência a BugLenz, governance ou e2e no código e na mensagem.

- [ ] T0. Branch `pkg/006-compatibilidade-sdk-atual` a partir de `main`; corpus atualizado em `governance/`
- [ ] T1. Confirmar na base: enum e `classify` em `models/session.rs`; `ingest_aggregates`; rotas de eventos e de estatísticas de sessão que o e2e vai consultar; como o `sentry-cli` do plugin faz upload (`/api/0/projects/{org}/{project}/files/...`) contra a instância
- [ ] T2. Servidor: `SessionStatus::Unhandled`, `classify`, `SessionAggregateItem.unhandled`, soma em `ingest_aggregates`; `cargo test` local
- [ ] T3. Testes: unitários (classificação, desserialização) e de integração (sessão e agregado `unhandled` refletidos nas estatísticas); `.changeset` patch para `@rustrak/server`
- [ ] T4. `e2e/react-app/`: app, `vite.config.ts` com o plugin, `run.mjs` (Playwright); build e execução local contra o servidor local; os quatro erros chegam
- [ ] T5. `scripts/e2e-react-assert.sh` e `.github/workflows/e2e-react.yml`; verde no PR; medições de G7/G8 no resumo do job
- [ ] T6. Fork público `integrall-tech/rustrak`; branch `fix/session-status-unhandled` sobre `upstream/main` com T2+T3; PR aberto em `rustrak/rustrak`; link no `DELTA-MANIFEST.md`
- [ ] T7. `DELTA-MANIFEST.md`, `CHANGES-FROM-UPSTREAM.md`; ADR-0005 passo 6 aponta para o job `e2e-react`; `GAP-ANALYSIS.md` G3 marcado como corrigido no fork e proposto ao upstream
- [ ] T8. `governance/baseline/006.md`: resultado do e2e (issues, frames, release health), medições de G7/G8, tempo do job
- [ ] T9. Conferir cada cenário de `specs/sdk.md` e fechar o pacote
