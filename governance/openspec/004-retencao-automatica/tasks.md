# 004 — Tarefas

- [x] T1. Migration `project_retention` (SQLite e PostgreSQL) e modelo
- [x] T2. `services/retention.rs`: leitura dos padrões, prazo efetivo, leitura e escrita por projeto, validação (testes primeiro)
- [x] T3. `workers/retention.rs`: passada, relatório, loop; teste de que apaga o antigo, mantém o novo e não toca projeto sem prazo
- [x] T4. (rotas registradas antes do escopo genérico de projetos; guarda no `e2e-react`) `routes/retention.rs` (`GET /api/retention`, `PUT /api/projects/{id}/retention`), OpenAPI e `openapi.json`
- [x] T5. Ligar o worker em `main.rs`; stack Swarm exige as três variáveis; documentação do operador
- [x] T6. `DELTA-MANIFEST.md`, baseline `governance/baseline/004.md`
- [ ] T7. Revisão humana (§5: retenção)
- [x] T8. Tela de configuração no dashboard (cinco catálogos; zh, ro, fr e es sem revisão por falante nativo) — feita em 2026-10-08
- [ ] T9. **(decisão D6)** Prazos padrão validados por quem responde por LGPD; configurar a instância do piloto
