# 021 — Catálogo pt-BR do dashboard

## Por quê

O dashboard do upstream fala inglês, chinês, francês, espanhol e romeno. Os usuários do BugLenz são
brasileiros; o roteiro previa o catálogo pt-BR (G5), com a proposta de devolvê-lo ao upstream
(ADR-0002). Sem ele, a tela de erros de produção, o login e a de retenção aparecem em inglês.

## O que muda

1. `apps/dashboard/src/shared/i18n/messages/pt.json`, com as 1 328 chaves do inglês, e o idioma `pt`
   registrado (`routing.ts`, `intl.ts`).
2. `locale.pt` ("Português (Brasil)") nos seis catálogos, para o seletor de idioma.
3. As regras de marca (`catalog-about-attribution`, `catalog-about-source-link`) ganham o texto em
   português; o catálogo mantém as 69 ocorrências do nome do produto que a regra de marca espera.
4. Testes: paridade de chaves com o inglês e resolução `pt-BR`/`pt-PT` → `pt`.

## Decisão de desenho

O código do idioma é **`pt`**, não `pt-BR`. A resolução do idioma do navegador casa pela **etiqueta
base** (`zh-CN` e `zh-TW` chegam a `zh`), então `pt-BR` e `pt-PT` chegam a `pt`; um código `pt-BR` nunca
casaria. O conteúdo é português do Brasil e o nome no seletor diz isso. O servidor aceita qualquer
etiqueta em `users.language`, então nada muda nele.

## O que não muda

- O servidor, a API e o cliente. Textos gerados no servidor (e-mails de alerta) seguem em inglês.
- A documentação (`apps/docs`).

## Dependências

Revisão por falante nativo (tarefa T3): o texto foi escrito por um modelo e conferido por script, não
por uma pessoa. Quem revisa segue o glossário em `governance/baseline/021.md`.
