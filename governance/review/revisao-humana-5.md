# Roteiro da revisão humana (CONSTITUTION §5)

**Data:** 2026-10-08 · Para: Edson (substituto: Neimar Chagas)

A CONSTITUTION §5 exige revisão humana, linha a linha, do que toca autenticação, scrubbing e
retenção. Três áreas estão pendentes. Este roteiro diz **onde olhar, o que conferir e onde eu tomei
uma decisão que é sua**. Cerca de 1 400 linhas de Rust no total, mais os testes; estimo de 2 a 3 horas.

Marcação: **[decisão]** = escolha minha que você pode reverter; **[limite]** = o que o código não faz.

---

## A. Scrubbing e exclusão por titular (pacote 005, já em `main`)

Garantia a conferir: **nenhum dado da lista de negação nem CPF/CNPJ/cartão/e-mail chega ao banco, e o
IP do cliente nunca é gravado** (ADR-0009, invariantes I3/I4).

| Arquivo | Linhas | O que é |
|---|---|---|
| `apps/server/src/scrub/keys.rs` | 128 | listas de chaves negadas e regra de identificador |
| `apps/server/src/scrub/text.rs` | 270 | máscaras de texto e validação de dígitos |
| `apps/server/src/scrub/mod.rs` | 53 | percurso recursivo do JSON |
| `apps/server/src/digest/processors/{event,transaction,span,span_v2,logs}.rs` | 26 | os cinco ganchos |
| `apps/server/src/routes/ingest.rs` | 1 | `remote_addr` passa a `None` |
| `apps/server/src/services/privacy.rs`, `routes/privacy.rs` | 195 | exclusão por titular |

**Conferir**

1. `keys.rs`: as listas `EXACT` e `CONTAINS` cobrem o que os seus produtos enviam? Falta algo como
   `bearer`, `jwt`, `otp`, `pin`, `cnh`, `renavam`, `pix`, `matricula`? (Cada instância acrescenta
   chaves por `RUSTRAK_SCRUB_EXTRA_KEYS`, mas o padrão vale para todas.) A exceção `tokens` (contagem,
   `max_tokens`) está certa?
2. `text.rs`: leia `find_runs` e `classify`. Os limites (`is_alphanumeric`, no máximo um separador
   entre dígitos) e a validação (dígitos verificadores de CPF/CNPJ, Luhn de 13 a 19) fazem o que a
   especificação diz? Há 22 vetores em `buglenz-sdk/shared/text-vectors.json`, conferidos contra este código.
3. Ganchos: o scrub roda **antes** de agrupar e gravar, e **de novo depois** da reescrita por source
   map? Em `event.rs` há duas chamadas (linhas 144 e 163): confirme que as duas estão onde deveriam.
4. Exclusão por titular: compara `user.id` como texto exato.

**[limite]** O que o scrub **não** pega:
- CPF/CNPJ em formatos fora dos reconhecidos (ex.: `529 982 247 25` com espaços), telefone, RG, nome,
  endereço, IP escrito em texto livre. Só e-mail, CPF, CNPJ e cartão válidos são mascarados no texto.
- O payload bruto fica em `INGEST_DIR` **sem tratamento** entre o recebimento e o digest (ADR-0009:
  o diretório precisa ficar em volume da instância, não em `/tmp` compartilhado).
- A exclusão por titular apaga eventos e transações; **logs e spans de um titular não** são
  selecionados por `user.id`.
- A rota de exclusão **respondia 404 na imagem `v0.16.0-itl.5`** (escondida pelo escopo de projetos);
  a correção está no PR #14. Antes dele, a exclusão por titular não funciona.

---

## B. Endurecimento do servidor (pacote 023, já em `main`)

Garantia a conferir: login não revela quais e-mails existem, senha gigante não custa Argon2,
webhook não alcança a rede interna.

| Arquivo | O que olhar |
|---|---|
| `models/user.rs` | `check_password_length` (1024 bytes), `run_dummy_password_verify` e o hash fixo |
| `routes/auth.rs` `login` | ordem: tamanho → busca → verificação fictícia se não existe → `session.clear()` e `renew()` |
| `routes/auth.rs` `accept_invitation`, `services/invitation.rs`, `services/users.rs` | o mesmo limite em convite, troca de senha e vínculo SSO |
| `services/notification/destination.rs` (101) | `check_url_with`: o que conta como interno |
| `services/notification/{webhook,custom_webhook,mod}.rs` | a checagem ao salvar, ao enviar e `redirect(Policy::none())` |
| `routes/ingest.rs` | `read_limited`: corpo lido no handler com o limite de 100 MB |

