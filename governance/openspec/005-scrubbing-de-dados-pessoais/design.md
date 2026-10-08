# 005 — Design

**Base:** `main` após o 006 (`v0.16.0-itl.4`, Rustrak `v0.16.0`). Branch `pkg/005-scrubbing-de-dados-pessoais`.
Decisões do Edson (2026-10-08): configuração por instância; IP nunca gravado; máscaras por
marcador integral; issue no upstream em paralelo.

Tudo [confirmado] na `v0.16.0` salvo indicação.

## 1. Onde o dado é persistido hoje

| Tipo | Processador (`src/digest/processors/`) | Parse | Persistência | IP |
|---|---|---|---|---|
| evento de erro | `event.rs` (`ErrorProcessor`) | `event.rs:141` `let mut event_data: Value = from_slice(...)`; depois `trim_oversized_event`, grouping (`:228`), denormalização, `write_digest` → `EventService::create` | `events.data` | `events.remote_addr` via `DigestWrite.remote_addr` (`:256`) |
| transação | `transaction.rs` | `:18` `let mut data: Value = from_slice(...)` | `transactions.data` + spans filhos (`insert_span`, `insert_root_span`) | `transactions.remote_addr` (`:123`, `ctx.remote_addr`) |
| logs | `logs.rs` | `LogContainer::parse(&work)` → `Vec<LogItem { body: String, attributes: Value, … }>` | `logs.body`, `logs.attributes` | — |
| span (legado) | `span.rs` `parse_item` (`:38` `let mut data`) | `spans.data` | — |
| spans v2 | `span_v2.rs` (`from_slice` → `Value`) | `spans.data` | — |
| sessão | `session.rs` | contadores apenas | `session_counts` | — (`did` é o id distinto, mantido: é o que o release health por usuário usa) |

O IP nasce em `routes/ingest.rs:68-71` (`req.connection_info().realip_remote_addr()`) e viaja em
`ProcessorCtx.remote_addr` / `EventMetadata.remote_addr` até os dois `INSERT`.

## 2. Módulo `scrub` (novo: `apps/server/src/scrub/mod.rs`, `keys.rs`, `text.rs`)

```rust
pub fn scrub_value(value: &mut serde_json::Value)   // em profundidade; idempotente
pub fn scrub_text(text: &str) -> Cow<'_, str>        // máscaras em uma string
```

### 2.1 Chaves (`keys.rs`)

Chave normalizada = minúsculas sem `_`, `-`, espaço e ponto. Duas listas em código:

- **exatas**: `password`, `passwd`, `pwd`, `senha`, `secret`, `token`, `accesstoken`,
  `refreshtoken`, `idtoken`, `authorization`, `auth`, `cookie`, `setcookie`, `session`,
  `sessionid`, `csrf`, `csrftoken`, `xsrftoken`, `apikey`, `privatekey`, `credentials`,
  `creditcard`, `cardnumber`, `cartao`, `cpf`, `cnpj`, `rg`, `ipaddress`, `remoteaddr`,
  `xforwardedfor`, `xrealip`;
- **por conteúdo** (a chave normalizada contém): `password`, `passwd`, `senha`, `secret`,
  `token`, `authorization`, `cookie`, `apikey`, `privatekey`, `creditcard`, `cardnumber`.

`RUSTRAK_SCRUB_EXTRA_KEYS` (lista separada por vírgula) acrescenta chaves **exatas**, normalizadas
do mesmo jeito; lida uma vez (`OnceLock`), pelo próprio módulo, para não tocar `config.rs`.
Nome com prefixo `RUSTRAK_` por coerência com as demais variáveis (zona C do ADR-0006 vale para
identificadores existentes; os novos seguem o padrão do upstream, o que facilita o PR).

Valor de chave negada → a string `"[Filtered]"` (o mesmo marcador do Sentry), seja string,
número, objeto ou lista. Chaves são comparadas em todo nível do JSON. Não há switch para
desligar (I4).

### 2.2 Texto (`text.rs`)

Varreduras manuais sobre cada string do payload, sem `regex` (não é dependência direta do crate;
acrescentar mexeria em `Cargo.toml` e `Cargo.lock`):

| Máscara | Reconhecimento | Validação |
|---|---|---|
| `[cpf]` | 11 dígitos, com ou sem `.`/`-` nas posições usuais, delimitados por não dígito | dígitos verificadores; rejeita sequências repetidas |
| `[cnpj]` | 14 dígitos, com ou sem `.`/`/`/`-` | dígitos verificadores |
| `[cartao]` | 13 a 19 dígitos, com espaços ou hífens entre grupos | Luhn |
| `[email]` | `local@dominio.tld` (local: letras, dígitos, `._%+-`; domínio com ao menos um ponto; TLD ≥ 2 letras) | — |

Ordem: cartão, CNPJ, CPF, e-mail. Aplica-se a toda string, inclusive `context_line` dos frames
e `message`. Chaves que são caminhos de arquivo não são tratadas de forma especial: os
delimitadores e os dígitos verificadores já evitam o falso positivo em hashes de bundle.

### 2.3 Garantias

- Idempotente: aplicar duas vezes dá o mesmo resultado (um marcador não casa com nenhuma regra).
- Determinística: o agrupamento (fingerprint sobre `exception.values[].type/value`,
  `transaction`) roda sobre o payload já limpo, então dois eventos com CPFs diferentes na
  mensagem caem na mesma issue. Esse é um efeito desejado.
- Sem alocação quando não há nada a trocar (`Cow`).

## 3. Pontos de chamada (delta em arquivos do upstream)

