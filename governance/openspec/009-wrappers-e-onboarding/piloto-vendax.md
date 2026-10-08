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

## Respostas do Edson (2026-10-08) e decisão que elas abrem

| Pergunta | Resposta |
|---|---|
| Licenças do Android e app de teste no aparelho | pode (feito; app desinstalado depois) |
| Ofuscação / crash nativo | **sim** (o app é ofuscado; crash nativo importa) |
| Onde está o Mattermost | **configuração pelo painel de administração**, sem host fixo agora |
| Versão de cada app | `package.json` (web), `pubspec.yaml` (Flutter); a tag do git também |
| Admin web | **React**; não há backend a monitorar |
| Instância de produção | **sem resposta** (D4, host do DSN, credencial do GHCR nos nós) |

### O que o ensaio no aparelho mostrou (`governance/baseline/009-flutter.md`)

- **Release ofuscado chega ilegível**: tipo `nz`, frames só com endereço. O **release sem ofuscação chega
  legível** e agrupa entre versões. Hoje a instância não traduz símbolos.
- **A ofuscação é escolha de quem cria cada app (decisão do Edson em 2026-10-08), não uma regra da instância.**
  O BugLenz precisa suportar os dois caminhos; o guia de onboarding explica a escolha e o que cada uma mostra:

| Caminho | Custo | Efeito |
|---|---|---|
| **Sem `--obfuscate` e sem `--split-debug-info`** | zero no servidor; pacote poucos MB maior e mais fácil de engenharia reversa | stack trace legível desde o primeiro dia; **o que o piloto usa agora** (pelo menos em `homolog`) |
| **Com ofuscação** | exige a symbolication no servidor (pacote 016): receber os `.symbols` por `debug_id`, traduzir endereços e nomes ofuscados e, no Android, o `mapping.txt` do R8 | tudo legível com o app protegido; **capacidade futura**, para os apps que a exigirem |

  Quem ofusca deve **guardar os símbolos de cada build** (CI), porque serão eles a traduzir os erros antigos
  quando a symbolication existir. Por ambiente também vale: `homolog` sem ofuscação, `production` à escolha do app.

## O que ainda falta de você

1. **Instância de produção:** D4 (onde roda), host do DSN (`errors.buglenz.dev` proposto) e a credencial do GHCR
   nos nós. É o que ainda impede o piloto de ir ao ar.
2. Confirmar que **a versão também vem da tag do git** (interpretei assim a resposta "tag").
3. O Mattermost entra pelo painel: quando houver o host, se for **interno** liste-o em `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`.
