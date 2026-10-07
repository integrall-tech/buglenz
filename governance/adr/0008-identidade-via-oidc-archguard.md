# ADR-0008 — Identidade via OIDC do ArchGuard

**Status:** Proposto
**Data:** 2026-10-07
**Decisores:** Edson Martins, Neimar Chagas

## Contexto

O Rustrak tem SSO OIDC genérico (authorization code com PKCE, descoberta, validação de ID token,
domínios permitidos, auto-provisionamento opcional), configurado por variáveis `OIDC_*`. A conta é
ligada por issuer e subject. Papéis não vêm do provedor: são atribuídos na instância (G12).
[confirmado]

## Decisão

1. A instância interna usa o ArchGuard como provedor OIDC, com `OIDC_AUTO_PROVISION=true` e
   `OIDC_ALLOWED_DOMAINS` restrito.
2. Senha local fica só para o usuário primário (contingência, invariante I9).
3. Mapeamento de grupos do provedor para papel global e papéis de projeto é implementado e
   proposto ao upstream (ADR-0002). Até lá, papéis são atribuídos manualmente por um Admin.

## Em aberto

Compatibilidade do ArchGuard com o fluxo que o Rustrak usa (descoberta, PKCE, claim
`email_verified`) precisa de teste. [não verificado]

## Consequências

Desligar alguém no ArchGuard impede novo login, mas não revoga tokens de API já emitidos por essa
pessoa; a revogação é manual até existir sincronização. [inferência]
