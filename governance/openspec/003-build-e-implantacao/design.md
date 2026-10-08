# 003 — Design

**Base:** `main` após o 002 (`v0.16.0-itl.2`, Rustrak `v0.16.0`). Branch: `pkg/003-build-e-implantacao`.

Decisões do Edson em 2026-10-08: registry **GHCR privado** (`ghcr.io/integrall-tech/...`), só
**`linux/amd64`**, pacote dividido (valores de D4/D8 ficam fora do repositório).

## 1. Imagem

### 1.1 O que o upstream já dá [confirmado]

`apps/server/Dockerfile`: `ARG FEATURES` (`sqlite` por padrão), `cargo-chef` em
`rust-1.98-slim-bookworm` com digest fixado, release build, distroless `cc-debian12:nonroot`,
`VOLUME /data`, `EXPOSE 8080`, dashboard copiado de `apps/server/static` se o contexto o tiver
(`scripts/bundle-dashboard.sh` copia `apps/dashboard/dist` para lá). O `rust-lint` da CI falha se
o `cargo-chef` e o `rust-toolchain.toml` divergirem.

### 1.2 O que muda no `Dockerfile` (zona B, ADR-0012)

Uma linha no estágio final: `ENV INGEST_DIR=/data/ingest`. A issue #359 do upstream descreve o
problema; o `config.rs` lê `INGEST_DIR` do ambiente e o diretório é criado no boot [confirmado].
Alternativa rejeitada: só na stack — qualquer outro uso da imagem perderia eventos. Candidato a
PR no upstream (ADR-0002), aberto em T-final se o Edson concordar.

### 1.3 Nome, tag, plataforma

| Item | Valor |
|---|---|
| Registry | `ghcr.io/integrall-tech` (privado; plano Free: 500 MB, 1 GB/mês de transferência em pacotes privados) |
| Imagem | `buglenz-server` (zona B) |
| Tags | `vX.Y.Z-itl.N` (a mesma do git, ADR-0005) e `latest` apontando para a última tag publicada |
| Plataforma | `linux/amd64`; `arm64` só por `workflow_dispatch` com input, se um dia for preciso |
| Features | `postgres` apenas (I7) |
| Rótulos OCI | `org.opencontainers.image.source`, `.revision`, `.version`, `.licenses=GPL-3.0-only`, `.title=BugLenz`, `.description` com a origem Rustrak |
| Retenção | as **10** versões mais recentes; as demais apagadas pelo próprio workflow (`actions/delete-package-versions`), para caber na cota |

