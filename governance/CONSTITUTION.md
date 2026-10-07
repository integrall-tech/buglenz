# BugLenz — CONSTITUTION

**Versão:** 0.4
**Status:** Proposta — requer assinatura de Edson Martins e Neimar Chagas
**Data:** 2026-10-07
**Origem:** fork de [rustrak/rustrak](https://github.com/rustrak/rustrak) `v0.15.2` (GPL-3.0-only)
**Nome do produto:** BugLenz (definido por Edson Martins em 2026-10-07)

---

## 1. Propósito

BugLenz é a plataforma de rastreamento de erros e saúde de release dos produtos da IntegrAllTech (web em
React, backend em Spring Boot, mobile em Flutter), operada em infraestrutura própria ou do
cliente, recebendo eventos dos SDKs oficiais do Sentry.

Não é uma reimplementação nem um produto novo de observabilidade. É um derivado governado de um
projeto upstream ativo. O valor que a IntegrAllTech agrega está na conformidade (LGPD), na
integração com a própria stack e na operação, não no motor de ingestão e agrupamento.

## 2. Contexto herdado

Medições completas em `GAP-ANALYSIS.md`, seção 2. Em resumo: servidor Rust de 37,7 mil linhas com
41,7 mil de teste; dashboard React 19 de 61 mil linhas; PostgreSQL ou SQLite; upstream com 9 meses
de vida, um release a cada 3 dias e um mantenedor responsável por 64% dos commits recentes.

## 3. Invariantes

Alterar qualquer invariante exige ADR aprovado com assinatura de Edson Martins **e** Neimar Chagas.

### I1 — Delta mínimo, upstream primeiro
Toda divergência do upstream está registrada em `DELTA-MANIFEST.md` com ADR associada. Correção
ou recurso sem especificidade da IntegrAllTech é proposto ao upstream antes de viver só no fork.

### I2 — O protocolo Sentry é o contrato
O fork não quebra compatibilidade com os SDKs oficiais do Sentry nem com `sentry-cli`. A
IntegrAllTech não mantém SDK próprio de captura.

### I3 — Nenhuma saída de rede não autorizada
A instância só inicia conexão com: o banco, o provedor OIDC configurado, o servidor SMTP
configurado e os canais de alerta configurados. Telemetria de uso e checagem de versão herdadas
são **removidas do código**, não desligadas por variável.

### I4 — Dado pessoal é tratado antes de persistir
Scrubbing no servidor é aplicado antes da gravação do evento. Endereço IP de origem não é
armazenado por padrão. A regra vale para eventos, transações, logs e sessões. Exceção única:
captura de tela não é tratável no servidor; só é aceita em projeto que a habilite, com
mascaramento na origem (ADR-0017).

### I5 — Retenção tem prazo e é automática
Todo projeto tem prazo de retenção por tipo de dado, aplicado por rotina do servidor. Não existe
projeto com retenção indefinida.

### I6 — Uma instância por cliente
Isolamento entre clientes é físico. A instância interna da IntegrAllTech é separada das de
clientes. Projetos e ambientes separam aplicações **dentro** de uma instância.

### I7 — PostgreSQL é o único banco suportado
A imagem distribuída é compilada com a feature `postgres`. SQLite permanece no código por
compatibilidade de merge, mas não é implantado.

### I8 — Fronteira da GPL
Código do fork (servidor, dashboard, `@rustrak/client`, `@rustrak/ui`, `@rustrak/mcp`) nunca é
importado, ligado ou copiado para produto de licença proprietária da IntegrAllTech, e o inverso
também vale. Quem recebe o binário recebe a oferta do código-fonte correspondente.

### I9 — Acesso humano por SSO
Login de pessoas passa pelo provedor OIDC da IntegrAllTech (ArchGuard). Senha local existe apenas
para o usuário primário, como acesso de contingência.

### I10 — Sincronização sempre em tag
O fork só incorpora o upstream a partir de tag estável, por merge, com a suíte completa passando.

### I11 — Distribuição apenas por registry privado
Imagens do fork não são publicadas em registry público.

### I12 — Soberania de dados
Eventos residem em infraestrutura sob controle do cliente ou da IntegrAllTech. A localização da
instância interna é decidida em ADR própria (decisão D4 do README) antes do primeiro dado de
produção.

### I13 — Modelo sugere, política decide
Saída de modelo de IA nunca é ação direta: passa por política em código versionado, com limiar e
registro. Nenhum modelo silencia, resolve ou apaga dado. Decisão tomada por pessoa prevalece sobre
a do modelo. Envio de conteúdo de evento a provedor externo de modelo exige ADR própria.

## 4. Fronteiras de escopo

**Em escopo na Fase 1:** erros e release health de aplicações web React e backends Spring Boot;
source maps; alertas; SSO; pt-BR; conformidade (I3, I4, I5).

**Fora de escopo na Fase 1:** session replay, profiling, monitoramento de cron e uptime,
multi-organização, alta disponibilidade, symbolication de mobile nativo (decisão D3 do README),
migração da UI para Mantine/Archbase, oferta SaaS.

**Fase posterior, fora do repositório do fork:** triagem assistida por IA (ADR-0013 a 0015, RFC-0001).

## 5. Governança

- Decisão arquitetural vira ADR numerada em `adr/`.
- Execução vira pacote OpenSpec em `openspec/`, com `proposal.md`, `design.md`, `tasks.md` e
  especificações WHEN-THEN.
- Implementação primária por Claude Code, com revisão humana linha a linha em qualquer arquivo
  que toque autenticação, scrubbing de dados pessoais ou exclusão por retenção.
- Afirmação técnica em documento do corpus leva marcação [confirmado] ou [inferência].

## 6. Assinaturas

| Papel | Nome | Data | Assinatura |
|---|---|---|---|
| Arquiteto de Software e Soluções | Edson Martins | | |
| Sócio-fundador | Neimar Chagas | | |
