# Mudanças em relação ao upstream

Resumo legível do [`DELTA-MANIFEST.md`](DELTA-MANIFEST.md). O manifesto é a fonte; este
arquivo é derivado dele e atualizado no mesmo commit.

**Base:** Rustrak `v0.16.0` (`4dbe5ce7`), 2026-10-07. Base inicial do fork: `v0.15.2`.

## Comportamento

### Dashboard em português do Brasil (pacote 021)

Novo idioma `pt` (1 328 textos), escolhido sozinho para navegadores `pt-BR` e `pt-PT` e disponível em
**Conta → Idioma** ("Português (Brasil)"). Escrito sem revisão por falante nativo ainda. Os e-mails de
alerta gerados pelo servidor continuam em inglês.

### Tipografia serifada e cartões (ADR-0023)

Os títulos de página usam Instrument Serif (autohospedada, OFL-1.1); os títulos de cartão passam para frase normal; os
cartões ganham raio de 1 rem e sombra suave. Tudo por CSS, sem editar componente: cabeçalhos de tabela e rótulos de
formulário continuam em caixa alta.

### Paleta quente e tema claro como padrão (ADR-0022)

O painel troca os cinzas neutros e o verde-limão por papel quente, tinta e um laranja, com a barra lateral de tinta nos
dois temas, como na tela de referência. **O tema claro é o padrão**; quem já escolheu um tema mantém a escolha. De
passagem, o botão primário claro (texto branco sobre limão, 2,9:1) deixa de ficar abaixo do AA: um teste fixa o
contraste dos dois temas.

### O identificador de usuário das sessões vira pseudônimo (I4)

O SDK monta o `did` da sessão a partir de `user.id` ou, na falta dele, do e-mail, do nome de usuário ou do IP.
O servidor o gravava como veio, em `session_users`. Agora grava um **pseudônimo com chave** (HMAC-SHA256 com a
`SESSION_SECRET_KEY`, prefixo `p1:`): a contagem de usuários distintos não muda, o valor original não fica no
banco. Trocar a `SESSION_SECRET_KEY` muda todos os pseudônimos (os usuários do dia contam de novo). Linhas
antigas, de antes desta versão, continuam com o valor original.

### Retenção automática (pacote 004, ADR-0009)

O upstream só limpa dados por ação manual. Aqui um worker aplica os prazos, a cada 24 h:

- Prazo por tipo, de **7 a 3650 dias** (`events`, `transactions` com seus spans, `logs`), da instância
  (`RUSTRAK_RETENTION_EVENTS_DAYS`, `RUSTRAK_RETENTION_TRANSACTIONS_DAYS`,
  `RUSTRAK_RETENTION_LOGS_DAYS`; sem valor embutido) e por projeto (`PUT /api/projects/{id}/retention`).
- Projeto sem prazo para um tipo não perde esse tipo; a passada avisa em `WARN` e
  `GET /api/retention` o lista como desprotegido.
- As **linhas de sessão** (`session_counts`, `session_users`) e o **histórico de alertas** de um projeto seguem o prazo de eventos dele; a exclusão por titular apaga também as linhas de sessão do titular.
- A stack Swarm do BugLenz traz padrão provisório de 90 dias (erros e logs) e 30 (transações), que se muda por variável.
- A limpeza manual e a tela de Storage continuam como estavam.
- O dashboard ganha **Configurações → Retention** (só administradores): padrões da instância, última
  passada e os prazos de cada projeto, editáveis.

### Sessões repetidas contadas uma vez (G24)

Um SDK que repete o estado final da mesma sessão (o Java, com sessão explícita) fazia o servidor contar
duas quedas e `healthy` negativo. O agregador agora lembra, por `sid`, o que já contou (até 100 000 ids
e 24 h): o primeiro `init` e o primeiro resultado contam; repetições não; um resultado pior depois move
a sessão de contador.

### Endurecimento do servidor (pacote 023, ADR-0018)

Correções de segurança que o upstream ainda não aceitou ([rustrak/rustrak#57](https://github.com/rustrak/rustrak/pull/57)):

- **Senha:** acima de 1024 bytes é recusada com 400 no login, no aceite de convite, na troca de
  senha e no vínculo SSO, antes de qualquer consulta ou Argon2. Não há tamanho mínimo (decisão do
  upstream mantida).
- **Login:** um e-mail inexistente gasta o mesmo Argon2 que uma senha errada, para o tempo de
  resposta não revelar quais e-mails têm conta.
- **Sessão:** login e aceite de convite descartam o que a sessão guardava antes e a renovam.
- **Webhooks:** o destino não pode ser loopback, rede privada (10/8, 172.16/12, 192.168/16),
  link-local (inclui `169.254.169.254`), CGNAT, `localhost` nem nomes `.local`/`.internal`. Vale ao
  salvar o canal e ao enviar (cobre a URL do roteamento da regra). O cliente HTTP não segue
  redirecionamentos. Quem **precisa** de um destino interno lista o host em
  `RUSTRAK_WEBHOOK_ALLOWED_HOSTS` (vírgula; o host como aparece na URL). Canais já salvos com
  destino interno passam a falhar no envio.
- **Ingest:** corpo acima de 100 MB responde 413 em JSON (era texto puro).
- Limite conhecido: o bloqueio olha o host escrito na URL; um nome público que resolve para IP
  interno passa. A política de saída de rede do nó (invariante I3) fecha isso.

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

### Marca BugLenz (pacote 007, ADR-0006)

- A interface, os alertas e a API dizem **BugLenz**: título da aba, logotipo, rodapés, os 68 textos
  de cada catálogo (en, es, fr, ro, zh), modelos de webhook, `actor` e rodapé dos alertas, remetente
  padrão `alerts@buglenz.dev` e título do OpenAPI. A tela "Sobre" traz a atribuição ("BugLenz é um
  fork do Rustrak, GPL-3.0") e um link para o `NOTICE.md` da versão em execução; o link para o
  rastreador de problemas do upstream foi removido.
- Os arquivos do upstream **não** são editados: a marca é aplicada no build, por `brand/` (26
  regras com contagem esperada), e `brand.yml` falha o PR se um merge do upstream mudar uma contagem.
- Não mudam: variáveis `RUSTRAK_*`, métricas `rustrak_*`, pacotes `@rustrak/*`, cabeçalhos
  `X-Rustrak-*` dos webhooks, tabelas, caminhos de API e logs do servidor.
- Logotipo e ícones são **provisórios** até a identidade visual (decisão D9).
- Proposta ao upstream de um nome de produto configurável: rustrak/rustrak#387 (issue; se aceita, as regras de `brand/` encolhem a cada versão).

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
- Proposta ao upstream: rustrak/rustrak#384 (issue; sem PR até haver interesse do mantenedor).

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
