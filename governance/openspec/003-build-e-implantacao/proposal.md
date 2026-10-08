# 003 — Build e implantação

## Por quê

Depois do 002 o fork tem um binário que não fala com ninguém que o operador não configurou. Falta
o caminho que leva esse binário até um servidor: uma imagem construída a partir de uma tag do
fork, publicada em registry privado (I11), e uma forma repetível de subi-la no Docker Swarm atrás
do Traefik, com métricas, logs e backup (ADR-0012). Sem isso, cada instância (ADR-0007) seria
montada à mão.

Dois problemas conhecidos do upstream entram aqui porque se manifestam só em implantação
(GAP-ANALYSIS §9): `INGEST_DIR` fora do volume `/data` (issue #359: eventos aceitos e não
digeridos se perdem ao recriar o container) e `RUSTRAK_BOOTSTRAP_TOKEN` que ignora o valor e
imprime um token aleatório (#356).

## O que muda

- Workflow `release-image.yml`: a cada tag `vX.Y.Z-itl.N`, constrói o dashboard, embute no
  contexto do servidor e publica `ghcr.io/integrall-tech/buglenz-server:<tag>` (PostgreSQL,
  `linux/amd64`), privada. Mantém as últimas N versões.
- `Dockerfile` do servidor: `INGEST_DIR` passa a ficar sob `/data` (uma linha; #359).
- `deploy/swarm/`: stack parametrizada (serviço `buglenz`, PostgreSQL, Traefik labels, volumes,
  secrets), `README` com o procedimento de primeira subida, provisionamento do admin e do token
  de API sem `RUSTRAK_BOOTSTRAP_TOKEN` (#356), rotina de backup e restauração.
- Workflow `licenses.yml`: regenera `THIRD-PARTY-LICENSES.md` em Linux a cada PR e falha se
  divergir do commitado (o inventário deixa de depender da máquina de quem gerou).

## O que não muda

- Nenhum arquivo em `apps/*/src`, `packages/*/src`, `migrations`. O binário é o do 002.
- Nenhuma imagem SQLite: I7 diz que PostgreSQL é o único banco implantado.
- Nenhuma imagem `ui` separada: ADR-0012 é um container com o dashboard embutido.
- Nenhum valor de ambiente real no repositório: hosts (D8), localização (D4), senhas e tokens
  entram como variáveis e secrets na implantação.
- Marca: a imagem se chama `buglenz-server` (zona B, ADR-0006), mas a UI ainda diz Rustrak até
  o pacote 007.

## Divisão acordada (2026-10-08)

O ROADMAP fazia o 003 depender de D4 e D8. Decidido: o pacote entrega o que não depende delas
(imagem, publicação, stack parametrizada, checagens); a implantação da instância interna com
hosts e localização reais é um passo operacional após D4 e D8, registrado em
`governance/baseline/003.md` quando acontecer.

## Impacto

- ADRs: 0012 (decisão), 0005 (imagem com a mesma tag do sync), 0006 (zona B), 0003 (a imagem
  privada não é distribuição a terceiros; a oferta de fonte segue valendo para quem a receber).
- Custo de CI: um build release em Docker por tag (~10 min em amd64) e uma geração de licenças
  por PR (~2 min).
- Risco: baixo. Publicação só por tag, em registry privado, com `GITHUB_TOKEN` do próprio
  repositório; nenhum segredo novo para publicar. Para **puxar** nos servidores é preciso uma
  credencial de leitura do GHCR, que é decisão de implantação (ver `design.md`, "Em aberto").
- Tamanho: P a M.
