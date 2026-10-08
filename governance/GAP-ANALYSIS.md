# Análise do Rustrak e gaps em relação ao mercado

**Versão:** 0.2 · **Data:** 2026-10-07 · **Autor:** Edson Martins (com Claude)
**Referência medida:** `rustrak/rustrak` tag `v0.15.2` (`ff75852c`, 2026-09-30); `main` em `98a1f861` (13 commits à frente)

Marcação de procedência: **[confirmado]** = lido no código ou reproduzido em teste nesta análise;
**[inferência]** = conclusão minha a partir do que foi confirmado; **[não verificado]** = não consegui checar.

---

## 1. Resumo

O Rustrak cumpre o papel de "Crashlytics para web": no teste de ponta a ponta, um app React 19
minificado, com o `@sentry/react` oficial, gerou issues agrupadas e com stack trace resolvido para
arquivo, linha e linha de código originais. A UI já é React 19 + TypeScript, e já existe SSO OIDC.

O que falta se divide em três grupos:

1. **Bloqueios de adoção (P0):** retenção automática, tratamento de dados pessoais, um defeito de
   compatibilidade que zera o release health com o SDK atual, e interface sem pt-BR.
2. **Qualidade de triagem (P1):** nomes de função deslocados após o source map, `in_app` não
   reclassificado, ausência de `tunnel` e de filtros de entrada, alertas só por evento.
3. **Cobertura (P2):** mobile nativo (dSYM, R8, símbolos Dart), anexos, feedback do usuário via
   SDK, session replay.

O risco estrutural não é técnico: o projeto tem 9 meses, um mantenedor dominante e licença GPL-3.0.

## 2. O que foi medido

| Dimensão | Valor | Procedência |
|---|---|---|
| Servidor | Rust (Actix-web 4, SQLx, Tokio) — 37.681 linhas em `src/`, 41.685 em `tests/` | [confirmado] |
| Testes do servidor | 1.412 atributos de teste por contagem estática; **1.351 executados e aprovados, 59 ignorados, 0 falhas** com SQLite na CI do fork (Rust 1.98.1); 19 e2e com PostgreSQL 16. JavaScript: 1.127 testes. Detalhe em `governance/baseline/001.md` | [confirmado por execução, 2026-10-07] |
| Dashboard | React 19.3, TypeScript 6, Vite 8, Tailwind 4, Base UI, TanStack Router/Table — 61.176 linhas (`apps/dashboard` + `packages/ui`) | [confirmado] |
| Banco | SQLite (padrão) ou PostgreSQL, escolhido em tempo de compilação por feature | [confirmado] |
| Migrations | 45 (PostgreSQL) e 32 (SQLite), em diretórios separados | [confirmado] |
| Toolchain | Rust 1.98 fixado em `rust-toolchain.toml`; Node 22.12+ (CI usa 24); `pnpm@12.6.0` em `packageManager` na `v0.15.2`, `pnpm@12.10.1` na `v0.16.0`. `cargo deny check licenses` passa em `apps/server`; em `packages/benchmarks` falha só por `license-not-encountered`, e o upstream não o roda lá | [confirmado] |
| Idiomas da UI | en, fr, ro, es, zh — 1.298 chaves em `en.json` | [confirmado] |
| Superfície de marca | 4.303 ocorrências de `rustrak` em 527 arquivos | [confirmado] |
| Histórico | primeiro commit em 2026-01-22; 1.144 commits; 17 autores | [confirmado] |
| Concentração | 311 de 488 commits (64%) dos últimos 90 dias são de um autor | [confirmado] |
| Cadência | 23 releases estáveis entre 2026-07-20 e 2026-09-30, cerca de uma a cada 3 dias | [confirmado] |
| Licença | GPL-3.0-only (servidor); `@rustrak/client` e `@rustrak/mcp` declaram GPL-3.0; sem CLA | [confirmado] |

## 3. Teste de ponta a ponta

Feito nesta análise, com o servidor compilado do código-fonte (SQLite, sem dashboard).

