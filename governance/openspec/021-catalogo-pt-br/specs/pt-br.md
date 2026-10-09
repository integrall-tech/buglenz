# 021 — Especificação

> **Emenda de 2026-10-09 (auditoria).** O idioma é guardado e escolhido como `pt` (não `pt-BR`); o rótulo mostra "Português (Brasil)". O navegador em `pt-BR` ou `pt-PT` resolve para `pt`.

**WHEN** o navegador declara `pt-BR` (ou `pt-PT`) e o usuário não escolheu idioma
**THEN** o dashboard abre em português

**WHEN** o usuário escolhe "Português (Brasil)" em Conta
**THEN** a escolha é salva na conta, vale em qualquer navegador e o restante do painel muda

**WHEN** qualquer mensagem é pedida em `pt`
**THEN** ela existe (nenhuma `MISSING_MESSAGE`) e seus marcadores `{...}` e tags `<...>` são os mesmos do inglês

**WHEN** o build aplica a marca
**THEN** `pt.json` tem as mesmas 69 ocorrências do nome do produto que os demais catálogos e o texto de atribuição em português
