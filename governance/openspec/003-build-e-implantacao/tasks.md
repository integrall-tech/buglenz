# 003 — Tarefas

Executado em 2026-10-08 no branch `pkg/003-build-e-implantacao`, PR integrall-tech/buglenz#5. T4 (primeira
publicação) acontece na primeira tag após o merge. Achados em `design.md`, seção 8.

Um commit por tarefa, conventional commits em inglês citando a tarefa. Toda alteração de arquivo
do upstream entra em `DELTA-MANIFEST.md` no mesmo commit.

- [x] T0. Branch `pkg/003-build-e-implantacao` a partir de `main` (= `v0.16.0-itl.2`); copiar o corpus atualizado para `governance/`
- [x] T1. `Dockerfile`: `ENV INGEST_DIR=/data/ingest`; build local da imagem `postgres` com dashboard embutido; medir tamanho; registrar no manifesto
- [x] T2. `licenses.yml`: regenerar `THIRD-PARTY-LICENSES.md` em Linux (container ou CI) e commitar como canônico; workflow com drift check verde no PR
- [x] T3. `release-image.yml`: build + push para `ghcr.io/integrall-tech/buglenz-server` por tag `v*-itl.*` e `workflow_dispatch`; ações pinadas por SHA; prune mantendo 10 versões
- [x] T4. Primeira publicação: tag `v0.16.0-itl.3` após o merge (o `push: tags` dispara o workflow da própria tag); conferir que o pacote ficou **privado** e vinculado ao repositório; digest na baseline
- [x] T5. `deploy/swarm/buglenz.stack.yml` + `README.md` + `provision.sh` + `backup.sh`, parametrizados; nenhum valor real
- [x] T6. Subir a stack em Swarm local de um nó com a imagem publicada (login no GHCR com token de leitura), Traefik de teste ou acesso direto na overlay: `/health` 200, `/metrics` só interno, evento ingerido via `@rustrak/test-sentry`, `provision.sh` cria admin e token sem `RUSTRAK_BOOTSTRAP_TOKEN` **Ressalva da auditoria de 2026-10-09:** o evento foi enviado com `curl`, não com `@rustrak/test-sentry`; o login no GHCR nos nós (`--with-registry-auth`) não foi testado; e `/metrics` só interno foi conferido só com `docker stack config`, sem Traefik real. Falta repetir no Swarm de produção.
- [x] T7. `backup.sh` + restauração em banco limpo; issue reaparece após restaurar
- [x] T8. `DELTA-MANIFEST.md`, `CHANGES-FROM-UPSTREAM.md`; `governance/baseline/003.md`
- [x] T9. Conferir cada cenário de `specs/deploy.md` e fechar o pacote; listar o que ficou "a confirmar" (BackupLenz, scrape, logs, credencial de pull, D4, D8)
