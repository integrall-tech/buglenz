# ADR-0006 — Marca em zonas, sem renomear identificadores

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

`rustrak` aparece 4.303 vezes em 527 arquivos: nome do crate, escopo npm `@rustrak/*`, variáveis
`RUSTRAK_*`, métricas `rustrak_*`, imagens Docker, textos de UI e documentação. [confirmado]

## Decisão

| Zona | O que é | Tratamento |
|---|---|---|
| A — visível ao usuário | título, logo, favicon, textos do catálogo i18n, e-mails e mensagens de alerta, tela de login | Recebe a marca do fork |
| B — operação | nome da imagem, nome do serviço no Swarm, rótulos de compose | Recebe o nome do fork |
| C — identificadores | crate `rustrak`, escopo `@rustrak/*`, variáveis `RUSTRAK_*`, métricas `rustrak_*`, tabelas, caminhos de API | **Não muda** |

A UI mantém em "Sobre" a origem ("baseado em Rustrak, GPL-3.0") e o link para o código-fonte.

## Mecanismo

A zona A não é editada na fonte. A marca é aplicada no build por uma camada de sobreposição
(`brand/`), com substituições declarativas de contagem conferida e arquivos de logotipo
substituídos por inteiro. Os arquivos do upstream ficam idênticos aos da tag. Detalhe no pacote
`openspec/007-rebrand-buglenz/`.

## Justificativa

Renomear a zona C tocaria centenas de arquivos e tornaria cada merge um conflito. A zona A
concentra-se no catálogo de mensagens e em poucos assets, o que mantém o delta pequeno. [inferência]

## Consequências

Painéis do Grafana usam métricas com prefixo `rustrak_`.

## Nome e domínios

O produto se chama **BugLenz** (decisão D1, fechada em 2026-10-07). Zona B: imagem
`buglenz-server`, serviço `buglenz` no Swarm.

Domínios informados pelo Edson: `buglenz.io`, `buglenz.app`, `buglenz.dev`. Proposta de uso, a
confirmar (decisão D8):

| Domínio | Uso proposto |
|---|---|
| `buglenz.app` | Instância interna: dashboard e ingestão (`PUBLIC_URL`) |
| `buglenz.dev` | Documentação e guia dos wrappers de SDK |
| `buglenz.io` | Página do produto |

O host de ingestão vai dentro do DSN e fica embutido em cada build de cada app. Trocar depois
exige novo build de todos; por isso o host é escolhido uma vez, antes do primeiro produto piloto.
`.app` e `.dev` só funcionam com HTTPS (TLDs com HSTS pré-carregado), o que casa com o ADR-0012.

Ressalva registrada: existe uma extensão de Chrome chamada "BugLens". Marca no INPI não verificada.
