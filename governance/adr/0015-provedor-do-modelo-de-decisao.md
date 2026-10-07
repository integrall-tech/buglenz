# ADR-0015 — Provedor do modelo de decisão e saída de dados

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O agente envia ao modelo tipo de exceção, mensagem, frames, breadcrumbs e tags. Mesmo depois do
scrubbing, mensagem de erro e URL podem conter dado pessoal. O Jev é API externa de fornecedor
dos EUA, em acesso antecipado, sem versionamento de longo prazo. Não confirmei se há opção de
execução em infraestrutura própria. [não verificado]

## Decisão

1. **Padrão: provedor local**, na infraestrutura de inferência da IntegrAllTech (vLLM), com saída
   restrita às opções e probabilidade extraída dos logprobs. Embeddings pelo TEI.
2. **Provedor externo (Jev ou outro) só com ADR própria**, assinada por Edson e Neimar, por
   instância, e nunca em instância de cliente sem aceite formal do cliente (I12).
3. Avaliação do Jev, se aprovada: em sombra, na instância interna, em paralelo ao provedor local,
   sobre as mesmas decisões, para comparar acerto e calibração antes de qualquer adoção.
4. A versão do modelo é fixada por decisão. Troca de versão volta a decisão para N0 até repetir o
   critério de promoção.

## Em aberto

Decisão D7 do README: autorizar ou não a avaliação do Jev em sombra na instância interna.

## Consequências

- Logprobs de LLM não vêm calibrados; a calibração é medida e corrigida por decisão (RFC-0001 §5).
- A carga de triagem disputa a GPU com os outros produtos. Volume esperado é por issue nova, não
  por evento, o que mantém a carga baixa. [inferência; não medido]
