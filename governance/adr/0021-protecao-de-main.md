# ADR-0021 — Proteção de `main`

**Estado:** aceita (2026-10-08), opção B escolhida pelo Edson; **rulesets aplicados em 2026-10-08** (`.github/rulesets/`: `main` e as tags `v*-itl.*`) ·
**Depende de:** ADR-0020 · **Substitui:** ADR-0019

## Contexto

Com o repositório público (ADR-0020), rulesets existem no plano Free, e os minutos de Actions deixaram de
ser um limite. Até agora a proteção de `main` era por convenção: todo trabalho entra por PR que o Edson
mescla, mas nada impede um `git push` direto ou um force-push.

Havia um obstáculo para exigir checks: os filtros por caminho da ADR-0019 faziam um workflow não rodar, e
**um check obrigatório que não roda fica pendente e bloqueia o PR para sempre**.

## Decisão

1. **Opção B: PR obrigatório, sem force-push, sem exclusão e com checks obrigatórios.** Os checks são os
   nomes dos jobs: `web`, `rust-lint`, `rust-test`, `postgres-e2e`, `brand`, `e2e-react`,
   `network-conformance` e `licenses`. Não se exige que a branch esteja atualizada, nem aprovação.
2. **Os workflows voltam a rodar em todo PR.** Os filtros por caminho da ADR-0019 são **desfeitos**
   (`ci.yml` volta a ser idêntico ao do upstream; `codeql.yml` já voltou na ADR-0020). Não se usa job
   "changes" com `if:`: seria uma mudança maior em arquivo do upstream, e um job pulado porque o job de que
   depende falhou passaria como sucesso. O custo é tempo de espera em PR só de documentação (a CI leva
   cerca de 12 minutos), não dinheiro.
3. **A janela de 15 minutos de rede volta a rodar a cada push em `main`** (o gatilho original), e não só à mão.
4. **As tags `v*-itl.*` não podem ser apagadas nem movidas** (o digest da imagem se amarra a elas).
5. Sem bypass. Um ruleset com erro se desativa pela interface (fica no log de auditoria).
6. Não ficam obrigatórios: `rust-release` (só roda em push), `CodeQL` e `Rust Security` (têm gatilhos próprios).

## Consequências

- Nenhum push direto em `main`, nem do Edson; nenhum merge com a CI vermelha.
- **Renomear um desses jobs quebra todo PR** até o ruleset ser atualizado: a mudança do nome e a do
  `protect-main.json` vão no mesmo PR.
- Uma CI vermelha por teste instável (os e2e dependem de tempo) bloqueia o merge: reexecutar o job.
- Se a CI do GitHub ficar indisponível (cobrança, queda), `main` fica sem merge até o Edson desativar o
  ruleset: é o preço de exigir os checks.
- **C (uma aprovação obrigatória)** só faz sentido com um segundo mantenedor com escrita (o Neimar é o
  substituto no ADR-0005): quem abre o PR não pode aprová-lo.
- A revisão humana linha a linha (CONSTITUTION §5) continua sendo processo: o ruleset garante que existe um
  PR com CI verde, não que alguém o leu.
- O sync por merge de tag do upstream (ADR-0005) passa por PR (`sync/**` → `main`), como já era.
