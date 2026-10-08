# Baseline `v0.16.0-itl.6`

**Data:** 2026-10-08 · **Commit:** `51879f77` (merge do PR #22) · **Roteiro:** `governance/release/publicar-imagem.md`

## O que a imagem traz em relação à `itl.5`

Pacote 023 (endurecimento: senha, tempo de login, sessão, destino de webhook, 413 em JSON), pacote 004
(retenção automática, API e tela), G24 (sessões repetidas contadas uma vez), pacote 021 (dashboard em
português do Brasil), correção da rota de exclusão por titular (404 na `itl.5`) e o `react-doctor`.

## Publicação

| Item | Valor |
|---|---|
| Imagem | `ghcr.io/integrall-tech/buglenz-server:v0.16.0-itl.6` (e `:latest`) |
| Digest | `sha256:7f94994df91079c40c930b033632f9f063a638d7d868a61b447357f07db6377e` |
| Pacote | **privado**, vinculado a `integrall-tech/buglenz`; versões retidas `itl.3` a `itl.6` e `latest` |
| Plataforma e tamanho | `linux/amd64`, 20,5 MB |
| Rótulos OCI | `revision` = `51879f775bf4…` (o commit da tag), `version` = `v0.16.0-itl.6`, `licenses` = GPL-3.0-only, `source`, `vendor` |
| Run de publicação | 37842182578: `image` e `prune` com sucesso |

## Antes da tag (CI de `main` no commit tagueado)

`CI` (web, Rust, PostgreSQL, os dois builds de release), `E2E React`, `Brand` e `CodeQL`: verdes.
`Network conformance`: verde com a **janela de 15 minutos** disparada à mão (ADR-0019). `Third-party
licenses` não rodou no último commit porque nenhuma dependência mudou (filtro por caminho).

## Conferido na imagem publicada

| Verificação | Resultado |
|---|---|
| Contrato dos wrappers (`contract/run.sh … all`) | React 14 e Spring Boot 16 verificações, todas verdes |
| `DELETE /api/projects/{id}/privacy/users/{id}` | **200** (na `itl.5`: 404) |
| `GET /api/retention` | 200 |
| `PUT /api/projects/{id}/retention` com `events_days: 3` / `30` | 400 (abaixo do piso de 7) / 200 |
| Login com senha de 1 100 bytes | 400 |
| Canal de webhook para `http://169.254.169.254/…` | 400 |
| Dashboard embutido | `<title>BugLenz - Error Tracking System`; um chunk `pt`; a única ocorrência de "Rustrak" nele é a atribuição |

## Não conferido

- A primeira passada da retenção na imagem (a stack exige as três variáveis; a imagem sem elas roda e
  só reporta projetos desprotegidos). Conferida no servidor local, não na imagem.
- A imagem só roda sob emulação `amd64` nesta máquina `arm64`; o contrato roda assim.
