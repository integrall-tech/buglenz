# 002 — Tarefas

Executado em 2026-10-07/08 no branch `pkg/002-remocao-de-egress`, PR integrall-tech/buglenz#4.
T2 e T3 em um commit (nenhum compila sem o outro); T7 com três correções após a primeira
execução na CI (ver `design.md`, seção 9). T9: janela de 15 min roda no push para `main` após o merge.

Um commit por tarefa, conventional commits em inglês citando a tarefa (`refactor: remove the
telemetry reporter (002/T2)`). Toda alteração ou remoção de arquivo do upstream entra em
`DELTA-MANIFEST.md` no mesmo commit.

- [x] T0. Confirmar que `main` contém o merge de `sync/2026-10-07` (`git merge-base main v0.16.0` = `4dbe5ce7`) e criar `pkg/002-remocao-de-egress`
- [x] T1. Refazer o inventário da seção 1 do `design.md` sobre a base atual (`grep` de `reqwest::Client`, `lettre`, `fetch(`, URLs literais); qualquer saída nova para e pergunta
- [x] T2. Remover `posthog.rs`, `reporter.rs`, `report.rs`, `identity.rs`, `volume.rs`, `resources.rs` e enxugar `telemetry/mod.rs`; `cargo build` limpo com `CARGO_BUILD_WARNINGS=deny`
- [x] T3. Remover `routes/telemetry.rs`, a configuração em `main.rs`, `routes/mod.rs`, `openapi.rs` e `TelemetryConfig` em `config.rs`; regenerar `openapi.json`
- [x] T4. Ajustar os testes do servidor (seção 2.3); `cargo test` e `cargo test --no-default-features --features postgres --test e2e_tests` sem falha
- [x] T5. `Dockerfile` sem o segredo de build; `.env.example` e os dois `docker-compose*.yml` sem as linhas de telemetria
- [x] T6. Dashboard: remover `version-check.ts` e `update-banner-slot.tsx`, ajustar `_authenticated.tsx`; `pnpm run ci:web` verde
- [x] T7. `scripts/egress-denylist.txt`, `scripts/network-conformance.sh` e `.github/workflows/network-conformance.yml`; primeira execução verde no PR, inclusive o autoteste das regras de rede
- [x] T8. `DELTA-MANIFEST.md` (todas as entradas da seção 5), `CHANGES-FROM-UPSTREAM.md` (comportamento visível ao operador, documentação do upstream que não se aplica), `THIRD-PARTY-LICENSES.md` regenerado
- [x] T9. Registrar `governance/baseline/002.md` com a janela de PR (3 min); a janela de 15 min roda no push para `main` após o merge e é anexada à baseline
- [x] T10. Conferir cada cenário de `specs/egress.md` e fechar o pacote
