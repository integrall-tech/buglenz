# ADR-0005 — Sincronização com o upstream

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

23 releases estáveis em 72 dias. Versão 0.x: o `CLAUDE.md` do upstream manda usar `minor` para
mudança incompatível. Há guia de migração por versão em `apps/docs/content/upgrading/`. [confirmado]

## Decisão

Merge periódico em tag, nunca rebase. Cadência quinzenal.

```
origin     → repositório privado da IntegrAllTech
upstream   → github.com/rustrak/rustrak (push desabilitado)

main              linha estável do fork
sync/AAAA-MM-DD   branch de merge do ciclo
```

1. `git fetch upstream --tags`
2. `git checkout -b sync/AAAA-MM-DD main`
3. `git merge --no-ff vX.Y.Z` — sempre tag estável, nunca `-rc`, nunca `upstream/main`
4. Ler `apps/docs/content/upgrading/` e o changelog entre as duas tags antes de resolver conflito
5. Conflito só é esperado em arquivo do `DELTA-MANIFEST.md`; conflito fora dele corrige o manifesto
   no mesmo PR
6. CI completa: suítes do upstream (SQLite e PostgreSQL), teste de conformidade de rede (ADR-0004),
   teste de ponta a ponta com app React minificado e source map (roteiro do `GAP-ANALYSIS.md` §3)
7. Revisão humana em arquivo que toque autenticação, scrubbing ou retenção
8. Merge em `main`, tag `vX.Y.Z-itl.N`, imagem com a mesma tag no registry privado

**Exceção de segurança:** release do upstream que corrige vulnerabilidade entra em até 72 h.

**A cada ciclo, também:** regenerar `THIRD-PARTY-LICENSES.md` com
`governance/tools/third-party-licenses.py`; conferir a versão de pnpm que a CI do upstream
instala (em 2026-10-07 o `pnpm/action-setup` instalou 12.6.0 com `packageManager` em 12.10.1
[confirmado]); registrar a nova baseline em `governance/baseline/`.

**Migrations:** o fork não edita migration do upstream. Migration própria usa timestamp e sufixo
`_itl`, nos dois diretórios (`postgres/` e `sqlite/`), para não quebrar a suíte herdada.

## Orçamento

Um responsável nomeado e um substituto. Se três ciclos seguidos passarem de 12 h de trabalho, o
ADR-0001 é reaberto.

## Consequências

O fork fica até duas semanas atrás do upstream, cerca de 4 a 5 releases. Aceitável: o que chega
de urgente entra pela exceção de segurança.
