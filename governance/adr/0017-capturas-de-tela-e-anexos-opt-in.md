# ADR-0017 — Capturas de tela e anexos são opt-in por projeto

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

Os SDKs mobile do Sentry capturam a tela no momento do erro quando `attachScreenshot` está
ligado, e enviam a imagem como item `attachment` do envelope. É opt-in no SDK "because they may
contain PII"; no Flutter o mascaramento de texto e imagens vem ligado por padrão. Em crash nativo
a captura é de melhor esforço e pode não existir. O recurso só existe em SDKs com interface
(mobile e desktop), não na web. [confirmado na documentação do sentry_flutter em 2026-10-07]

O Rustrak descarta todo item `attachment` e o dashboard não tem tela de anexos. [confirmado]

## Decisão

1. O BugLenz passa a aceitar anexos de dois tipos: captura de tela (PNG/JPEG) e hierarquia de
   views (JSON). Os demais tipos continuam descartados.
2. **Desligado por padrão.** Cada projeto habilita explicitamente, e a habilitação fica registrada
   com autor e data.
3. Imagem não passa por scrubbing no servidor: não há como aplicar I4 a pixels. A proteção é na
   origem: o wrapper de SDK (ADR-0011) mantém o mascaramento ligado e não expõe opção de desligar.
4. Retenção própria e mais curta que a de eventos (proposta: 30 dias), aplicada pelo mesmo worker.
5. Acesso ao anexo exige o mesmo papel de projeto que o evento; download fica na trilha de
   atividade.

## Consequências

- Exceção declarada ao invariante I4, limitada a projetos que habilitarem. A CONSTITUTION registra.
- Para web não há captura de tela por evento; o equivalente seria session replay, fora de escopo.
- Volume em disco cresce; limite por arquivo e por evento é obrigatório.
