# 009 — Tarefas

Dividido em três blocos: o que pode andar já (A), o que precisa de decisão do Edson (B) e o que só
acontece com a instância em produção (C). Nada em C é iniciado antes de B estar respondido.

## A. Sem dependência pendente

- [x] T1. **Gravar o tráfego real do SDK Java** (feito em 2026-10-08, `governance/baseline/009-t1.md`; achado G24) contra uma instância local (`sentry-spring-boot` com Spring Boot 3, Java 21): itens do envelope, status de sessão enviado, cabeçalhos; anotar o que diverge do que o servidor aceita. Se o SDK Java enviar algo que o servidor descarta, abrir pacote ou issue antes do wrapper
- [x] T2. (feito em 2026-10-08, `buglenz-sdk/shared/`: lista única, 23 vetores de chaves e 22 de texto conferidos contra o servidor) Definir a lista de chaves e padrões da camada 1 (`beforeSend`/`beforeBreadcrumb`) a partir da lista do módulo `scrub` do servidor, em um arquivo único por plataforma, com teste que compara as duas listas **Ressalva da auditoria de 2026-10-09:** o teste que compara a lista de chaves do wrapper com a do servidor só roda quando `BUGLENZ_SERVER_KEYS_RS` aponta para o `keys.rs` do servidor, e a CI não define a variável; além disso só compara `exact` e `contains`.

## B. Depende de decisão

- [x] T3. **(decidido em 2026-10-08)** Um repositório `integrall-tech/buglenz-sdk`, licença MIT; criado privado. Falta definir o registry (npm e Maven) e se o repositório passa a público
- [x] T4. (feito em 2026-10-08, `buglenz-sdk/react`, 58 testes; sem integração de roteador ainda) Wrapper React: `initBugLenz`, `ErrorBoundary`, `identify`, `brandSourceMaps`; testes unitários; confirmar o nome atual da opção de PII no SDK 11.x
- [x] T5. (feito em 2026-10-08, `buglenz-sdk/spring-boot`, 59 testes; conferido ponta a ponta: nenhum valor original no envelope que sai da JVM) Wrapper Spring Boot: auto-configuração `buglenz.*`, `beforeSend`, falha na partida sem DSN em produção; testes
- [x] T6. (feito em 2026-10-08: `buglenz-sdk/contract`, `docs/matriz.md`, CI `unit`; o job `contract` na CI depende do segredo `GHCR_READ_TOKEN`) Testes de contrato na CI contra a imagem `buglenz-server` publicada (React e Spring Boot) e a matriz de homologação com o primeiro par verde **Ressalva da auditoria de 2026-10-09:** o job `contract` só roda por disparo manual e precisa do segredo `GHCR_READ_TOKEN`; nenhum PR o dispara. A matriz registra execuções locais.
- [x] T7. **(decidido em 2026-10-08: letra b)** Pacote irmão do Archbase, com versão e repositório próprios, anunciado junto; sem dependência do Archbase nem de Mantine no wrapper. Como o wrapper React é publicado "junto do Archbase" (ADR-0011): mesmo pacote, pacote irmão ou dependência opcional; sem copiar código GPL e sem importar `@rustrak/*`
- [x] T8. (feito em 2026-10-08: `buglenz-sdk/docs/onboarding.md` em português; `docs/matriz.md` está em inglês) `docs/onboarding.md` em português e `docs/matriz.md`
- [x] T9. **(decidido em 2026-10-08)** Piloto **VendaX.ai**: `vendax-admin-web` e `vendax-mobile` (Flutter), production e homolog, alertas no Mattermost; ver `piloto-vendax.md`

## C. Com a instância em produção

- [ ] T10. Implantar a instância do piloto com o pacote 003: D4, host do DSN, credencial do GHCR, backup, scrape, logs; `provision.sh` para o admin e o token de CI
- [ ] T11. Onboarding do piloto em homologação: erro provocado, issue com frame legível, sem dado pessoal; depois em produção
- [ ] T12. Primeiro alerta real configurado; conferir o remetente `alerts@buglenz.dev` (ou o definido) e o `actor` "BugLenz"
- [ ] T13. Registrar `governance/baseline/009.md` e abrir o relógio dos 30 dias; **registrar, com o Edson, a posição sobre retenção** (pacote 004 antes da produção, ou limpeza manual como exceção, seção 8 do design)
- [ ] T14. Amostragem manual aos 30 dias: nenhum evento com dado da lista de negação

## Flutter (consequência do piloto)

- [x] T15. Gravar o SDK Dart e o `sentry_flutter` contra a instância: `governance/baseline/009-dart.md` (Dart) e `009-flutter.md` (Android real: debug, release, release ofuscado, crash). iOS não verificado
- [x] T16. (feito em 2026-10-08, `buglenz-sdk/flutter`, 55 testes) Wrapper Flutter no `buglenz-sdk` (`buglenz_flutter`): `release` `<app>@<versão>`, sem PII por padrão, `beforeSend` e `beforeBreadcrumb` com a lista compartilhada de chaves e as máscaras de texto (vetores compartilhados), testes
- [ ] T17. Teste de contrato do Flutter (Dart contra a imagem publicada) e linha na `docs/matriz.md`; o `recorder.py` do contrato precisa ler requisições *chunked*
- [x] T18. **(decidido em 2026-10-08)** A ofuscação é escolha de quem cria cada app; o BugLenz suporta os dois caminhos. O piloto usa release sem ofuscação (ao menos em `homolog`). Guia de onboarding com a seção Flutter publicado. A symbolication fica como capacidade futura para quem ofusca (pacote 016); ver `baseline/009-flutter.md`
- [ ] T19. Guardar os símbolos de cada build ofuscado no CI do app (a symbolication futura depende deles)
