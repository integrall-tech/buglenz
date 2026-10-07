# 016 — Design

Marcação: [confirmado] = lido no Rustrak `v0.15.2` ou verificado nesta análise; [inferência] =
desenho proposto ou comportamento do protocolo descrito de memória, a conferir na tarefa T1.

## 1. Como o evento chega

| Origem | O que o SDK envia | Arquivo necessário |
|---|---|---|
| Java/Kotlin com R8 | frames com `module`, `function`, `lineno` ofuscados; `debug_meta.images[]` com `type: "proguard"` e `uuid` | `mapping.txt` do build, identificado pelo mesmo UUID |
| Swift/Objective-C | frames com `instruction_addr`; `debug_meta.images[]` com `type: "macho"`, `debug_id`, `image_addr`, `image_size`, `arch` | dSYM do build |
| Dart com `--obfuscate` (Android) | frames nativos com `instruction_addr`; imagem `type: "elf"` | `.symbols` gerado por `--split-debug-info` |
| Dart com `--obfuscate` (iOS) | idem, imagem `type: "macho"` | dSYM do `App.framework` |
| Crash em código NDK | frames nativos, imagem `elf` | `.so` com símbolos |

Tudo acima é [inferência] sobre o formato; a tarefa T1 confirma com eventos reais.

## 2. Upload

Reaproveita o upload em pedaços existente (`/api/0/organizations/{org}/chunk-upload/`) [confirmado].

| Mudança | Detalhe |
|---|---|
| Anunciar capacidade | acrescentar `debug_files` e `proguard` à lista `accept` da resposta de `chunk-upload` |
| Montagem | `POST /api/0/projects/{org}/{project}/files/difs/assemble/`: recebe, por checksum, nome, `debug_id` e lista de pedaços; responde `not_found` (com pedaços faltantes), `created`, `assembling`, `ok` ou `error` |
| Reprocessamento | `POST /api/0/projects/{org}/{project}/reprocessing/`: o `sentry-cli` chama ao fim do upload. Etapas 1 e 2 respondem 200 sem ação; a etapa 3 enfileira |
| Consulta | `GET /api/projects/{id}/debug-files` (lista, filtro por `debug_id`) e `DELETE` por id, na API própria |

O worker de montagem existente ganha um segundo tipo de trabalho. Ao montar, o servidor abre o
arquivo com `symbolic` (ou `proguard`), extrai `debug_id`, tipo, arquitetura e quais informações
contém (tabela de símbolos, DWARF), e rejeita o que não reconhece. Um dSYM ou `.so` "fat" gera uma
linha por arquitetura.

## 3. Armazenamento

```
debug_files
  id, project_id, debug_id, kind (proguard | macho | elf), arch,
  object_name, size_bytes, checksum, has_symtab, has_debug_info,
  created_at, last_used_at
  UNIQUE (project_id, debug_id, kind, arch)
```

- Arquivo bruto em disco, em diretório irmão do de source maps (`DEBUG_FILES_STORAGE_PATH`).
- Para nativo, o servidor converte uma vez para **symcache** (formato de consulta do `symbolic`) e
  guarda ao lado; a consulta por endereço usa o symcache mapeado em memória.
- Cache em memória com teto (`DEBUG_FILES_CACHE_MB`), no molde de `SOURCEMAP_CACHE_MB` [confirmado
  que este existe].
- Migrations nos dois diretórios (`postgres/` e `sqlite/`), conforme ADR-0005.
- Limites: tamanho máximo por arquivo (`MAX_DEBUG_FILE_BYTES`, proposta 512 MB) e por projeto.

## 4. Resolução no digest

Ponto de encaixe [confirmado]: `EventProcessor` chama `sourcemap::rewrite_frames` e depois
`calculate_grouping_key`. A nova ordem:

```
rewrite_frames (source map JS)
→ remap_proguard      (se há imagem proguard)
→ symbolicate_native  (se há frames com instruction_addr)
→ classificar in_app
→ calculate_grouping_key
```

**R8/ProGuard.** Para cada exceção e cada thread: traduzir tipo e módulo da exceção, e cada frame
(classe, método, linha). Um frame ofuscado pode virar vários (código embutido pelo R8); a ordem
segue o que o `proguard` devolve.

