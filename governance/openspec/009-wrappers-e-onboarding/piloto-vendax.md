# Piloto: VendaX.ai (D12)

**Decidido em 2026-10-08 (Edson):** produto **VendaX.ai**; aplicações **admin web** e **app mobile
Flutter**; usado por **clientes**; ambientes **production e homolog**; canal de alerta **configurável**
(hoje usam **Mattermost**).

## Isso muda o plano do 009

O plano original supunha **web React + backend Spring Boot**. O piloto é **web + Flutter** e não cita backend.

| Aplicação | Wrapper | Estado |
|---|---|---|
| admin web (React, a confirmar) | `@integrall/buglenz-react` 0.1.0 | pronto e homologado contra a `itl.6` |
| app mobile Flutter | **não existe** | `sentry` (Dart) verificado contra a instância; `sentry_flutter` **não verificado**; wrapper a escrever (T16) |
| backend (Spring Boot, se houver) | `buglenz-spring-boot-starter` | pronto, se o VendaX.ai tiver backend a monitorar |

**Consequência para a D3:** incluir o Flutter no piloto é responder "sim" à D3 ("Flutter entra na
Fase 1?") **ao menos para erros Dart sem ofuscação**. Se o app de loja é compilado com `--obfuscate
--split-debug-info`, ou se importam crashes nativos, o pacote 016 (symbolication) deixa de ser opcional
(G17). **Isso é o maior risco do piloto.**

## Projetos na instância (um por aplicação, ADR-0007)

| Projeto (slug proposto) | Plataforma | Ambientes |
|---|---|---|
| `vendax-admin-web` | `javascript-react` | `production`, `homolog` |
| `vendax-mobile` | `flutter` | `production`, `homolog` |

(O ambiente é o campo `environment` do SDK, não projeto separado.)

## Como cada app liga

- **Web:** `initBugLenz({ app: 'vendax-admin-web', version, environment, tenant, cliente })`; source maps
  com `brandSourceMaps` no CI; `identify({ id })` só com id interno. Roteiro: `buglenz-sdk/docs/onboarding.md`.
- **Flutter:** `release = vendax-mobile@<versão do pubspec>` (o `+build` passa); `sendDefaultPii` desligado e
  filtro de dados pessoais antes do envio (T16); sem *screenshots* nem anexos por padrão (ADR-0017).
- **Versão** (a pergunta 6 ficou sem resposta): proposta, web = `version` do `package.json` injetado no build;
  Flutter = `version` do `pubspec.yaml`. A confirmar.

## Alertas: Mattermost, configurável

O painel já tem canal por integração. O **Mattermost** recebe webhook de entrada no formato do Slack: usar
"Webhook personalizado" com o modelo de **Slack, Mattermost e Rocket.Chat**, ou "Slack" quando a URL for
`hooks.slack.com` (não é o caso). Regra `new_issue` por projeto, apontando para o canal.

- Se o Mattermost fica na **rede interna** (host privado, `*.internal`, IP 10.x…), o servidor **bloqueia** o
  destino por padrão (ADR-0018). É preciso listar o host em `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`. Se tem
  endereço público, não precisa.

## Por serem clientes

- A camada 1 do wrapper e o scrub do servidor passam a ser o principal controle de privacidade (dado de
  cliente final, LGPD). A validação dos prazos de retenção pelo responsável por LGPD continua
  recomendada (D6 está provisória em 90/30/90).
- Exclusão por titular existe (`DELETE /api/projects/{id}/privacy/users/{user_id}`); cobre eventos e
  transações, não logs nem spans (limite registrado no roteiro de revisão).
- Login do piloto na instância: senha local até o SSO (pacote 008).

## O que ainda falta de você

1. **Aceitar licenças do Android SDK** (`flutter doctor --android-licenses`) e dizer se posso instalar um app de
   teste no aparelho Android conectado, para verificar o `sentry_flutter` de verdade. Sem isso, o Flutter
   fica "SDK Dart verificado, `sentry_flutter` não".
2. O app Flutter é compilado com **`--obfuscate`/`--split-debug-info`**? Importam **crashes nativos**?
3. **Onde está o Mattermost** (host)? É interno?
4. De onde vem a **versão** de cada app?
5. O admin web é **React**? Há **backend** a monitorar?
6. **Instância de produção:** D4 (onde), host do DSN (`errors.buglenz.dev` proposto), credencial do GHCR nos nós.
