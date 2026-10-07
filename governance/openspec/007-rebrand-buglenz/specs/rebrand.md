# 007 — Especificação

## Fonte intacta

**WHEN** `git diff --name-status <tag-upstream> main` é filtrado pelos arquivos tocados neste pacote
**THEN** só aparecem arquivos adicionados sob `brand/`, `BUGLENZ.md` e os arquivos de build do fork; nenhum arquivo do upstream é modificado

## Interface

**WHEN** o dashboard construído com a sobreposição é aberto
**THEN** o título da aba é "BugLenz" e o logotipo exibido é o do BugLenz na tela de login, no cabeçalho, na tela de convite e na tela de erro

**WHEN** o texto visível do dashboard construído é varrido em cada idioma
**THEN** "Rustrak" só aparece nas ocorrências da lista de exceções de atribuição

**WHEN** a tela "Sobre" das configurações é aberta
**THEN** ela mostra "baseado em Rustrak", a licença GPL-3.0 e o link para o código-fonte da versão em execução

## Alertas

**WHEN** um alerta de `new_issue` é enviado por e-mail
**THEN** o rodapé e o remetente dizem BugLenz, e o remetente usa o endereço de `SMTP_FROM`

**WHEN** um alerta é enviado por webhook ou Slack
**THEN** o campo `actor` é "BugLenz"

## Zona C preservada

**WHEN** a instância com a marca aplicada é inspecionada
**THEN** as variáveis `RUSTRAK_*`, as métricas `rustrak_*`, as chaves `rustrak:*` do navegador, os caminhos de API e o esquema do banco são os mesmos do upstream

**WHEN** o app de teste React com `@sentry/react` envia erros e source maps
**THEN** o roteiro de ponta a ponta do `GAP-ANALYSIS.md` §3 passa sem alteração

## Proteção contra deriva

**WHEN** um merge do upstream altera o número de ocorrências de uma regra de `rules.json`
**THEN** o build falha e a mensagem cita a regra e as contagens esperada e encontrada

**WHEN** uma regra casaria com uma chave de catálogo, um nome de variável ou um import
**THEN** `verify.mjs` falha antes de gerar a imagem
