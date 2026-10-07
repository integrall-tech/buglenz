# ADR-0013 — IA como sensor; a política é código

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

Triagem de erro é feita de decisões pequenas, de opções fixas e volume alto: é ruído ou é
acionável, qual a prioridade, de quem é. Hoje o Rustrak deriva a prioridade só do nível do evento
(`derive_priority` em `services/issue.rs`): todo `error` nasce `high`. No teste, as três issues
vieram como `high`. [confirmado]

## Decisão

1. **O modelo observa, o código decide.** Um modelo de decisão responde perguntas tipadas
   (`choice`, `noulli`, `score`) e devolve rótulo com probabilidade. Limiares, combinação de
   respostas e ação resultante ficam em configuração versionada no repositório do agente.
2. **Perguntas pequenas.** Cada decisão é decomposta na menor unidade semântica e medida
   isoladamente. Caso novo se resolve adicionando pergunta, limiar e caso de teste.
3. **O que cabe num `if` fica num `if`.** Contagem de usuários afetados, comparação de release e
   janela de tempo são cálculo, não julgamento, e não passam por modelo.
4. **Entrada estruturada e já tratada.** O modelo recebe campos do evento lidos pela API da
   instância, depois do scrubbing (I4). Nunca resumo gerado por LLM.
5. **Níveis de autonomia por decisão:**

| Nível | O que o agente faz | Promoção |
|---|---|---|
| N0 sombra | Registra a decisão no próprio log; nada aparece na issue | padrão inicial |
| N1 sugestão | Comenta na issue com rótulo, probabilidade e motivo | critério da RFC-0001 §5 |
| N2 ação reversível | Ajusta prioridade ou responsável | critério da RFC-0001 §5 + aprovação do Edson |
| N3 silenciar ou resolver | — | **não permitido** sem ADR própria |

6. **Humano prevalece.** Prioridade ou responsável definidos por pessoa não são sobrescritos. O
   Rustrak já marca `priority_locked_at` quando a prioridade é ajustada. [campo confirmado;
   semântica a validar no pacote 018]
7. **LLM gerador só sob demanda e só para texto:** resumo de causa provável e resumo semanal.
   Nunca decide.

## Consequências

- Toda decisão fica auditável: entrada, versão do modelo, saída, probabilidade, limiar e ação.
- Probabilidade de modelo não é acerto. Sem conjunto rotulado não há promoção de nível.
- O invariante I13 da CONSTITUTION passa a registrar os itens 1, 5 e 6.
