# 005 — Tarefas

Executado em 2026-10-08 no branch `pkg/005-scrubbing-de-dados-pessoais`, PR integrall-tech/buglenz#7. Achados em `design.md`, seção 9.

Um commit por tarefa, conventional commits em inglês citando a tarefa. Alteração de arquivo do
upstream entra em `DELTA-MANIFEST.md` no mesmo commit. **Revisão linha a linha do Edson** no PR
(CONSTITUTION §5): módulo `scrub`, pontos de chamada, serviço de exclusão.

- [x] T0. Branch `pkg/005-scrubbing-de-dados-pessoais` a partir de `main`; corpus em `governance/`
- [x] T1. Confirmar na base os pontos de chamada da seção 3 do `design.md` e o padrão de contadores de `storage.rs`/`issue.rs`
- [x] T2. Módulo `scrub`: chaves (`keys.rs`), texto (`text.rs`), `scrub_value`; testes unitários; `cargo test --test unit_tests`
- [x] T3. Pontos de chamada nos cinco processadores e `remote_addr = None` na ingestão; `cargo test` completo (SQLite e PostgreSQL e2e)
- [x] T4. Teste de integração `scrub_test.rs` (evento, transação, logs, span; `remote_addr` nulo; agrupamento com CPFs diferentes)
- [x] T5. Exclusão por titular: serviço por backend com ajuste de contadores, rota admin, OpenAPI regenerado; `privacy_test.rs`
- [x] T6. `e2e-react`: usuário com e-mail e mensagem com CPF/senha; assert dos marcadores e do `user.id` preservado; verde no PR
- [x] T7. Issue de proposta em `rustrak/rustrak` (aberta como #384 em 2026-10-08, texto aprovado pelo Edson); link no manifesto
- [x] T8. `DELTA-MANIFEST.md`, `CHANGES-FROM-UPSTREAM.md` (o que o operador vê: `[Filtered]`, marcadores, IP nulo, variável `RUSTRAK_SCRUB_EXTRA_KEYS`, endpoint); `deploy/swarm/README.md` cita a variável
- [x] T9. `governance/baseline/005.md` (testes, custo do scrub medido no digest, e2e); GAP G2 marcado; conferir cada cenário de `specs/scrub.md`