Imagem distroless do upstream: ~20 MB [inferência a partir do alvo "distroless image under
20MB" do `apps/server/CLAUDE.md`; medir em T5].

## 2. Workflow `release-image.yml` (novo, zona G)

Adaptado do `docker-publish.yml` do upstream removido no 001 (lido na tag) [confirmado]:

```
on:
  push: { tags: ["v*-itl.*"] }
  workflow_dispatch: { inputs: { tag, platforms (default linux/amd64) } }
permissions: { contents: read, packages: write }
jobs:
  image:
    runs-on: ubuntu-latest
    - checkout na tag
    - pnpm + node 24; pnpm install --frozen-lockfile
    - pnpm exec turbo run build --filter=@rustrak/dashboard; bash scripts/bundle-dashboard.sh
    - docker/setup-buildx-action, docker/login-action (ghcr.io, GITHUB_TOKEN)
    - docker/build-push-action: context ./apps/server, build-args FEATURES=postgres,
      platforms linux/amd64, tags <tag> e latest, labels OCI, push
    - resumo no job: digest, tamanho, `docker run --rm <img> --version` se o binário aceitar [a confirmar]
  prune:
    - actions/delete-package-versions: min-versions-to-keep 10
```

Ações pinadas por SHA, como o upstream: `docker/setup-buildx-action` v4.4.1, `docker/login-action`
v4.6.0, `docker/build-push-action` v7.4.0 (SHAs do `docker-publish.yml`);
`actions/delete-package-versions` com SHA lido no T3.

Nenhum segredo novo: `GITHUB_TOKEN` publica no GHCR da organização desde que o pacote seja
vinculado ao repositório na primeira publicação [inferência; T4 confirma, e se a organização
exigir aprovação de pacote, o Edson libera uma vez].

Push para o `upstream` continua impossível (`DISABLED`); o workflow só fala com `ghcr.io`.

## 3. Licenças geradas em Linux: `licenses.yml` (novo, zona G)

A cada `pull_request` e `push` em `main`: `cargo deny list` nos dois crates, `pnpm -r licenses
list`, `governance/tools/third-party-licenses.py`, `git diff --exit-code THIRD-PARTY-LICENSES.md`.
Falha se o arquivo commitado divergir. Em T2 o arquivo é regenerado em Linux (o atual tem os
opcionais `darwin-arm64`) e passa a ser o canônico. O ADR-0005 já manda regenerar a cada sync;
agora a CI cobra.

## 4. Stack Swarm (`deploy/swarm/`, novos, zona B)

### 4.1 `buglenz.stack.yml`

Tradução do `docker-compose.postgres.yml` do upstream [confirmado] para Swarm:

| Serviço | Detalhes |
|---|---|
| `buglenz` | `image: ghcr.io/integrall-tech/buglenz-server:${BUGLENZ_TAG}`; `deploy.replicas: 1` (G20); `update_config.order: stop-first` (fila de ingestão local; duas réplicas não compartilham spool); `environment`: `DATABASE_URL` via secret, `SESSION_SECRET_KEY` via secret, `PUBLIC_URL=https://${BUGLENZ_HOST}`, `SSL_PROXY=true`, `RUSTRAK_METRICS=on`, `RUST_LOG=info`, `INGEST_DIR=/data/ingest`; volume `buglenz_data:/data`; rede `traefik` + rede interna `buglenz` |
| `postgres` | `postgres:16-alpine` (o que a suíte e2e do upstream testa) ou servidor existente (ADR-0012); volume `buglenz_pg`; só na rede interna; `healthcheck` do upstream |
| Traefik (labels em `buglenz`) | router `Host(${BUGLENZ_HOST})` com TLS; **middleware que nega `/metrics`** (`PathPrefix(/metrics)` → `ipAllowList` só da rede interna, ou router separado sem entrypoint público); limite de corpo (`buffering.maxRequestBodyBytes`) e rate limit como segunda barreira |
| Scrape | `/metrics` só pela rede `buglenz`/overlay interna; como o VictoriaMetrics descobre o alvo **[a confirmar com Edson/Neimar]**: labels no serviço ou alvo estático no `vmagent` |
| Logs | stdout; coletor do Swarm para o Loki **[a confirmar: driver Docker ou Promtail/Alloy]** |

Secrets do Swarm (`docker secret create`): `buglenz_database_url`, `buglenz_session_secret`,
`buglenz_pg_password`. O servidor lê `DATABASE_URL` e `SESSION_SECRET_KEY` do ambiente, não de
arquivo [confirmado em `config.rs`]; a stack usa um `entrypoint` fino? **Não**: a imagem é
distroless sem shell. Solução: `environment` com os valores vindos de `.env` do `docker stack
deploy` (ficam no Swarm raft, cifrados em repouso) — ou um init container. Decisão: `.env` +
`docker stack deploy -c buglenz.stack.yml buglenz` lendo variáveis do ambiente do operador;
documentado no README. [inferência: avaliar em T6 se vale propor `*_FILE` ao upstream]

### 4.2 `README.md` da stack

1. Pré-requisitos: Swarm, rede `traefik` externa, credencial de **leitura** do GHCR nos nós
   (`docker login ghcr.io`), DNS do host (D8) apontando para o Traefik.
2. Primeira subida: variáveis, `docker stack deploy`, verificação `GET /health` e `/metrics`
   pela rede interna.
3. Provisionamento **sem `RUSTRAK_BOOTSTRAP_TOKEN`** (#356): `CREATE_SUPERUSER=email:senha` na
   primeira subida; login; `POST /api/tokens` para o token de automação; depois remover
   `CREATE_SUPERUSER` do ambiente. Script `deploy/swarm/provision.sh` com `curl` faz os três
   passos [padrão do script de conformidade do 002, confirmado].
4. Atualização: trocar `BUGLENZ_TAG`, `docker stack deploy` de novo; `stop-first` esvazia o
   spool antes de parar [inferência sobre o comportamento do spool: pendentes são reprocessados
   no boot, ADR-0012].
5. Backup e restauração: `pg_dump` diário do banco e cópia do volume `buglenz_data` (source maps
   e spool). Como o **BackupLenz** consome isso **[a confirmar]**; até lá, `deploy/swarm/backup.sh`
   com `pg_dump | gzip` e `tar` do volume, e o procedimento de restauração testado em T7.

### 4.3 O que fica fora do repositório

`BUGLENZ_HOST` (D8), região/infra da instância interna (D4), senhas, tokens, certificado. Entram
no `.env` do operador e no cofre da IntegrAllTech.

## 5. Baseline 003

`governance/baseline/003.md`: tamanho da imagem, tempo do workflow de publicação, digest da
primeira imagem, tempo do `licenses.yml`, resultado da subida da stack em ambiente de teste
(Swarm local de um nó, `docker swarm init`, em T7) com `/health`, `/metrics` interno e ingestão
de um evento pelo `@rustrak/test-sentry`.

## 6. Delta deste pacote

| Arquivo | Zona | ADR | Natureza |
|---|---|---|---|
| `apps/server/Dockerfile` | B | 0012 | Alterado. `ENV INGEST_DIR=/data/ingest` |
| `.github/workflows/release-image.yml` | G | 0012, 0005 | Novo |
| `.github/workflows/licenses.yml` | G | 0003, 0005 | Novo |
| `deploy/swarm/buglenz.stack.yml`, `deploy/swarm/README.md`, `deploy/swarm/provision.sh`, `deploy/swarm/backup.sh` | B | 0012 | Novos |
| `THIRD-PARTY-LICENSES.md` | G | 0003 | Regenerado em Linux |

## 7. Em aberto (não bloqueia T1–T5)

- Credencial de leitura do GHCR nos nós do Swarm: PAT de uma conta de serviço com `read:packages`,
  ou GitHub App. Escolha do Edson antes da primeira implantação.
- BackupLenz: forma de consumo.
- Descoberta de alvos pelo VictoriaMetrics e coleta de logs para o Loki na infra atual.
- Painel Grafana (`rustrak_ingest_*`, `rustrak_spool_pending`, `rustrak_http_*`) e alertas: entram
  quando a forma de scrape estiver definida; JSON em `deploy/grafana/`.
- D4 e D8: implantação da instância interna.
