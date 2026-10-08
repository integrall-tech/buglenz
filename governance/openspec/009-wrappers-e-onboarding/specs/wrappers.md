# 009 — Especificação

## Fronteira de licença

**WHEN** os manifestos de dependência dos wrappers (`package.json`, `pom.xml`, `build.gradle`) são lidos
**THEN** nenhum referencia `@rustrak/*` nem código do fork, e o repositório dos wrappers não contém arquivo copiado de `apps/dashboard` ou `packages/ui`

## Wrapper React

**WHEN** um app inicializa o wrapper com `app: "vendax-web"`, `version: "1.4.2"`, `environment: "production"`, `tenant` e `cliente`
**THEN** os eventos chegam à instância com `release` = `vendax-web@1.4.2`, o `environment` informado e as tags `tenant` e `cliente`

**WHEN** o app não passa DSN
**THEN** a inicialização falha de forma visível (não fica em silêncio)

**WHEN** o app lança um erro cuja mensagem contém e-mail e CPF, com cabeçalho `Cookie` e campo `password` no corpo
**THEN** a issue na instância não contém nenhum desses valores, já no evento que o SDK enviou (camada 1) e, por redundância, depois do digest (camada 2)

**WHEN** `identify` recebe um objeto com `email` ou `cpf`
**THEN** o wrapper recusa (lança em teste, descarta e avisa em produção) e só o `id` é enviado

**WHEN** o app é construído com `brandSourceMaps` e o token da instância
**THEN** os source maps sobem para a instância, os `.map` não ficam no `dist`, e o frame do erro aparece com arquivo e linha originais

## Wrapper Spring Boot

**WHEN** a aplicação define `buglenz.dsn`, `buglenz.app`, `buglenz.environment` e o artefato tem versão `2.0.1`
**THEN** os eventos chegam com `release` = `<app>@2.0.1`, `environment` e as tags, e `sentry.send-default-pii` é `false`

**WHEN** a aplicação Spring Boot sobe com o wrapper e lança uma exceção
**THEN** o wrapper não iniciou nenhuma sessão: a instância não recebe itens `session` desse app e o release health do projeto continua vazio, sem valores negativos

**WHEN** `buglenz.dsn` está ausente com `environment` = `production`
**THEN** a aplicação não inicia e a mensagem diz qual propriedade falta

**WHEN** uma exceção chega ao `beforeSend` com cabeçalho `Authorization`, parâmetro `password` e e-mail na mensagem
**THEN** o evento enviado não os contém

## Contrato com a instância

**WHEN** o teste de contrato roda contra a imagem `buglenz-server` publicada, para cada plataforma
**THEN** a issue aparece com `release`, tags e frames corretos, a sessão é contada no release health, e o evento armazenado não tem nenhum valor da lista de negação

**WHEN** uma nova versão de SDK é proposta
**THEN** ela só entra na matriz depois de o teste de contrato passar com ela contra a versão vigente do fork; um par que falha fica marcado como não homologado

## Piloto e critério de saída

**WHEN** o produto piloto envia o primeiro erro de produção
**THEN** a issue tem frame legível, não contém dado da lista de negação, e o remetente e o `actor` dos alertas dizem BugLenz

**WHEN** passaram 30 dias de produção
**THEN** a amostragem manual não encontra dado da lista de negação, **e** a instância tem retenção ativa (pacote 004) ou exceção registrada e aprovada pelo Edson e por Neimar Chagas
