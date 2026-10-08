# Mudanças em relação ao upstream

Resumo legível do [`DELTA-MANIFEST.md`](DELTA-MANIFEST.md). O manifesto é a fonte; este
arquivo é derivado dele e atualizado no mesmo commit.

**Base:** Rustrak `v0.16.0` (`4dbe5ce7`), 2026-10-07. Base inicial do fork: `v0.15.2`.

## Comportamento

### Telemetria e checagem de versão removidas (pacote 002, ADR-0004)

A instância não inicia nenhuma conexão por conta própria (CONSTITUTION I3). O que mudou em
relação ao upstream, visível para quem opera:

- O relatório anônimo para `us.i.posthog.com` **não existe mais no código**. Não há chave a
  compilar, nem switch: `RUSTRAK_TELEMETRY` e `DO_NOT_TRACK` são ignoradas se estiverem no
  ambiente. O log de boot não fala em telemetria.
- `GET /api/telemetry/preview` responde `404`, como qualquer rota inexistente sob `/api`.
- O dashboard não consulta `rustrak.github.io/rustrak/versions.json` e não mostra aviso de nova
  versão. O switch de build `VITE_RUSTRAK_VERSION_CHECK_ENABLED` não é lido.
- `/metrics` (Prometheus, `RUSTRAK_METRICS=on`) e os contadores que o alimentam continuam
  iguais aos da `v0.16.0`.
- A coluna `installation.telemetry_id` continua no schema, sem uso; migrations não mudam.
- O workflow `network-conformance.yml` verifica, a cada PR e push, que nenhum token de
  `scripts/egress-denylist.txt` aparece no código, no binário ou no bundle, e que o servidor,
  rodando sob bloqueio de saída, não tenta nenhuma conexão.

A documentação do upstream (`README.md` §Telemetry, `apps/docs/content/configuration/telemetry.mdx`,
`upgrading/0-15.mdx`) continua descrevendo o recurso; ela não se aplica a este fork e não foi
editada (a marca e os textos são do pacote 007).

Fora disso, o servidor, o dashboard e os pacotes `@rustrak/*` são os da tag base. Nenhuma
migration foi tocada.

## Conformidade de licença

- `NOTICE.md` adicionado: origem, copyright do upstream e das modificações, oferta de
  código-fonte (GPL-3.0 §6). `LICENSE` permanece inalterado.
- `THIRD-PARTY-LICENSES.md` adicionado: inventário das licenças das dependências (428 crates no
  servidor, 248 nos benchmarks, 1.277 pacotes npm), gerado por script em `governance/tools/`.

## CI e publicação

- Removidos os workflows que publicam fora do repositório: `release.yml` (npm),
  `docker-publish.yml` (Docker Hub) e `deploy-docs.yml` (GitHub Pages), e o `FUNDING.yml`.
  Nenhum commit em `main` publica imagem, pacote ou site. O build de imagem para o registry
  privado da IntegrAllTech é do pacote 003.
- Mantidos `ci.yml`, `codeql.yml` e `rust-security.yml`, inalterados.

## Governança

- `DELTA-MANIFEST.md` e este arquivo adicionados.
- `governance/` adicionado com o corpus de governança do BugLenz (CONSTITUTION, análise de gaps,
  ADRs, RFCs, pacotes OpenSpec). Fica fora da raiz para não colidir com arquivos que o upstream
  venha a criar. A baseline medida do upstream na tag está em `governance/baseline/001.md`.
- `CLAUDE.md` da raiz: seção final apontando para `governance/`, o manifesto e o `NOTICE.md`.
  É a única alteração em arquivo de documentação do upstream.
