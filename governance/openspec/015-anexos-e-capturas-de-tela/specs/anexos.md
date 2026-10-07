# 015 — Especificação

**WHEN** um app Flutter com `attachScreenshot` envia um erro Dart para projeto com capturas habilitadas
**THEN** o evento exibe a miniatura e a aba "Anexos" lista a imagem

**WHEN** o mesmo envelope chega a um projeto com capturas desabilitadas
**THEN** o evento é gravado normalmente e nenhum anexo é armazenado

**WHEN** um anexo excede o limite por arquivo
**THEN** o anexo é descartado, o evento é gravado e o contador de anexos rejeitados sobe

**WHEN** o item declara `image/png` mas o conteúdo não é uma imagem
**THEN** o anexo é descartado

**WHEN** o evento associado é rejeitado por cota
**THEN** o anexo não permanece em disco

**WHEN** um usuário sem acesso ao projeto pede o download
**THEN** a resposta é 404 ou 403, como para o evento

**WHEN** o prazo de retenção de anexos do projeto vence
**THEN** os arquivos e as linhas correspondentes são removidos e o evento continua disponível

**WHEN** os eventos de um titular são excluídos
**THEN** os anexos desses eventos também são removidos

**WHEN** o app sofre crash nativo e o SDK não consegue capturar a tela
**THEN** o evento chega sem anexo e nada falha
