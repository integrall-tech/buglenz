# 009 — Tarefas

Dividido em três blocos: o que pode andar já (A), o que precisa de decisão do Edson (B) e o que só
acontece com a instância em produção (C). Nada em C é iniciado antes de B estar respondido.

## A. Sem dependência pendente

- [x] T1. **Gravar o tráfego real do SDK Java** (feito em 2026-10-08, `governance/baseline/009-t1.md`; achado G24) contra uma instância local (`sentry-spring-boot` com Spring Boot 3, Java 21): itens do envelope, status de sessão enviado, cabeçalhos; anotar o que diverge do que o servidor aceita. Se o SDK Java enviar algo que o servidor descarta, abrir pacote ou issue antes do wrapper
- [x] T2. (feito em 2026-10-08, `buglenz-sdk/shared/`: lista única, 23 vetores de chaves e 22 de texto conferidos contra o servidor) Definir a lista de chaves e padrões da camada 1 (`beforeSend`/`beforeBreadcrumb`) a partir da lista do módulo `scrub` do servidor, em um arquivo único por plataforma, com teste que compara as duas listas

## B. Depende de decisão

- [x] T3. **(decidido em 2026-10-08)** Um repositório `integrall-tech/buglenz-sdk`, licença MIT; criado privado. Falta definir o registry (npm e Maven) e se o repositório passa a público
- [x] T4. (feito em 2026-10-08, `buglenz-sdk/react`, 58 testes; sem integração de roteador ainda) Wrapper React: `initBugLenz`, `ErrorBoundary`, `identify`, `brandSourceMaps`; testes unitários; confirmar o nome atual da opção de PII no SDK 11.x
- [x] T5. (feito em 2026-10-08, `buglenz-sdk/spring-boot`, 59 testes; conferido ponta a ponta: nenhum valor original no envelope que sai da JVM) Wrapper Spring Boot: auto-configuração `buglenz.*`, `beforeSend`, falha na partida sem DSN em produção; testes
- [ ] T6. Testes de contrato na CI contra a imagem `buglenz-server` publicada (React e Spring Boot) e a matriz de homologação com o primeiro par verde
- [x] T7. **(decidido em 2026-10-08: letra b)** Pacote irmão do Archbase, com versão e repositório próprios, anunciado junto; sem dependência do Archbase nem de Mantine no wrapper. Como o wrapper React é publicado "junto do Archbase" (ADR-0011): mesmo pacote, pacote irmão ou dependência opcional; sem copiar código GPL e sem importar `@rustrak/*`
- [ ] T8. `docs/onboarding.md` em português e `docs/matriz.md`
- [ ] T9. **(decisão)** Nomear o produto piloto e os projetos na instância (um por app implantável)

## C. Com a instância em produção

- [ ] T10. Implantar a instância do piloto com o pacote 003: D4, host do DSN, credencial do GHCR, backup, scrape, logs; `provision.sh` para o admin e o token de CI
- [ ] T11. Onboarding do piloto em homologação: erro provocado, issue com frame legível, sem dado pessoal; depois em produção
- [ ] T12. Primeiro alerta real configurado; conferir o remetente `alerts@buglenz.dev` (ou o definido) e o `actor` "BugLenz"
- [ ] T13. Registrar `governance/baseline/009.md` e abrir o relógio dos 30 dias; **registrar, com o Edson, a posição sobre retenção** (pacote 004 antes da produção, ou limpeza manual como exceção, seção 8 do design)
- [ ] T14. Amostragem manual aos 30 dias: nenhum evento com dado da lista de negação
