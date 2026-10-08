# 023 — Correções de segurança do servidor

## Por quê

A verificação do PR #57 do upstream (ADR-0018) mostrou que quatro falhas conhecidas continuam na
base do fork: oráculo de tempo no login (H-1), DoS por tamanho de senha (H-2/M-3), SSRF por URL de
webhook (H-4) e fixação de sessão no login (M-2). A instância do piloto recebe tráfego real e tem
login por senha local até o SSO (pacote 008); o roteiro pede resolver isso **antes do primeiro dado
de produção**.

## O que muda

1. Login: executa uma verificação Argon2 contra um hash fixo quando o e-mail não existe, para
   igualar o tempo (H-1).
2. Senha: limite superior de 1024 bytes no login, no aceite de convite, na troca de senha e no
   vínculo SSO, antes de qualquer consulta ou Argon2 (H-2/M-3). **Sem mínimo**: o upstream decidiu
   assim e o mínimo é política (ver ADR-0018).
3. Webhook: `validate_config` recusa destinos de loopback, redes privadas (RFC 1918), link-local
   (169.254.0.0/16) e nomes como `localhost`, `*.local`, `*.internal` (H-4). Atenção: a validação
   de configuração não impede DNS que resolve para IP interno; avaliar a checagem também no envio.
4. Login e registro renovam o identificador da sessão antes de gravar o usuário (M-2).
5. M-1: teste que envia corpo acima do limite ao ingest e confere 413 em JSON; corrige só se falhar.

## O que não muda

- Mensagens de erro 5xx (H-3 já está corrigido no upstream).
- O SSO, que já renova a sessão.

## Impacto

- Delta no manifesto: `routes/auth.rs`, `models/user.rs`, `services/notification/webhook.rs`, testes.
- Risco: o limite de 8 caracteres pode recusar senhas existentes ao registrar; não afeta login de
  contas já criadas (só o limite superior vale no login). Webhooks já cadastrados com destino interno
  deixam de ser aceitos na edição.
- Tamanho: P a M. Revisão humana obrigatória (§5).
