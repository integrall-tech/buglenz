# 006 — Compatibilidade com o SDK atual

## Por quê

O protocolo de sessões do Sentry ganhou na versão 1.6.0 o status **`unhandled`**: "um erro não
tratado ocorreu, mas o processo não terminou" [confirmado em develop.sentry.dev]. O SDK
JavaScript 11.x passou a enviá-lo no lugar de `crashed` para erros não tratados no navegador
(changelog, PR getsentry/sentry-javascript#22475; `SessionStatus = 'ok' | 'exited' | 'crashed' |
'abnormal' | 'unhandled'` em `@sentry/core` 11.5.0) [confirmado no tarball do npm].

O servidor na `v0.16.0` só conhece `ok`, `exited`, `crashed`, `abnormal` e `errored`
(`models/session.rs`), e um item de sessão com status desconhecido é descartado com o aviso
"session item: bad JSON, treating as Other" [confirmado]. Resultado medido no GAP §3: quatro
erros em um app React 19 com `@sentry/react` 11.5.0, e o release health informa crash-free 100% e
zero sessões com erro (G3). É o único gap P0 que é bug, e afeta qualquer app React da
IntegrAllTech desde o primeiro dia.

Também falta ao fork o **teste de ponta a ponta com app React minificado e source map** que o
ADR-0005 (passo 6) exige em cada sincronização: hoje ele existe só como roteiro manual no GAP §3.

## O que muda

- Servidor: `SessionStatus::Unhandled` aceito em sessões individuais e `unhandled` aceito nos
  agregados; ambos contam como sessão **com erro** (não como crash), porque o processo sobreviveu.
  Testes unitários e de integração junto.
- **PR no upstream** (ADR-0002) com exatamente essa mudança, a partir de um fork público
  `integrall-tech/rustrak`. Enquanto o PR não entra, a mudança vive no fork e consta no
  `DELTA-MANIFEST.md`; no sync em que entrar, o delta some.
- Novo workflow `e2e-react.yml` e app `e2e/react-app/` (React 19 + Vite, `@sentry/react` 11.5.0
  fixado, `@sentry/vite-plugin` para source maps), fora do workspace pnpm para não tocar o
  `pnpm-lock.yaml` do upstream: sobe o servidor, provisiona projeto e token pela API, faz upload
  dos source maps, dispara os quatro erros do roteiro do GAP em Chromium headless e verifica pela
  API: 3 issues, frames com arquivo e linha originais, release health com sessão `unhandled`
  contada.

## O que não muda

- Nomes de função deslocados após source map (G7) e `in_app` sem reclassificação (G8): o teste os
  **mede e registra na baseline**, não os corrige; são do pacote 010.
- Dashboard: não há coluna nova; `unhandled` aparece onde "errored" já aparece.
- Migrations: nenhuma; `session_counts` já tem a coluna `errored`.
- Wrapper de SDK dos apps (ADR-0011): fora do repositório do fork; este pacote só garante que o
  servidor aceita o que o SDK oficial manda.

## Impacto

- ADRs: 0011 (contexto), 0002 (PR no upstream), 0005 (o job e2e vira parte do ciclo de sync).
- Risco: baixo. Acrescenta um valor a um enum e um campo opcional a um struct; o caminho antigo
  não muda. A classificação de `unhandled` como "errored" é a leitura do protocolo [inferência,
  explicada no design]; se o upstream preferir outra, o fork segue o upstream.
- Custo de CI: um job de ~8 min por PR (build do servidor em cache, instalação do app, Chromium).
- Tamanho: P.
