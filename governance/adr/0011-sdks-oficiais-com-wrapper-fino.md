# ADR-0011 — SDKs oficiais do Sentry com wrapper fino por plataforma

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

A compatibilidade depende da versão do SDK. No teste, o `@sentry/react` 11.5.0 enviou um status
de sessão que o servidor 0.15.2 não reconhece (G3). O SDK evolui mais rápido que o servidor.

## Decisão

1. Os apps usam os SDKs oficiais: `@sentry/react` (web), `sentry-spring-boot` (backend),
   `sentry_flutter` (mobile). Nenhum SDK próprio (invariante I2).
2. Cada plataforma tem um **wrapper fino** da IntegrAllTech, fora do repositório do fork e fora do
   alcance da GPL, que fixa:
   - versão do SDK homologada contra a versão do fork (matriz no repositório do wrapper);
   - `release` no formato `<app>@<versão>`, `environment`, tags padrão (`cliente`, `tenant`);
   - `sendDefaultPii: false`, `beforeSend` e `beforeBreadcrumb` do ADR-0009;
   - no React: Error Boundary padrão e integração com o roteador; publicado junto do Archbase.
3. Upload de source maps no CI de cada app pelo plugin oficial (`@sentry/vite-plugin`), com token
   de API da instância.
4. Atualização de SDK só após o teste de ponta a ponta do ADR-0005 passar com a nova versão.

## Consequências

- Os wrappers ficam em `integrall-tech/buglenz-sdk`, sob **licença MIT** (2026-10-08), sem referência ao fork.

- Source maps não são publicados junto do bundle em produção; vão só para a instância.
- Enquanto G9 (tunnel) não for resolvido, navegadores com bloqueador podem não enviar eventos.
- Flutter ofuscado e crash nativo dependem da decisão D3 (G17).
