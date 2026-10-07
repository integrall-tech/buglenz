# ADR-0012 — Implantação e observabilidade

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

Infra padrão: Docker Swarm + Traefik; observabilidade em VictoriaMetrics, Grafana, Loki e Tempo.
O Rustrak roda em um container, expõe `/metrics` sem autenticação quando `RUSTRAK_METRICS=on` e
escreve logs em stdout. Estado em disco: `INGEST_DIR` e `SOURCEMAP_STORAGE_PATH`. [confirmado]

## Decisão

- Imagem compilada com `--no-default-features --features postgres`, Rust fixado em
  `rust-toolchain.toml`, publicada no registry privado (I7, I11).
- Um serviço no Swarm com **uma réplica** (G20: a fila de ingestão é local ao processo).
- PostgreSQL dedicado ou banco dedicado em servidor existente; versão mínima a confirmar.
- Volume persistente para `/data` (source maps) e para `INGEST_DIR`, que sai de `/tmp`. O
  upstream reconhece o problema na issue #359: a imagem declara `VOLUME /data` mas `INGEST_DIR`
  fica em `/tmp/rustrak/ingest`, e eventos aceitos e não digeridos se perdem ao recriar o
  container [confirmado na issue, 2026-10-07]. A issue #356 registra que
  `RUSTRAK_BOOTSTRAP_TOKEN` ignora o valor informado e imprime um token aleatório em stderr, o
  que afeta o provisionamento automatizado; o pacote 003 precisa contornar.
- Traefik: TLS, `SSL_PROXY=true`, `PUBLIC_URL` definido. A rota `/metrics` **não** é publicada;
  o scrape vem pela rede interna.
- Rotas de ingestão (`/api/{id}/envelope/`, `/api/{id}/store/`) e dashboard no mesmo host. Limite
  de corpo e rate limit no Traefik como segunda barreira.
- Logs para o Loki pelo coletor do Swarm. Painel no Grafana com `rustrak_ingest_*` e alerta para
  fila de digest pendente e rejeições.
- Backup do banco e do volume de source maps pelo BackupLenz.

## Em aberto

Localização da instância interna (decisão D4). A infraestrutura em nuvem atual está em
`us-east-1`; eventos contêm dados pessoais de usuários de clientes. [inferência sobre o impacto]

## Consequências

Indisponibilidade da instância não derruba os apps: o SDK descarta o envio. Reinício perde no
máximo o que estava em memória; pendentes em `INGEST_DIR` são reprocessados no boot. [confirmado
no código; não testado aqui]
