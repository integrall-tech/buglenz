# Auditoria: código contra a especificação (2026-10-09)

**Pergunta:** concluímos tudo o que foi especificado? **Base:** `main` em `2ed1d60a`, imagem `v0.16.0-itl.6`.

**Método:** (1) tarefas de cada `openspec/*/tasks.md`; (2) `DELTA-MANIFEST.md` contra `git diff v0.16.0 HEAD`
(`governance/tools/check-manifest.py`); (3) as 13 invariantes da CONSTITUTION contra o código; (4) as lacunas do GAP
contra o ROADMAP; (5) o CI do último commit de `main`. **Não** houve reexecução cenário por cenário dos 145 `WHEN/THEN`
das especificações: não existe uma matriz de rastreabilidade automática, e a evidência de cada pacote está na sua baseline.

## Resultado

| Frente | Resultado |
|---|---|
| Manifesto | 264 de 264 arquivos divergentes cobertos; nenhuma linha obsoleta |
| CI de `main` | 13 de 13 checks verdes (CI, brand, e2e, licenças, rede, PostgreSQL, os dois builds de release, cargo-deny, CodeQL) |
| Imagem `itl.6` | atual em código; depois dela entraram só um teste e os padrões de retenção da stack |
| Fase 1, critério de saída | **não atendido**: falta a instância de produção e os 30 dias do piloto |

### Pacotes

| Pacote | Estado |
|---|---|
| 001, 002, 003, 005, 006, 007 | feitos e verificados |
| 004 retenção, 023 segurança | feitos; faltam as revisões §5 (004/T7, 023/T8) e a validação dos prazos por LGPD (004/T9) |
| 021 pt | feito e revisado; falta decidir os e-mails de alerta do servidor (T5) |
| 009 wrappers | React, Spring Boot e Flutter feitos; faltam T10 a T14 (instância e 30 dias), T17 (contrato Flutter) e T19 |
| 008, 010 a 014, 017 a 020, 022 | não iniciados (sem especificação detalhada) |
| 015 anexos, 016 symbolication | especificados, 0 de 33 tarefas feitas |

### Invariantes

| | Estado |
|---|---|
| I1, I2, I3, I7, I10, I11 | atendidas |
| I6, I8, I12 | atendidas no desenho; D2 (formato da oferta de código-fonte) e D4 (local da instância) abertas |
| **I4** | **violada nas sessões** (achado 1) |
| **I5** | **parcial** (achado 2) |
| **I9** | não atendida: SSO é o pacote 008 |
| I13 | não aplicável (Fase 3) |

## Achados e o que foi feito

1. **I4 nas sessões.** O SDK monta o `did` de `user.id || user.email || user.username` e o servidor o gravava como veio em
   `session_users.did`. **Corrigido no PR #32:** pseudônimo com chave (HMAC-SHA256 com a `SESSION_SECRET_KEY`, prefixo `p1:`),
   que preserva a contagem de usuários distintos; mascarar como texto os juntaria.
2. **I5 só cobria eventos, transações (com spans) e logs.** `session_counts`, `session_users` e `alert_history` cresciam sem
   prazo. **Corrigido no PR #33:** seguem o prazo de eventos do projeto.
3. **Exclusão por titular** não apagava as linhas de `session_users`. **Corrigido no PR #33.** Os spans das transações do
   titular já eram apagados; logs não têm `user.id` e continuam fora.
4. **Desvios intencionais da especificação** (decididos e registrados): piso de 7 dias de retenção (a spec dizia 1); sem senha
   mínima; idioma `pt` e não `pt-BR`; piloto web e Flutter, não React e Spring Boot; ADR-0019 substituída pelas 0020 e 0021.
5. **Documentação defasada:** o 001/T3 (proteger `main`) estava aberto e os rulesets já estão aplicados. Corrigido aqui.

## Limites que continuam

- O servidor sozinho não impõe prazo de retenção: quem impõe é o padrão 90/30/90 da stack Swarm. Um servidor rodado fora
  da stack lista os projetos como desprotegidos e não apaga.
- Linhas de `session_users` gravadas antes do pseudônimo continuam com o valor original (não há instância de produção).
- Os arquivos brutos do `INGEST_DIR` ficam fora da retenção (o worker de recuperação os trata).
- Os e2e e alguns testes dependem de tempo e falham sob carga; reexecutar o job resolve. O de bootstrap foi corrigido (PR #31).

## Pendências que só o Edson resolve

D2 (jurídico), D4 (onde roda a instância), D13, D7, D9; a instância de produção; as revisões §5 de scrub, 023 e retenção.
