# ADR-0014 — Agente de triagem fora do fork, em ArchFlow

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O ArchFlow já suporta modelos de decisão no estilo do Jev (informado pelo Edson em 2026-10-07; o
contrato não foi lido nesta análise). O fork deve ter delta mínimo (ADR-0002) e seu código não se
mistura com código proprietário (ADR-0003, I8).

O Rustrak já expõe o necessário para um agente externo [confirmado]:
- alerta por webhook em `new_issue`, `regression` e `unmute`, com `alert_id`, projeto, issue e
  `issue_url`, e assinatura HMAC;
- leitura de issue, eventos, estatísticas, atividade e valores de tag por REST;
- escrita de `priority`, `assigned_to`, `status` e comentário por REST.

## Decisão

1. A IA de triagem é um **serviço separado**, construído em ArchFlow, em repositório próprio e
   licença da IntegrAllTech. Fala com a instância só por webhook e REST.
2. **Nenhum código de IA entra no fork.** A primeira versão não altera arquivo do upstream.
3. Um agente por instância (I6), com usuário de serviço próprio (`triagem-bot`), papel Editor
   apenas nos projetos habilitados, e token de API desse usuário.
4. O agente é idempotente por `alert_id` e tolera reenvio do webhook.
5. O agente passa a ser o **roteador de alertas**: a regra de alerta da instância aponta para o
   webhook do agente, e ele decide canal e momento (decisão T5 da RFC-0001).
6. Capacidade que faltar ao ArchFlow vira documento de pedido ao ArchFlow, em termos genéricos.
   Nada específico de rastreamento de erros entra no framework.

## Requisitos sobre o ArchFlow (a conferir)

| Requisito | Para quê |
|---|---|
| Primitivas `choice`, `noulli`, `score` com probabilidade | RFC-0001 §3 |
| Entrada estruturada (records), não texto montado | ADR-0013 item 4 |
| Provedor do modelo configurável por decisão | ADR-0015 |
| Registro de decisão com versão do modelo e hash da entrada | auditoria |
| Execução em sombra e limiar por decisão | níveis N0 a N2 |
| Timeout e fallback determinístico | agente fora do ar não bloqueia triagem |

## Consequências

- Indisponibilidade do agente não afeta ingestão; a issue fica com a prioridade padrão.
- Limitações herdadas: tokens sem escopo (G13) e webhook só por issue, não por evento.
- Ajustes futuros no fork (ex.: campo de rótulo na issue) seguem o ADR-0002: PR no upstream.
