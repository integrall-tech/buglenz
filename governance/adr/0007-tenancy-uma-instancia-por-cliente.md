# ADR-0007 — Tenancy: uma instância por cliente

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O Rustrak não tem organizações: usuários, papéis globais e projetos vivem em um espaço único por
instância, e o upstream recomenda instâncias separadas para organizações distintas. [confirmado]

## Decisão

- **Instância interna** da IntegrAllTech para os produtos próprios.
- **Uma instância por cliente** quando o produto roda na infraestrutura do cliente ou quando o
  contrato exige segregação.
- Dentro de uma instância: **um projeto por aplicação implantável** (ex.: `vendax-web`,
  `vendax-api`), e `environment` do SDK separa produção de homologação. Projeto separado por
  ambiente só quando a retenção ou o acesso precisarem ser diferentes.
- Acesso por papel de projeto (Viewer/Editor/Admin).

## Consequências

- Não se implementa multi-organização no fork (G19 fica fora de escopo).
- Erros de um mesmo produto em vários clientes ficam espalhados por instâncias; visão consolidada,
  se necessária, é problema de outra camada.
- Custo por instância é baixo: um container e um banco. [inferência; consumo não medido aqui]
