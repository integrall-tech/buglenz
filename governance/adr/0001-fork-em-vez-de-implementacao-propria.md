# ADR-0001 — Fork do Rustrak em vez de implementação própria

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

Os produtos da IntegrAllTech não têm rastreamento de erros de frontend. O Firebase Crashlytics não
atende web. Avaliados em 2026-10-07: Traceway (Go, MIT), Rustrak (Rust, GPL-3.0), Temps,
OpenObserve, Uptrace, HoldFast, SigNoz, SkyWalking. Decisão do Edson: seguir com o Rustrak.

## Decisão

Derivar do Rustrak `v0.15.2` por fork governado.

## Justificativa

- O protocolo de ingestão é o do Sentry. Os SDKs de React, Spring Boot e Flutter são os oficiais,
  mantidos por terceiros sob MIT; o fork só sustenta o servidor. [confirmado no teste]
- O caminho crítico (parser de envelope, agrupamento com paridade do Sentry, source maps) já
  existe com 41,7 mil linhas de teste. Reescrever em Java não agrega e erra em silêncio.
- A UI já é React 19 + TypeScript, competência do time. [confirmado]
- Um processo, um banco PostgreSQL: cabe no Docker Swarm sem novo componente de infraestrutura.

## Alternativas rejeitadas

- **Usar a imagem do upstream sem fork.** Rejeitada por I3, I4 e I5: telemetria e checagem de
  versão no código, sem scrubbing e sem retenção automática. Ver ADR-0002 sobre manter o fork fino.
- **Traceway.** Licença MIT e cobertura maior, mas SDK de browser próprio e ClickHouse.
- **Implementação própria em Spring Boot.** Meses de trabalho para chegar ao que já está testado.

## Consequências

- Rust entra na base de código mantida. O time não tem Rust como competência principal; a
  implementação é por Claude Code e a revisão humana fica concentrada nos pontos da CONSTITUTION §5.
- A licença GPL-3.0 impõe as fronteiras do ADR-0003.
- Critério de saída: se três ciclos de sincronização seguidos passarem do orçamento do ADR-0005,
  ou se o upstream for abandonado ou mudar de licença, este ADR é reaberto.
