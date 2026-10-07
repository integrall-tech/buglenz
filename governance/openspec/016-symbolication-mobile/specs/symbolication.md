# 016 — Especificação

## Upload

**WHEN** o `sentry-cli` consulta as capacidades de upload
**THEN** a resposta inclui `debug_files` e `proguard`

**WHEN** o plugin Gradle do Sentry conclui o build de release de um app com R8
**THEN** o mapeamento aparece na lista de arquivos de debug do projeto com o UUID do build

**WHEN** o `sentry_dart_plugin` roda após um build Flutter com `--obfuscate --split-debug-info`
**THEN** os arquivos de símbolos de cada arquitetura aparecem na lista, cada um com seu `debug_id`

**WHEN** o mesmo arquivo é enviado duas vezes
**THEN** existe uma única entrada e o segundo upload termina com sucesso

**WHEN** um arquivo excede o limite configurado ou não é um formato reconhecido
**THEN** a montagem responde `error` com o motivo e nada é gravado

**WHEN** um token de usuário sem papel Editor no projeto tenta enviar
**THEN** a resposta é 403

## R8/ProGuard

**WHEN** chega um evento Kotlin ofuscado cujo UUID de mapeamento está armazenado
**THEN** tipo da exceção, classes, métodos e linhas gravados são os do código original

**WHEN** um frame ofuscado corresponde a código embutido pelo R8
**THEN** o evento gravado contém os frames originais na ordem de chamada

## Nativo

**WHEN** chega um erro Dart de um build Flutter ofuscado (Android arm64) com símbolos armazenados
**THEN** cada frame do app tem função, arquivo `.dart` e linha

**WHEN** chega um crash Swift com dSYM armazenado
**THEN** os frames do app têm nome de função legível (demangle) e os frames de sistema mantêm imagem e endereço

**WHEN** o evento traz frames de mais de uma imagem e só uma tem arquivo de debug
**THEN** os frames dessa imagem são resolvidos e os demais ficam marcados `missing_debug_file`

## Agrupamento

**WHEN** dois eventos do mesmo erro vêm de builds diferentes, ambos com símbolos
**THEN** caem na mesma issue

**WHEN** a resolução estoura o tempo máximo
**THEN** o evento é gravado sem símbolos, marcado, e o digest segue

## Símbolos ausentes

**WHEN** um evento chega sem arquivo de debug correspondente
**THEN** a issue exibe o aviso com os `debug_id` faltantes

**WHEN** (etapa 3) o arquivo de debug chega depois do evento
**THEN** o evento é resolvido de novo e o aviso desaparece

## Sem regressão

**WHEN** o roteiro React de ponta a ponta do `GAP-ANALYSIS.md` §3 é executado
**THEN** o resultado é o mesmo de antes do pacote

**WHEN** um evento não tem imagem `proguard` nem frame com `instruction_addr`
**THEN** as etapas novas não o alteram