**Nativo.** Para cada frame com `instruction_addr`:
1. achar a imagem cujo intervalo `[image_addr, image_addr + image_size)` contém o endereço;
2. buscar o symcache por `debug_id` e arquitetura;
3. consultar o endereço relativo; em frame que não é o do topo, usar o ajuste de endereço de
   retorno do `symbolic`;
4. preencher `function` (com demangle de Swift, C++ e Rust), `filename`, `lineno`, `package`;
   expandir funções embutidas em frames adicionais;
5. marcar o resultado por frame: `symbolicated`, `missing_symbol`, `missing_debug_file`,
   `malformed`.

O evento ganha um resumo (`symbolication: { status, missing_debug_ids[] }`) gravado junto dos
campos desnormalizados, para a interface e para a etapa 3.

**`in_app`.** Frames de imagens do sistema e de bibliotecas de terceiros recebem `in_app: false`
por regra de caminho/pacote configurável por projeto; isso também resolve G8 para mobile.

O trabalho de CPU roda fora do executor assíncrono (`spawn_blocking`), com tempo máximo por
evento; estourou o tempo, o evento segue sem símbolos e marcado.

## 5. Símbolos ausentes e reprocessamento (etapa 3)

- Evento sem arquivo de debug é processado na hora, agrupado pelo que houver, e marcado.
- A issue mostra o aviso com os `debug_id` faltantes e o comando de upload.
- Quando chega um arquivo cujo `debug_id` consta em eventos marcados dos últimos N dias, o
  servidor refaz a resolução desses eventos. **Reagrupar** eventos já atribuídos a uma issue muda
  contagens e pode fundir issues: a semântica exata pede ADR própria antes da etapa 3.
- Limpeza: arquivo de debug sem uso há mais que o prazo de retenção do projeto é removido pelo
  worker de retenção (pacote 004).

## 6. Interface

- Configurações do projeto → "Arquivos de debug": lista, tamanho, arquitetura, último uso, excluir.
- Evento: selo por frame não resolvido; faixa de aviso com `debug_id` e instrução de upload.
- A tela de evento já usa `module`/`package` como rótulo e tem seção de threads [confirmado]; não
  verifiquei como ela exibe um frame que só tem endereço.
- Tela de Storage passa a contar arquivos de debug.

## 7. Lado dos apps (entra nos wrappers do ADR-0011)

Nomes de opção de memória, a conferir nas versões fixadas. [inferência]

| App | Build | Upload |
|---|---|---|
| Flutter | `flutter build --obfuscate --split-debug-info=<dir>` | `sentry_dart_plugin` com `url` do BugLenz, `project` = slug, token de API; envia símbolos de Android e iOS e source maps de web |
| Android | R8 ligado | plugin Gradle do Sentry com upload automático de mapeamento e de símbolos nativos; `url` em `sentry.properties` |
| iOS | arquivo com dSYM | fase de build ou fastlane chamando `sentry-cli debug-files upload` |

Regra do CI: o upload acontece **antes** de o app ser distribuído; falha de upload falha o
pipeline de release.

## 8. Segurança e limites

- Upload só com token de API de quem tem papel Editor ou Admin no projeto.
- Arquivos compactados: recusar caminho com `..`, limitar tamanho descompactado e número de
  entradas.
- Interpretação de binário com teto de memória e de tempo; falha de parse vira `error` na
  montagem, nunca derruba o worker.
- Arquivo de debug não contém dado pessoal, mas revela nomes internos do código do cliente:
  mesmo controle de acesso do projeto.

## 9. Testes

| Nível | O que |
|---|---|
| Unitário | mapeamento R8 com embutimento; busca de imagem por endereço; ajuste de endereço de retorno; demangle |
| Dourado | pares (evento bruto, evento esperado) para Kotlin+R8, Flutter Android arm64, Flutter iOS arm64, Swift |
| Contrato | reprodução do tráfego gravado do `sentry-cli`, do plugin Gradle e do `sentry_dart_plugin` (T1) |
| Ponta a ponta | app Flutter ofuscado em emulador Android no CI; iOS com artefatos gerados em macOS e versionados como fixture |

Fixtures de iOS exigem uma máquina macOS para serem geradas.

## 10. Metas propostas

- 100% dos frames do app resolvidos nas fixtures; frames de sistema do iOS ficam de fora da conta.
- Resolução de um evento com cache quente abaixo de 50 ms no p95; com cache frio abaixo de 2 s.
- Memória do processo dentro do teto configurado com 20 arquivos de debug em uso.

Valores a validar na execução. [inferência]