- App: React 19.3 + Vite, build de produção minificado, `@sentry/react` 11.5.0.
- Source maps enviados pelo `@sentry/vite-plugin` 5.4.1 (artifact bundle com debug ID).
- Erros disparados em Chromium headless: exceção em handler de clique (2×), promise rejeitada,
  erro de render capturado por `Sentry.ErrorBoundary`.

| Verificação | Resultado |
|---|---|
| Upload de source maps pelo plugin oficial | Funcionou sem ajuste (`org` é ignorado, `project` = slug) |
| Ingestão do envelope pelo browser | 6 requisições, todas 200 |
| Agrupamento | 4 erros → 3 issues; os dois erros iguais caíram na mesma issue (`event_count` 2) |
| Arquivo e linha originais | Corretos (`src/pedido.js:2`, `src/main.jsx:16`) |
| Linha de código de contexto | Presente e correta |
| Usuário, release, environment, breadcrumbs | Presentes no evento |
| Nome da função após source map | **Deslocado em um frame** (ver G7) |
| `in_app` | **Todos os frames `true`**, inclusive `node_modules/react-dom` (ver G8) |
| Release health | **Sessão descartada**: `crash_free` 100% e `errored` 0 apesar dos 4 erros (ver G3) |
| Telemetria | Log de boot: "Telemetry is off: no telemetry key was compiled into this binary" |

Limites do teste: compilei com Rust 1.97 (o 1.98 fixado não estava disponível aqui); banco SQLite,
não PostgreSQL; build de debug, então não medi memória nem latência. Não testei o dashboard.

## 4. O que já vem pronto

Todos [confirmado] por código ou documentação do repositório.

- Protocolo Sentry: endpoints `envelope` e `store`; itens `event`, `transaction`, `span` (legado e
  Spans v2), `log`, `session`, `sessions`; gzip, deflate, brotli e zstd.
- Ingestão em duas fases (grava em disco e responde; digest assíncrono), com recuperação de
  pendentes após reinício.
- Agrupamento determinístico com fingerprint customizado e paridade declarada com o Sentry.
- Source maps por artifact bundle e debug ID; API de releases compatível com `sentry-cli`.
- Issues: resolver, silenciar, atribuir, prioridade, comentários, favoritos, atividade, regressão
  automática, busca textual em tipo, valor, transação e culprit.
- Release health (sessões e usuários crash-free), performance (p95, waterfall de spans), logs
  estruturados, traces de agentes de IA.
- Alertas em `new_issue`, `regression` e `unmute` para Slack, e-mail (SMTP), webhook e webhook
  com corpo em template; fila de retentativa.
- Usuários: papéis globais Admin/Member, papéis por projeto Viewer/Editor/Admin, convite por link.
- SSO OIDC (authorization code + PKCE, descoberta, domínios permitidos, auto-provisionamento).
- Rate limit global e por projeto, persistido no banco.
- `/metrics` em formato Prometheus (desligado por padrão, sem autenticação).
- API REST com OpenAPI, cliente TypeScript e servidor MCP.

## 5. Gaps

### P0 — resolver antes de receber dados de produção

