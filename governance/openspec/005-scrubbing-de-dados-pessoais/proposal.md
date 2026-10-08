# 005 — Scrubbing de dados pessoais

## Por quê

O invariante I4 diz que dado pessoal é tratado antes de persistir: scrubbing no servidor antes da
gravação, para eventos, transações, logs e sessões, e endereço IP não armazenado por padrão. O
Rustrak `v0.16.0` grava o payload do SDK como chegou (`events.data`, `transactions.data`,
`logs.body/attributes`, `spans.data`), grava o IP de origem em `events.remote_addr` e
`transactions.remote_addr`, e trata scrubbing como "Relay-only concern" (G2) [confirmado].

Eventos de erro de aplicações B2B carregam e-mail de usuário, cabeçalhos HTTP com cookie e
authorization, corpo de requisição com CPF e senha, breadcrumbs com URLs e textos digitados
(ADR-0009). Sem a camada no servidor, o invariante depende só de cada app configurar o SDK
direito — e um app que esqueça contamina a instância inteira. É o segundo P0 do GAP e parte do
critério de saída da Fase 1: "sem evento contendo dado da lista de negação em amostragem manual".

## O que muda

- Novo módulo `scrub` no servidor, chamado pelo digest de cada tipo de item **antes** de qualquer
  persistência (e antes do agrupamento, para que o fingerprint seja do dado já limpo):
  - **lista de negação por chave**: valores de chaves como `password`, `senha`, `token`,
    `authorization`, `cookie`, `secret`, `cpf`, `cnpj`, `ip_address` viram `"[Filtered]"`;
    lista padrão em código, acréscimos por instância pela variável `RUSTRAK_SCRUB_EXTRA_KEYS`;
  - **máscaras em texto livre**: CPF, CNPJ (com dígitos verificadores válidos), número de cartão
    (Luhn) e e-mail viram `[cpf]`, `[cnpj]`, `[cartao]`, `[email]` em qualquer string do payload.
- **IP não gravado**: `remote_addr` deixa de ser lido na ingestão; `events.remote_addr` e
  `transactions.remote_addr` ficam nulos; `user.ip_address` cai na lista de negação.
- **Exclusão por titular**: `DELETE /api/projects/{project_id}/privacy/users/{user_id}`
  (admin) apaga eventos e transações cujo `user.id` seja o informado, mantendo os contadores de
  issue e de projeto consistentes como a limpeza de storage faz.
- Teste de ponta a ponta: o job `e2e-react` passa a enviar um evento com e-mail, CPF, senha e
  cookie e verifica que nada disso chega ao banco.
- **Proposta ao upstream** (ADR-0002, G2 "propor ao upstream; implementar no fork se não houver
  interesse"): issue em `rustrak/rustrak` descrevendo a camada, aberta em paralelo; a
  implementação não espera a resposta.

Decisões do Edson (2026-10-08): configuração por **instância** agora (por projeto depois); IP
**nunca** gravado agora (opt-in por projeto depois); máscaras por **marcador integral**; issue
no upstream em paralelo.

## O que não muda

- Nenhuma migration: as colunas `remote_addr` continuam no schema, nulas.
- Dashboard: o valor `[Filtered]` e os marcadores aparecem onde o dado apareceria.
- Nada no caminho síncrono de ingestão: o scrubbing roda no digest (ADR-0009). O dado bruto
  fica em `INGEST_DIR` até o digest — por isso o 003 o pôs sob `/data`, no volume da instância.
- Retenção automática (camada 3 do ADR-0009) é o pacote 004, bloqueado por D6.
- Nenhuma dependência nova no `Cargo.toml`: as máscaras são varreduras manuais, não `regex`.

## Impacto

- ADRs: 0009 (decisão), 0002 (proposta ao upstream), 0011 (o wrapper do SDK continua sendo a
  camada 1; esta é a camada 2).
- **Revisão humana obrigatória** (CONSTITUTION §5): o módulo `scrub` e os pontos de chamada
  tocam scrubbing; o Edson revisa linha a linha antes do merge.
- Delta: um módulo novo; uma linha por processador (5) e uma na rota de ingestão; rota e serviço
  novos para a exclusão; registro em `main.rs`, `routes/mod.rs`, `openapi.rs`. Conflito provável
  em sync só nos pontos de chamada.
- Risco: falso positivo das máscaras (um número de 11 dígitos que seja CPF válido por acaso,
  um e-mail em uma mensagem de erro que o desenvolvedor queria ver). Aceito: é o lado seguro, e a
  validação de dígitos verificadores reduz o ruído.
- Tamanho: M.
