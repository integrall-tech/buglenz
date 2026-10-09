# 003 — Especificação

> **Emenda de 2026-10-09 (auditoria).** O cenário "o delta é o mesmo do 002, sem arquivo novo" valia **no fim do pacote 003**. Dois pontos ainda dependem do Swarm de produção: `/metrics` negado ao público (só conferido com `docker stack config`) e o serviço se chama `server` na stack `buglenz` (`buglenz_server`). A limpeza do GHCR conta versões: cada release ocupa três, então o limite passou de 10 para 30 (cerca de dez releases).

## Imagem

**WHEN** uma tag `vX.Y.Z-itl.N` é publicada no repositório
**THEN** o workflow `release-image.yml` publica `ghcr.io/integrall-tech/buglenz-server:vX.Y.Z-itl.N` e move `latest` para ela, para `linux/amd64`, compilada com `--features postgres`

**WHEN** a imagem é inspecionada (`docker buildx imagetools inspect` ou `docker inspect`)
**THEN** os rótulos `org.opencontainers.image.source`, `revision`, `version` e `licenses=GPL-3.0-only` estão presentes e `revision` é o commit da tag

**WHEN** o pacote `buglenz-server` é consultado no GHCR
**THEN** sua visibilidade é **privada** e ele está vinculado ao repositório `integrall-tech/buglenz`

**WHEN** existem mais de 10 versões publicadas
**THEN** o job `prune` apaga as mais antigas, mantendo 10

**WHEN** o container sobe sem `INGEST_DIR` no ambiente
**THEN** o spool fica em `/data/ingest`, dentro do volume `/data`

**WHEN** `strings` é aplicado ao binário dentro da imagem
**THEN** não contém `posthog` nem `versions.json` (o 002 vale para a imagem)

## Licenças

**WHEN** `licenses.yml` roda em um PR
**THEN** regenera `THIRD-PARTY-LICENSES.md` em Linux e falha se o arquivo commitado divergir

## Stack

**WHEN** `docker stack deploy -c deploy/swarm/buglenz.stack.yml buglenz` roda em um Swarm com as variáveis definidas
**THEN** o serviço `buglenz` sobe com uma réplica, `GET /health` responde `200` pelo Traefik e o DSN gerado carrega `PUBLIC_URL`

**WHEN** `GET /metrics` é feito pelo host público
**THEN** a resposta é `403` ou `404` (negada no Traefik); pela rede interna, `200` com `rustrak_ingest_*`

**WHEN** `provision.sh` roda contra a instância recém-subida
**THEN** existe um admin, um token de API foi criado via `POST /api/tokens` e `RUSTRAK_BOOTSTRAP_TOKEN` não foi usado

**WHEN** um evento é enviado pelo `@rustrak/test-sentry` para o DSN
**THEN** a issue aparece em `GET /api/projects/{id}/issues`

**WHEN** `BUGLENZ_TAG` muda e a stack é reimplantada
**THEN** a réplica antiga para antes de a nova subir (`stop-first`) e eventos pendentes em `/data/ingest` são digeridos pela nova

## Backup

**WHEN** `backup.sh` roda
**THEN** produz um `pg_dump` comprimido e um arquivo do volume `/data` com data no nome

**WHEN** o dump é restaurado em um banco vazio e o volume é recolocado
**THEN** a instância sobe e a issue do teste anterior está lá

## Delta controlado

**WHEN** `git diff --name-only v0.16.0 main` é filtrado por `apps/*/src`, `packages/*/src` e `apps/server/migrations`
**THEN** o resultado é o do 002, sem arquivo novo

**WHEN** o repositório é pesquisado por hosts, senhas e tokens reais (`BUGLENZ_HOST`, `POSTGRES_PASSWORD`, `SESSION_SECRET_KEY`, `ghp_`)
**THEN** só há variáveis e exemplos; nenhum valor real
