# 007 — Rebrand: Rustrak → BugLenz

## Por quê

O produto se chama BugLenz (decisão D1). Quem usa o dashboard, recebe um alerta ou abre a tela de
login deve ver BugLenz. Ao mesmo tempo, o fork precisa continuar fazendo merge do upstream a cada
quinzena (ADR-0005), e `rustrak` aparece 4.303 vezes em 527 arquivos. Renomear na fonte tornaria
cada sincronização um conflito.

## O que muda

- Tudo que é visível ao usuário (zona A do ADR-0006) passa a dizer BugLenz.
- Nomes de operação (zona B) passam a `buglenz`.
- A marca é aplicada **no build**, por uma camada de sobreposição (`brand/`), e não por edição dos
  arquivos do upstream.

## O que não muda

Identificadores (zona C): crate `rustrak`, pacotes `@rustrak/*`, variáveis `RUSTRAK_*`, métricas
`rustrak_*`, chaves de `localStorage` `rustrak:*`, nomes de tabela e caminhos de API. A atribuição
ao projeto de origem e a licença permanecem visíveis (GPL-3.0).

## Impacto

- ADR: 0006 (atualizado com o mecanismo), 0002, 0003.
- Depende de: 003 (Dockerfile próprio do fork), identidade visual (decisão D9), domínios (D8).
- Risco: substituição de texto atingir identificador. Mitigado por regras declarativas com
  contagem esperada e por verificação automática da zona C.
