# 001 — Design

## Remotes e ponto de partida

```bash
git clone https://github.com/rustrak/rustrak.git <repo>
cd <repo>
git remote rename origin upstream
git remote set-url --push upstream DISABLED
git remote add origin <url-do-repositorio-privado>
git checkout -b main v0.15.2        # ff75852c
git push -u origin main
```

`main` nasce da tag, não de `upstream/main` (ADR-0005). `--push DISABLED` impede push acidental
para o repositório de terceiro.

## Artefatos

| Arquivo | Origem |
|---|---|
| `LICENSE` | preservado, inalterado |
| `NOTICE.md` | novo: origem (`rustrak/rustrak`, tag, commit), copyright do upstream, copyright da IntegrAllTech sobre as modificações |
| `DELTA-MANIFEST.md` | novo: tabela `Arquivo \| Zona \| ADR \| Natureza da divergência`; nasce com as entradas deste pacote |
| `CHANGES-FROM-UPSTREAM.md` | novo: resumo legível do delta, gerado do manifesto |
| `THIRD-PARTY-LICENSES.md` | novo: saída de `cargo deny list` e do inventário de licenças do pnpm |
| `governance/` | este corpus: CONSTITUTION, GAP-ANALYSIS, `adr/`, `openspec/` |
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
Referência desta análise: 1.429 funções de teste no servidor. [confirmado por contagem estática,
não por execução]

## Pendências que este pacote não resolve

- Revisão jurídica da GPL (D2): o repositório pode ser criado antes; nenhuma entrega a cliente
  ocorre antes dela.
- Leitura das issues e PRs abertos do upstream para cruzar com os gaps.
