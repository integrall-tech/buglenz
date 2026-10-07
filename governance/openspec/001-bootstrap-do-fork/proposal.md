# 001 — Bootstrap do fork

## Por quê

Antes de qualquer mudança de comportamento, o fork precisa existir como repositório governado:
com a origem rastreável, os artefatos que a GPL-3.0 exige, o CI do upstream rodando na
infraestrutura da IntegrAllTech e uma linha de base medida contra a qual os próximos pacotes são
comparados.

## O que muda

- Repositório privado criado a partir da tag `v0.15.2` do upstream.
- Remotes configurados com push para o upstream desabilitado.
- Artefatos de conformidade e de governança adicionados.
- Workflows de publicação do upstream removidos; CI de verificação mantido.
- Baseline de testes registrada.

## O que não muda

Nenhum arquivo de `apps/server/src`, `apps/dashboard/src` ou `packages/*/src`. Nenhuma migration.
O binário produzido por este pacote é funcionalmente o do upstream `v0.15.2`.

## Impacto

- ADRs: 0001, 0002, 0003, 0005.
- Risco: baixo. O único efeito externo possível é publicação acidental em registry ou npm
  públicos, que a remoção dos workflows elimina.
