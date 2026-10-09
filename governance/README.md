# BugLenz — corpus de governança

**Versão:** 0.29 · **Data:** 2026-10-09 · **Status:** proposta para revisão · **Pacote 001 executado** (PR [integrall-tech/buglenz#1](https://github.com/integrall-tech/buglenz/pull/1)) · **Base sincronizada para `v0.16.0`** (PR #3) · **Pacote 002 executado**

Especificação do **BugLenz**: migração do [Rustrak](https://github.com/rustrak/rustrak) `v0.15.2` para um fork
governado da IntegrAllTech, no formato spec-driven (CONSTITUTION, ADRs, pacotes OpenSpec).

## Conteúdo

| Arquivo | O que é |
|---|---|
| `GAP-ANALYSIS.md` | Medição do upstream, teste de ponta a ponta com React 19 e 23 gaps priorizados |
| `CONSTITUTION.md` | Propósito, 13 invariantes, escopo e governança |
| `adr/0001` a `0012` | Decisões: fork, delta mínimo, GPL, egress, sincronização, marca, tenancy, identidade, dados pessoais, UI, SDKs, implantação |
| `adr/0013` a `0015` | IA: sensor e política, agente em ArchFlow fora do fork, provedor do modelo |
| `adr/0016` e `0017` | Mobile: symbolication com as bibliotecas do Sentry; capturas de tela opt-in |
| `rfc/0001` | Catálogo de decisões de triagem: perguntas, política, medição, níveis de autonomia |
| `openspec/ROADMAP.md` | 22 pacotes em cinco fases |
| `openspec/015-anexos-e-capturas-de-tela/` | Capturas de tela e hierarquia de views |
| `openspec/016-symbolication-mobile/` | Stack trace legível para Android, iOS e Flutter ofuscado |
| `openspec/009-wrappers-e-onboarding/` | Wrappers React e Spring Boot, testes de contrato, onboarding e primeiro piloto. Detalhado; bloqueado por decisões do Edson |
| `openspec/007-rebrand-buglenz/` | Rebrand Rustrak → BugLenz por sobreposição no build |
| `openspec/001-bootstrap-do-fork/` | Primeiro pacote, **executado em 2026-10-07**; T3 pendente (D10) |
| `openspec/007-rebrand-buglenz/` | Marca BugLenz por sobreposição no build; atribuição ao Rustrak preservada. **Executado em 2026-10-08**; logotipo provisório (D9); proposta ao upstream: rustrak/rustrak#387 |
| `openspec/005-scrubbing-de-dados-pessoais/` | Scrubbing no servidor (chaves, máscaras, IP), exclusão por titular. **Executado em 2026-10-08** (PR #7); issue no upstream: rustrak/rustrak#384 |
| `openspec/006-compatibilidade-sdk-atual/` | Status de sessão `unhandled`, e2e React na CI, primeiro PR ao upstream (rustrak/rustrak#383). **Executado em 2026-10-08** (PR #6) |
| `openspec/003-build-e-implantacao/` | Imagem em GHCR privado, workflow por tag, stack Swarm parametrizada, licenças geradas na CI. **Executado em 2026-10-08** (PR #5); primeira publicação na tag `v0.16.0-itl.3` |
| `openspec/002-remocao-de-egress/` | Remoção da telemetria e da checagem de versão; teste de conformidade de rede. **Executado em 2026-10-08** sobre a `v0.16.0` (PR #4) |
| `PROMPT-CLAUDE-CODE.md` | Prompt de início para o Claude Code executar o pacote 001 |

O repositório do fork é `integrall-tech/buglenz` (**público** desde 2026-10-08, ADR-0020). Este corpus vive nele em `governance/`,
junto com `governance/baseline/001.md` (baseline medida) e `governance/tools/` (geradores).

## Ordem de leitura

1. `GAP-ANALYSIS.md` §1, §3 e §5
2. `CONSTITUTION.md` §3
3. ADR-0002 e ADR-0003, que condicionam as demais
4. `openspec/ROADMAP.md`

## Decisões pendentes

| # | Decisão | Bloqueia | Quem |
|---|---|---|---|
| D1 | ~~Nome do produto~~ **Fechada: BugLenz** (2026-10-07) | — | Edson |
| D2 | Revisão jurídica da leitura da GPL-3.0 (ADR-0003) | qualquer entrega em infraestrutura de cliente | advogado |
| D3 | Apps Flutter entram na Fase 1? Se sim, os pacotes 016 (pelo menos etapas 1 e 2) e 015 sobem de prioridade | escopo da Fase 1 | Edson | **Em prática, sim para erros Dart sem ofuscação** (o piloto inclui um app Flutter); ofuscação e crash nativo, abertos (T18 do 009)
| D4 | Localização da instância interna (I12) | pacote 003 | Edson, Neimar |
| D5 | ~~Responsável e substituto pela sincronização quinzenal (ADR-0005)~~ **Fechada (2026-10-07): responsável Edson Martins, substituto Neimar Chagas.** Primeiro ciclo: `sync/2026-10-07` → `v0.16.0` | — | Edson |
| D6 | ~~Prazos padrão de retenção (ADR-0009)~~ **Definida provisoriamente em 2026-10-08, por delegação do Edson: 90 dias para erros e logs, 30 para transações e spans** (o que o ADR-0009 propunha), como padrão da stack Swarm; o servidor não embute valor. O responsável por LGPD pode ajustar por instância ou por projeto | pacote 004 | Edson (provisório); responsável por LGPD (validação) |
| D7 | Avaliar o Jev em sombra na instância interna, ao lado do provedor local (ADR-0015) | pacote 018 | Edson, Neimar |
| D8 | ~~Papel de cada domínio e host do DSN (ADR-0006)~~ **Parcialmente fechada (2026-10-08): `buglenz.dev` é o domínio de tudo.** Proposta `errors.buglenz.dev` (instância e host do DSN), `docs.buglenz.dev`, `alerts@buglenz.dev`; falta confirmar o host do DSN antes do primeiro app piloto | primeiro app piloto | Edson |
| D9 | Identidade visual do BugLenz: logotipo, ícones, cores. **Provisórios no pacote 007** (logotipo tipográfico e ícones gerados) | versão final do 007 | Edson |
| D10 | ~~Plano GitHub Team/Pro para proteger `main`~~ **Destravada (ADR-0020) e decidida: opção B da ADR-0021** (PR obrigatório, sem force-push, oito checks obrigatórios, tags de release protegidas). Os rulesets estão em `.github/rulesets/` e **aguardam o Edson aplicar** | T3 do pacote 001 | Edson |
| D11 | ~~Repositório, licença e registry dos wrappers~~ **Parcialmente fechada (2026-10-08):** wrapper React é pacote irmão do Archbase; um repositório `integrall-tech/buglenz-sdk` sob MIT, sem referência ao fork. Falta: registry npm/Maven e visibilidade do repositório | pacote 009, bloco B | Edson |
| D12 | ~~Produto piloto e seus projetos~~ **Decidida em 2026-10-08: VendaX.ai**, com `vendax-admin-web` e `vendax-mobile` (Flutter), `production` e `homolog`, alertas no Mattermost (`openspec/009-wrappers-e-onboarding/piloto-vendax.md`). Faltam respostas do Edson sobre o Flutter (ofuscação, crash nativo), o Mattermost, as versões e a instância | pacote 009, bloco B | Edson |
| D13 | Posição sobre retenção na entrada em produção do piloto: pacote 004 antes, ou limpeza manual agendada como exceção aprovada | início do relógio de 30 dias | Edson, Neimar |

## O que não foi verificado

- ~~Issues e PRs abertos do upstream~~ Lidos em 2026-10-07 (pacote 001, T12): `GAP-ANALYSIS.md` §9.
- ~~Build com Rust 1.98 e execução da suíte de testes~~ Executados na CI do fork com Rust 1.98.1,
  SQLite e PostgreSQL 16: `governance/baseline/001.md`. 1.351 testes Rust e 1.127 JavaScript, 0 falhas.
- Dashboard em execução, consumo de memória e latência.
- PR #57 do upstream (6 correções de segurança no servidor, aberto desde maio de 2026): se as
  correções entraram por outro commit ou seguem pendentes na `v0.15.2`.
- Fluxo OIDC contra o ArchGuard.
- ~~`sentry-spring-boot` contra a instância~~ Verificado em 2026-10-08 (`governance/baseline/009-t1.md`). `sentry_flutter` segue não verificado.
- Contrato do ArchFlow para modelos de decisão (requisitos listados no ADR-0014).
- Fluxo webhook → agente → API de issue; as rotas foram lidas, não exercitadas.
- Formato exato dos eventos mobile e do upload de símbolos pelo `sentry-cli`: descritos de memória; a tarefa T1 dos pacotes 015 e 016 grava o tráfego real antes de implementar.

## Mudanças

- **0.29:** auditoria cenário a cenário (`baseline/auditoria-2026-10-09-cenarios.md`): 119 cenários, 54 verificados, 27 parciais, 1 não implementado. Achados de privacidade corrigidos (`did` das sessões em pseudônimo, e-mail em campo de id, relatos de usuário, CPF como número, spans avulsos na retenção, retenção de sessões e de histórico de alertas), paridade da camada 1 nos wrappers, suíte de integração em PostgreSQL e varredura do binário publicado na CI. ADR-0022 (paleta quente, tema claro padrão) e ADR-0023 (títulos serifados e cartões). Specs e tarefas ganham emendas e ressalvas datadas; ADR-0021 corrigida. **Pendentes:** assinaturas da CONSTITUTION, estado das ADRs 0001–0017, evento bruto em disco, D2, D4, D7, D9, D13.
- **0.28:** D12 decidida (piloto VendaX.ai: admin web e app Flutter). A D3 passa a valer na prática para erros Dart sem ofuscação; ver `piloto-vendax.md` e `baseline/009-dart.md`.
- **0.27:** D6 definida provisoriamente (90/30/90 dias) como padrão da stack Swarm, por delegação do Edson; o servidor continua sem valor embutido. Dois rulesets aplicados em `main` e nas tags de release (D10); catálogo `pt` revisado e proposto ao upstream (rustrak/rustrak#389).
- **0.26:** ADR-0021 (opção B) e rulesets para `main` (com checks obrigatórios) e para as tags de release, prontos para o Edson aplicar (D10). A ADR-0019 fica substituída: os workflows voltam a rodar em todo PR.
- **0.25:** ADR-0020: o repositório do fork é público. I11 (imagens em registry privado) não muda. Destrava a D10 e simplifica a D2; o CodeQL volta.
- **0.24:** ADR-0019 (economia de minutos de CI): filtros por caminho nos workflows, CodeQL só à mão, janela de 15 min de rede à mão antes de cada tag. Motivo: a cota gratuita do GitHub Actions acabou.
- **0.23:** pacote 021 feito: dashboard em português do Brasil (idioma `pt`, 1 328 chaves), sem revisão por falante nativo ainda. Glossário em `governance/baseline/021.md`.
- **0.22:** G24 corrigido: sessões reportadas mais de uma vez (SDK Java) contadas uma vez; verificado com o `sentry-spring-boot` 8.60.0 real (`total 1, crashed 1, healthy 0`, antes `crashed 2, healthy -1`). O wrapper Spring Boot continua sem ligar sessões por escolha, mas deixa de haver risco de release health corrompido se um app ligar.
- **0.21:** tela de retenção no dashboard (`/settings/retention`, só administradores), cliente `@rustrak/client` com `retention.get()` e `retention.updateProject()`, textos nos cinco catálogos.
- **0.20:** pacote 004 (retenção) implementado no servidor: worker, prazos por projeto e tipo, `GET /api/retention` e `PUT /api/projects/{id}/retention`. **Defeito do pacote 005 encontrado e corrigido:** a rota de exclusão por titular respondia 404 na imagem publicada, escondida pelo escopo genérico de projetos; os testes de integração não viam porque montam o módulo sozinho. Passou a haver uma guarda no `e2e-react`.
- **0.19:** PR #57 do upstream verificado contra a base `v0.16.0`: H-1, H-2, H-4 e M-2 **presentes** no fork; H-3 já corrigido; M-1 a conferir. ADR-0018 (proposta) e pacote 023. Decisão do Edson: aprovar a abordagem (trazer ao fork e oferecer ao upstream).
- **0.18:** T1 do 009 executado: `sentry-spring-boot` 8.60.0 verificado contra a instância (item sai de "não
  verificado"); novo gap **G24** (terminais repetidos da mesma sessão são contados em duplicidade).
- **0.17:** pacote 009 detalhado em três blocos (A: já; B: decisão; C: com a instância). Registrado que o
  critério de saída da Fase 1 não fecha sem o 004 (retenção), bloqueado por D6, e que o 009 não é
  independente como se afirmou antes.
- **0.16:** pacote 007 executado (sobreposição de marca em `brand/`, 26 regras, `brand.yml`); D8
  fechada para `buglenz.dev`; D9 segue aberta (logotipo e ícones provisórios).
- **0.15:** pacote 005 executado; G2 corrigido no fork; achado: o scrub precisa rodar também depois da
  reescrita por source map (o código-fonte reinserido nos frames é payload).
- **0.14:** pacote 005 detalhado com as decisões de 2026-10-08 (configuração por instância, IP nunca
  gravado, máscaras por marcador, issue no upstream em paralelo).
- **0.13:** pacote 006 executado; G3 corrigido no fork e proposto ao upstream (#383); ADR-0005 passo 6
  nomeia os jobs `network-conformance`, `e2e-react` e `licenses`; baseline em `governance/baseline/006.md`.
- **0.12:** pacote 006 detalhado; G3 esclarecido: `unhandled` é status do protocolo (1.6.0) enviado
  pelo SDK JS 11.x em vez de `crashed`; classificado como errored no fork [inferência a validar no PR do upstream].
- **0.11:** pacote 003 executado; `design.md` do 003 com a seção 8 (o que a execução revelou);
  baseline em `governance/baseline/003.md`.
- **0.10:** pacote 003 detalhado: GHCR privado (I11 mantido), só `amd64`, dividido — valores de D4/D8
  ficam para a implantação; backup (BackupLenz) e descoberta de métricas/logs marcados "a confirmar".
- **0.9:** pacote 002 executado; `design.md` do 002 ganha a seção 9 com o que a execução revelou;
  spec do 002 ajustada (corpo do 404). Baseline em `governance/baseline/002.md`.
- **0.8:** D5 fechada; primeiro ciclo do ADR-0005 (`v0.15.2` → `v0.16.0`, sem conflito); pacote
  002 detalhado sobre a `v0.16.0`; ADR-0004 e ADR-0012 corrigidos quanto à procedência do
  `/metrics` (existe a partir da `v0.16.0`, não na `v0.15.2`); **correção do achado A5 da 0.7**:
  a `v0.15.2` fixava `pnpm@12.6.0` e a CI instalou exatamente isso; o 12.10.1 é da `v0.16.0`.
- **0.7:** achados da execução do pacote 001 (A1 a A11 do relatório): zona G no ADR-0006;
  contagem de testes corrigida para a executada; `design.md` do 001 com `checkout -B`, lista
  completa de `governance/`, dependência de plano do GitHub (D10) e aviso sobre workflows no
  primeiro push; ADR-0003 com licenças FSL/LGPL para o advogado; ADR-0005 com nota sobre pnpm na
  CI; `GAP-ANALYSIS.md` §9 com issues e PRs do upstream; ROADMAP com #359 e #356 no pacote 003 e
  #355 no 013; ADR-0008 e ADR-0012 referenciando essas issues.
- **0.6:** mobile: ADR-0016 e 0017, pacotes 015 e 016 detalhados, pacote 022 separado para feedback.
- **0.5:** pacote 007 de rebrand detalhado; pt-BR separado no pacote 021; decisão D9.
- **0.4:** nome do produto definido como BugLenz; domínios registrados no ADR-0006; decisão D8.
- **0.3:** prompt de início para o Claude Code (pacote 001).
- **0.2:** triagem assistida por IA: ADR-0013 a 0015, RFC-0001, invariante I13, pacotes 018 a 020, decisão D7.
- **0.1:** análise de gaps, CONSTITUTION, ADR-0001 a 0012, roadmap, pacote 001.
