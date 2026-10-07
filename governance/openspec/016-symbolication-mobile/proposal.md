# 016 — Symbolication mobile

## Por quê

Apps mobile em produção são compilados com ofuscação e sem símbolos. O servidor recebe os eventos,
mas mostra `a.b.c.d()` no Android, endereços de memória no iOS e no Flutter com `--obfuscate`.
Sem nome de função não há triagem nem agrupamento estável.

## O que muda

- O servidor aceita arquivos de debug pelo protocolo do `sentry-cli`.
- O digest resolve frames Java/Kotlin (R8) e nativos (ELF, Mach-O) antes de agrupar.
- A interface lista os arquivos de debug e avisa quando faltam símbolos para um evento.
- Cada app mobile passa a enviar símbolos no CI.

## Etapas

| Etapa | Entrega | Destrava |
|---|---|---|
| 1 | Upload de arquivos de debug + R8/ProGuard | Android nativo (Java/Kotlin) |
| 2 | Frames nativos com `symbolic` (ELF e Mach-O) | Flutter ofuscado em Android e iOS; Swift e Objective-C |
| 3 | Aviso de símbolos ausentes, reprocessamento, limpeza | Operação do dia a dia |

Cada etapa é entregável sozinha.

## O que não muda

Eventos de JavaScript e o fluxo de source maps. Minidump e os demais itens do "Fora de escopo" do
ADR-0016.

## Impacto

- ADR: 0016, 0002, 0005, 0011.
- Depende de: 003 (implantação), 004 (retenção), decisão D3.
- Maior pacote do roadmap. Risco principal: divergência entre o que descrevo do protocolo do
  `sentry-cli` e o que as versões fixadas realmente enviam; por isso a tarefa T1 grava o tráfego
  real antes de qualquer implementação.
