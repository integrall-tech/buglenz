# ADR-0024 — O repositório dos wrappers é público

**Estado:** aceita (2026-10-09, decisão do Edson) · **Fecha:** a parte aberta da D11 · **Relaciona-se com:** ADR-0003 (fronteira
da licença), ADR-0011 (SDKs oficiais com wrapper fino), ADR-0020 (o repositório do fork é público)

## Contexto

Os wrappers (`@integrall/buglenz-react`, o starter Spring Boot e `buglenz_flutter`) vivem em `integrall-tech/buglenz-sdk`, sob
**MIT**, separados do fork (GPL-3.0) para que o código dos apps de quem os usa não herde a GPL (I8). O repositório era privado.

Um repositório privado consome os minutos pagos de Actions da conta. A cota acabou e os jobs passaram a falhar sem iniciar ("recent
account payments have failed or your spending limit needs to be increased"), o que deixou os PRs sem CI. Foi o mesmo problema que
levou à ADR-0020 para o fork.

## Decisão

1. **`integrall-tech/buglenz-sdk` passa a ser público**, sob MIT (feito em 2026-10-09 pelo Edson).
2. **Antes de virar público**, o histórico completo (12 commits, 4 branches) foi varrido atrás de segredos, dados pessoais e nomes
   internos: nada encontrado. A única menção ao upstream é a regra da CI que a proíbe. A CI do repositório continua falhando se um
   wrapper depender de `@rustrak/*` ou mencionar o código do fork.
3. **A publicação em registry (npm, Maven, pub.dev) continua uma decisão à parte.** Tornar o código visível não publica pacote algum.

## Consequências

- A CI do repositório volta a rodar em todo PR sem custo de cota; o job de contrato segue manual (precisa do segredo de leitura da
  imagem, que continua privada, I11).
- Qualquer pessoa pode ler, copiar e usar os wrappers sob MIT. O que ninguém pode é ler a imagem do servidor: ela segue no registry
  privado.
- Não se publica nada novo que identifique clientes: exemplos usam `vendax-*` e `acme` como nomes de app e de cliente fictícios.
