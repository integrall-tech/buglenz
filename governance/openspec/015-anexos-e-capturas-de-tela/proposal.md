# 015 — Anexos e capturas de tela

## Por quê

Hoje nenhum evento chega com imagem da tela. Os SDKs mobile sabem capturá-la no momento do erro,
mas o servidor descarta todo item `attachment` do envelope e o dashboard não tem onde exibi-lo.
[confirmado]

## O que muda

- O servidor aceita captura de tela e hierarquia de views como anexos de um evento.
- A tela do evento mostra miniatura e aba de anexos.
- A aceitação é opt-in por projeto (ADR-0017).

## O que não muda

Web: os SDKs de browser não capturam a tela por evento. Demais tipos de anexo (arquivos
arbitrários, minidump) continuam descartados.

## Impacto

- ADR: 0017, 0009, 0011. Exceção registrada ao invariante I4.
- Depende de: 004 (retenção) e 005 (scrubbing). Independe do pacote 016.
- Limitação conhecida: em crash nativo a captura é de melhor esforço; em iOS costuma não existir,
  porque capturar exige a thread de interface. Funciona bem para erros Dart, Kotlin e Swift
  tratados pelo SDK.
