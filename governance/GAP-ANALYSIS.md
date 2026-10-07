# Análise do Rustrak e gaps em relação ao mercado

**Versão:** 0.1 · **Data:** 2026-10-07 · **Autor:** Edson Martins (com Claude)
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
| Testes do servidor | 1.429 funções de teste (unit, integração, e2e com Postgres via testcontainers) | [confirmado] |
| Dashboard | React 19.3, TypeScript 6, Vite 8, Tailwind 4, Base UI, TanStack Router/Table — 61.176 linhas (`apps/dashboard` + `packages/ui`) | [confirmado] |
| Banco | SQLite (padrão) ou PostgreSQL, escolhido em tempo de compilação por feature | [confirmado] |
| Migrations | 45 (PostgreSQL) e 32 (SQLite), em diretórios separados | [confirmado] |
| Toolchain | Rust 1.98 fixado em `rust-toolchain.toml`; Node 22.12+, pnpm 12 | [confirmado] |
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
| G2 | **Sem scrubbing de dados pessoais no servidor.** O IP de origem é gravado em `events.remote_addr` e o payload do SDK é persistido como chegou. | `routes/ingest.rs`, `services/event.rs`; o próprio código cita scrubbing de PII como "Relay-only concern". | [confirmado] |
| G3 | **Status de sessão `unhandled` rejeitado.** O `@sentry/react` 11.5.0 envia esse status; o servidor só aceita `ok`, `exited`, `crashed`, `abnormal`, `errored` e descarta o item. Release health fica errado. | `models/session.rs:63`; log `session item: bad JSON ... unknown variant unhandled`. | [confirmado] |
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