| # | Gap | Evidência | Procedência |
|---|---|---|---|
| G1 | **Sem retenção automática.** A limpeza é manual, por API ou tela de Storage. | Só há workers de sessão, montagem de source map, retentativa de alerta e recuperação de ingestão; o FAQ diz "There is no automatic retention yet". O README afirma o contrário. | [confirmado] |
| G2 | **Sem scrubbing de dados pessoais no servidor.** O IP de origem é gravado em `events.remote_addr` e o payload do SDK é persistido como chegou. **Corrigido no fork pelo pacote 005** (módulo `scrub` no digest, IP não gravado, exclusão por titular); proposto ao upstream (rustrak/rustrak#384). | `routes/ingest.rs`, `services/event.rs`; o próprio código cita scrubbing de PII como "Relay-only concern". | [confirmado] |
| G3 | **Status de sessão `unhandled` rejeitado.** `unhandled` é status do protocolo de sessões desde a 1.6.0 ("erro não tratado, processo não terminou"), também como contador nos agregados; o SDK JS 11.x o envia em vez de `crashed` (getsentry/sentry-javascript#22475). O servidor só aceita `ok`, `exited`, `crashed`, `abnormal` e `errored` e descarta o item. Release health fica errado. **Corrigido no fork pelo pacote 006** (classificado como errored) e proposto ao upstream em rustrak/rustrak#383. | `models/session.rs:63`; log `session item: bad JSON ... unknown variant unhandled`. develop.sentry.dev/sdk/telemetry/sessions; `@sentry/core` 11.5.0 `session.d.ts:28`. | [confirmado] |
| G4 | **Saída de rede herdada.** Relatório anônimo a cada 6 h para `us.i.posthog.com`; o dashboard consulta `rustrak.github.io/rustrak/versions.json` a partir do navegador de cada usuário. | `telemetry/posthog.rs:10`; `shared/api/version-check.ts:8`. A telemetria fica inativa em build sem `RUSTRAK_TELEMETRY_KEY`, que é o caso de um fork. | [confirmado] |
| G5 | **Sem pt-BR.** | `shared/i18n/messages/`. | [confirmado] |
| G6 | **Sem criptografia em repouso e sem trilha de auditoria.** | FAQ ("Rustrak doesn't add additional encryption"); nenhuma tabela ou serviço de auditoria. | [confirmado] |

### P1 — qualidade de triagem e operação

| # | Gap | Evidência | Procedência |
|---|---|---|---|
| G7 | **Nome de função deslocado após source map.** O frame em `pedido.js:2` (dentro de `calcularTotalPedido`) aparece como `jl`; o frame chamador recebe `calcularTotalPedido`. Arquivo e linha estão certos; o nome é o do símbolo no ponto da chamada. O culprit da issue herda o nome minificado. | Teste (seção 3); `services/sourcemap.rs:1062`. | [confirmado] |
| G8 | **`in_app` não é reclassificado.** O servidor só lê o valor enviado pelo SDK; frames de `node_modules` contam como código da aplicação. | Teste; `in_app` só aparece em `services/grouping.rs:667`. | [confirmado] |
| G9 | **Sem endpoint `tunnel`.** A autenticação exige `sentry_key` na query ou no header; o DSN do cabeçalho do envelope não é usado. Bloqueadores de anúncio barram o envio direto do browser. | `auth/extractors.rs:124-150`. | [confirmado]; o impacto é [inferência] |
| G10 | **Sem filtros de entrada nem origens permitidas.** CORS aceita qualquer origem; não há filtro de extensões, localhost ou crawlers. | `middleware/cors.rs`. | [confirmado] |
| G11 | **Alertas só por evento.** Não há alerta por limiar, frequência ou queda de crash-free. Sem Telegram, Teams ou WhatsApp nativos (possível via webhook com template). | `models/alert.rs:56`; `services/notification/`. | [confirmado] |
| G12 | **SSO sem mapeamento de grupos para papéis**; um provedor por instância, configurado por variável de ambiente; convites não enviam e-mail. | `auth/oidc.rs` usa só issuer, subject e e-mail. | [confirmado] |
| G13 | **Tokens de API sem escopo** (herdam as permissões do usuário); uma chave DSN por projeto. | `models/auth_token.rs`, `models/project.rs:185`. | [confirmado] |
| G14 | **Busca limitada.** Sem filtro por tag, usuário, release ou ambiente na listagem de issues. | `pagination/mod.rs:120`. | [confirmado] |
| G15 | **Sem integração com rastreadores de tarefa** (Jira, GitHub, GitLab), suspect commits ou code owners. | Nenhuma ocorrência no servidor. | [confirmado] |

### P2 — cobertura de plataforma

| # | Gap | Evidência | Procedência |
|---|---|---|---|
| G16 | **Itens de envelope ignorados:** `attachment`, `feedback`/`user_report`, `check_in`, `profile`, `replay_event`, `replay_recording`, `client_report`. Feedback do usuário existe só por REST. | `ingest/envelope.rs:97-125`; `routes/ingest.rs:182`. | [confirmado] |
| G17 | **Sem symbolication nativa:** dSYM (iOS), ProGuard/R8 (Android), símbolos Dart, minidump. Flutter com `--obfuscate` e crashes nativos chegam ilegíveis. | Nenhuma ocorrência no servidor. | ausência [confirmado]; efeito [inferência] |
| G18 | **Sem session replay.** | FAQ. | [confirmado] |
| G19 | **Sem multi-organização.** "Rustrak is designed for single-team use." | FAQ. | [confirmado] |

### P3 — escala e sustentação

| # | Gap | Evidência | Procedência |
|---|---|---|---|
| G20 | **Nó único.** Fila de ingestão e source maps em disco local, digest dentro do processo; sem armazenamento de objetos. | `INGEST_DIR`, `SOURCEMAP_STORAGE_PATH`. | [confirmado]; "sem HA" é [inferência] |
| G21 | **Eventos em uma tabela única com `data JSONB`, sem particionamento.** | migration inicial do PostgreSQL. | [confirmado] |
| G22 | **Risco de upstream:** 9 meses de vida, versão 0.x com quebra em minor, sem `SECURITY.md`, financiado por GitHub Sponsors. | repositório. | [confirmado] |
| G23 | **GPL-3.0-only**, inclusive no cliente TypeScript. | `Cargo.toml`, `packages/client/package.json`. | [confirmado] |
| G24 | **Terminais repetidos da mesma sessão são contados mais de uma vez.** `ingest_session` não guarda estado por `sid`; cada atualização terminal incrementa um contador. O `sentry-spring-boot` 8.60.0 com sessão explícita envia `crashed` duas vezes para a mesma sessão (na queda e no `endSession`): `total 1, crashed 2, healthy -1, crash-free -1.0`. O SDK JavaScript envia um terminal só, por isso o G3 não mostrou. Afeta qualquer SDK que repita o terminal (Java, provavelmente Android e Flutter). Evidência em `governance/baseline/009-t1.md` | `workers/session_aggregator.rs` (`ingest_session`); execução de 2026-10-08 | [confirmado] |

Não consegui ler as issues e PRs abertos do upstream (o README lista 40 e 9); a página bloqueia
leitura automatizada. Vale uma passada manual para ver quais gaps acima já têm trabalho em curso.
[não verificado]

## 6. Posição em relação ao mercado

Colunas de terceiros vêm de documentação pública e do meu conhecimento, não de teste; "?" marca o
que não sei afirmar. [inferência]

| Capacidade | Crashlytics | Sentry | GlitchTip | Bugsink | Rustrak 0.15.2 |
|---|---|---|---|---|---|
| Web/React | Não | Sim | Sim | Sim | Sim |
| Source maps | — | Sim | Sim | Sim | Sim (G7, G8) |
| Crash-free por release | Sim | Sim | Não | Não | Sim (G3) |
| Alerta por velocidade/limiar | Sim | Sim | Sim | ? | Não (G11) |
| Performance/tracing | Separado | Sim | Básico | Não | Sim |
| Session replay | Não | Sim | Não | Não | Não (G18) |
| Mobile nativo simbolizado | Sim | Sim | ? | Não | Não (G17) |
| Scrubbing de PII no servidor | — | Sim | Parcial | Não | Não (G2) |
| Retenção automática | Sim (90 dias) | Sim | Sim | Sim | Não (G1) |
| Multi-organização | Por projeto Firebase | Sim | Sim | Times | Não (G19) |
| SSO | Conta Google | Sim | Sim | ? | OIDC (G12) |
| Self-hosted leve | Não | Não | Médio | Sim | Sim |

O Rustrak já iguala ou supera GlitchTip e Bugsink em release health, performance e logs. Perde
para os dois em retenção, organizações e maturidade, e para o Sentry em tudo que depende do Relay
(scrubbing, filtros, tunnel) e em mobile.

## 7. Encaixe na stack da IntegrAllTech

| Item da stack | Situação | Procedência |
|---|---|---|
| React 19 + TypeScript | SDK oficial `@sentry/react` funcionou no teste | [confirmado] |
| Mantine v9 + Archbase | A UI do Rustrak usa Tailwind + Base UI. Archbase só entra do lado dos apps, como wrapper do SDK | [confirmado] / [inferência] |
| Java 21 + Spring Boot 3 | `sentry-spring-boot` aponta para o DSN; stack trace Java não precisa de symbolication | [inferência] |
| Flutter | `sentry_flutter` funciona para erros Dart sem ofuscação; builds ofuscados e crashes nativos esbarram em G17 | [inferência] |
| PostgreSQL 15+ | Suportado por feature de build; o compose do upstream usa `postgres:16`. Versão mínima não documentada | [confirmado] / [não verificado] |
| Docker Swarm + Traefik | Um container, sem estado além do volume `/data`; a documentação cobre proxy reverso | [confirmado] |
| VictoriaMetrics/Grafana/Loki | `/metrics` Prometheus; logs em stdout | [confirmado] |
| ArchGuard | OIDC genérico; falta mapear grupos para papéis (G12) | [inferência] |
| LGPD e soberania | G1, G2, G4 e G6 são o trabalho obrigatório | [inferência] |

## 8. Decisões que os gaps pedem

1. G1, G2, G4: entram como invariantes da CONSTITUTION e viram os primeiros pacotes após o bootstrap.
2. G3, G7, G8, G9: são correções genéricas; a proposta é contribuí-las ao upstream (ADR-0002).
3. G17: decidir se os apps Flutter entram na Fase 1. Se sim, o custo do fork sobe bastante.
4. G18, G19: fora de escopo; multi-cliente se resolve com uma instância por cliente (ADR-0007).

## 9. Trabalho em curso no upstream (leitura de 2026-10-07; PR da IntegrAllTech em 2026-10-08)

Leitura das issues e PRs abertos de `rustrak/rustrak` pela API do GitHub (`gh api`), feita no
pacote 001 (T12): 40 issues e 7 PRs abertos. Títulos e corpos lidos; nenhum PR foi testado.
A relação com cada gap é [inferência] a partir do texto, salvo indicação.

### Gaps com trabalho em curso

| Gap | Item no upstream | Estado | Relação |
|---|---|---|---|
| G1 retenção | #329 "Postgres: unusable CHAR indexes, unbounded span queries, and manual-only retention" (issue, set/2026) | aberta, sem comentários | Relata a ausência de retenção automática como problema de operação; não há PR. Reforça G1 e G21 |
| G1 retenção | #93 "source map storage retention and cleanup policy" (issue, mai/2026) | aberta | Retenção só de source maps |
| G4 egress | #375 "derive the telemetry window from lifetime counters" (issue, out/2026, do mantenedor) | aberta | O upstream segue investindo na telemetria anônima; confirma a classificação "fork" de G4 no ADR-0002 |
| G10 filtros | #348 "Add project-wide environment filtering" (PR, set/2026, externo) | aberto, 4 comentários, inclui migrations | Filtro de ambiente na consulta, não filtro de entrada. Toca migrations: atenção no próximo sync se for mesclado |
| G10 filtros | #170 "Issues — Phase 6 (snooze/ignore, merge, sharing, event filters)" (issue) | aberta | Filtros de evento planejados, sem PR |
| G11 alertas | #371 "alert rules honor a minimum issue level" (PR, out/2026, externo) e #370 (issue) | aberto, 2 comentários | Primeira condição de alerta além de "todo evento". Alerta por limiar ou frequência continua sem trabalho |
| G11 alertas | #367 "scope cooldown per issue instead of per rule" (issue) | aberta | Refinamento do cooldown |
| G12 OIDC → papéis | #355 "OIDC: map IdP groups to roles and allow SSO-only login" (issue, set/2026) | aberta, sem comentários | **Pedido idêntico ao G12.** Candidato natural a PR da IntegrAllTech no upstream (ADR-0002, ADR-0008) |
| G12 OIDC | #365 "Hide or adapt password change for SSO-only accounts", #358 "Explicit admin on SSO login on an empty database", #360 "Don't abort startup when OIDC discovery fails" (issues) | abertas | Mesma frente; relevantes ao pacote de ArchGuard |
| G13 tokens | #357 "Store API tokens hashed and then show them only once" (issue) | aberta | Segurança de token, não escopo |
| G14 busca | #369 "look up project events by user or request ID" (PR, set/2026, externo) e #350 (issue) | aberto, 2 comentários | Busca por identidade de usuário ou request ID, não filtro por tag/release/ambiente |
| G15 integrações | #10 "extensible integrations system (GitHub, Linear, Jira)", #310 "Forge integration" (issues) | abertas desde jan e set/2026 | Sem PR |
| G16 itens de envelope | #164 "Sentry Crons (monitor check-in ingestion)" (PR do mantenedor, jun/2026) | aberto, 3 comentários, parado desde set | `check_in`; os demais itens de #143 seguem abertos |
| G18 replay | #121 "Session Replay support" (issue) | aberta | Fora do escopo da Fase 1 |
| G20 nó único | #94 "pluggable storage backends (S3, GCS, Azure Blob)", #128 "RFC: custom S3-compatible storage server" (issues) | abertas | Sem PR |
| G21 tabela de eventos | #329 (acima), #202 "chore: postgres 18" (issue) | abertas | #329 traz medições reais de um deployment com 23 M de spans: índices `CHAR(n)` ignorados, consultas de span sem limite |
| G22 risco de upstream (**verificado em 2026-10-08: H-1, H-2, H-4 e M-2 presentes no fork; H-3 já corrigido; M-1 a conferir; ver ADR-0018 e pacote 023**) | #163 "v1.0.0 — Definition of Done" (issue) | aberta | Critério de 1.0 em discussão |
| G22 risco de upstream | #57 "fix(security): address 6 server vulnerabilities (H-1 … M-2)" (PR do mantenedor, mai/2026) | **aberto há 5 meses**, 1 comentário | Corrige oráculo de tempo no login, DoS por tamanho de senha e outros. Verificar no pacote de auditoria se as correções entraram por outro caminho; se não, é candidato a cherry-pick com ADR |

### Gaps sem trabalho em curso

G3: **PR da IntegrAllTech aberto em 2026-10-08** (rustrak/rustrak#383, pacote 006).


G2 scrubbing, G5 pt-BR, G6 criptografia e auditoria, G7 nome de função,
G8 `in_app`, G9 tunnel, G17 symbolication, G19 multi-organização, G23 GPL. Nenhuma issue ou PR
aberto toca esses pontos. Para G3, G7, G8 e G9 (classificados como "PR no upstream" no ADR-0002)
o caminho está livre; para G5, idem.

### Itens sem gap correspondente, relevantes aos pacotes seguintes

| Item | Pacote afetado | Por quê |
|---|---|---|
| #359 "Default `INGEST_DIR` is outside the `/data` volume in the Docker image" (bug) | 003 implantação (ADR-0012) | Eventos aceitos e não digeridos se perdem ao recriar o container. A imagem do fork deve definir `INGEST_DIR` sob `/data` |
| #356 "`RUSTRAK_BOOTSTRAP_TOKEN` ignores its value and prints the token to stderr" (bug) | 003 implantação | Afeta o provisionamento automatizado |
| #366 "Revoke existing sessions when a password changes", #47 "Auth rate-limiting", #48/#55 validação de força de senha (PR externo de abr/2026 parado) | auditoria de segurança | Pontos de autenticação sem correção no upstream |
| #362 "Setting to stop members from creating projects" | papéis (ADR-0008) | Controle de permissão que o ArchGuard não resolve sozinho |
| #347 "perf(dashboard): adopt TanStack Query and trim route bundles" (PR do mantenedor), #335 (issue) | 007 rebrand, 010 UI | Refatoração grande do dashboard em andamento; o inventário da zona A (ADR-0006) deve ser refeito após o merge |
| #92 "ui: allow deploying on another basePath", #36 "Standalone mode for WebView UI" | 003 implantação | Relevante se a instância for servida sob um prefixo |

### Observações sobre o upstream

- Dos 7 PRs abertos, 3 são do mantenedor (#347, #164, #57) e 4 são externos (#371, #369, #348,
  #55). O PR externo mais antigo (#55, abr/2026) está parado há cinco meses; os três de set/out
  de 2026 têm resposta do mantenedor em comentários. [confirmado pelos metadados]
- 36 das 40 issues não têm comentário. A triagem por rótulos parou em set/2026: as 13 issues
  mais recentes não têm rótulo. [confirmado pelos metadados]
- Isso sustenta a regra do ADR-0002 de manter no fork, com ADR, o que ficar parado no upstream
  por mais de 30 dias.
