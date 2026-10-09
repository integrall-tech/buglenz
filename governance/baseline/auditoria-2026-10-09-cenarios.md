# Segunda auditoria: cenário a cenário (2026-10-09)

Complementa `auditoria-2026-10-09.md`. Aquela olhou pacotes, manifesto e invariantes; esta confere **cada cenário
WHEN/THEN** dos dez pacotes implementados (119) contra o código e os testes. Quatro leitores independentes, só leitura,
sem rodar teste; os achados mais graves foram conferidos à mão depois. "Verificado" quer dizer que existe código e um
teste que falharia se quebrasse, não que o teste passa hoje (isso é da CI).

## Placar

| Veredito | Qtd |
|---|---|
| Verificado | 54 |
| Implementado sem teste | 18 |
| Parcial | 27 |
| Diverge da spec | 10 |
| Depende de verificação externa | 9 |
| Não implementado | 1 (009: homologar versão nova de SDK pela CI) |
| **Total** | **119** (mais 26 em 015 e 016, não iniciados) |

## Achados e o que foi feito

| # | Achado | Gravidade | Situação |
|---|---|---|---|
| 1 | `user.id`/`*_id` com e-mail gravado em claro (servidor e wrappers) | I4 | PRs `buglenz` #44 e `buglenz-sdk` #2 |
| 2 | `user_reports` sem scrub e sem prazo | I4, I5 | PR `buglenz` #45 |
| 3 | Spans avulsos nunca expiravam | I5 | PR `buglenz` #43 (mesclado) |
| 4 | CPF/CNPJ como número JSON e e-mail como chave de objeto passavam | I4 | PR `buglenz` #47 |
| 5 | Evento bruto gravado em disco antes do scrub | I4 (ADR-0009 reconhece) | **Em aberto**: decisão de arquitetura |
| 6 | Suíte de integração nunca rodava em PostgreSQL na CI | I7 | PR `buglenz` #46 (516 de 517 à mão; 1 teste portado) |
| 7 | Contrato dos wrappers só por disparo manual | — | **Em aberto**; PR `buglenz-sdk` #1 (Flutter) ainda aberto |
| 8 | Camada 1 divergente entre wrappers (contextos, transações, fragmento de URL) | I4 | `buglenz-sdk` #2 (React, Spring); Flutter após o #1 |
| 9 | Binário publicado nunca escaneado na CI; OpenAPI dizia "Rustrak Team" | I3, 007 | PR `buglenz` #48; binário da `itl.6` conferido limpo à mão |
| 10 | Limpeza do GHCR mantinha 3 releases; rótulo de revisão errado em disparo manual | 003 | PR `buglenz` #48 |
| 11 | Rede em tempo de execução mais fraca que a spec ("exatamente 1", sondas 2xx) | I3 | **Em aberto**: precisa de máquina com iptables |
| 12 | Specs desatualizadas (10 cenários falsos como escritos) | — | Emendas datadas nas specs (este PR) |
| 13 | Tarefas marcadas sem serem literalmente verdade (6) | — | Ressalvas nas tarefas (este PR) |
| 14 | Assinaturas da CONSTITUTION em branco; ADRs 0001–0017 "Proposto" | governança | **Só o Edson**; ADR-0021 corrigida |

## Limites que continuam

- Os testes do Spring e do Flutter não verificam o nome da transação (o SDK Java não oferece setter).
- O OpenAPI de marca é guardado por regras com contagem; um texto novo do upstream citando o nome numa doc de rota só seria
  pego por um teste do documento gerado, que exige compilar o servidor com marca.
- Um inteiro qualquer de 11 dígitos tem cerca de 1% de chance de parecer CPF (ADR-0009).
