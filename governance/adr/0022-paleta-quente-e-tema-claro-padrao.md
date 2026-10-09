# ADR-0022 — Paleta quente e tema claro como padrão

**Estado:** aceita (2026-10-09), pedido do Edson com uma tela de referência · **Relaciona-se com:** ADR-0006 (marca),
D9 (identidade visual definitiva, ainda aberta)

## Contexto

O painel herdado do upstream é escuro por padrão, em cinzas neutros com verde-limão como cor de ação. O Edson pediu
uma paleta no estilo Claude, a partir de uma tela de referência: papel quente, cartões brancos, barra lateral de
tinta, um laranja como cor da marca e botões de ação em tinta. O tema claro passa a ser o padrão.

Medição do estado anterior (feita ao escrever o teste): no tema claro, o texto branco sobre o verde-limão do botão
primário tinha **2,9:1**, abaixo do mínimo de 4,5:1 do AA.

## Decisão

1. **Paleta.** Claro: fundo `oklch(0.965 0.005 85)` (papel), cartão branco, tinta `oklch(0.2 0.004 85)`, borda
   quente. Escuro: o mesmo desenho em tinta quente. **Laranja** `oklch(0.68 0.19 45)` como marca, anel de foco e item
   ativo da barra lateral.
2. **A ação primária é tinta no claro e laranja no escuro.** Texto branco sobre esse laranja dá cerca de 3:1; por isso
   no claro o laranja não carrega texto, e no escuro ele leva texto de tinta (5,7:1).
3. **A barra lateral é de tinta nos dois temas.** Seus componentes usam os tokens da página (`text-muted-foreground`,
   `bg-primary`…); em vez de editá-los, o CSS os redefine dentro de `[data-sidebar="sidebar"]`. Menus e diálogos
   saem da barra por portal e mantêm o tema da página.
4. **Tema claro por padrão** (`defaultTheme="light"` e `color-scheme: light` no HTML estático). Quem já escolheu um
   tema mantém a escolha; "sistema" continua disponível.
5. **Teste.** `apps/dashboard/src/shared/lib/palette.test.ts` fixa AA nos pares de leitura dos dois temas, a barra
   lateral e a ausência do limão antigo.

## Fora do escopo

- A fonte serifada dos títulos da referência (exigiria uma fonte nova, autohospedada). Pode ser feita à parte.
- O logotipo e os ícones (`icon.png`, wordmark): seguem os atuais até a D9.
- `packages/ui/src/styles/tokens.css` (só o Storybook usa): não foi tocado.

## Consequências

- O painel muda de aparência para quem o usa; nenhuma API ou dado muda.
- Três arquivos do upstream passam a divergir (`styles.css`, `main.tsx`, `index.html`), todos no manifesto.
- Se a D9 trouxer outra cor de marca, a troca é de valores em `styles.css`; o teste diz se continua legível.
