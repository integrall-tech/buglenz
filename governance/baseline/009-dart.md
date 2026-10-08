# 009 / Flutter — o SDK Dart contra a instância

**Data:** 2026-10-08 · **Servidor:** `main` (SQLite, debug) · **Cliente:** `sentry` (Dart) **9.30.1**, Dart 3.11, executado
com `dart run` (JIT, sem Flutter). Tudo [confirmado] por execução, exceto onde marcado.

## O que foi verificado

Um programa Dart inicializa o SDK com `release = vendax-mobile@1.0.0+1`, `environment = homolog`,
usuário com e-mail e IP, uma tag, `extra.password`, um breadcrumb com e-mail, lança uma exceção com CPF e
e-mail na mensagem e captura também uma mensagem avulsa.

| Aspecto | Resultado |
|---|---|
| Aceitação | o servidor aceita os envelopes do SDK Dart (gzip, enviados em *chunked*, sem `Content-Length`) |
| Issues | 2: a exceção e a mensagem (`Log Message: …`) |
| `release`, `environment`, tag | `vendax-mobile@1.0.0+1` (o `+` do `pubspec.yaml` passa), `homolog`, `cliente=acme` |
| Plataforma | `other`; SDK `sentry.dart 9.30.1` |
| Stack | 2 frames legíveis: `probe.dart:31 main` → `probe.dart:11 recusarPedido`, `in_app` verdadeiro |
| Privacidade (camada 2 do servidor) | título `… cpf [cpf] ([email])`; `user.email` `[email]`; `user.ip_address` `[Filtered]`; `extra.password` `[Filtered]`; breadcrumb `[email]`; nenhum `remote_addr` gravado |

**O SDK cru manda o dado pessoal inteiro** (e-mail, IP, senha em `extra`, CPF na mensagem), como o Java e o
JavaScript: um wrapper com camada 1 é necessário também para Flutter.

## Equívoco que corrigi durante a medição

Meu proxy de gravação devolveu **400 "Empty envelope headers"** a todos os envelopes. Não era defeito do
servidor: o SDK Dart envia a requisição em *chunked*, sem `Content-Length`, e o proxy leu zero bytes.
Sem o proxy, tudo é aceito. Para gravar o tráfego do Flutter é preciso um proxy que leia *chunked*
(`recorder.py` do `buglenz-sdk` **também** precisa disso).

## O que **não** foi verificado

- **`sentry_flutter` em dispositivo.** É ele que um app Flutter usa; traz as camadas nativas (Android e
  iOS), sessões e *release health*, ANR, *breadcrumbs* automáticos e crash nativo. Nesta máquina há um
  Android físico conectado, mas as licenças do Android SDK não estão aceitas e o Xcode está incompleto
  (sem iOS nem macOS). Rodar exige aceitar licenças e instalar um app de teste no aparelho: decisão sua.
- **Build de release (AOT), ofuscação e `--split-debug-info`.** Em `dart run` os frames são legíveis. Um app
  de loja costuma ser compilado com `--obfuscate --split-debug-info`, e aí os frames chegam ilegíveis e a
  instância **não** os traduz (G17, pacote 016, decisão D3).
- **Crash nativo** (JNI, NDK, Kotlin/Swift): sem symbolication no servidor (G17).
- **Sessões do Flutter** (a camada nativa envia `session`): o servidor conta a mesma sessão uma vez
  depois do G24, mas isso não foi exercitado com o Flutter.
