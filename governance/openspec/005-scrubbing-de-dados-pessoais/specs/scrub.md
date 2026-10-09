# 005 — Especificação

> **Emenda de 2026-10-09 (auditoria).** O cenário do delta vazio de migrações valia **no fim do pacote 005** (a migração de retenção veio no 004). Tratamentos acrescentados depois, sem cenário próprio: o `did` das sessões vira pseudônimo com chave; e-mail em campo `*id` é mascarado (`user.id` continua o id do usuário, mas um e-mail nele não); CPF/CNPJ enviado como número e e-mail como chave de objeto são mascarados; `user_reports` passa pelo scrub. Ver a ADR-0009 (notas de 2026-10-09).

## Chaves negadas

**WHEN** um payload traz, em qualquer nível, uma chave cuja forma normalizada esteja na lista de negação (`password`, `senha`, `token`, `authorization`, `cookie`, `secret`, `apiKey`, `x-api-key`, `cpf`, `cnpj`, `ip_address`, …)
**THEN** o valor persistido é a string `"[Filtered]"`, qualquer que fosse o tipo original

**WHEN** `RUSTRAK_SCRUB_EXTRA_KEYS=documento, telefone` está no ambiente
**THEN** chaves `documento`, `Documento`, `telefone` e `TELEFONE` também viram `"[Filtered]"`

**WHEN** a chave é `author` ou `session_replay_id`
**THEN** o valor é mantido (não casa nem exato nem por conteúdo das listas)

## Máscaras

**WHEN** uma string contém `123.456.789-09` (CPF com dígitos verificadores válidos) ou `12345678909`
**THEN** o trecho vira `[cpf]`; `123.456.789-00` (inválido) e `11111111111` ficam como estão

**WHEN** uma string contém um CNPJ válido, formatado ou não
**THEN** o trecho vira `[cnpj]`

**WHEN** uma string contém `4111 1111 1111 1111` (Luhn válido)
**THEN** o trecho vira `[cartao]`; `4111 1111 1111 1112` fica

**WHEN** uma string contém `ana.silva@example.com`
**THEN** o trecho vira `[email]`; o restante da string é preservado

**WHEN** `scrub_value` é aplicado duas vezes ao mesmo payload
**THEN** o resultado da segunda aplicação é idêntico ao da primeira

## Persistência

**WHEN** um envelope de evento com `request.headers.Cookie`, `request.data.password`, `user.email`, `user.ip_address` e uma mensagem com CPF e e-mail é ingerido e digerido
**THEN** `events.data` não contém nenhum dos valores originais; contém `[Filtered]`, `[cpf]` e `[email]`; `events.remote_addr` é `NULL`; `user.id` é preservado

**WHEN** uma transação com `request.headers.Authorization` e spans filhos com `data.password` é ingerida
**THEN** `transactions.data` e os `spans.data` filhos não contêm os valores; `transactions.remote_addr` é `NULL`

**WHEN** um container de logs com `body` contendo um e-mail e `attributes.token` é ingerido
**THEN** `logs.body` contém `[email]` e `logs.attributes.token` é `"[Filtered]"`

**WHEN** um span avulso (legado ou v2) com `data.senha` é ingerido
**THEN** `spans.data.senha` é `"[Filtered]"`

**WHEN** dois eventos com a mesma exceção e CPFs diferentes na mensagem são ingeridos
**THEN** caem na mesma issue (o agrupamento vê `[cpf]` nos dois)

## Exclusão por titular

**WHEN** um admin chama `DELETE /api/projects/{id}/privacy/users/u-1` e existem eventos de `u-1` em duas issues e eventos de `u-2`
**THEN** os eventos de `u-1` são apagados, os de `u-2` ficam, `event_count` das issues e os contadores do projeto batem com o que sobrou, e a resposta diz quantos eventos e transações saíram

**WHEN** um membro (não admin) chama a rota
**THEN** `403`

**WHEN** o projeto não existe
**THEN** `404`

## Ponta a ponta

**WHEN** o app e2e identifica o usuário com `id` e `email`, põe uma senha em `extra.password` e lança um erro cuja mensagem contém um CPF
**THEN** o evento no servidor tem `user.id` preservado, `user.email` = `[email]`, `extra.password` = `[Filtered]`, a mensagem com `[cpf]`, e nenhum dos valores originais aparece no evento (uma senha em texto livre, fora de uma chave negada, **não** é detectável: é responsabilidade da camada 1, ADR-0011)

## Delta controlado

**WHEN** `git diff --name-only v0.16.0 main` é filtrado por `apps/server/migrations`, `Cargo.toml`, `Cargo.lock`
**THEN** o resultado é vazio (nenhuma migration, nenhuma dependência nova)

**WHEN** `ci.yml`, `network-conformance`, `e2e-react` e `licenses` rodam
**THEN** todos verdes; o custo do scrub por evento aparece na baseline
