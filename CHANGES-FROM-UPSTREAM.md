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

### Dados pessoais tratados antes de persistir (pacote 005, ADR-0009)

- Todo payload (evento, transação e seus spans, logs, spans avulsos e v2) passa pelo módulo
  `scrub` no digest, antes do agrupamento e de qualquer gravação:
  - valores de chaves negadas (`password`, `senha`, `token`, `authorization`, `cookie`, `secret`,
    `api_key`, `cpf`, `cnpj`, `ip_address`, …; lista completa em `apps/server/src/scrub/keys.rs`)
    viram `"[Filtered]"`; `RUSTRAK_SCRUB_EXTRA_KEYS=chave1,chave2` acrescenta chaves por instância;
  - em texto livre, CPF e CNPJ válidos, números de cartão (Luhn) e e-mails viram `[cpf]`,
    `[cnpj]`, `[cartao]`, `[email]`; identificadores (`*_id`, timestamps, `release`) não são
    mascarados.
- O IP do cliente não é lido na ingestão: `events.remote_addr` e `transactions.remote_addr` ficam
  nulos (as colunas continuam no schema).
- Consequência visível: sem `user.id`, eventos de usuários diferentes contam como **um** usuário
  afetado (o e-mail, que o upstream usava como reserva, está mascarado). O wrapper do SDK define
  `user.id` (ADR-0011).
- `DELETE /api/projects/{id}/privacy/users/{user_id}` (admin): apaga eventos e transações do
  `user.id` informado, com os contadores de issue e projeto ajustados. Issues vazias ficam.
- Proposta ao upstream: issue em rustrak/rustrak (link no manifesto quando aberta).

### Status de sessão `unhandled` (pacote 006, ADR-0011) — proposto ao upstream

- Sessões com status `unhandled` (protocolo 1.6.0; o SDK JavaScript 11.x o envia em vez de
  `crashed` para erros não tratados) são aceitas e contadas como **errored**, e o contador
  `unhandled` dos agregados também. Na `v0.16.0` o item era descartado e o release health de um app
  React que lançou erros mostrava crash-free 100% e zero sessões com erro.
- Mudança idêntica proposta ao upstream em rustrak/rustrak#383; quando entrar, este item sai
  daqui e do manifesto.
- Novo job `e2e-react` na CI: app React 19 minificado com `@sentry/react` 11.5.0 e source maps
  contra o servidor do PR; é o teste de ponta a ponta do ADR-0005.

### Build e implantação (pacote 003, ADR-0012)

- A imagem do servidor é `ghcr.io/integrall-tech/buglenz-server:<vX.Y.Z-itl.N>` (privada,
  `linux/amd64`, PostgreSQL, dashboard embutido), publicada por `release-image.yml` a cada tag do
  fork. Não há imagem SQLite nem imagem `ui` separada; o nome muda de `rustrak-server` para
  `buglenz-server` (zona B).
- `INGEST_DIR` passa a ser `/data/ingest` na imagem, dentro do volume (issue #359 do upstream).
- `deploy/swarm/` traz a stack Swarm + Traefik, `provision.sh` (cria o token de automação pela API,
  já que `RUSTRAK_BOOTSTRAP_TOKEN` ignora o valor informado, #356) e `backup.sh`.
- `THIRD-PARTY-LICENSES.md` é gerado em Linux pela CI e conferido a cada PR.

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
