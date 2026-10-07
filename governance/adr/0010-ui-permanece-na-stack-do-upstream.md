# ADR-0010 — A UI permanece na stack do upstream

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

A stack fixa da IntegrAllTech é React 19 + TypeScript + Mantine v9 + Archbase. O dashboard do
Rustrak é React 19 + TypeScript + Vite + Tailwind 4 + Base UI + TanStack Router/Table, com design
system próprio em `packages/ui`: 61 mil linhas no total. [confirmado]

## Decisão

Não migrar para Mantine/Archbase. O fork mantém a stack de UI do upstream e limita as mudanças de
interface a: catálogo `pt-BR`, marca (ADR-0006), remoção do aviso de nova versão (ADR-0004) e
telas dos recursos novos (retenção, scrubbing), escritas com os componentes de `packages/ui`.

## Justificativa

- Reescrever a UI transformaria o fork fino em produto paralelo e inviabilizaria o ADR-0005.
- Colocar Archbase dentro de código GPL exigiria licenciá-lo sob GPL (ADR-0003).
- A base (React 19, TypeScript) é a mesma; a diferença é a biblioteca de componentes.

## Consequências

Exceção registrada à stack fixa, restrita a este produto. Novas telas seguem as convenções do
upstream (Biome, `use-intl`, TanStack Router).
