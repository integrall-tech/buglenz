# ADR-0018 — Correções de segurança do servidor que o upstream ainda não aceitou

**Estado:** aceita (2026-10-08; o Edson pediu a implementação). Revisão humana do código ainda pendente (§5) · **Depende de:** ADR-0002, ADR-0005

## Contexto

O upstream tem aberto desde 2026-05-11 o PR #57, "fix(security): address 6 server vulnerabilities",
do próprio mantenedor, parado desde 2026-05-21 e hoje em conflito com `main`. O ROADMAP exige
verificar o PR antes do primeiro dado de produção e, se as correções faltarem na base do fork,
trazê-las com ADR própria.

Verificação na base `v0.16.0` (leitura do código, 2026-10-08) [confirmado por leitura; não por
exploração]:

| Item | Falha | Estado no fork |
|---|---|---|
| H-1 | Oráculo de tempo no login: e-mail inexistente responde sem Argon2 | **presente** (`routes/auth.rs`, `login` retorna antes de verificar senha) |
| H-2 / M-3 | DoS por tamanho de senha | **presente** (nenhum limite no login nem no aceite de convite) |
| H-3 | Vazamento de detalhe interno nos erros 5xx | **já corrigido no upstream** (`INTERNAL_ERROR_MESSAGE` fixo em `error.rs`) |
| H-4 | SSRF por URL de webhook | **presente** (`validate_config` só confere o esquema `http`/`https`) |
| M-1 | Corpo grande no ingest sem 413 em JSON | **presente**: 413, mas em texto puro (confirmado por execução com 101 MB) |
| M-2 | Fixação de sessão | **presente no login**; o fluxo SSO já renova a sessão |

## Decisão

1. Trazer para o fork as correções **H-1, H-2/M-3, H-4 e M-2** (e M-1 se um teste provar a falha),
   uma por commit, cada uma precedida de teste que falha.
2. Partir do conteúdo do PR #57 quando for aplicável e reescrever o que conflita com `main`. O
   trabalho entra no `DELTA-MANIFEST.md` com este ADR.
3. **Oferecer ao upstream** o mesmo conteúdo rebaseado (ADR-0002): comentar no PR #57 e abrir PR
   novo se o mantenedor aceitar. Se o upstream mesclar, a divergência some no próximo sync.
4. As mudanças tocam autenticação e validação de entrada: **revisão humana linha a linha**
   (CONSTITUTION §5) antes de ir para produção.

## Consequências

- O fork diverge do upstream em `routes/auth.rs`, `models/user.rs`, `services/notification/webhook.rs`
  e nos testes, até o upstream aceitar.
- Conflito previsível no sync se o upstream corrigir de outro jeito: resolver a favor do upstream e
  manter os testes.

## Decisões tomadas na implementação (2026-10-08)

- **Sem tamanho mínimo de senha.** O PR #57 impõe 8 a 1024; a base tem teste que fixa a decisão do
  upstream de não ter mínimo (`accept_invitation_allows_short_password_but_requires_nonempty`). Só o
  limite superior, que é o que protege contra abuso, foi trazido. Mínimo é política: decisão do Edson.
- **M-2 em armazenamento por cookie.** A sessão é um `CookieSessionStore` (estado assinado no
  cookie): não existe identificador de sessão no servidor para fixar. O que o login passa a fazer é
  descartar o que a sessão guardava antes (estado de SSO ou algo plantado) e renová-la.
  Consequência registrada [inferência]: no logout o cookie não é revogado no servidor; um cookie
  roubado vale até expirar. Fora do escopo deste pacote.
- **H-4 vale também no envio.** A URL do roteamento da regra não passa por `validate_config`; a
  checagem roda no `send` dos dois notificadores. O cliente HTTP dos notificadores deixou de seguir
  redirecionamentos, ou um endpoint público redirecionaria para o endereço interno.
- **Exceção por instância: `RUSTRAK_WEBHOOK_ALLOWED_HOSTS`**, vazia por padrão, sem chave que
  desligue a checagem. Existe porque destinos internos são legítimos (o agente de triagem do
  ADR-0014 fica na rede interna).
- **Limite conhecido:** nomes públicos que resolvem para IP interno passam; a política de saída de
  rede do nó (I3) fecha isso.
