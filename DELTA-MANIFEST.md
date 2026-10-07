# DELTA-MANIFEST

Relação de **toda** divergência deste repositório em relação ao upstream
[rustrak/rustrak](https://github.com/rustrak/rustrak). Base atual: tag `v0.15.2`,
commit `ff75852c1fefda0c1409ec26f14e9e6417810ae4`.

Regras (CONSTITUTION I1, ADR-0002, ADR-0005):

- Todo arquivo do upstream alterado ou removido consta aqui, com a ADR que o justifica.
  Divergência sem ADR não entra no repositório.
- Arquivo novo também consta, para que a sincronização saiba o que é do fork.
- A entrada entra **no mesmo commit** que a divergência.
- Em cada sincronização com o upstream, conflito fora dos arquivos listados aqui é erro do
  manifesto e é corrigido no mesmo PR.

Verificação: `git diff --name-only v0.15.2 main` deve ser um subconjunto da coluna
"Arquivo" (expandindo `governance/**`).

## Zonas

As zonas A, B e C são as do ADR-0006 (marca). A zona G não existe no ADR-0006 e é definida aqui:

| Zona | O que é |
|---|---|
| A | Visível ao usuário: textos, logotipo, e-mails |
| B | Operação: nome de imagem, serviço, rótulos de compose |
| C | Identificadores: crate, escopo npm, variáveis, métricas, tabelas, API. **Não muda** |
| G | Governança e CI: arquivos fora do produto (workflows, documentos, manifestos). Não afetam o binário |

## Entradas

| Arquivo | Zona | ADR | Natureza da divergência |
|---|---|---|---|
| `NOTICE.md` | G | 0003 | Novo. Atribuição ao upstream, copyright das modificações, oferta de código-fonte |
| `DELTA-MANIFEST.md` | G | 0002 | Novo. Este arquivo |
| `CHANGES-FROM-UPSTREAM.md` | G | 0002 | Novo. Resumo legível deste manifesto |

## Histórico de bases

| Data | Base do upstream | Pacote |
|---|---|---|
| 2026-10-07 | `v0.15.2` (`ff75852c`) | 001 bootstrap |
