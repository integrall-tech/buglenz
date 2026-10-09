# RFC-0002 — O evento bruto em disco antes do scrub

**Estado:** proposta, **aguarda decisão do Edson** (2026-10-09) · **Relaciona-se com:** CONSTITUTION I4, ADR-0009 (consequências)

## O problema

A ingestão é em duas fases (herdada do upstream): a rota valida o envelope, **grava o item bruto em `INGEST_DIR`** e responde 200;
um trabalhador assíncrono lê o arquivo, faz o scrub, agrupa, grava no banco e apaga o arquivo. O scrub roda **depois** da gravação em
disco. A I4 diz "dado pessoal é tratado antes de persistir"; o evento bruto, com e-mail, CPF e cabeçalhos, **fica em disco** do
momento da recepção até o fim da digestão, e **indefinidamente se a digestão falhar** (o arquivo espera o trabalhador de recuperação).
A ADR-0009 reconhece isso e exige que `INGEST_DIR` fique em volume da instância, mas não elimina o dado bruto.

O arquivo é um JSON com o payload em base64, um por evento, `project-<id>-<uuid>.pending.json`, apagado após o digest.

## O que se mediu (2026-10-09)

Um evento típico de 20 KB (40 frames, 60 breadcrumbs, contexto e usuário), em modo release, numa máquina de desenvolvimento
(arm64): **parse + scrub + serialização = 0,445 ms; só parse + serialização = 0,121 ms**, ou seja, o scrub custa cerca de **0,32 ms**
por evento. Cresce de forma linear: um evento de 1 MB leva da ordem de 20 ms. O alvo do servidor é P99 de ingestão abaixo de 50 ms.
A baseline do pacote 005 dizia "não medido".

## Opções

| | A. Scrub antes de gravar | B. Proteger o arquivo bruto | C. Manter e documentar |
|---|---|---|---|
| Como | A rota faz parse do item, roda o scrub e grava o JSON **já limpo**; o digest repete o scrub (idempotente) depois da reescrita de source map | Permissão `0600`, volume dedicado e efêmero (tmpfs ou disco criptografado), prazo máximo no arquivo e limpeza ativa dos que falharam | Nada muda; ADR-0009 já registra |
| Dado pessoal em disco | **Nenhum** | Sim, por pouco tempo, protegido | Sim |
| I4 literal | **Cumpre** | Não cumpre; mitiga | Não cumpre |
| Custo na ingestão | +0,3 ms por evento típico (+~20 ms em 1 MB); pode ir para `spawn_blocking` | Nenhum | Nenhum |
| Risco | Altera a rota quente e o formato do arquivo (migração do que já está pendente); payload que não é JSON válido precisa de regra (rejeitar ou gravar descartado) | Não elimina o problema, só reduz a janela | Mantém a violação |
| Diverge do upstream | Sim (rota de ingestão e `storage.rs`) | Pouco (configuração) | Não |
| Contribuível ao upstream | Não sem antes decidir o formato | Em parte | — |

## Recomendação

**A**, em duas etapas pequenas: (1) uma mudança atrás de variável (`RUSTRAK_SCRUB_BEFORE_SPOOL`, desligada por padrão), com testes
(nenhum byte de CPF ou e-mail no arquivo gravado; payload inválido tem regra; digest idempotente) e medição sob carga com
`packages/benchmarks`; (2) ligar por padrão na imagem BugLenz depois de a medição confirmar o orçamento. Enquanto isso, **B como
mitigação imediata** na stack Swarm (volume dedicado, `0600`): é configuração, não código.

Por que não só B: o dado continua em disco e, no pior caso (digestão com falha), por tempo indeterminado. Por que não C: a I4 é
literal e a violação é fácil de evitar por 0,3 ms.

## O que preciso de você

1. Escolher A, B ou C (ou A com B).
2. Se A: ok para a divergência na rota de ingestão (entra no `DELTA-MANIFEST.md` com ADR), e quem faz a revisão §5 (a rota mexe em
   scrubbing).
3. Se a imagem de produção ainda não tem volume dedicado para `INGEST_DIR`: isso entra no pacote 003-prod.
