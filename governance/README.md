# BugLenz — corpus de governança

**Versão:** 0.6 · **Data:** 2026-10-07 · **Status:** proposta para revisão

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
| `openspec/007-rebrand-buglenz/` | Rebrand Rustrak → BugLenz por sobreposição no build |
| `openspec/001-bootstrap-do-fork/` | Primeiro pacote, pronto para execução pelo Claude Code |
| `PROMPT-CLAUDE-CODE.md` | Prompt de início para o Claude Code executar o pacote 001 |

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
| D3 | Apps Flutter entram na Fase 1? Se sim, os pacotes 016 (pelo menos etapas 1 e 2) e 015 sobem de prioridade | escopo da Fase 1 | Edson |
| D4 | Localização da instância interna (I12) | pacote 003 | Edson, Neimar |
| D5 | Responsável e substituto pela sincronização quinzenal (ADR-0005) | primeiro ciclo após o pacote 001 | Edson |
| D6 | Prazos padrão de retenção (ADR-0009) | pacote 004 | responsável por LGPD |
| D7 | Avaliar o Jev em sombra na instância interna, ao lado do provedor local (ADR-0015) | pacote 018 | Edson, Neimar |
| D8 | Papel de cada domínio e host do DSN (ADR-0006) | pacote 003 | Edson |
| D9 | Identidade visual do BugLenz: logotipo, ícones, cores | pacote 007 | Edson |

## O que não foi verificado

- Issues e PRs abertos do upstream (leitura automatizada bloqueada).
- Build com Rust 1.98 e execução da suíte de testes; o teste usou Rust 1.97 e SQLite.
- Dashboard em execução, consumo de memória e latência.
- Fluxo OIDC contra o ArchGuard.
- `sentry-spring-boot` e `sentry_flutter` contra a instância.
- Contrato do ArchFlow para modelos de decisão (requisitos listados no ADR-0014).
- Fluxo webhook → agente → API de issue; as rotas foram lidas, não exercitadas.
- Formato exato dos eventos mobile e do upload de símbolos pelo `sentry-cli`: descritos de memória; a tarefa T1 dos pacotes 015 e 016 grava o tráfego real antes de implementar.

## Mudanças

- **0.6:** mobile: ADR-0016 e 0017, pacotes 015 e 016 detalhados, pacote 022 separado para feedback.
- **0.5:** pacote 007 de rebrand detalhado; pt-BR separado no pacote 021; decisão D9.
- **0.4:** nome do produto definido como BugLenz; domínios registrados no ADR-0006; decisão D8.
- **0.3:** prompt de início para o Claude Code (pacote 001).
- **0.2:** triagem assistida por IA: ADR-0013 a 0015, RFC-0001, invariante I13, pacotes 018 a 020, decisão D7.
- **0.1:** análise de gaps, CONSTITUTION, ADR-0001 a 0012, roadmap, pacote 001.
