# 009 — Wrappers de SDK e onboarding do primeiro produto

## Por quê

O fork já recebe, agrupa, trata dados pessoais e entrega os alertas (pacotes 001 a 007). Falta o
lado dos apps: nenhum produto da IntegrAllTech manda erro para a instância ainda. O ADR-0011 decide
como: SDKs oficiais do Sentry (I2) com um **wrapper fino** por plataforma, que fixa a versão
homologada, o formato de `release`, as tags e a camada 1 de proteção de dados do ADR-0009. Sem o
wrapper, cada app configura o SDK do seu jeito, e a camada 2 (o scrub do servidor, pacote 005)
passa a ser a única proteção.

Este pacote também é o que leva um produto real a produção e começa o relógio dos 30 dias do
critério de saída da Fase 1.

## O que muda

1. **Wrapper React** (`@sentry/react` 11.x): função de inicialização única, `release`
   `<app>@<versão>`, `environment`, tags `cliente` e `tenant`, `sendDefaultPii: false`,
   `beforeSend`/`beforeBreadcrumb` alinhados ao ADR-0009, `ErrorBoundary` padrão, integração com o
   roteador, identificação de usuário só por id interno, e um auxiliar de configuração do
   `@sentry/vite-plugin` para os source maps.
2. **Wrapper Spring Boot** (`sentry-spring-boot` para Spring Boot 3, Java 21): propriedades
   `buglenz.*` que preenchem `sentry.*`, os mesmos padrões de `release`, `environment`, tags e
   privacidade, e `beforeSend` equivalente.
3. **Matriz de homologação**: versão do SDK × versão do fork, atualizada só depois do teste de
   ponta a ponta passar com a versão nova (ADR-0011, item 4).
4. **Testes de contrato contra uma instância real**: o `e2e-react` do fork já cobre o SDK React;
   o equivalente para Spring Boot é novo e grava primeiro o tráfego real, porque
   `sentry-spring-boot` contra a instância é uma das coisas listadas como **não verificadas**
   no README do corpus.
5. **Source maps no CI de cada app**, com token de API da instância.
6. **Guia de onboarding** de um app: do projeto na instância ao primeiro erro visto.
7. **Primeiro produto piloto** em produção, com a instância implantada (Swarm, pacote 003).

## O que não muda

- O repositório do fork. Os wrappers vivem **fora** dele e fora do alcance da GPL (ADR-0003, itens 4
  e 5): não importam `@rustrak/*`, não copiam código do dashboard e falam com a instância só pelo
  protocolo do Sentry e por REST.
- O servidor. Qualquer incompatibilidade que o contrato revelar é achado do pacote 006 ou de um
  novo, não ajuste silencioso aqui.
- Flutter, `sentry_flutter` e symbolication mobile (D3, pacotes 015 e 016).

## Dependências, em ordem

| O que | Estado | Quem |
|---|---|---|
| Pacotes 005 e 006 | feitos | — |
| Repositório e registry dos wrappers | **não definidos** | Edson |
| Produto piloto (web React + backend Spring Boot) | **não nomeado** | Edson |
| Instância em produção: localização (D4), host do DSN (D8, proposta `errors.buglenz.dev`), credencial de leitura do GHCR, backup, scrape e logs | **não implantada** | Edson, Neimar |
| Pacote 004 (retenção) para o **critério de saída** da Fase 1 | bloqueado por D6 | responsável por LGPD |
| Detalhes do Archbase (como o wrapper React é publicado "junto") | **fora do corpus** | Edson |

## Impacto

- ADRs: 0011 (decisão), 0009 (camada 1), 0003 (fronteira de licença), 0007 (um projeto por app).
- Risco: o escopo inclui um produto real. A parte de especificação e os wrappers podem avançar sem
  ele; a entrada em produção, não.
- Tamanho: M, e a maior parte da duração é implantação e coordenação, não código.
