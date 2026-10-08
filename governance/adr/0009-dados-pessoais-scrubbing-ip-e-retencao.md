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
