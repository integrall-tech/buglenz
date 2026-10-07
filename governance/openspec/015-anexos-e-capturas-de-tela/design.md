# 015 — Design

Marcação: [confirmado] = lido no código ou na documentação; [inferência] = proposta.

## Entrada

Item de envelope `attachment`, com cabeçalhos `filename`, `content_type`, `attachment_type` e
`length`. O item exige um evento associado; o parser do Rustrak já o classifica assim
(`requires_event`), e depois o descarta em `routes/ingest.rs`. [confirmado]

| `attachment_type` | Conteúdo | Tratamento |
|---|---|---|
| `event.attachment` com imagem PNG ou JPEG | captura de tela | aceito |
| `event.view_hierarchy` | JSON da árvore de views | aceito |
| qualquer outro | — | descartado, com contador em `/metrics` |

O anexo pode vir no mesmo envelope do evento ou em envelope separado com o mesmo `event_id`.
[inferência; conferir com SDK real na tarefa T1]

## Regras de aceitação

1. Projeto com "aceitar capturas de tela" desligado (padrão): item descartado.
2. Limite por arquivo (proposta: 2 MB) e por evento (proposta: 5 MB). O envelope inteiro já é
   limitado a 100 MB. [confirmado]
3. O conteúdo é validado pelo cabeçalho do arquivo, não só pelo `content_type` declarado.
4. Anexos contam em cota própria por projeto, separada da de eventos.

## Armazenamento

```
event_attachments
  id, project_id, event_id, issue_id, kind (screenshot | view_hierarchy),
  filename, content_type, size_bytes, checksum, created_at
```

Arquivo em disco, em diretório próprio (`ATTACHMENTS_STORAGE_PATH`). Segue a ingestão em duas
fases: grava em `INGEST_DIR`, responde ao SDK, e o digest move para o destino quando o evento é
gravado. Anexo cujo evento foi rejeitado ou limitado por cota é apagado.

## Leitura

- `GET /api/projects/{id}/events/{event_id}/attachments` e download por id, com o papel de
  projeto exigido para ler o evento.
- Download registrado na atividade da issue.
- Resposta com `Content-Disposition` e sem execução de conteúdo no navegador.

## Interface

Miniatura no cabeçalho do evento; aba "Anexos"; visualizador da hierarquia de views em árvore.
Chave "aceitar capturas de tela" nas configurações do projeto, com aviso sobre dados pessoais.

## Dados pessoais

- Mascaramento é feito no app. O wrapper de SDK liga `attachScreenshot` só em projeto habilitado,
  mantém `maskAllText` e `maskAllImages` ligados e não oferece como desligá-los. No Flutter o
  mascaramento de capturas vem ligado por padrão. [confirmado na documentação]
- Retenção própria (proposta: 30 dias), aplicada pelo worker do pacote 004.
- A exclusão por titular do ADR-0009 apaga também os anexos dos eventos removidos.

## Lado dos apps

| SDK | O que ligar |
|---|---|
| `sentry_flutter` | `attachScreenshot`, raiz envolvida por `SentryWidget`; `attachViewHierarchy` opcional |
| Android e iOS nativos | opções equivalentes de captura de tela e hierarquia |

Nomes exatos a conferir nas versões fixadas. [inferência para Android e iOS]
