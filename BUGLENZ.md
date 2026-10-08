# BugLenz

BugLenz é a plataforma de rastreamento de erros e de saúde de release da IntegrAllTech. Recebe
eventos dos SDKs oficiais do Sentry (web em React, backends Spring Boot, mobile em Flutter) e roda
em infraestrutura própria ou do cliente.

## Origem e licença

BugLenz é um **fork do [Rustrak](https://github.com/rustrak/rustrak)**, um servidor de
rastreamento de erros compatível com o Sentry. Este repositório parte da tag `v0.15.2` do
upstream e acompanha as tags estáveis seguintes por merge (hoje `v0.16.0`; ver `NOTICE.md` e
`DELTA-MANIFEST.md`).

- Licença: **GNU GPL-3.0-only**, a mesma do Rustrak. O texto está em [`LICENSE`](LICENSE).
- Copyright: Abian Suarez e os demais contribuidores do Rustrak sobre o código de origem; IntegrAllTech
  sobre as modificações. Detalhes em [`NOTICE.md`](NOTICE.md).
- Quem recebe o BugLenz (binário ou imagem) tem direito ao código-fonte correspondente. O código-fonte é
  este repositório, na tag da versão entregue (`vX.Y.Z-itl.N`). A forma de entrega a clientes está sob
  revisão jurídica (decisão D2).

Este arquivo é a apresentação do fork. O `README.md` da raiz é o do upstream e continua como era.

## O que o fork muda

Tudo o que difere do upstream está em [`DELTA-MANIFEST.md`](DELTA-MANIFEST.md), com a decisão que
o justifica, e resumido em [`CHANGES-FROM-UPSTREAM.md`](CHANGES-FROM-UPSTREAM.md). Em linhas gerais:

- sem telemetria nem checagem de versão: a instância só abre conexão com o banco, o provedor OIDC,
  o servidor SMTP e os canais de alerta configurados;
- dados pessoais tratados antes de persistir, e IP do cliente não gravado;
- a marca BugLenz é aplicada **no build**, por uma camada de sobreposição em [`brand/`](brand/); os
  arquivos do upstream não são editados, e os identificadores (`RUSTRAK_*`, `rustrak_*`,
  `@rustrak/*`, tabelas, caminhos de API) não mudam;
- imagem PostgreSQL publicada em registry privado, stack para Docker Swarm em
  [`deploy/swarm/`](deploy/swarm/).

## Como a marca é aplicada

```bash
node brand/apply.mjs buglenz --dest /tmp/branded --var release=v0.16.0-itl.5
node brand/verify.mjs buglenz --branded /tmp/branded
node --test "brand/test/*.test.mjs"
```

As regras ficam em `brand/buglenz/rules.json`, cada uma com o número de ocorrências esperado: um
merge do upstream que acrescente ou retire um texto de marca faz o build falhar citando a regra.
O logotipo e os ícones são **provisórios** até a identidade visual (decisão D9).

## Governança

Constituição, ADRs, pacotes de execução e linhas de base medidas estão em [`governance/`](governance/),
em português. O ponto de partida é `governance/README.md`.

## Endereços

| Uso | Endereço |
|---|---|
| Instância e DSN | `errors.buglenz.dev` (a confirmar) |
| Documentação | `docs.buglenz.dev` |
| Remetente dos alertas | `alerts@buglenz.dev` |