**Conferir**

1. O hash fixo de `DUMMY_PASSWORD_HASH` tem os mesmos parâmetros do `Argon2::default()` usado em
   `hash_password`? (Se a biblioteca mudar o padrão, o tempo volta a diferir; o teste de temporização
   avisa.)
2. `destination.rs`: a lista de faixas (`127/8`, `10/8`, `172.16/12`, `192.168/16`, `169.254/16`,
   `100.64/10`, `0/8`, IPv6 `::1`, `fc00::/7`, `fe80::/10`, IPv4 mapeado) e os nomes (`localhost`,
   `.localhost`, `.local`, `.internal`).

**[decisão]** Sem tamanho mínimo de senha (o upstream tem teste que fixa isso). Quer um mínimo?
**[decisão]** `RUSTRAK_WEBHOOK_ALLOWED_HOSTS` existe e vem vazia; o agente de triagem (ADR-0014) vai precisar dela.
**[decisão]** O cliente HTTP dos notificadores não segue mais redirecionamentos.

**[limite]**
- Um nome público que resolve para IP interno passa; fecha-se com a política de saída de rede do nó.
- A sessão é um cookie assinado (`CookieSessionStore`): `session.clear()/renew()` descarta o estado
  anterior ao login, mas **o logout não revoga um cookie roubado**; ele vale até expirar.
- O teste de temporização compara medianas e pode oscilar em máquina muito carregada.

---

## C. Retenção automática (pacote 004, PR #14)

Garantia a conferir: **só apaga o que passou do prazo, e só dos tipos que têm prazo**. É destrutivo
e irreversível.

| Arquivo | Linhas | O que olhar |
|---|---|---|
| `migrations/*/20261008000000_project_retention.up.sql` | 9 | tabela e `ON DELETE CASCADE` |
| `services/retention.rs` | 238 | leitura das variáveis, prazo efetivo, validação (1..3650), `set`/`all` |
| `workers/retention.rs` | 196 | a passada: quem recebe cada `execute_cleanup` |
| `routes/retention.rs` | 133 | portão de administrador |
| `main.rs` | — | ordem das rotas (antes do escopo de projetos) e início do worker |

**Conferir**

1. `parse_days`: `0`, negativo, texto, vazio e acima de 3650 **não** viram padrão (teste unitário).
2. `run_once`: para cada tipo com prazo, uma chamada `execute_cleanup(projeto, dias, filtro só desse tipo)`.
   Um tipo sem prazo **não** é chamado. Confirme que `CleanupFilter` de cada passo marca só um tipo.
3. `execute_cleanup` (já existente, `services/storage.rs`) também apaga as **issues que ficaram sem
   evento**; isso vale para a passada automática.
4. Os dois `PUT`/`GET` exigem `actor.is_admin()`.

**[decisão]** Sem valor embutido: sem as variáveis, nada é apagado e os projetos aparecem como
desprotegidos. A stack Swarm do BugLenz **exige** as três variáveis.
**[decisão]** A passada roda 60 s depois de **cada** partida. Uma variável digitada com valor pequeno
demais (por exemplo `1`) apaga quase tudo na primeira partida, sem prévia. O mínimo é 1 dia. **Quer
um modo "só relatar" na primeira passada depois de mudar um prazo, ou um piso por tipo (por exemplo
7 dias)?** É a decisão que eu mais gostaria de ver sua.

**[limite]**
- O relatório da última passada vive em memória (some ao reiniciar).
- A passada e a limpeza manual podem rodar juntas (são idempotentes, só disputam carga).
- Os prazos reais dependem da D6 (responsável por LGPD); o ADR-0009 propõe 90/30/90 dias.

---

## Como rodar o que eu rodei

```bash
cd apps/server
cargo test --locked                       # SQLite: unitários, integração, e2e
cargo test --locked --no-default-features --features postgres --test e2e_tests   # precisa de Docker
cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings
```

Testes que valem abrir primeiro: `tests/unit/scrub_test.rs`, `tests/integration/scrub_test.rs`,
`tests/integration/security_test.rs`, `tests/unit/destination_test.rs`,
`tests/integration/retention_test.rs`. Cada correção do 023 e da retenção tem teste que falhou antes.

## O que registrar ao terminar

Uma linha por área em `governance/baseline/`: quem revisou, quando, o que mudou por causa da revisão.
