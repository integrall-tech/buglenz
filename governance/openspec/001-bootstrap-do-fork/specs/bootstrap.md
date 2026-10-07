# 001 — Especificação

## Origem e remotes

**WHEN** `git merge-base main v0.15.2` é executado no repositório do fork
**THEN** o resultado é o commit `ff75852c` da tag `v0.15.2` do upstream

**WHEN** `git remote get-url --push upstream` é executado
**THEN** o valor retornado é `DISABLED`

**WHEN** alguém executa `git push upstream main`
**THEN** o comando falha sem contatar `github.com/rustrak/rustrak`

## Conformidade de licença

**WHEN** `LICENSE` do fork é comparado com o da tag `v0.15.2`
**THEN** os arquivos são idênticos

**WHEN** `NOTICE.md` é lido
**THEN** ele identifica o projeto de origem, a tag, o commit, a licença GPL-3.0-only e o copyright das modificações

## Delta controlado

**WHEN** `git diff --name-only v0.15.2 main` é executado ao fim do pacote
**THEN** todo arquivo modificado ou removido do upstream consta em `DELTA-MANIFEST.md` com ADR associada

**WHEN** a mesma diferença é filtrada por `apps/*/src`, `packages/*/src` e `apps/server/migrations`
**THEN** o resultado é vazio

## Publicação

**WHEN** um commit chega a `main`
**THEN** nenhum workflow publica imagem em registry público, pacote no npm ou site de documentação

## Verificação herdada

**WHEN** `ci.yml` roda em `main`
**THEN** os jobs `web`, `rust-lint`, `rust-test` e `postgres-e2e` terminam com sucesso

**WHEN** a baseline é registrada
**THEN** `governance/baseline/001.md` contém versões de toolchain, contagem de testes por suíte e resultado de cada job
