# 016 — Tarefas

## Preparação
- [ ] T0. Abrir discussão no upstream sobre symbolication nativa e registrar a resposta (ADR-0002)
- [ ] T1. Gravar o tráfego real de upload de `sentry-cli`, plugin Gradle e `sentry_dart_plugin` nas versões fixadas, contra um servidor de captura; corrigir `design.md` §1 e §2 com o que for observado
- [ ] T2. Gerar as fixtures: Kotlin+R8, Flutter Android, Flutter iOS e Swift (as de iOS em macOS)
- [ ] T3. Adicionar `symbolic` e `proguard`; passar `cargo deny` e registrar em `THIRD-PARTY-LICENSES.md`

## Etapa 1 — upload e R8
- [ ] T4. Migration `debug_files` (PostgreSQL e SQLite) e armazenamento em disco
- [ ] T5. Capacidades `debug_files` e `proguard` em `chunk-upload`; endpoint `difs/assemble`; endpoint `reprocessing` sem ação
- [ ] T6. Tipo de trabalho novo no worker de montagem, com validação e limites
- [ ] T7. Etapa `remap_proguard` no digest, antes do agrupamento
- [ ] T8. API de listagem e exclusão; tela "Arquivos de debug"
- [ ] T9. Testes unitários, dourados e de contrato da etapa 1

## Etapa 2 — nativo
- [ ] T10. Conversão para symcache na montagem e cache em memória com teto
- [ ] T11. Etapa `symbolicate_native`: busca de imagem, consulta, embutidas, demangle, status por frame
- [ ] T12. Regra de `in_app` para imagens de sistema e de terceiros
- [ ] T13. Resumo de symbolication no evento; selo por frame e faixa de aviso na interface
- [ ] T14. Testes dourados de Flutter (Android e iOS) e Swift; ponta a ponta com emulador Android

## Etapa 3 — operação
- [ ] T15. ADR da semântica de reagrupamento
- [ ] T16. Reprocessamento ao chegar arquivo de debug; `reprocessing` passa a enfileirar
- [ ] T17. Limpeza integrada ao worker de retenção; contagem na tela de Storage
- [ ] T18. Métricas Prometheus: arquivos, bytes, tempo de resolução, frames sem símbolo

## Fechamento
- [ ] T19. Atualizar wrappers e pipelines dos apps (ADR-0011) e o guia em `buglenz.dev`
- [ ] T20. Conferir cada cenário de `specs/symbolication.md`
- [ ] T21. Atualizar `DELTA-MANIFEST.md` ou abrir os PRs no upstream, conforme T0
