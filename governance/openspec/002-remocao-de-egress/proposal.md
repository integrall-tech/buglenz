# 002 — Remoção de telemetria e de egress herdado

## Por quê

O invariante I3 da CONSTITUTION diz que a instância só inicia conexão com o banco, o provedor
OIDC, o servidor SMTP e os canais de alerta configurados, e que telemetria e checagem de versão
herdadas são **removidas do código**, não desligadas por variável. O Rustrak `v0.16.0` tem duas
saídas que violam isso (ADR-0004, gap G4):

1. O servidor envia um relatório anônimo a `https://us.i.posthog.com/i/v0/e/` dez minutos após o
   boot e depois a cada seis horas, se uma chave foi compilada no binário. [confirmado]
2. O dashboard, no navegador de cada usuário autenticado, busca
   `https://rustrak.github.io/rustrak/versions.json` para mostrar um aviso de nova versão. [confirmado]

Hoje nenhuma das duas dispara em um build do fork: a chave não é compilada e o aviso só aparece
se houver versão mais nova. Isso não basta: um segredo colocado no build, uma variável de ambiente
ou um merge futuro reativam a saída sem ninguém notar. Remover o caminho de envio e verificar a
ausência de egress na CI é o que torna I3 uma propriedade verificada, não uma configuração.

## O que muda

- Sai do servidor o repórter de telemetria (sink PostHog, agendador, relatório, identidade de
  instância, amostragem de recursos), a rota `GET /api/telemetry/preview`, as variáveis
  `RUSTRAK_TELEMETRY` e `DO_NOT_TRACK`, o segredo de build `rustrak_telemetry_key` do `Dockerfile`.
- Sai do dashboard a checagem de versão e o aviso de atualização.
- Entra um **teste de conformidade de rede** na CI, em duas camadas: estática (nenhum host
  proibido no código, no binário e no bundle) e em execução (a instância roda sob bloqueio de
  saída, ingere eventos e não tenta nenhuma conexão).

## O que não muda

- `/metrics` (Prometheus, opt-in por `RUSTRAK_METRICS=on`), os contadores que o alimentam
  (`telemetry/counters.rs`, `telemetry/metrics.rs`) e o middleware que conta respostas.
- OIDC, SMTP, Slack, webhooks: saídas iniciadas pelo operador ao configurá-las (I3).
- Links de navegação para `docs.sentry.io` e `github.com` na UI: o usuário clica, a instância não
  conecta.
- Migrations: a coluna `installation.telemetry_id` fica no schema, sem uso.
- `README.md` e `apps/docs`: documentação do upstream não é editada neste pacote; a divergência
  fica registrada em `CHANGES-FROM-UPSTREAM.md`. A marca e os textos são do pacote 007.

## Impacto

- ADRs: 0004 (decisão), 0002 (delta permanente: o upstream quer a telemetria), 0005 (arquivos
  que vão conflitar em sync: `main.rs`, `telemetry/mod.rs`, `config.rs`, `openapi.rs`,
  `openapi.json`, `Dockerfile`).
- Comportamento visível para o operador: `RUSTRAK_TELEMETRY` e `DO_NOT_TRACK` passam a ser
  ignoradas; `GET /api/telemetry/preview` responde 404; o log de boot não fala mais em telemetria;
  o dashboard não mostra aviso de nova versão.
- Risco: baixo. O código removido não participa da ingestão nem do digest. O risco real é de
  compilação (`CARGO_BUILD_WARNINGS=deny` na CI transforma import não usado em erro) e de
  conflito em merges futuros, aceito no ADR-0004.
- Tamanho: P. Cerca de 900 linhas removidas do servidor, 30 testes removidos, 1 workflow e 1
  script novos.
