# ADR-0009 — Dados pessoais: scrubbing, IP e retenção

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

[confirmado] O Rustrak grava o payload do SDK como chegou (`events.data`), grava o IP de origem em
`remote_addr`, não aplica scrubbing no servidor e não tem retenção automática. Eventos de erro de
aplicações B2B carregam e-mail de usuário, cabeçalhos HTTP, corpo de requisição e breadcrumbs com
URLs e textos digitados.

## Decisão

Três camadas, todas obrigatórias:

**1. Nos SDKs (ADR-0011).** `sendDefaultPii` desligado; `beforeSend` e `beforeBreadcrumb` padrão no
wrapper; identificação de usuário por id interno, não por e-mail ou documento.

**2. No servidor, antes de persistir (invariante I4).** Etapa de scrubbing no digest, para
eventos, transações, logs e spans:
- remoção de valores cujas chaves casem com lista de negação (`password`, `senha`, `token`,
  `authorization`, `cookie`, `secret`, `cpf`, `cnpj`, `cartao`…), configurável por instância;
- mascaramento por padrão de CPF, CNPJ, cartão e e-mail em texto livre;
- `remote_addr` e `user.ip_address` não gravados, salvo opção explícita por projeto.

**3. Retenção automática (invariante I5).** Worker diário que aplica o prazo por projeto e por
tipo de dado, reaproveitando a lógica de limpeza já existente em `services/storage.rs`. Padrão
proposto: 90 dias para eventos e logs, 30 para transações e spans. Source maps seguem o ciclo de
vida do release.

## Complementos

- Exclusão por titular: endpoint administrativo que apaga eventos de um `user.id` em uma instância.
- Criptografia em repouso é responsabilidade da infraestrutura (volume e PostgreSQL).
- Trilha de auditoria de ações administrativas (G6) fica para pacote posterior.

## Consequências

O scrubbing roda fora do caminho síncrono de ingestão, então não afeta a latência do SDK.
[inferência] O dado bruto fica em `INGEST_DIR` até o digest; esse diretório precisa estar em
volume da instância, não em `/tmp` compartilhado. Os prazos padrão precisam de validação com quem
responde por LGPD na IntegrAllTech.

## Nota de 2026-10-08

No `@sentry/react` 11.x a opção `sendDefaultPii` foi substituída por `dataCollection`, que coleta tudo por padrão (usuário, cookies, cabeçalhos, corpos, query strings). A camada 1 do wrapper React desliga cada categoria. No Spring Boot, `sentry.send-default-pii=false` continua válido, mas o SDK cru ainda envia e-mail em `user` e `extra`; por isso o wrapper também filtra o evento.

## Nota de 2026-10-08 (D6)

Os prazos padrão de retenção passam a ter valor na stack de implantação: **90 dias para erros (eventos) e
logs, 30 dias para transações e spans**, os números propostos acima. Foram definidos pelo dono do projeto,
por delegação, como **padrão provisório**: o texto original dizia que precisavam de validação de quem
responde por LGPD, e isso continua valendo. O servidor segue sem valor embutido (nada é apagado sem
variável ou prazo por projeto); o padrão vive em `deploy/swarm/buglenz.stack.yml`. O piso é de 7 dias.

## Nota de 2026-10-09 (sessões)

A auditoria contra a CONSTITUTION (I4 vale para eventos, transações, logs **e sessões**) achou que o `did` da
sessão era gravado como o SDK o mandava, e o SDK JavaScript o monta de `user.id || user.email || user.username`.
Agora é gravado como **pseudônimo com chave** (HMAC-SHA256 com a `SESSION_SECRET_KEY`, prefixo `p1:`), e não
como texto mascarado, porque `[email]` juntaria todos os e-mails num único usuário e quebraria a contagem de
usuários distintos. Quem faz a exclusão por titular de sessões usa a mesma função (`scrub::pseudonym`).

Na mesma auditoria, a retenção (I5) passou a cobrir também `session_counts`, `session_users` e `alert_history`, que
seguem o prazo de eventos do projeto, e a exclusão por titular passou a apagar as linhas de `session_users` do
titular.

## Nota de 2026-10-09 (campos de id, segunda rodada da auditoria)

O scrubber tratava toda chave terminada em `id` como identificador e não mascarava o valor. O motivo é válido (um id de
span só de dígitos que passe no Luhn viraria `[cartao]` e colidiria com os vizinhos), mas o SDK JavaScript monta
`user.id` a partir do **e-mail** quando não há id, e o e-mail era gravado em claro, também em `customer_id` e afins.
Agora o valor de uma chave `*id` passa **só pela máscara de e-mail**; CPF, CNPJ e cartão continuam sem máscara ali, porque
um número em campo de id é quase sempre um id (e a máscara por dígito verificador colidiria). `release`, `dist` e os
carimbos de tempo seguem isentos de tudo.

Consequência: o pedido de exclusão por titular casa por `user.id` exato. Para quem foi gravado como `[email]`, o valor
original não existe mais no banco; a exclusão desses eventos não é possível por esse caminho (e o dado pessoal já não
está lá). Um CPF enviado como `user.id` continua gravado: a alternativa é um pseudônimo com chave, como no `did` das
sessões, e fica como decisão em aberto.

## Nota de 2026-10-09 (números e chaves, segunda rodada)

O scrubber só olhava **textos**. Duas formas de dado pessoal passavam: um CPF ou CNPJ enviado como **número JSON**
(`{"documento": 52998224725}`) e um e-mail usado como **chave de objeto** (tags são um mapa). Agora um número inteiro de 11 ou 14
dígitos que passa no dígito verificador vira a string `"[cpf]"` ou `"[cnpj]"`, e uma chave com e-mail é renomeada para
`[email]` (duas chaves que colidem ficam numeradas, `[email] (2)`, para nenhum valor se perder). **Cartão não se aplica a
número:** 13 a 19 dígitos passam no Luhn uma vez em dez e comeriam carimbos de tempo em milissegundos ou microssegundos, e
um cartão de 16 dígitos nem cabe num número JSON sem perder precisão. Números sob chave de identificador seguem
intocados. Custo conhecido: um inteiro qualquer de 11 dígitos tem cerca de 1% de chance de passar no dígito verificador de
CPF; em campos de métrica isso é raro, e o valor vira texto.

