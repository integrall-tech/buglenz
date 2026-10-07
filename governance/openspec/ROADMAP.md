# Roadmap de pacotes OpenSpec

**Versão:** 0.6 · **Data:** 2026-10-07 · Pacote 001 executado (T3 pendente, D10)

Os pacotes 001, 007, 015 e 016 estão detalhados. Os demais ganham `proposal`, `design`, `tasks` e specs quando o
anterior fechar, incorporando o que a execução revelar. Tamanho: P (dias), M (uma a duas semanas),
G (mais que isso). São ordens de grandeza, não estimativas. [inferência]

## Fase 0 — Fundação

| Pacote | Entrega | Gaps | ADR | Depende de | Tam. |
|---|---|---|---|---|---|
| 001 bootstrap-do-fork | Repositório privado, remotes, artefatos de conformidade, CI herdado verde, baseline — **feito em 2026-10-07**, exceto T3 | — | 0001, 0002, 0003, 0005 | D2 em andamento; D10 para T3 | P |
| 002 remocao-de-egress | Telemetria e checagem de versão fora do código; teste de conformidade de rede. **Detalhado sobre a `v0.16.0`** (sync de 2026-10-07). O upstream segue investindo na telemetria (issue #375) | G4 | 0004 | 001, sync `v0.16.0` | P |
| 003 build-e-implantacao | Imagem PostgreSQL no registry privado; stack Swarm + Traefik; métricas e logs; backup; `INGEST_DIR` sob `/data` (issue #359 do upstream); contorno para `RUSTRAK_BOOTSTRAP_TOKEN` (#356); `THIRD-PARTY-LICENSES.md` gerado na CI | G20 | 0012 | 002, D4, D8 | M |

## Fase 1 — Mínimo para dados de produção

| Pacote | Entrega | Gaps | ADR | Depende de | Tam. |
|---|---|---|---|---|---|
| 004 retencao-automatica | Prazo por projeto e tipo; worker diário; tela de configuração | G1 | 0009 | 003 | M |
| 005 scrubbing-de-dados-pessoais | Etapa no digest; lista de negação; máscaras de CPF/CNPJ/cartão/e-mail; IP não gravado; exclusão por titular | G2 | 0009 | 003 | M |
| 006 compatibilidade-sdk-atual | Sessão `unhandled`; teste de ponta a ponta com `@sentry/react` fixado; PR no upstream | G3 | 0011 | 001 | P |
| 007 rebrand-buglenz | Marca BugLenz nas zonas A e B, aplicada por sobreposição no build; atribuição ao Rustrak preservada | — | 0006 | 003, D8, D9 | M |
| 021 catalogo-pt-br | Catálogo `pt-BR` (1.298 chaves), proposto ao upstream | G5 | 0002, 0010 | 001 | P |
| 008 sso-archguard | Instância interna atrás do ArchGuard; teste do fluxo OIDC | — | 0008 | 003 | P |
| 009 wrappers-e-onboarding | Wrapper React (com Archbase) e Spring Boot; source maps no CI; primeiro produto piloto | — | 0011 | 005, 006 | M |

**Critério de saída da Fase 1:** um produto piloto em produção enviando erros por 30 dias, com
retenção e scrubbing ativos, sem evento contendo dado da lista de negação em amostragem manual.

**Antes do primeiro dado de produção:** verificar o PR #57 do upstream (6 correções de segurança
no servidor: oráculo de tempo no login, DoS por tamanho de senha e outras; aberto desde maio de
2026 e parado). Se as correções não estiverem na base do fork, entram com ADR própria. Ver
`GAP-ANALYSIS.md` §9.

## Fase 2 — Qualidade de triagem

| Pacote | Entrega | Gaps | Tam. |
|---|---|---|---|
| 010 symbolication-js | Nome de função pelo escopo; `in_app` falso para `node_modules`; culprit correto | G7, G8 | M |
| 011 tunnel-e-filtros-de-entrada | Endpoint `tunnel`; origens permitidas por projeto; filtros de extensão, localhost e crawler | G9, G10 | M |
| 012 alertas-por-limiar | Regras por frequência e por queda de crash-free; canal Telegram | G11 | M |
| 013 grupos-oidc-para-papeis | Claim de grupos → papel global e de projeto; responde à issue #355 do upstream | G12 | P |
| 014 tokens-com-escopo-e-auditoria | Escopo por token; trilha de ações administrativas | G6, G13 | M |

## Fase 3 — Cobertura (sujeita à decisão D3)

| Pacote | Entrega | Gaps | Tam. |
|---|---|---|---|
| 015 anexos-e-capturas-de-tela | Captura de tela e hierarquia de views como anexos, opt-in por projeto (ADR-0017) | G16 | M |
| 022 feedback-do-usuario | Item `feedback` do envelope | G16 | P |
| 016 symbolication-mobile | Em três etapas: upload + R8; nativo com `symbolic` (Flutter ofuscado, iOS); reprocessamento (ADR-0016) | G17 | G |
| 017 busca-por-tag | Filtro por tag, release, ambiente e usuário | G14 | M |

## Fase 4 — Triagem assistida por IA (fora do repositório do fork)

Executada no repositório do agente, em ArchFlow (ADR-0014). Depende de dados reais da Fase 1.

| Pacote | Entrega | ADR/RFC | Depende de | Tam. |
|---|---|---|---|---|
| 018 agente-de-triagem-em-sombra | Agente recebe webhook, lê a issue, responde T1, T2 e T4 em N0; registro de decisões; coleta de rótulos; painel de concordância e calibração | 0013, 0014, 0015, RFC-0001 | 005, 009, D7 | M |
| 019 sugestoes-e-roteamento-de-alerta | Promoção a N1 e N2 conforme critério; T5 como roteador de alertas; T3 com embeddings | RFC-0001 §4 e §5 | 018 com critério atingido | M |
| 020 resumo-de-causa | S1 e S2; fluxo S3 com o servidor MCP | RFC-0001 §6 | 018 | P |

## Fora do roadmap

Session replay (G18), multi-organização (G19), alta disponibilidade (G20), integrações com
rastreadores de tarefa (G15), particionamento de eventos (G21). Reavaliar após a Fase 1.
