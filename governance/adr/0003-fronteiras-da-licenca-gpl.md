# ADR-0003 — Fronteiras da licença GPL-3.0

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

> Esta ADR registra uma leitura técnica da licença. **Não é parecer jurídico** e depende de
> revisão por advogado antes da primeira entrega a cliente (decisão D2 do README).

## Contexto

O servidor é `GPL-3.0-only`; `@rustrak/client` e `@rustrak/mcp` declaram GPL-3.0; não há CLA nem
licença comercial alternativa. O dashboard é entregue ao navegador como JavaScript. [confirmado]

## Decisão

1. **O fork permanece GPL-3.0-only.** Todo código que a IntegrAllTech escrever dentro dele também.
2. **Uso interno não é distribuição.** Rodar a instância em infraestrutura da IntegrAllTech, mesmo
   recebendo eventos de aplicações de clientes, não obriga a publicar o código (GPL não é AGPL).
   [inferência]
3. **Instalar na infraestrutura do cliente é distribuição.** A entrega inclui a oferta do
   código-fonte correspondente, e o cliente pode redistribuí-lo. [inferência]
4. **Produtos proprietários não importam código do fork.** VendaX, Alçada, Power View e os demais
   falam com a instância pelos SDKs do Sentry (MIT) ou por REST. `@rustrak/client` e `@rustrak/ui`
   não entram em `package.json` de produto proprietário.
5. **Archbase não entra no dashboard do fork**, a menos que a IntegrAllTech aceite licenciar a
   parte usada sob GPL. É o que sustenta o ADR-0010.
6. `LICENSE` é preservado; `NOTICE.md` registra a origem e o copyright das modificações.

## Pontos para o advogado

- Servir o dashboard a usuários do cliente a partir de servidor da IntegrAllTech conta como
  distribuição do código do dashboard?
- Formato aceitável da oferta de código-fonte na entrega em cliente.
- Compatibilidade de licença das dependências (o upstream já restringe via `deny.toml`).
- Dependências **de desenvolvimento e build** com licença fora de MIT/Apache/BSD, levantadas no
  `THIRD-PARTY-LICENSES.md` do pacote 001 [confirmado]: `@sentry/cli` e `sentry` (npm) sob
  **FSL-1.1** (MIT e Apache-2.0 após dois anos); `@img/sharp-libvips` sob **LGPL-3.0-or-later**;
  `caniuse-lite` sob CC-BY-4.0. Nenhuma entra no binário do servidor nem no bundle do dashboard
  servido ao navegador [inferência: são ferramentas de build]. Confirmar se isso basta.

## Consequências

Um wrapper de SDK para os apps (ADR-0011) é código da IntegrAllTech sobre biblioteca MIT e fica
fora do alcance da GPL. Um cliente de API para os produtos é gerado a partir do `openapi.json`,
não copiado de `@rustrak/client`.
