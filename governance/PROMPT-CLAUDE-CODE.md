# Prompt de início — Claude Code

Como usar: descompacte este corpus em uma pasta (ex.: `~/buglenz-corpus/`), abra o Claude
Code em uma pasta de trabalho vazia, preencha os dois campos entre `<<` `>>` e cole o bloco abaixo.

---

```text
Você vai executar o pacote OpenSpec 001 (bootstrap) do BugLenz, fork governado do Rustrak da
IntegrAllTech. Sou Edson Martins, arquiteto responsável; as decisões que os documentos não
cobrirem são minhas.

## Contexto

O Rustrak (github.com/rustrak/rustrak, GPL-3.0-only) é um servidor de rastreamento de erros
compatível com os SDKs do Sentry: servidor em Rust, dashboard em React 19. Vamos mantê-lo como
fork fino: o menor delta possível em relação ao upstream, sincronizado por merge quinzenal em tag.
Por isso este primeiro pacote não muda comportamento nenhum. Ele só cria o repositório governado,
os artefatos de conformidade, o CI e uma linha de base medida.

- Corpus de governança: <<CAMINHO_DO_CORPUS>>
- Repositório privado de destino (origin): <<URL_DO_REPOSITORIO_PRIVADO>>
- Ponto de partida: tag v0.15.2 do upstream, commit ff75852c

## Antes de qualquer comando, leia nesta ordem

1. README.md do corpus (índice e decisões pendentes D2 a D9)
2. CONSTITUTION.md, seção 3 (invariantes) e seção 5 (governança)
3. adr/0002, adr/0003 e adr/0005
4. openspec/001-bootstrap-do-fork/: proposal.md, design.md, tasks.md, specs/bootstrap.md
5. Depois de clonar: CLAUDE.md da raiz do Rustrak e apps/server/CLAUDE.md

Em seguida me apresente, em poucas linhas, o plano de execução e qualquer ponto em que o design
não bata com o que você encontrou no repositório. Espere meu ok antes de criar o repositório.

## Tarefa

Executar T1 a T13 de openspec/001-bootstrap-do-fork/tasks.md, na ordem, seguindo o design.md.
O pacote termina quando todos os cenários WHEN-THEN de specs/bootstrap.md forem verdadeiros e
você tiver me mostrado a evidência de cada um (comando executado e saída).

## Limites deste pacote

- Não altere nenhum arquivo em apps/*/src, packages/*/src nem apps/server/migrations. Se algo
  ali parecer errado, anote e me avise; a correção pertence a outro pacote.
- Os únicos arquivos do upstream que mudam são: os workflows e o FUNDING.yml listados no
  design.md (removidos) e o CLAUDE.md da raiz (acréscimo de uma seção ao final, sem tocar no
  texto existente).
- Toda alteração ou remoção de arquivo do upstream entra em DELTA-MANIFEST.md no mesmo commit,
  com a ADR que a justifica. Divergência sem ADR não entra.
- Nunca faça push para o remote upstream. Configure `git remote set-url --push upstream DISABLED`
  logo após o clone e confirme antes de seguir.
- Não publique nada fora do repositório privado: nem imagem, nem pacote npm, nem site.
- O LICENSE fica byte a byte igual ao da tag.
- Não comece os pacotes 002 em diante. Os pacotes 018 a 020 (agente de IA) são de outro
  repositório e não fazem parte deste trabalho.

## Convenções

- Commits pequenos, um por tarefa, no padrão do upstream: conventional commits em inglês
  (`chore: ...`, `docs: ...`, `ci: ...`), citando a tarefa (ex.: `ci: remove upstream publish
  workflows (001/T8)`).
- Documentos de governança ficam em governance/ e em português.
- Em qualquer documento que você escrever, marque afirmação técnica como [confirmado] (você leu
  no código ou executou) ou [inferência]. Não registre como confirmado o que não verificou.
- Trabalhe em branch `pkg/001-bootstrap` e abra PR para `main`. A proteção da branch (T3) pode
  exigir que eu a configure; se você não tiver permissão, me diga o que configurar.

## Quando parar e me perguntar

- Qualquer escolha que dependa das decisões pendentes D2 a D9 do README. Não decida por mim.
- O produto se chama BugLenz, mas neste pacote o nome só aparece em NOTICE.md e em governance/.
  A troca de marca na interface e nos nomes de imagem é do pacote 007 (ADR-0006).
- CI falhando por motivo que não seja de ambiente (segredo, runner, cache). Falha de teste
  herdado é achado, não algo a consertar neste pacote.
- Toolchain: o upstream fixa Rust 1.98, Node 22.12+ e pnpm 12. Se o ambiente não tiver essas
  versões, pare e me avise em vez de trocar a versão fixada.
- Qualquer coisa que exija alterar arquivo fora da lista de "Limites".
- T12 (ler issues e PRs abertos do upstream): se você não tiver acesso à API do GitHub para o
  repositório rustrak/rustrak, me avise; não tente contornar.

## Entrega

Ao final, me envie um relatório curto com:

1. Tabela dos cenários de specs/bootstrap.md: cenário, comando de verificação, resultado.
2. Conteúdo final do DELTA-MANIFEST.md.
3. Resumo de governance/baseline/001.md: versões de toolchain, testes por suíte, tempo dos jobs.
4. Achados: tudo que divergiu do design.md ou do GAP-ANALYSIS.md, com a correção sugerida para
   o corpus. Esses achados voltam para os documentos antes do pacote 002.
5. O que ficou pendente e por quê.
```

---

## Depois do pacote 001

Os achados do relatório voltam para o corpus (nova versão) antes de detalhar o pacote 002
(remoção de egress, ADR-0004). O prompt do pacote seguinte reaproveita este, trocando a seção
"Tarefa" e a lista de "Limites" pelo `DELTA` permitido naquele pacote.
