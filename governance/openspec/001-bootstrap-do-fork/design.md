# 001 — Design

## Remotes e ponto de partida

```bash
git clone https://github.com/rustrak/rustrak.git <repo>
cd <repo>
git remote rename origin upstream
git remote set-url --push upstream DISABLED
git remote add origin <url-do-repositorio-privado>
git checkout -B main v0.15.2        # ff75852c; -B porque o clone já traz a main do upstream
git push -u origin main
```

`main` nasce da tag, não de `upstream/main` (ADR-0005). `--push DISABLED` impede push acidental
para o repositório de terceiro.

Executado em 2026-10-07: origin `git@github.com:integrall-tech/buglenz.git`, criado com
`gh repo create --private`. [confirmado]

**Aviso [confirmado]:** o primeiro push de `main` ainda contém os workflows do upstream, e o
GitHub os executa: `ci.yml` e `release.yml` rodaram. O `release.yml` falhou no checkout por falta
do segredo `token`, antes de qualquer passo de publicação. Sem segredos configurados no
repositório novo o risco é nulo, mas vale conferir o run. Alternativa: criar o repositório com
Actions desabilitadas e reabilitar no PR do pacote.

## Artefatos

| Arquivo | Origem |
|---|---|
| `LICENSE` | preservado, inalterado |
| `NOTICE.md` | novo: origem (`rustrak/rustrak`, tag, commit), copyright do upstream, copyright da IntegrAllTech sobre as modificações |
| `DELTA-MANIFEST.md` | novo: tabela `Arquivo \| Zona \| ADR \| Natureza da divergência`; nasce com as entradas deste pacote. Zonas A, B, C do ADR-0006 mais a zona **G** (governança e CI) |
| `CHANGES-FROM-UPSTREAM.md` | novo: resumo legível do delta, gerado do manifesto |
| `THIRD-PARTY-LICENSES.md` | novo: saída de `cargo deny list` (`apps/server` e `packages/benchmarks`) e de `pnpm licenses list`, agrupada por licença pelo script `governance/tools/third-party-licenses.py`. O inventário npm inclui pacotes opcionais da plataforma em que foi gerado; a partir do pacote 003, gerar na CI (Linux) |
| `governance/` | este corpus inteiro: `README.md`, CONSTITUTION, GAP-ANALYSIS, `adr/`, `rfc/`, `openspec/`, `PROMPT-CLAUDE-CODE.md`; mais `baseline/` e `tools/`, que nascem no fork |
| `CLAUDE.md` (raiz) | acrescido de uma seção final apontando para `governance/` e para o manifesto; o texto do upstream fica intacto acima |

O corpus vai em `governance/` e não na raiz para não colidir com arquivos que o upstream venha a
criar. A edição do `CLAUDE.md` é a única alteração em arquivo existente do upstream neste pacote,
além dos workflows.

## CI

Estado no upstream [confirmado]: `ci.yml`, `codeql.yml`, `rust-security.yml`, `release.yml`,
`docker-publish.yml`, `deploy-docs.yml`; `.github/FUNDING.yml`.

| Workflow | Ação | Motivo |
|---|---|---|
| `ci.yml` | mantido | lint e testes, inclusive `postgres-e2e` |
| `codeql.yml`, `rust-security.yml` | mantidos | análise estática e auditoria de dependências |
| `release.yml` | removido | versiona e publica pacotes `@rustrak/*` |
| `docker-publish.yml` | removido | publica `rustrak/rustrak-server` e `rustrak/rustrak-ui` no Docker Hub |
| `deploy-docs.yml` | removido | publica o site de documentação do upstream |
| `.github/FUNDING.yml` | removido | patrocínio do autor original não se aplica ao repositório privado |

O build de imagem para o registry privado é do pacote 003. Cada remoção é uma linha do manifesto.

## Baseline

Registrar em `governance/baseline/001.md`: versão do Rust e do Node usadas, resultado de
`pnpm run ci` e do job `postgres-e2e`, contagem de testes por suíte e tempo de cada job.

Referência medida em 2026-10-07 [confirmado por execução na CI]: servidor com SQLite **1.351
passaram, 59 ignorados, 0 falharam** (unittests 332, `unit_tests` 477, `integration_tests` 514,
`e2e_tests` 19, `alert_cooldown_test` 9); `e2e_tests` com PostgreSQL 16: 19; JavaScript 1.127
em 111 arquivos (client 504, dashboard 308, ui 185, mcp 130). A contagem estática anterior
(1.429) foi substituída: `grep` de atributos de teste na tag dá 1.412.

## Proteção da branch (T3)

Rulesets e branch protection em repositório privado exigem GitHub Team ou Pro; no plano Free a
API responde 403 "Upgrade to GitHub Pro or make this repository public" [confirmado em
2026-10-07]. Decisão D10 do README. Configuração pretendida, quando possível: ruleset em
`refs/heads/main` com `pull_request`, `required_status_checks` (`web`, `rust-lint`,
`rust-test`, `postgres-e2e`), `non_fast_forward` e `deletion`.

## Pendências que este pacote não resolve

- Revisão jurídica da GPL (D2): o repositório pode ser criado antes; nenhuma entrega a cliente
  ocorre antes dela.
- Proteção de `main` (T3), até a decisão D10.
