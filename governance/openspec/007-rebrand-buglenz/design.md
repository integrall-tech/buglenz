# 007 — Design

Marcação: [confirmado] = medido no `v0.15.2`; [inferência] = proposta.

## Inventário da zona A [confirmado]

| Onde | O que | Quantidade |
|---|---|---|
| `apps/dashboard/src/shared/i18n/messages/*.json` | "Rustrak" em textos de interface | 69 por catálogo, 5 catálogos |
| `apps/dashboard/index.html` | `<title>` e `<meta description>` | 2 |
| `apps/dashboard/public/` | `icon.png`, `apple-icon.png` | 2 arquivos |
| `apps/dashboard/src/shared/ui/components/rustrak-wordmark.tsx` | Logotipo em SVG (a palavra convertida em traçado), `aria-label` | 1 componente; usado em login, cabeçalho, convite, vínculo de conta, erro e "Sobre" |
| `packages/ui/src/components/brand/wordmark.tsx` | Mesmo logotipo no design system | 1 componente |
| `apps/server/src/services/notification/email.rs` | Rodapé "This alert was sent by Rustrak…" (HTML e texto) | 2 |
| `apps/server/src/services/{alert,notification/*}.rs` | `actor: "Rustrak"` no corpo dos alertas | 6 |
| `apps/server/src/services/notification/custom_webhook.rs` | Modelos prontos com "Rustrak: {{ issue.title }}" e `"source":"rustrak"` | 4 fora de teste |
| `apps/server/src/openapi.rs` | Título da API | 1 |
| Remetente padrão | `alerts@rustrak.local` | resolvido por `SMTP_FROM`, sem tocar código |

Fora do rebrand: `apps/docs` (895 ocorrências; o site do upstream não é publicado pelo fork, e a
documentação do BugLenz vive em `buglenz.dev`) e o `README.md` da raiz, que fica como o do
upstream. A apresentação do fork fica em um arquivo novo, `BUGLENZ.md`.

O aviso de nova versão (`update-banner.tsx`) usa o logotipo, mas já terá sido removido pelo
pacote 002.

## Mecanismo: sobreposição no build

```
brand/
├── buglenz/
│   ├── assets/            icon.png, apple-icon.png, wordmark.svg
│   ├── overrides/         arquivos inteiros que substituem os do upstream
│   │   ├── apps/dashboard/src/shared/ui/components/rustrak-wordmark.tsx
│   │   └── packages/ui/src/components/brand/wordmark.tsx
│   └── rules.json         substituições declarativas
├── apply.mjs              aplica overrides e regras sobre uma cópia da árvore
└── verify.mjs             confere o resultado
```

1. O build do fork (Dockerfile próprio, pacote 003) copia a árvore para um diretório de trabalho e
   roda `brand/apply.mjs buglenz` antes de `pnpm build` e `cargo build`.
2. **Overrides** substituem o arquivo inteiro. Os dois componentes de logotipo mantêm o nome do
   arquivo e a assinatura exportada (`RustrakWordmark`, mesmas props), para não tocar nos
   arquivos que os importam.
3. **Regras** em `rules.json`: `{ arquivo, padrão, substituição, ocorrências_esperadas }`. Nos
   catálogos JSON a regra age só sobre valores, nunca sobre chaves.
4. Se a contagem real diferir da esperada, o build falha citando a regra. É o que acontece quando
   um merge do upstream acrescenta ou remove um texto com a marca: alguém revisa e ajusta o número.
5. Os arquivos do upstream no git ficam **idênticos** aos da tag. O `DELTA-MANIFEST.md` ganha
   apenas arquivos novos.

Por que não editar a fonte: 345 linhas de catálogo, mais os alertas do servidor, mudam com
frequência no upstream; cada edição nossa viraria conflito recorrente. [inferência]

## Atribuição e licença

- Tela "Sobre" das configurações, que já existe (`settings/about.tsx`) [confirmado]: "BugLenz — baseado em Rustrak, GPL-3.0", com link para o código-fonte
  correspondente à versão em execução.
- `LICENSE` e avisos de copyright intactos; `NOTICE.md` já existe desde o pacote 001.
- `verify.mjs` mantém uma lista de exceções com as ocorrências de "Rustrak" que **devem**
  continuar visíveis (atribuição).

## Zona B

Imagem `buglenz-server`, serviço e rótulos `buglenz` no stack do Swarm, `SMTP_FROM` no domínio
definido em D8, `OIDC_PROVIDER_NAME` conforme o ArchGuard.

## Contribuição ao upstream (ADR-0002)

Abrir no upstream a proposta de um nome de produto configurável (constante única ou variável de
ambiente usada pela UI e pelos alertas). Se aceita, `rules.json` encolhe a cada versão até sobrar
só o logotipo. [não verificado: disposição do mantenedor]

## Identidade visual (decisão D9)

Ativos necessários antes da execução: logotipo em SVG (versão clara e escura), ícone 512×512,
ícone Apple 180×180 e, se desejado, tokens de cor. A UI nasce em tema escuro. Sem os ativos, o
pacote pode ser executado com logotipo tipográfico provisório.

## 7. O que a execução revelou (2026-10-08) [confirmado na `v0.16.0`]

- **Inventário maior que o previsto.** Além do que a tabela lista: os modelos de webhook do
  dashboard (`features/alert/model/message-template.ts`: 5× `Rustrak: {{`, `actor`, `"source":
  "rustrak"`), os rodapés `© <ano> Rustrak` de `login.tsx` e `error-screen.tsx`, o título de
  `settings.tsx`, `topbar.tsx` do design system, o placeholder `alerts@rustrak.local` do catálogo, a
  mensagem de SSO de `users.rs` e o `actor "Rustrak Test"` do alerta de teste. Os "modelos
  prontos" que a tabela atribuía a `custom_webhook.rs` existem **só em módulos de teste**.
- **Contagens medidas:** 68 `Rustrak` por catálogo (nos 5), todos em valores, nenhum em chave;
  catálogos regravam idênticos (`JSON.stringify(…, null, 2) + "\n"`), o que permite `catalog-set`.
- **Zona C que o design não citava:** cabeçalhos `X-Rustrak-Signature/Timestamp/Request-ID` do
  webhook, `X-Rustrak-Incident`, nomes de tipos `RustrakError`/`RustrakClient` (dezenas de usos no
  dashboard). Por isso as regras são por arquivo e por texto exato, nunca globais.
- **Mecanismo no build:** o workflow aplica a marca a uma **cópia** (`--dest`), verifica e constrói
  a imagem da cópia; o `Dockerfile` do upstream não muda. `--in-place` só roda com `CI=true`.
  Os diretórios do próprio fork (`brand/`, `governance/`, `e2e/`, `deploy/`) ficam fora da cópia
  só na raiz (`apps/server/tests/e2e` é mantido).
- **Arquivos substituídos por inteiro** (overrides) saem da contagem de zona C e, em troca, a
  verificação exige que mantenham os nomes exportados (`RustrakWordmark`, `Wordmark`).
- **"Sobre"**: o link do rastreador do upstream foi removido e o de repositório aponta para o
  `NOTICE.md` da tag (`${release}`); atribuição em cinco idiomas (en, es, fr, ro, zh).
- **Texto de marketing do upstream** no painel do login ("50MB", "<50ms", "10k+") não é marca e
  não foi alterado; não é medição deste fork.
- **D8 fechada (2026-10-08):** domínio `buglenz.dev`; propostas `errors.buglenz.dev` (instância e
  DSN, a confirmar), `docs.buglenz.dev`, remetente `alerts@buglenz.dev`.
