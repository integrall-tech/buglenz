# 023 — Tarefas

- [ ] T1. Baixar o PR #57 (`gh pr checkout 57 --repo rustrak/rustrak` num clone separado), ler os 10 arquivos e listar o que aplica sem conflito em `v0.16.0`
- [ ] T2. H-1: teste que mede a diferença de tempo (e-mail inexistente × senha errada) e falha; correção; teste verde
- [ ] T3. H-2/M-3: testes de limite (registro 7 e 1025 caracteres, login 1025) e correção
- [ ] T4. M-2: teste que confere cookie de sessão diferente antes e depois do login; correção
- [ ] T5. H-4: testes por URL (127.0.0.1, 10.0.0.1, 192.168.1.1, 169.254.169.254, `localhost`, `x.internal`, `x.local`, e um destino público aceito); correção; decidir sobre a checagem no envio
- [ ] T6. M-1: teste de corpo acima do limite no ingest; corrigir só se falhar
- [ ] T7. `DELTA-MANIFEST.md` com ADR-0018; baseline `governance/baseline/023.md`
- [ ] T8. **Revisão humana linha a linha (§5)** antes de implantar
- [ ] T9. Comentar no PR #57 do upstream oferecendo a versão rebaseada; abrir PR novo se o mantenedor responder
