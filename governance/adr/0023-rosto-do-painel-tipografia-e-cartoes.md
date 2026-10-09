# ADR-0023 — Rosto do painel: tipografia serifada e cartões

**Estado:** aceita (2026-10-09), pedido do Edson depois de ver a primeira paleta ("ainda está muito pobre
visualmente") · **Complementa:** ADR-0022

## Contexto

A ADR-0022 trocou as cores, mas o painel continuou com a cara do upstream: títulos de cartão em caixa alta com
espaçamento largo, título de página no mesmo sans em negrito de todo o resto e cartões chapados. A tela de referência
usa um título serifado grande, títulos de cartão em frase normal e cartões mais arredondados com sombra suave.

Os títulos em caixa alta aparecem em mais de 50 lugares do código. Reescrevê-los componente a componente aumentaria muito
a divergência do upstream (ADR-0002) e o custo de cada sincronização.

## Decisão

1. **Tudo por CSS, num bloco só de `styles.css`**, sem editar componente. O bloco fica fora de `@layer` de propósito,
   para vencer as classes utilitárias que os componentes carregam (`uppercase`, `font-extrabold`, `rounded-xl`, `ring-1`).
2. **Títulos de página (`h1`)** na Instrument Serif (licença OFL-1.1, autohospedada por `@fontsource`, sem requisição
   externa), peso 400.
3. **Títulos de cartão** (`[data-slot="card-title"]`) em frase normal, peso 500, cor de texto. Cabeçalhos de tabela
   e rótulos de formulário continuam em caixa alta: ali o recurso comunica "coluna" e "campo".
4. **Cartões** com raio de 1 rem e sombra em duas camadas (contorno de 1 px e sombra suave); no escuro, contorno
   claro e sombra mais escura.

5. **Frase de resumo na Visão geral** (segundo passe). Uma frase serifada abaixo do título responde "há algo errado?"
   antes de qualquer gráfico: "calmo" quando nenhum problema novo apareceu no período, senão quantos problemas novos
   e eventos. A expressão em destaque vai em itálico no laranja de texto (`--brand-text`, AA sobre o fundo, o
   laranja de foco sozinho dá 3:1). Se a consulta falha, a frase some: cada cartão já mostra a sua falha.
6. **Mini gráfico no cartão de eventos**: uma linha pequena com preenchimento, sem eixos, desenhada em SVG próprio
   (`sparkline.ts` calcula a geometria e tem teste). É dica de forma ao lado de um número que já basta sozinho.

## Fora do escopo

- O ícone e o wordmark do topo (D9). O seletor de ambiente e o atalho de busca da referência (não existem no produto).
- Mini gráficos nos demais cartões: só o de eventos tem série temporal no servidor.

## Consequências

- Uma dependência nova (`@fontsource/instrument-serif` 5.3.0), no inventário de licenças.
- Se a D9 trouxer outra identidade, a fonte e o bloco saem num só lugar.
