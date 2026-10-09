# 001 — Tarefas

Executado em 2026-10-07 no branch `pkg/001-bootstrap`, PR integrall-tech/buglenz#1, um commit por
tarefa. Evidência dos cenários no relatório do pacote e em `governance/baseline/001.md`.

- [x] T1. Criar o repositório privado e configurar os remotes conforme `design.md`
- [x] T2. Criar `main` a partir de `v0.15.2` e conferir o hash `ff75852c`
- [x] T3. Proteger `main`: PR obrigatório, CI obrigatória, sem force-push — **feito em 2026-10-08** com os rulesets de `.github/rulesets/` (ADR-0020 e ADR-0021): PR, oito checks obrigatórios, sem force-push nem exclusão, e as tags `v*-itl.*` protegidas
- [x] T4. Adicionar `NOTICE.md`
- [x] T5. Adicionar `DELTA-MANIFEST.md` e `CHANGES-FROM-UPSTREAM.md`
- [x] T6. Copiar o corpus para `governance/`
- [x] T7. Acrescentar a seção de governança ao `CLAUDE.md` da raiz e registrar no manifesto
- [x] T8. Remover `release.yml`, `docker-publish.yml`, `deploy-docs.yml` e `FUNDING.yml`; registrar no manifesto
- [x] T9. Rodar `ci.yml` no repositório privado; corrigir apenas o que for de ambiente (segredos, runners) — nada precisou de correção
- [x] T10. Gerar `THIRD-PARTY-LICENSES.md`
- [x] T11. Registrar a baseline em `governance/baseline/001.md`
- [x] T12. Ler issues e PRs abertos do upstream; anotar em `GAP-ANALYSIS.md` quais gaps já têm trabalho em curso
- [x] T13. Conferir cada cenário de `specs/bootstrap.md` e fechar o pacote
