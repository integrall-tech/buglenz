# 009 — Design

Marcação: [confirmado] = visto no código ou em execução neste projeto; [inferência] = proposta;
[a confirmar] = depende de resposta do Edson.

## 1. Onde vivem os wrappers [a confirmar]

ADR-0011: "fora do repositório do fork e fora do alcance da GPL". Proposta [inferência]: **um
repositório privado `integrall-tech/buglenz-sdk`** com dois pacotes (`react/` e `spring-boot/`) e
a matriz de homologação, licença proprietária da IntegrAllTech, publicados em registry privado
(npm e Maven). Alternativa: um repositório por plataforma. Em qualquer caso:

- nenhum `package.json`, `pom.xml` ou `build.gradle` referencia `@rustrak/*` nem copia código de
  `apps/dashboard` ou `packages/ui` (ADR-0003, item 4); o job de CI do repositório falha se
  encontrar `rustrak` em dependências;
- o contrato com a instância é o protocolo do Sentry e, quando preciso, o `openapi.json` publicado
  pela própria instância (cliente gerado, não copiado de `@rustrak/client`).

## 2. Wrapper React

**Archbase (decidido, 2026-10-08):** o wrapper é um **pacote irmão** (`@integrall/buglenz-react`, nome a confirmar), com versão e repositório próprios, anunciado junto do Archbase. Não depende do Archbase nem de Mantine: o fallback do `ErrorBoundary` é HTML simples e aceita um componente passado pelo app, de modo que o Archbase pode fornecê-lo sem acoplar o wrapper.

### 2.1 Interface [inferência]

```ts
initBugLenz({
  dsn: string,                 // do projeto na instância; sem fallback
  app: string,                 // "vendax-web"  → release "<app>@<version>"
  version: string,             // injetada no build
  environment: 'production' | 'homolog' | string,
  tenant?: string,             // tag
  cliente?: string,            // tag
  extra?: Partial<BrowserOptions>, // só o que o wrapper não fixa
}): void
export { ErrorBoundary }       // com fallback padrão
export function identify(user: { id: string }): void   // rejeita e-mail e documento
export function brandSourceMaps(opts): Plugin          // vite-plugin com SENTRY_URL da instância
```

### 2.2 O que fixa [confirmado onde há teste]

| Item | Valor | Evidência |
|---|---|---|
| SDK | `@sentry/react` **11.5.0** homologado contra Rustrak `v0.16.0` + pacote 006 | `e2e-react` na CI do fork |
| `sendDefaultPii` | `false` | ADR-0009. O tipo `BrowserOptions` do 11.x **não** tem a propriedade [confirmado no 006: erro de compilação]; confirmar o nome atual no `@sentry/core` antes de usar |
| Dedupe | ligado (padrão do SDK); dois erros iguais seguidos viram um envelope | achado D2 do 006 |
| Sessões | automáticas; o servidor aceita `unhandled`; um terminal por sessão | pacote 006; o SDK Java difere (seção 3) |
| `beforeSend` / `beforeBreadcrumb` | removem cabeçalhos `cookie`/`authorization`, corpo de requisição e texto digitado; o servidor ainda mascara (camada 2) | ADR-0009 |
| Usuário | só `id` interno; `identify` lança em ambiente de teste se receber `email`/`cpf` | ADR-0009 |
| Tunnel | **indisponível** (G9): bloqueadores de anúncio podem barrar o envio direto do navegador | GAP G9 |

### 2.3 Source maps

`@sentry/vite-plugin` 5.x com `url` da instância e `telemetry: false`; o servidor ignora `org` e
usa o slug do projeto [confirmado no GAP §3 e no `e2e-react`]. Os `.map` são apagados do `dist`
depois do upload (`filesToDeleteAfterUpload`). Token de API da instância, no cofre do CI do app.

## 3. Wrapper Spring Boot [a verificar]

`sentry-spring-boot-starter-jakarta` (Spring Boot 3, Java 21). O starter lê `sentry.*`; o wrapper é
um **auto-configurador** pequeno que:

