# 004 — Design

## Modelo

`project_retention (project_id PK → projects ON DELETE CASCADE, events_days, transactions_days,
logs_days INTEGER NULL, updated_at)`. Uma linha por projeto com ao menos um prazo próprio; sem linha,
vale o padrão. Valor aceito: **7 a 3650 dias**. O piso de 7 é deliberado: a primeira passada roda 60 s depois de cada partida, e um prazo digitado pequeno demais (`1`) apagaria quase tudo de uma vez. A limpeza manual mantém o mínimo de 1 dia, porque ali há prévia e confirmação.

## Prazo efetivo

`efetivo(tipo) = prazo do projeto (se houver) senão padrão da instância (se houver) senão nenhum`.
Spans seguem as transações (cascata). Logs e eventos são independentes.

**Relatos de usuário** (`user_reports`, segunda rodada): seguem o prazo de eventos; o texto passa pelo scrub ao ser gravado
(e-mail vira `[email]`, CPF vira `[cpf]`). Ficam fora da exclusão por titular, que casa por `user.id`.

**Sessões e histórico de alertas** (auditoria de 2026-10-09, invariante I5): `session_counts`, `session_users` e
`alert_history` de um projeto seguem o prazo de **eventos** dele e saem na mesma passada; sem prazo de eventos,
não se apaga nada. A exclusão por titular também apaga as linhas de `session_users` do titular (o pseudônimo
e o id cru, de antes do pseudônimo). Ficam de fora: os arquivos brutos do `INGEST_DIR`, que o worker de
recuperação já trata.

## Worker

```
dorme 60 s; loop: passada(); dorme intervalo
passada():
  para cada projeto:
    para cada tipo com prazo efetivo: execute_cleanup_in_batches(projeto, dias, filtro do tipo)
    tipos sem prazo → WARN e entra em `desprotegidos`
  guarda o relatório (remoções, desprotegidos, início, fim)
```

- Reaproveita `StorageService::execute_cleanup_in_batches` (lotes de 10 000, pausa no SQLite).
- Não usa o `CleanupJob` (que é para a limpeza manual): as duas operações são idempotentes e
  concorrentes sem prejuízo, só disputam carga.
- Uma falha em um projeto é registrada e a passada segue para o próximo.
- A passada é visível em `GET /api/retention` (`last_run`), em memória: some na reinicialização e é
  refeita 60 s depois.

## API

Somente administradores globais, como o Storage. `PUT` aceita `{events_days, transactions_days,
logs_days}`; `null` limpa o prazo do tipo; campo ausente não muda; fora de 7..3650 é 400 com o campo.

## Decisões abertas

- Os prazos (D6). Até lá, a instância de teste usa os números do ADR-0009 só por variável.
- A tela de configuração fica para depois da API, porque exige catálogos de cinco idiomas.
