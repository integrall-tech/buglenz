# RFC-0001 — Catálogo de decisões de triagem

**Versão:** 0.1 · **Data:** 2026-10-07 · **Status:** Proposta
**Relaciona:** ADR-0013, ADR-0014, ADR-0015 · **Executa em:** pacotes 018 a 020

Marcação: [confirmado] = lido no código do Rustrak `v0.15.2`; [inferência] = proposta de desenho.

## 1. Fluxo

```
SDK → instância (ingestão, scrubbing, digest, agrupamento)
        │ webhook new_issue / regression (HMAC, alert_id)
        ▼
   agente de triagem (ArchFlow)
        │ 1. lê issue, último evento, estatísticas e tags por REST
        │ 2. calcula os fatos determinísticos (§2)
        │ 3. faz as perguntas ao modelo de decisão (§3)
        │ 4. aplica a política versionada (§4)
        │ 5. registra a decisão; conforme o nível, comenta ou ajusta a issue
        ▼
   instância (comentário, prioridade, responsável)  +  canal de alerta
```

O agente consulta periodicamente a atividade das issues para colher o que as pessoas fizeram
depois (mudança de prioridade, atribuição, silenciar, resolver). Isso forma o conjunto rotulado.

## 2. Fatos determinísticos (sem modelo)

| Fato | Origem |
|---|---|
| `usuarios_afetados`, `eventos`, tendência | campos `user_count`, `event_count`, `trend` da issue [confirmado] |
| `release_do_evento` anterior ao release em que a issue foi resolvida | `first_release`, `last_release`, release do evento [confirmado]. Decide "regressão real × ocorrência residual" sem modelo |
| `ambiente` | campo `environment` do evento |
| `frame_de_topo_em_dependencia` | caminho do frame contém `node_modules` (contorna G8) |
| `prioridade_travada` | `priority_locked_at` não nulo [confirmado] |

## 3. Perguntas ao modelo

Entradas comuns (E): tipo e valor da exceção, mecanismo, até 8 frames de topo (arquivo, função,
linha, linha de contexto), transação, até 10 breadcrumbs, tags de browser e sistema, plataforma do
projeto. Sem `user`, sem cabeçalhos, sem corpo de requisição.

| ID | Primitiva | Pergunta | Opções | Entradas extras |
|---|---|---|---|---|
| T1 | `choice` | O que originou este erro? | `aplicacao`, `extensao_do_navegador`, `robo_ou_crawler`, `rede_ou_dispositivo_do_usuario`, `ambiente_de_desenvolvimento` | user agent, origem do script |
| T2a | `noulli` | O erro impede o usuário de concluir a tarefa em que estava? | sim / não | — |
| T2b | `noulli` | O erro ocorre em fluxo crítico do produto? | sim / não | lista de fluxos críticos do projeto (ex.: login, pedido, pagamento) |
| T2c | `noulli` | A mensagem indica perda ou corrupção de dado? | sim / não | — |
| T3 | `noulli` | Estas duas issues têm a mesma causa? | sim / não | issue candidata; candidatos vêm dos k vizinhos por embedding |
| T4 | `choice` | Qual time é o dono provável? | times configurados no projeto, mais `indefinido` | descrição de cada time e seus módulos |
| T5 | `choice` | Quando avisar? | `agora`, `resumo_diario`, `nao_avisar` | resultado de T1 e da prioridade; horário |

Toda pergunta tem a opção de escape (`indefinido` ou probabilidade na faixa de incerteza), para
não forçar escolha.

## 4. Política (código versionado)

```
se prioridade_travada → não tocar em prioridade

origem = T1
se origem ≠ aplicacao e p ≥ limiar_T1:
    sugerir silenciar (N1); nunca silenciar sozinho (ADR-0013, N3)
    T5 = nao_avisar

prioridade =
    alta   se (T2c) ou (T2a e T2b) ou usuarios_afetados ≥ limite_alto
    baixa  se não T2a e não T2b e usuarios_afetados ≤ limite_baixo
    média  nos demais casos
    → resposta na faixa de incerteza conta como "não decidido" e mantém a prioridade atual

duplicata: se T3 = sim com p ≥ limiar_T3 → comentar nas duas issues com o vínculo (N1)
dono:      se T4 ≠ indefinido com p ≥ limiar_T4 → atribuir (N2) ou sugerir (N1)
aviso:     T5 decide canal e momento; prioridade alta sempre avisa agora
```

Limiares e limites ficam por projeto, com valor inicial único e ajuste após o modo sombra.
Mudança de prioridade só ocorre se a nova decisão se mantiver em duas avaliações seguidas
(histerese), para a issue não oscilar a cada regressão.

O Rustrak não tem fusão de issues [confirmado: sem rota de merge], então T3 nunca passa de N1.

## 5. Medição e promoção de nível

- **Rótulo:** ação humana sobre a issue em até 7 dias (prioridade final, responsável final,
  silenciada como ruído). Issue sem ação humana não conta.
- **Por decisão:** concordância com o rótulo, matriz de confusão, curva de calibração (probabilidade
  declarada × acerto observado em faixas) e cobertura (fração fora da faixa de incerteza).
- **Robustez:** repetir a pergunta com identificadores aleatórios inseridos na entrada; a resposta
  deve se manter.
- **Promoção N0 → N1:** pelo menos 200 decisões rotuladas e concordância ≥ 85% fora da faixa de
  incerteza. **N1 → N2:** mais 200 decisões em N1 com sugestão aceita em ≥ 90% dos casos.
  Valores propostos, a validar com os primeiros dados. [inferência]
- **Rebaixamento automático:** concordância abaixo do critério em janela de 30 dias volta a decisão
  um nível.

## 6. Texto gerado (LLM, sob demanda)

| ID | O que gera | Quando | Onde aparece |
|---|---|---|---|
| S1 | Causa provável e ponto do código a olhar, com base no stack resolvido e nos breadcrumbs | issue de prioridade alta, ou a pedido | comentário na issue |
| S2 | Resumo semanal por projeto: issues novas, regressões, crash-free por release | semanal | canal do time |
| S3 | Correção proposta | a pedido, no Claude Code, pelo servidor MCP do Rustrak | PR do desenvolvedor |

S1 e S2 identificam-se como texto gerado. Nenhum dos três altera estado da issue.

## 7. Fora de escopo

Scrubbing por modelo (a barreira é determinística, ADR-0009), decisão por evento em tempo de
ingestão, silenciar ou resolver automaticamente, treinamento de modelo próprio. Este último pode
voltar quando o conjunto rotulado justificar.

## 8. Perguntas abertas

1. Contrato atual do ArchFlow para modelos de decisão: confere com a tabela do ADR-0014?
2. Canal de alerta: direto (Slack, e-mail) ou via Linktor?
3. Quem cadastra fluxos críticos e times por projeto, e onde (configuração do agente ou tela)?
