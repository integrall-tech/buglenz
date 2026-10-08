# BugLenz no Docker Swarm

Implantação de uma instância (ADR-0007: uma por cliente, mais a interna) com a stack
`buglenz.stack.yml`: o servidor BugLenz (API + dashboard, uma réplica) e um PostgreSQL 16,
atrás do Traefik. Decisões em `governance/adr/0012-implantacao-e-observabilidade.md`.

Nada aqui é específico de uma instância. Host, senhas e chaves entram pelo ambiente de quem
implanta e ficam no cofre da IntegrAllTech, nunca neste repositório.

## Pré-requisitos

- Swarm inicializado; uma rede overlay externa para o Traefik (padrão `traefik`), com o Traefik
  já publicando `websecure` e um `certresolver` (padrão `letsencrypt`).
- DNS do host público (`BUGLENZ_HOST`, sob `buglenz.dev`: proposta `errors.buglenz.dev`) apontando para o Traefik. O host entra no DSN de cada
  aplicação; trocá-lo depois exige novo build dos apps (ADR-0006, decisão D8).
- Credencial de **leitura** do GHCR em cada nó que pode rodar o serviço: a imagem é privada.
  `docker login ghcr.io -u <usuário> --password-stdin` com um token que tenha `read:packages`;
  depois `docker stack deploy --with-registry-auth`. Qual credencial (conta de serviço ou GitHub
  App) é decisão de implantação, fora deste pacote.

## Primeira subida

```bash
cat > buglenz.env <<EOF
BUGLENZ_TAG=v0.16.0-itl.2
BUGLENZ_HOST=errors.buglenz.dev
POSTGRES_PASSWORD=$(openssl rand -hex 16)
SESSION_SECRET_KEY=$(openssl rand -hex 32)
CREATE_SUPERUSER=admin@example.com:$(openssl rand -base64 18)
EOF
chmod 600 buglenz.env

set -a; source buglenz.env; set +a
docker stack deploy --with-registry-auth -c buglenz.stack.yml buglenz
docker service ls --filter name=buglenz
curl -fsS https://$BUGLENZ_HOST/health
```

O `CREATE_SUPERUSER` cria o primeiro administrador no primeiro boot. Guarde a senha no cofre.

## Provisionamento (token de automação e primeiro projeto)

O `RUSTRAK_BOOTSTRAP_TOKEN` do upstream ignora o valor informado e imprime um token aleatório no
stderr do serviço (issue #356), inútil para automação. Em vez dele:

```bash
BUGLENZ_URL=https://$BUGLENZ_HOST \
BUGLENZ_ADMIN_EMAIL=admin@example.com BUGLENZ_ADMIN_PASSWORD='...' \
deploy/swarm/provision.sh vendax-web
```

Imprime `BUGLENZ_API_TOKEN`, `PROJECT_ID` e `DSN` uma única vez. Fale sempre com o host HTTPS
(pelo Traefik): com `SSL_PROXY=true` a cookie de sessão é `Secure`, e um `curl` em HTTP puro
recebe `401` no passo seguinte ao login. Depois disso, remova
`CREATE_SUPERUSER` do `buglenz.env` e reimplante: a variável só serve ao primeiro boot.

## Dados pessoais

O servidor remove dados pessoais antes de gravar (ADR-0009): chaves negadas viram `[Filtered]`,
CPF/CNPJ/cartão/e-mail em texto viram marcadores, e o IP do cliente não é gravado. Para
acrescentar chaves negadas nesta instância (nomes de campo dos seus apps), defina
`RUSTRAK_SCRUB_EXTRA_KEYS=documento,telefone` no ambiente do serviço. Exclusão por titular:
`DELETE /api/projects/{id}/privacy/users/{user_id}` com token de admin.

Destinos de webhook internos (um servidor de chat ou o agente de triagem na mesma rede) são
bloqueados por padrão; liste os hosts permitidos, como aparecem na URL, em
`RUSTRAK_WEBHOOK_ALLOWED_HOSTS=archflow.internal,10.1.2.3` (ADR-0018).

## Métricas e logs

- `/metrics` está ligado (`RUSTRAK_METRICS=on`) e **não** é publicado pelo Traefik: o router
  exclui `PathPrefix(/metrics)`. O coletor (vmagent) entra na rede `buglenz_internal`
  (`attachable: true`) e lê `http://server:8080/metrics`. Como o VictoriaMetrics da
  IntegrAllTech descobre alvos no Swarm ainda está por definir; até lá, alvo estático.
- Logs vão para stdout no formato do `env_logger`; o coletor do Swarm (driver Docker ou
  Promtail/Alloy, a definir) leva ao Loki. `RUST_LOG=info` por padrão.
- Painel e alertas do Grafana (`rustrak_ingest_*`, `rustrak_spool_pending`,
  `rustrak_http_*`): entram quando a forma de scrape estiver definida.

## Atualização

Troque `BUGLENZ_TAG` no `buglenz.env` e reimplante. `update_config.order: stop-first` para a
réplica antiga antes de subir a nova: o spool em `/data/ingest` é lido pela nova no boot. Para
voltar: `docker service rollback buglenz_server`.

Migrations do banco rodam no boot do servidor. Antes de uma atualização de versão minor do
upstream (`0.x` admite quebra em minor), leia `apps/docs/content/upgrading/` da tag nova e faça
backup.

## Backup e restauração

```bash
deploy/swarm/backup.sh backup /srv/backups/buglenz        # pg_dump + tar do volume /data
deploy/swarm/backup.sh restore buglenz-<stamp>.sql.gz buglenz-<stamp>-data.tar.gz
```

A restauração para o servidor, recria o banco, repõe o volume e religa. Como o BackupLenz vai
consumir esses arquivos (ou substituir o script) está por definir.

## O que não está aqui

- Valores reais de host, região e segredos (decisões D4 e D8; cofre).
- A imagem: construída e publicada por `.github/workflows/release-image.yml` a cada tag
  `vX.Y.Z-itl.N`.
- Marca na interface: a UI ainda diz "Rustrak" até o pacote 007.
