# 006 — Design

**Base:** `main` após o 003 (`v0.16.0-itl.3`, Rustrak `v0.16.0`). Branch `pkg/006-compatibilidade-sdk-atual`.
Decisão do Edson (2026-10-08): fork público do upstream em `integrall-tech/rustrak` para o PR.

## 1. O protocolo e o SDK [confirmado]

| Fonte | O que diz |
|---|---|
| develop.sentry.dev, *Sessions* | `status`: `ok`, `exited`, `crashed`, `abnormal`, `unhandled` (1.6.0: "an unhandled error occurred but the process did not terminate"); `errored` **não** é status, é classificação derivada ("`ok` com `errors > 0`") |
| idem, *Session Aggregates* | item `sessions`: `aggregates[]` com `started`, `did`, `exited`, `errored`, `crashed`, `abnormal`, **`unhandled`** (1.6.0, "count of sessions with `unhandled` status") |
| `@sentry/core` 11.5.0, `session.d.ts:28` | `type SessionStatus = 'ok' \| 'exited' \| 'crashed' \| 'abnormal' \| 'unhandled'` |
| `@sentry/core` 11.5.0, `client.js` | `_unhandledSessionStatus = "crashed"` na base; o cliente browser passa a `unhandled` ("feat(browser)!: Send session status `unhandled` instead of `crashed` for unhandled errors", #22475) |
| Rustrak `v0.16.0`, `models/session.rs:61-68` | `enum SessionStatus { Ok, Exited, Crashed, Abnormal, Errored }` (`Errored` aceito embora não exista no protocolo) |
| `ingest/envelope.rs:112` | status desconhecido → "session item: bad JSON, treating as Other" e o item é ignorado |
| `workers/session_aggregator.rs:148-176` | `classify()`: `Crashed`→crashed, `Abnormal`→abnormal, `Errored`→errored, `Exited\|Ok` com `errors>0`→errored, senão healthy |
| `models/session.rs:41-46` | `SessionAggregateItem { started, did?, exited, errored, crashed, abnormal }` — sem `unhandled`; serde ignora o campo e o número se perde |

## 2. Mudança no servidor (zona G → delta temporário, até o upstream aceitar)

| Arquivo | Mudança |
|---|---|
| `apps/server/src/models/session.rs` | `SessionStatus::Unhandled` (serde `lowercase` já cobre); `is_terminal()` continua `!= Ok` (o protocolo chama `unhandled` de terminal); `classify()`: `Unhandled → SessionOutcome::Errored`; `SessionAggregateItem.unhandled: i64` com `#[serde(default)]` |
| `apps/server/src/workers/session_aggregator.rs` | em `ingest_aggregates`, `entry.errored += item.unhandled` (ao lado de `errored`) |
| `apps/server/tests/unit/...` | `classify(Unhandled, 0) == Errored`; desserialização de `{"status":"unhandled"}` e de agregado com `unhandled: 2` |
| `apps/server/tests/integration/...` | envelope com item `session` status `unhandled` → `GET` das estatísticas de sessão do projeto mostra `errored = 1`, `crashed = 0`; idem para `sessions` agregado |
| `.changeset/<nome>.md` | `"@rustrak/server": patch` — convenção do upstream para o PR; idêntico no fork para o merge futuro ser limpo |

**Por que "errored" e não "crashed"** [inferência, a confirmar com o upstream no PR]: o protocolo
define `unhandled` como "o processo não terminou"; crash-free mede processos que terminaram.
Contar como crash inflaria o crash-free negativamente; ignorar (comportamento atual) o inflaria
positivamente. "Errored" é o que o Sentry já faz com `ok`+`errors>0`, e o SDK envia `errors ≥ 1`
junto com `unhandled` (`client.js:476`).

Nada no dashboard muda: as telas de release health já mostram errored/crashed/abnormal.

## 3. PR no upstream

```
gh repo fork rustrak/rustrak --org integrall-tech --clone=false   # público; é o upstream + patch
git remote add itl-fork git@github.com:integrall-tech/rustrak.git
git checkout -b fix/session-status-unhandled upstream/main
# cherry-pick dos commits de servidor do pkg/006 (sem governance/, sem e2e/)
git push itl-fork fix/session-status-unhandled
gh pr create --repo rustrak/rustrak ...
```

Base do PR: `upstream/main` (não a tag), para o mantenedor aplicar direto. Commit em inglês, teste
junto (convenção do upstream). Título: `fix(sessions): accept the unhandled session status
(protocol 1.6.0)`. Corpo cita o protocolo, o changelog do SDK e o efeito no release health. Se o
upstream pedir `crashed` em vez de `errored`, o fork segue o upstream no sync seguinte. Prazo do
ADR-0002: 30 dias parado → fica no fork com ADR (0011 cobre).

O fork público `integrall-tech/rustrak` **não recebe nada do BugLenz**: só branches de PR para o
upstream. Isso o mantém fora do I11 e sem obrigação nova de GPL (é o código do upstream).

## 4. Teste de ponta a ponta: `e2e/react-app/` + `.github/workflows/e2e-react.yml` (zona G)

### 4.1 O app

`e2e/react-app/`, **fora do workspace pnpm** (`pnpm install --ignore-workspace`, lockfile próprio),
para não tocar `pnpm-workspace.yaml` nem `pnpm-lock.yaml` do upstream.

- Vite 8 + React 19 + TypeScript, `@sentry/react` **11.5.0** fixado, `@sentry/vite-plugin` 5.x.
- `Sentry.init({ dsn, release: 'e2e-react@1.0.0', environment: 'e2e', tracesSampleRate: 0 })`;
  sessão automática do SDK browser (padrão).
- Quatro gatilhos, os do GAP §3: botão com `throw` em handler de clique (clicado 2×), botão com
  promise rejeitada, botão que faz um componente lançar no render dentro de `Sentry.ErrorBoundary`.
  Código em arquivos separados (`src/pedido.ts`, `src/App.tsx`) para o source map ter o que
  provar.
- `vite.config.ts`: `sourcemap: true`; plugin do Sentry com `url: process.env.SENTRY_URL`,
  `authToken`, `project: process.env.SENTRY_PROJECT` (slug), `org` qualquer (ignorado pelo
  servidor [confirmado no GAP]), `telemetry: false`.
- `e2e/react-app/run.mjs`: Playwright (Chromium) abre `vite preview`, clica os quatro botões,
  espera o flush do SDK, fecha a página (dispara o fim da sessão).

### 4.2 O workflow

```
on: pull_request, push: {branches: [main, next, sync/**]}, workflow_dispatch
jobs: e2e-react (ubuntu-latest, ~8 min):
  checkout; setup-rust-toolchain (cache); cargo build --locked (SQLite, debug)
  sobe o servidor (CREATE_SUPERUSER, SESSION_SECRET_KEY, RUSTRAK_DASHBOARD=off, INGEST_DIR temp)
  provisiona pela API: login → POST /api/projects {name: e2e-react, platform: javascript-react}
    → POST /api/tokens (reaproveita o padrão de scripts/network-conformance.sh e deploy/swarm/provision.sh)
  pnpm + node 24; cd e2e/react-app && pnpm install --ignore-workspace --frozen-lockfile
  SENTRY_URL=http://127.0.0.1:8080 SENTRY_AUTH_TOKEN=… SENTRY_PROJECT=e2e-react pnpm build   # upload dos maps
  pnpm exec playwright install --with-deps chromium; node run.mjs http://127.0.0.1:4173 <DSN>
  verifica (scripts/e2e-react-assert.sh, curl + python3):
    GET /api/projects/{id}/issues            → 3 issues; a do clique com event_count 2
    GET /api/issues/{id}/events/latest       → frames com filename terminando em src/pedido.ts
                                                e linha original; context_line presente
    GET /api/projects/{id}/sessions/...      → errored ≥ 1, crashed = 0 (o item unhandled contou)
    grava em $GITHUB_STEP_SUMMARY: nome de função do frame (G7) e in_app dos frames de
      node_modules (G8), como medição para o pacote 010
```

Rotas exatas de eventos e de estatísticas de sessão: confirmar em T1 (`routes/events.rs`,
`routes/sessions.rs`); o GAP as exercitou pela API em 2026-10-07.

O servidor roda com o binário do PR, então o job prova a correção de G3 e falha se um sync futuro
a perder. Sem bloqueio de rede aqui: isso é do `network-conformance`. `@sentry/vite-plugin` usa o
`sentry-cli`, que fala só com `SENTRY_URL` (a instância local); sua telemetria própria fica
desligada por `telemetry: false` [a confirmar no log do upload].

### 4.3 Como o ADR-0005 passa a usar

Passo 6 do ciclo de sync: "teste de ponta a ponta com app React minificado e source map" = este
job, que roda no push para `sync/**`. A tarefa T-final atualiza o ADR-0005 com o nome do job.

## 5. Delta deste pacote

| Arquivo | Zona | ADR | Natureza |
|---|---|---|---|
| `apps/server/src/models/session.rs`, `apps/server/src/workers/session_aggregator.rs` | G | 0011, 0002 | alterados; **temporário**: saem do manifesto quando o PR do upstream entrar por sync |
| `apps/server/tests/unit/*`, `apps/server/tests/integration/*` (arquivos a nomear em T2) | G | 0011 | alterados/novos; idem |
| `.changeset/*.md` | G | 0002 | novo; idem |
| `e2e/react-app/**`, `scripts/e2e-react-assert.sh`, `.github/workflows/e2e-react.yml` | G | 0005, 0011 | novos; permanentes |

## 6. Em aberto

- Se o upstream classificar `unhandled` de outra forma, o fork acompanha.
- Pinagem do `@sentry/react` no app e2e: 11.5.0 agora; a atualização segue o ADR-0011 (só após o
  e2e passar com a versão nova — este mesmo job, com o pin trocado).
- D2 não afeta: o fork público do upstream é código do upstream.
