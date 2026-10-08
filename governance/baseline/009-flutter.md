# 009 / Flutter — `sentry_flutter` num Android real contra a instância

**Data:** 2026-10-08 · **Servidor:** `main` (SQLite, debug, em `127.0.0.1` ligado ao aparelho por `adb reverse`) ·
**Aparelho:** Samsung SM-X115 (tablet), Android 16 · **Cliente:** `sentry_flutter` **9.30.1** com o wrapper
`buglenz_flutter` 0.1.0, Flutter 3.41.2. App de teste instalado e **desinstalado** no fim. Tudo [confirmado] por execução.

## O que chegou, por cenário

| Cenário | Build | Resultado na instância |
|---|---|---|
| Exceção tratada, com e-mail e CPF na mensagem | debug | issue `PedidoException: … cpf [cpf] ([email])`; frames legíveis (`main.dart:18 recusarPedido`) |
| Erro não tratado (zona do Dart) | debug | capturado; agrupa com o anterior |
| Erro de render do framework Flutter | debug | issue `StateError: … [email]` |
| **Release sem ofuscação** | release (AOT) | **legível**: tipo `PedidoException`, `main.dart:18 recusarPedido`, `in_app`; **agrupa** com os builds de debug |
| **Release ofuscado** (`--obfuscate --split-debug-info`) | release (AOT) | **ilegível**: tipo `nz` (nome ofuscado, muda a cada build), frames só `instruction_addr` (`0x0000006e505793f3`), sem função, arquivo nem linha; vem `debug_meta` com o `debug_id` do ELF do app |
| Crash da camada Android (`nativeCrash()`) | debug | issue `RuntimeException: FlutterSentry Native Integration…`, `level: fatal`, `handled: false`, `platform: java`, frames Kotlin/Java legíveis; sem *minidump* |

## Privacidade, no aparelho real

- `user` armazenado: só `id`. `extra.password` `[Filtered]`. Breadcrumb `[email]`. Nenhum valor original gravado; nenhum `remote_addr`.
- **Contextos do dispositivo:** o evento trazia um **identificador do aparelho** (`device.id`), o **horário exato de boot**
  e campos que podem ter o nome do aparelho. O wrapper agora os remove (`name`, `id`, `device_unique_identifier`,
  `boot_time`); modelo, marca, SO, memória e fuso ficam. Conferido no aparelho depois da correção.
- **Evento criado pela camada nativa (o crash):** **não passa pelos filtros do Dart.** Levou `user.email` e `ip_address`
  do escopo para a rede; a instância os mascarou (`[email]`, `[Filtered]`), então nada ficou gravado, mas o valor
  original saiu do aparelho. Regra para o app: **nunca** colocar e-mail, documento ou IP no escopo; usar `identify(id)`.

## Sessões (release health)

`vendax-mobile@1.0.3+4`: 2 sessões, 2 `crashed` (dois crashes, cada sessão contada uma vez, G24 corrigido). Os erros
tratados e os não tratados do Dart aparecem como sessões `errored`/`crashed` conforme o SDK os reporta; o servidor
conta o que recebe. Não separei, por item, o que o SDK Flutter envia (sem proxy de gravação *chunked*).

## O que isto decide

1. **Release sem ofuscação funciona hoje, sem nenhum trabalho no servidor.** É o caminho de menor custo para o piloto.
2. **Release ofuscado só é útil com symbolication no servidor** (pacote 016, G17): precisa receber os símbolos do
   `--split-debug-info` (`.symbols`, ELF com DWARF, chave `debug_id`) e traduzir `instruction_addr` e o nome da
   classe na ingestão. É trabalho grande e novo; hoje não existe.
3. Crashes **do Android** chegam (camada JVM), com frames legíveis **em debug**. Em release o R8 costuma ofuscar
   as classes Java/Kotlin; sem o `mapping.txt` eles ficam ilegíveis (não medido).

## Não verificado

- **iOS** (sem Xcode completo nesta máquina) e **macOS**.
- **Crash NDK/nativo de C/C++** (*minidump*): `nativeCrash()` deste SDK lançou uma exceção Java, não um sinal nativo.
- **Android release com R8** (frames Java/Kotlin ofuscados): não medi.
- Outros aparelhos e versões do Android; ANR; *app hang*.
- O contrato automatizado do Flutter contra a imagem publicada (T17).
