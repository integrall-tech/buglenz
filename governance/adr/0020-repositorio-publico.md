# ADR-0020 — O repositório do fork é público

**Estado:** aceita (2026-10-08, decisão do Edson) · **Altera:** ADR-0005 (a origem), ADR-0019 (parte)
**Não altera:** CONSTITUTION I11 (imagens só em registry privado)

## Contexto

O repositório `integrall-tech/buglenz` nasceu privado (pacote 001). Em 2026-10-08 a cota gratuita de
GitHub Actions acabou, a CI deixou de iniciar e três coisas dependiam de o plano ter mais recursos: a
proteção de `main` (D10), o CodeQL e a oferta de código-fonte exigida pela GPL-3.0 (D2). A ideia de
publicar o fork já existia.

## Decisão

1. **O repositório `integrall-tech/buglenz` passa a ser público**, com o corpo, o histórico e a
   `governance/`. Antes de publicar foi feita uma varredura: nenhum segredo nos 1 344 commits (só dados
   de teste do upstream), os workflows só usam `GITHUB_TOKEN` e nenhum usa `pull_request_target`.
2. **A CONSTITUTION I11 não muda.** Ela trata de **imagens**: a imagem `buglenz-server` continua em
   GHCR **privado**. Publicar o código não publica a imagem.
3. **`buglenz-sdk` (MIT) continua privado** até decisão própria.
4. `rustrak/rustrak` (o fork público usado para propostas ao upstream) segue como está.

## Consequências

- **CI:** minutos de Actions em repositório público são gratuitos. Os filtros por caminho da
  ADR-0019 podem ficar (reduzem tempo); o **CodeQL volta** a rodar (o motivo de estar desligado era o
  plano), e a janela de 15 min de rede pode voltar a rodar em `main` se o Edson quiser.
- **Proteção de `main` (D10):** branch protection e rulesets passam a existir sem plano pago. A decisão
  de exigir PR e checks é do Edson; se os checks forem obrigatórios, os filtros por caminho precisam
  virar checks sempre presentes (um workflow que não roda não reporta).
- **Segurança do repositório:** secret scanning e push protection ficam ligados.
- **GPL (D2):** o código-fonte de cada versão distribuída fica disponível nas tags públicas, o que
  simplifica a oferta de código; o **formato** da oferta ao cliente continua para o advogado.
- **O que fica exposto:** a `governance/` (CONSTITUTION, ADRs, lacunas e riscos conhecidos do upstream,
  roteiros de revisão), nomes de pessoas responsáveis e de produtos internos da IntegrAllTech citados
  no corpus, e o endereço de e-mail dos autores nos metadados dos commits. Isso foi aceito na decisão.
- **Dado novo vai para o público:** quem escreve um ADR, uma baseline ou uma mensagem de commit escreve
  para um leitor externo. Nada de segredo, de endereço interno, de nome de cliente ou de dado pessoal
  em corpo de PR, issue, comentário ou arquivo.
- **ADR-0005:** `origin` passa a ser o repositório público; o resto do fluxo de sync não muda.
