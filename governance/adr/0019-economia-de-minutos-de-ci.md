# ADR-0019 — Economia de minutos de CI

**Estado:** aceita (2026-10-08); a parte do CodeQL foi revertida pela ADR-0020 (repositório público) · **Depende de:** ADR-0002, ADR-0005

## Contexto

O repositório é privado, a organização está no plano Free e a cota gratuita de minutos do GitHub
Actions acabou em 2026-10-08: todo job passou a ser recusado antes de começar ("recent account
payments have failed or your spending limit needs to be increased"). A CI do fork é pesada (Rust,
PostgreSQL por contêiner, Playwright, uma janela de 15 minutos de bloqueio de rede) e cada PR
disparava de 6 a 7 workflows, inclusive os de documentação. Um workflow, o CodeQL, **falhava em toda
execução** porque o plano não tem GitHub Advanced Security.

## Decisão

1. **Mudanças só de documentação não rodam nada pesado.** `governance/**`, `deploy/**`, `.changeset/**`
   e `**.md` ficam fora de `ci.yml`; `brand.yml` ignora `governance/**`, `deploy/**` e `.changeset/**`.
2. **Cada verificação roda quando algo que ela enxerga muda:**
   - `e2e-react.yml`: servidor, dashboard, cliente, `e2e/`, o script de asserções;
   - `network-conformance.yml`: servidor, dashboard, pacotes e o script e a lista de negação (a janela
     de 15 minutos deixa de rodar a cada push em `main`: roda em branches `sync/**` e **à mão antes de
     cada tag**);
   - `licenses.yml`: manifestos de dependência, `DELTA-MANIFEST.md` (de onde lê a base) e o relatório.
3. **`codeql.yml` só roda à mão** até o plano incluir Advanced Security.
4. Nada de proteção de `main` depende desses checks (D10 está aberta); se um dia houver, os filtros
   por caminho precisam virar checks sempre presentes (um workflow que não roda não reporta).

## Consequências

- Menos minutos por PR, e nenhum gasto com um workflow que nunca poderia passar.
- **A janela de 15 minutos de rede deixa de ser automática em `main`.** O invariante I3 continua
  verificado a cada PR que pode abrir uma conexão (janela de 3 minutos); antes de cada tag, rodar
  `Network conformance` por `workflow_dispatch` com a janela de 15 minutos.
- Os filtros ficam em arquivos do upstream (`ci.yml`, `codeql.yml`); no sync, o conflito vem aqui e se
  resolve mantendo o gatilho do fork.
- Quando a cobrança for resolvida (ou o repositório mudar de plano), os filtros podem ficar: reduzem
  custo sem perder cobertura.
