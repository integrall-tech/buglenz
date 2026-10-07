# ADR-0002 — Fork fino: delta mínimo e contribuição ao upstream

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O upstream publica um release a cada 3 dias e está em 0.x, com quebra permitida em minor. Cada
arquivo que o fork altera é um conflito de merge em potencial. Como a licença é GPL-3.0, manter
uma correção só no fork não protege nenhum segredo quando o binário é entregue a um cliente.

## Decisão

1. O fork altera o menor número possível de arquivos do upstream. Código novo vai em arquivos
   novos; pontos de ligação em arquivos existentes são mínimos e listados no `DELTA-MANIFEST.md`.
2. Correção ou recurso genérico é aberto como PR no upstream. Se aceito, sai do delta no merge
   seguinte. Se recusado ou parado por mais de 30 dias, fica no fork com ADR.
3. Só fica permanentemente no fork o que é específico: marca, remoção de egress, políticas de
   LGPD que o upstream não queira, integração com ArchGuard.

## Classificação inicial dos gaps

| Gap | Destino |
|---|---|
| G3 sessão `unhandled`, G7 nome de função, G8 `in_app`, G9 tunnel | PR no upstream |
| G5 pt-BR | PR no upstream (já mantém cinco catálogos de idioma) |
| G1 retenção automática, G2 scrubbing, G10 filtros | Propor ao upstream; implementar no fork se não houver interesse |
| G4 remoção de telemetria e checagem de versão | Fork (o upstream quer a telemetria) |
| G12 grupos OIDC → papéis | Propor ao upstream |
| Marca, CI, registry | Fork |

## Consequências

- Parte do trabalho da IntegrAllTech fica pública sob GPL, com o nome dos autores.
- O fork depende da disposição do mantenedor em revisar PRs. [não verificado]
- Contribuições seguem as convenções do upstream: commits em inglês, teste junto com a mudança.
