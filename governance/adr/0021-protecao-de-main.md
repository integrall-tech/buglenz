# ADR-0021 — Proteção de `main`

**Estado:** proposta (2026-10-08), aguarda decisão do Edson (D10) · **Depende de:** ADR-0020, ADR-0019

## Contexto

Com o repositório público (ADR-0020), rulesets existem no plano Free. Até agora a proteção de `main` era
por convenção: todo trabalho entra por PR que o Edson mescla, mas nada impede um `git push` direto ou um
force-push. Os checks da CI têm filtros por caminho (ADR-0019), e **um workflow que não roda por causa de
filtro de caminho deixa o check "pendente", e um check obrigatório pendente bloqueia o PR para sempre**.

## Opções

**A. PR obrigatório, sem force-push, sem exclusão; sem checks obrigatórios** (arquivos em
`.github/rulesets/`). Evita o acidente (push direto, reescrever `main`) e não bloqueia PR de
documentação. Tags de release `v*-itl.*` não podem ser apagadas nem movidas (o digest da imagem se
amarra a elas). **Recomendada agora.**

**B. A + checks obrigatórios** (`CI`, `Brand`, `E2E React`...). Exige antes uma mudança nos workflows: tirar
os filtros de caminho de `on:` e decidir, dentro de cada workflow, por job (`if:` a partir de um job
"changes"), o que roda. Um job pulado conta como sucesso para um check obrigatório. É mudança em
arquivos do upstream (`ci.yml`) e vale um PR próprio.

**C. B + uma aprovação obrigatória.** Só faz sentido quando houver um segundo mantenedor com permissão de
escrita (o Neimar é o substituto no ADR-0005): quem abre o PR não pode aprová-lo, então com uma pessoa só
o merge ficaria impossível.

## Decisão proposta

Aplicar **A** agora. **B** num PR seguinte, se o Edson quiser checks obrigatórios. **C** quando houver
segundo mantenedor.

## Consequências

- Nenhum push direto em `main`, nem do Edson; um erro de ruleset se desfaz com um comando
  (`.github/rulesets/README.md`).
- Sem bypass: se algo travar, o Edson desativa o ruleset pela interface, e isso fica no log de auditoria.
- A revisão humana linha a linha (CONSTITUTION §5) continua sendo processo, não regra do GitHub: o ruleset
  garante que existe um PR, não que alguém o leu.
- O sync por merge de tag do upstream (ADR-0005) passa por PR (`sync/**` → `main`), como já era.
