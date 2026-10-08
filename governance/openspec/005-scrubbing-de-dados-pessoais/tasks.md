# 005 — Tarefas

Um commit por tarefa, conventional commits em inglês citando a tarefa. Alteração de arquivo do
upstream entra em `DELTA-MANIFEST.md` no mesmo commit. **Revisão linha a linha do Edson** no PR
(CONSTITUTION §5): módulo `scrub`, pontos de chamada, serviço de exclusão.

- [ ] T0. Branch `pkg/005-scrubbing-de-dados-pessoais` a partir de `main`; corpus em `governance/`
- [ ] T1. Confirmar na base os pontos de chamada da seção 3 do `design.md` e o padrão de contadores de `storage.rs`/`issue.rs`
- [ ] T2. Módulo `scrub`: chaves (`keys.rs`), texto (`text.rs`), `scrub_value`; testes unitários; `cargo test --test unit_tests`
- [ ] T3. Pontos de chamada nos cinco processadores e `remote_addr = None` na ingestão; `cargo test` completo (SQLite e PostgreSQL e2e)
- [ ] T4. Teste de integração `scrub_test.rs` (evento, transação, logs, span; `remote_addr` nulo; agrupamento com CPFs diferentes)
- [ ] T5. Exclusão por titular: serviço por backend com ajuste de contadores, rota admin, OpenAPI regenerado; `privacy_test.rs`
- [ ] T6. `e2e-react`: usuário com e-mail e mensagem com CPF/senha; assert dos marcadores e do `user.id` preservado; verde no PR
- [ ] T7. Issue de proposta em `rustrak/rustrak` (texto para aprovação do Edson antes de abrir); link no manifesto
- [ ] T8. `DELTA-MANIFEST.md`, `CHANGES-FROM-UPSTREAM.md` (o que o operador vê: `[Filtered]`, marcadores, IP nulo, variável `RUSTRAK_SCRUB_EXTRA_KEYS`, endpoint); `deploy/swarm/README.md` cita a variável
- [ ] T9. `governance/baseline/005.md` (testes, custo do scrub medido no digest, e2e); GAP G2 marcado; conferir cada cenário de `specs/scrub.md`
