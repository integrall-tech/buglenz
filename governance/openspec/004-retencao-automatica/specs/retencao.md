# 004 — Especificação

**WHEN** a passada roda e um projeto tem prazo efetivo de 30 dias para eventos
**THEN** os eventos mais antigos que 30 dias são apagados, os mais novos ficam, e as issues que ficaram sem evento saem

**WHEN** um projeto não tem prazo próprio e o padrão da instância de transações é 14 dias
**THEN** as transações (e seus spans) mais antigas que 14 dias são apagadas

**WHEN** um projeto não tem prazo efetivo para logs
**THEN** os logs desse projeto não são apagados, a passada registra um aviso e a API o lista como desprotegido

**WHEN** a passada falha em um projeto
**THEN** o erro é registrado e os demais projetos são processados

**WHEN** um administrador define `events_days = 0` ou `4000`
**THEN** a resposta é 400 apontando o campo, e nada muda

**WHEN** um administrador define `logs_days = null`
**THEN** o prazo próprio de logs do projeto é removido e vale o padrão da instância

**WHEN** quem chama não é administrador global
**THEN** a resposta é 403

**WHEN** o servidor reinicia
**THEN** a primeira passada ocorre cerca de 60 s depois da partida