| Arquivo | Linha | Mudança |
|---|---|---|
| `routes/ingest.rs` | 68-71 | `let remote_addr: Option<String> = None;` com comentário ADR-0009 — o IP não entra no `ProcessorCtx` nem no `EventMetadata` |
| `digest/processors/event.rs` | após `:141` (parse) | `crate::scrub::scrub_value(&mut event_data);` |
| `digest/processors/transaction.rs` | após `:18` | `crate::scrub::scrub_value(&mut data);` (os spans filhos estão dentro de `data`) |
| `digest/processors/logs.rs` | no laço, antes do `INSERT` | `let body = scrub_text(&log.body)`; `scrub_value(&mut attributes)` |
| `digest/processors/span.rs` | `parse_item`, após `:38` | `scrub_value(&mut data)` |
| `digest/processors/span_v2.rs` | após o parse do container | `scrub_value(&mut data)` |
| `lib.rs` | — | `pub mod scrub;` |

Seis arquivos do upstream, uma a duas linhas cada. Sessões não passam pelo scrub: não carregam
texto livre além de `release`/`environment`, e `did` é o identificador distinto que o SDK já
anonimiza por configuração (camada 1, ADR-0011).

## 4. Exclusão por titular

`DELETE /api/projects/{project_id}/privacy/users/{user_id}` — admin do projeto ou global.
Novo `routes/privacy.rs` + `services/privacy.rs`; registro em `routes/mod.rs`, `main.rs`
(antes de `routes::storage::configure`) e `openapi.rs`; `openapi.json` regenerado.

Serviço, por backend (o padrão de `services/issue.rs::delete_by_filter` e de
`services/storage.rs::execute_cleanup_in_batches`):

1. seleciona `events.id, issue_id` com `user.id` igual — SQLite `json_extract(data, '$.user.id') = $2`,
   PostgreSQL `data->'user'->>'id' = $2` — para uma tabela temporária, em lotes;
2. subtrai de `issues` e de `projects` os contadores (`stored_event_count`,
   `digested_event_count`) como `execute_cleanup_in_batches` faz, no mesmo commit;
3. apaga os eventos do lote; repete;
4. apaga `transactions` com o mesmo `user.id` (sem contadores a ajustar) e seus spans
   (`transaction_id`) — a confirmar em T6 se há FK em cascata;
5. responde `{ "events": n, "transactions": m }`.

`user.id` é comparado como string exata. Issues que ficarem com zero eventos **não** são
apagadas (ficam como histórico sem dado pessoal); registrado como escolha.

## 5. Testes

- `tests/unit/scrub_test.rs`: chaves (exatas, por conteúdo, extras por variável, normalização,
  aninhamento em listas e objetos), máscaras (CPF válido e inválido, formatado e não, CNPJ,
  cartão com e sem Luhn, e-mail, texto misto, idempotência, string sem nada a trocar).
- `tests/integration/scrub_test.rs`: envelope de evento com `request.headers.Cookie`,
  `request.data.password`, `user.email`, `user.ip_address`, mensagem com CPF e e-mail → após o
  digest, `events.data` não contém nenhum dos valores originais, contém `[Filtered]`/`[cpf]`/
  `[email]`, e `events.remote_addr IS NULL`; o mesmo para transação (com span filho), logs e
  span; dois eventos com CPFs diferentes na mensagem caem na mesma issue.
- `tests/integration/privacy_test.rs`: três eventos de dois usuários em duas issues → `DELETE`
  do usuário A remove os dele, contadores das issues e do projeto batem, os do usuário B ficam;
  `403` para membro, `404` para projeto inexistente.
- `e2e-react`: `Sentry.setUser({ id: 'u-1', email: 'ana@example.com' })` e um erro com CPF e
  senha na mensagem; o assert confirma `[email]`/`[cpf]`/`[Filtered]` no evento e `user.id`
  preservado.

## 6. Delta deste pacote

| Arquivo | Zona | ADR | Natureza |
|---|---|---|---|
| `apps/server/src/scrub/{mod,keys,text}.rs`, `src/routes/privacy.rs`, `src/services/privacy.rs` | G | 0009 | novos |
| `apps/server/src/lib.rs`, `src/routes/mod.rs`, `src/main.rs`, `src/openapi.rs`, `openapi.json` | G | 0009 | registro |
| `apps/server/src/routes/ingest.rs`, `src/digest/processors/{event,transaction,logs,span,span_v2}.rs` | G | 0009 | uma a duas linhas cada |
| `apps/server/tests/unit/scrub_test.rs`, `tests/integration/{scrub,privacy}_test.rs`, `tests/unit/mod.rs`, `tests/integration/mod.rs` | G | 0009 | novos / registro |
| `e2e/react-app/src/*`, `scripts/e2e-react-assert.sh` | G | 0009 | cenário de PII |

## 7. Upstream

Issue em `rustrak/rustrak`: "Server-side PII scrubbing before persistence (deny-list keys,
masking, no remote_addr)". Descreve o desenho acima e oferece o PR. Se houver interesse, o PR
sai do fork público com o módulo e os pontos de chamada; se não, fica no fork com este ADR.

## 8. Em aberto

- Opt-in de IP por projeto e lista de negação por projeto (coluna em `projects`, migration
  `_itl`, API, tela): pacote posterior, quando houver caso de uso.
- Logs e spans na exclusão por titular: o protocolo não os liga a `user.id` de forma fixa;
  ficam fora até haver atributo acordado no wrapper (ADR-0011).
- Nomes das chaves negadas vindas de apps Java/Flutter (`senha`, `cpf` cobertos; `documento`,
  `telefone`? decidir com quem responde por LGPD): `RUSTRAK_SCRUB_EXTRA_KEYS` cobre até lá.
