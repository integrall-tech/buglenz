# ADR-0016 — Symbolication mobile com as bibliotecas do Sentry

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O servidor não tem nenhum tratamento de símbolos nativos: não aceita arquivos de debug (a
capacidade anunciada ao `sentry-cli` é `release_files`, `sources`, `artifact_bundles`,
`artifact_bundles_v2`) e não lê `instruction_addr`. Stack trace de Android com R8, de iOS e de
Flutter ofuscado chega ilegível (G17). [confirmado]

O que já existe e serve de base [confirmado]: upload em pedaços com worker de montagem; etapa de
reescrita de frames antes do agrupamento (`digest/processors/event.rs`); cache de source maps em
memória; a tela de evento já exibe `module`/`package` e uma seção de threads.

## Decisão

1. Usar as bibliotecas que o próprio Sentry publica: `symbolic` 13.x (MIT) para Mach-O/dSYM, ELF e
   DWARF, e `proguard` 5.x (BSD-3-Clause) para mapeamentos R8. As duas licenças já constam na
   lista permitida do `deny.toml`. [confirmado em crates.io em 2026-10-07]
2. Manter o protocolo do `sentry-cli`: os plugins oficiais (Gradle, `sentry_dart_plugin`, Xcode)
   enviam símbolos sem adaptação. Nenhuma ferramenta própria de upload (I2).
3. A resolução roda no digest, **antes** do agrupamento, no mesmo ponto da reescrita por source map.
4. Entrega em três etapas independentes: R8/ProGuard; nativo (ELF e Mach-O); reprocessamento.
5. É recurso genérico: propor ao upstream antes de implementar (ADR-0002). Se não houver
   interesse, fica no fork em arquivos novos, com migrations de sufixo `_itl`.

## Fora de escopo

Minidump, PDB e Windows, Unity IL2CPP, WebAssembly, símbolos de sistema da Apple, contexto de
código-fonte para frames nativos e Java.

## Consequências

- Frames de bibliotecas de sistema do iOS ficam como imagem + endereço.
- Arquivos de debug são grandes; entram na conta de armazenamento e na retenção (I5).
- O servidor passa a interpretar binários enviados por clientes autenticados: limites de tamanho,
  de tempo e de memória são obrigatórios.
- Sem símbolos enviados antes da distribuição do app, o evento é agrupado por endereço. A etapa 3
  trata disso; até lá, o upload no CI é condição de release (ADR-0011).