- expõe `buglenz.dsn`, `buglenz.app`, `buglenz.environment`, `buglenz.tenant`, `buglenz.cliente`;
- define `sentry.release = <app>@<versão do artefato>`, `sentry.send-default-pii = false`;
- registra um `SentryOptions.BeforeSendCallback` que remove cabeçalhos sensíveis, corpo e
  parâmetros conhecidos (mesma lista de chaves do ADR-0009);
- lança na partida se `buglenz.dsn` estiver ausente em `production`.

**Verificado em 2026-10-08 (T1, `governance/baseline/009-t1.md`)** com Spring Boot 3.5.16, Java 21 e
`sentry-spring-boot-starter-jakarta` 8.60.0 contra a instância:

- o envelope (gzip, `X-Sentry-Auth`, itens `event`) é aceito; `release`, `environment`, tags,
  `mechanism` e `level` chegam corretos, e o scrub do pacote 005 age sobre o evento real;
- o SDK cru **envia PII integral** (e-mail em `user` e breadcrumb, `extra.password`, CPF na mensagem):
  a camada 1 do wrapper é necessária;
- **sem `startSession()` o SDK não envia sessão nenhuma.** Com sessão explícita, envia `crashed`
  duas vezes para o mesmo `sid`, e o servidor conta duas quedas e `healthy -1` (**G24**).
  Decisão de desenho: **o wrapper Spring Boot não liga sessões** (release health é do front-end e do
  mobile); o G24 vira trabalho de servidor antes de qualquer SDK que repita o terminal.

## 4. Testes de contrato

| Plataforma | Teste | Onde |
|---|---|---|
| React | o `e2e-react` do fork, com o app e2e passando a usar o wrapper em vez do SDK cru | CI do repositório dos wrappers, contra a imagem `buglenz-server` publicada (`ghcr.io/…:vX.Y.Z-itl.N`) |
| Spring Boot | app de exemplo que lança uma exceção, com e-mail, CPF e `password` num campo; a asserção lê a issue pela API: sem os valores originais, `release` correto, tags, sessão contada | idem |
| Matriz | job que, para cada par (SDK, versão do fork) declarado, roda os dois testes; só um par verde entra na matriz | idem |

Os testes sobem a instância do mesmo jeito que o `e2e-react` do fork (SQLite ou o compose
PostgreSQL do upstream), com a imagem publicada.

## 5. Onboarding de um app

1. Admin cria o projeto na instância (`vendax-web`, `vendax-api`: um projeto por app, ADR-0007) e
   copia o DSN; token de API para o CI.
2. App adiciona o wrapper, define `app`, `version`, `environment`, tags.
3. CI do app envia source maps.
4. Primeiro erro provocado em homologação; conferir issue, frame legível e ausência de dado
   pessoal; só então produção.
5. Alerta: um canal (Slack, e-mail ou webhook) e uma regra `new_issue` no projeto.

Guia em `docs/onboarding.md` do repositório dos wrappers, em português.

## 6. Produto piloto [a confirmar]

Critérios propostos [inferência]: web React + backend Spring Boot (exercita os dois wrappers),
interno ou de baixo risco, equipe que consiga reagir a falso positivo de scrub, sem dado sensível
de saúde ou financeiro nos erros. O ADR-0009 cita o e-mail do usuário, cabeçalhos HTTP, corpo de
requisição e breadcrumbs como o conteúdo típico.

## 7. Implantação da instância [a confirmar]

Usa o pacote 003: stack Swarm, imagem privada, `buglenz.dev`. O que falta para o piloto: D4
(localização, I12), host do DSN (proposta `errors.buglenz.dev`), credencial de leitura do GHCR nos
nós, provedor de login (senha local do usuário primário até o 008), backup (BackupLenz), scrape e
logs. O host do DSN fica embutido em cada build de app: **trocá-lo depois exige novo build**.

## 8. O critério de saída da Fase 1 não fecha só com este pacote

ROADMAP: "um produto piloto em produção enviando erros por 30 dias, com **retenção e scrubbing
ativos**, sem evento contendo dado da lista de negação em amostragem manual". O scrubbing está
pronto (005). A **retenção** é o pacote 004, bloqueado pela decisão D6 (prazos). Sem ela, a
instância do piloto acumularia dados sem prazo, o que o invariante I5 proíbe: ou o 004 entra antes
da produção, ou a produção começa com limpeza manual agendada e registrada como exceção.
