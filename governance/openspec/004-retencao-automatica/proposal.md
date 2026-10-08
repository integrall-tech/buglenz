# 004 — Retenção automática

## Por quê

O invariante I5 diz que todo projeto tem prazo de retenção por tipo de dado, aplicado por rotina
do servidor, e que não existe projeto com retenção indefinida. O servidor tem a limpeza (`StorageService`,
por API e tela de Storage), mas só **manual** (G1). Sem a rotina, a instância do piloto acumularia
dados sem prazo; sem os prazos decididos (D6, responsável por LGPD), ela não pode ir para produção.

## O que muda

1. **Prazo por projeto e por tipo** (eventos, transações com seus spans, logs), em uma tabela nova
   `project_retention`. Valor nulo = usa o padrão da instância.
2. **Padrão da instância** pelas variáveis `RUSTRAK_RETENTION_EVENTS_DAYS`,
   `RUSTRAK_RETENTION_TRANSACTIONS_DAYS` e `RUSTRAK_RETENTION_LOGS_DAYS`. **Sem valor embutido**:
   os números do ADR-0009 (90/30/30) são proposta e dependem da D6.
3. **Worker** que roda 60 s depois da partida e a cada `RUSTRAK_RETENTION_INTERVAL_HOURS` (padrão
   24), aplicando o prazo efetivo de cada projeto com a limpeza em lotes que já existe.
4. **Projeto sem prazo efetivo** para algum tipo não é limpo nesse tipo, é registrado em `WARN` a
   cada passada e aparece como desprotegido na API. A implantação do BugLenz torna as três variáveis
   **obrigatórias** (`${VAR:?}` na stack), então produção não sobe sem decisão.
5. **API de administração**: `GET /api/retention` (padrões, projetos, prazos efetivos, última
   passada) e `PUT /api/projects/{id}/retention` (define ou limpa o prazo de um projeto).
6. **Tela de configuração** no dashboard (tarefa à parte, depois da API).

## O que não muda

- A limpeza manual por API e pela tela de Storage continua igual.
- Source maps seguem o ciclo de vida do release (limpeza própria, ADR-0009).
- O arquivo bruto em `INGEST_DIR` antes do digest não é tocado.

## Dependências

| O que | Estado |
|---|---|
| Pacote 003 (implantação) | feito |
| D6, prazos padrão | **aberta** (responsável por LGPD); o código não depende dela, a produção sim |
| D13, posição do piloto sobre retenção | com este pacote pronto, a "limpeza manual como exceção" deixa de ser necessária |

## Impacto

- ADR-0009 (item 3). Delta no manifesto: migration nova nos dois bancos, `workers/retention.rs`,
  `services/retention.rs`, `routes/retention.rs`, `openapi.json`, `main.rs`, `openapi.rs`.
- Risco: apagar dado por engano. Mitigação: piso de 7 dias por tipo (variáveis e prazo por projeto), pré-visualização igual à da limpeza
  manual, lotes curtos, teste de que um projeto sem prazo não perde nada.
- Tamanho: M.
