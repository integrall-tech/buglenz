# 023 — Especificação

**WHEN** alguém tenta entrar com um e-mail que não existe
**THEN** a resposta é 401 `Invalid credentials` e o tempo de resposta é da mesma ordem que o de uma senha errada para conta existente

**WHEN** uma senha com mais de 1024 caracteres chega ao login
**THEN** a resposta é 4xx sem consulta ao banco e sem Argon2

**WHEN** o registro recebe senha com menos de 8 ou mais de 1024 caracteres
**THEN** a resposta é 400 de validação

**WHEN** o login tem sucesso
**THEN** o identificador de sessão enviado antes do login não é mais válido e a resposta traz um novo

**WHEN** um webhook é criado ou editado com URL para loopback, rede privada, link-local, `localhost`, `*.local` ou `*.internal`
**THEN** a resposta é 400 de validação

**WHEN** um webhook é criado com URL pública HTTPS
**THEN** ele é aceito

**WHEN** o ingest recebe corpo acima do limite
**THEN** a resposta é 413 com corpo JSON
