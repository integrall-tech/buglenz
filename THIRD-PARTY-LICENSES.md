# Licenças de terceiros

Inventário das dependências do BugLenz e das licenças que elas declaram. Gerado por
`governance/tools/third-party-licenses.py` a partir de `cargo deny list` (Rust) e
`pnpm -r licenses list` (JavaScript); o cabeçalho do script tem os comandos. Este arquivo é
regenerado a cada sincronização com o upstream.

Data: 2026-10-07 (último commit dos lockfiles). Base: Rustrak `v0.16.0`.

A allow-list de licenças que o upstream aceita está em `deny.toml` e é verificada pelo
workflow `rust-security.yml` (`cargo deny check advisories licenses` em `apps/server`).
Não há verificação equivalente para o lado JavaScript no upstream.

O inventário JavaScript cobre dependências de produção e de desenvolvimento e inclui os
pacotes opcionais da plataforma em que foi gerado (binários de `@sentry/cli`, `sharp`,
`esbuild`, etc.). Em outra plataforma a lista de opcionais muda.

| Inventário | Pacotes |
|---|---|
| Rust, `apps/server` | 428 |
| Rust, `packages/benchmarks` | 258 |
| JavaScript, workspace pnpm | 1270 |

## Rust

### Rust: `apps/server` (crate `rustrak`)

428 crates.

**MIT OR Apache-2.0** (243)

`actix-codec 0.5.4`, `actix-cors 0.7.2`, `actix-files 0.7.0`, `actix-http 3.18.12`, `actix-macros 0.2.5`, `actix-multipart 0.8.5`, `actix-multipart-derive 0.8.1`, `actix-router 0.5.4`, `actix-rt 2.15.0`, `actix-server 2.9.7`, `actix-service 2.0.3`, `actix-session 0.11.0`, `actix-utils 3.0.2`, `actix-web 4.15.0`, `actix-web-codegen 4.4.0`, `aead 0.5.2`, `aes 0.8.4`, `allocator-api2 0.2.21`, `android_system_properties 0.1.6`, `anstream 1.0.0`, `anstyle 1.0.14`, `anstyle-parse 1.0.0`, `anstyle-query 1.1.5`, `anstyle-wincon 3.0.11`, `anyhow 1.0.104`, `argon2 0.6.0`, `async-trait 0.1.92`, `base64 0.20.0`, `base64 0.21.7`, `base64 0.22.1`, `base64 0.23.1`, `bitflags 2.13.2`, `blake2 0.11.0`, `block-buffer 0.10.4`, `block-buffer 0.12.1`, `bumpalo 3.20.3`, `bytestring 1.5.1`, `cc 1.5.1`, `cfg-if 1.0.5`, `chacha20 0.10.2`, `chrono 0.4.45`, `chrono-tz 0.10.4`, `cipher 0.4.4`, `cmake 0.1.58`, `colorchoice 1.0.5`, `cookie 0.16.2`, `core-foundation 0.10.1`, `core-foundation-sys 0.8.7`, `core_detect 1.0.0`, `cpufeatures 0.2.17`, `cpufeatures 0.3.1`, `crc 3.4.0`, `crc-catalog 2.5.0`, `crc32fast 1.5.2`, `crossbeam-queue 0.3.14`, `crossbeam-utils 0.8.23`, `crypto-common 0.1.6`, `crypto-common 0.2.2`, `ctr 0.9.2`, `curve25519-dalek-derive 0.1.1`, `deranged 0.5.8`, `digest 0.10.7`, `digest 0.11.3`, `displaydoc 0.2.7`, `dyn-clone 1.0.20`, `either 1.18.0`, `email-encoding 0.4.2`, `env_filter 2.0.0`, `env_logger 0.11.11`, `erased-serde 0.4.10`, `errno 0.3.14`, `ff 0.13.1`, `find-msvc-tools 0.1.14`, `flate2 1.1.10`, `foreign-types 0.3.2`, `foreign-types-shared 0.1.1`, `form_urlencoded 1.2.2`, `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-executor 0.3.34`, `futures-intrusive 0.5.0`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`, `getrandom 0.2.17`, `getrandom 0.3.4`, `getrandom 0.4.3`, `group 0.13.0`, `hashbrown 0.16.1`, `hashbrown 0.17.1`, `hashlink 0.11.1`, `heck 0.5.0`, `hex 0.4.3`, `hkdf 0.12.4`, `hmac 0.12.1`, `hmac 0.13.0`, `http 0.2.12`, `http 1.5.0`, `httparse 1.10.1`, `httpdate 1.0.3`, `hybrid-array 0.4.15`, `hyper-tls 0.6.0`, `iana-time-zone 0.1.65`, `iana-time-zone-haiku 0.1.2`, `ident_case 1.0.1`, `idna 1.1.0`, `if_chain 1.0.3`, `impl-more 0.3.8`, `inout 0.1.4`, `ipnet 2.12.2`, `is_terminal_polyfill 1.70.2`, `itertools 0.10.5`, `itoa 1.0.18`, `jni 0.22.4`, `jni-macros 0.22.4`, `jni-sys 0.4.1`, `jni-sys-macros 0.4.1`, `jobserver 0.1.35`, `js-sys 0.3.106`, `language-tags 0.3.2`, `lazy_static 1.5.0`, `libc 0.2.189`, `local-waker 0.1.4`, `lock_api 0.4.14`, `log 0.4.34`, `mime 0.3.17`, `native-tls 0.2.18`, `num-bigint-dig 0.8.6`, `num-conv 0.2.2`, `num-integer 0.1.47`, `num-iter 0.1.46`, `num-traits 0.2.19`, `oauth2 5.0.0`, `once_cell 1.21.4`, `once_cell_polyfill 1.70.2`, `opaque-debug 0.3.1`, `openssl-macros 0.1.1`, `openssl-probe 0.2.1`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `password-hash 0.6.1`, `percent-encoding 2.3.2`, `pkg-config 0.3.34`, `powerfmt 0.2.0`, `ppv-lite86 0.2.21`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rand 0.10.3`, `rand 0.8.8`, `rand 0.9.5`, `rand_chacha 0.3.1`, `rand_chacha 0.9.0`, `rand_core 0.10.1`, `rand_core 0.6.4`, `rand_core 0.9.5`, `regex 1.13.1`, `regex-automata 0.4.18`, `regex-lite 0.1.9`, `regex-syntax 0.8.11`, `reqwest 0.13.5`, `rsa 0.9.10`, `rustc_version 0.4.1`, `rustls-pki-types 1.15.1`, `rustls-platform-verifier 0.7.1`, `rustls-platform-verifier-android 0.2.0`, `rustversion 1.0.23`, `scopeguard 1.2.0`, `security-framework 3.7.0`, `security-framework-sys 2.17.0`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_path_to_error 0.1.20`, `serde_plain 1.0.2`, `serde_urlencoded 0.7.1`, `serde_with 3.24.0`, `serde_with_macros 3.24.0`, `sha1 0.11.0`, `sha2 0.10.9`, `sha2 0.11.0`, `shlex 2.0.1`, `signal-hook-registry 1.4.8`, `simdutf8 0.1.5`, `siphasher 1.0.4`, `slug 0.1.6`, `smallvec 1.16.2`, `socket2 0.6.5`, `sqlx 0.9.0`, `sqlx-core 0.9.0`, `sqlx-macros 0.9.0`, `sqlx-macros-core 0.9.0`, `sqlx-sqlite 0.9.0`, `stable_deref_trait 1.2.1`, `syn 2.0.119`, `syn 3.0.6`, `tempfile 3.27.0`, `thiserror 1.0.69`, `thiserror 2.0.21`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `time 0.3.53`, `time-core 0.1.9`, `time-macros 0.2.31`, `tokio-rustls 0.26.5`, `typed-path 0.12.3`, `typeid 1.0.3`, `typenum 1.20.1`, `unicase 2.9.0`, `unicode-segmentation 1.13.3`, `unicode-xid 0.2.6`, `universal-hash 0.5.1`, `url 2.5.8`, `v_escape-base 0.1.0`, `v_htmlescape 0.17.0`, `vcpkg 0.2.15`, `version_check 0.9.5`, `wasm-bindgen 0.2.129`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-shared 0.2.129`, `web-sys 0.3.106`, `windows-core 0.62.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.52.0`, `windows-sys 0.61.2`, `windows-targets 0.52.6`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_msvc 0.52.6`, `windows_i686_gnu 0.52.6`, `windows_i686_gnullvm 0.52.6`, `windows_i686_msvc 0.52.6`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_msvc 0.52.6`

**MIT** (68)

`atoi 2.0.0`, `base64-simd 0.8.0`, `bitvec 1.1.1`, `bytes 1.12.1`, `combine 4.6.8`, `convert_case 0.10.0`, `darling 0.24.1`, `darling_core 0.24.1`, `darling_macro 0.24.1`, `data-encoding 2.11.1`, `derive_more 2.1.1`, `derive_more-impl 2.1.1`, `dotenvy 0.15.7`, `dynfmt 0.2.0`, `email_address 0.2.9`, `fancy-regex 0.19.2`, `fs_extra 1.3.0`, `funty 2.0.0`, `generic-array 0.14.9`, `h2 0.4.19`, `http-body 1.1.0`, `http-body-util 0.1.5`, `http-range 0.1.5`, `hyper 1.11.1`, `hyper-util 0.1.21`, `lettre 0.11.23`, `libm 0.2.16`, `libmimalloc-sys 0.1.49`, `libsqlite3-sys 0.37.0`, `mimalloc 0.1.52`, `mime_guess 2.0.5`, `mio 1.2.3`, `nom 8.0.0`, `openidconnect 4.0.1`, `openssl-sys 0.9.117`, `ordered-float 2.10.1`, `outref 0.5.2`, `phf 0.12.1`, `phf_shared 0.12.1`, `radium 0.7.0`, `redox_syscall 0.5.18`, `schannel 0.1.29`, `serde-value 0.7.0`, `simd-adler32 0.3.10`, `slab 0.4.12`, `spin 0.9.9`, `strsim 0.11.1`, `synstructure 0.14.0`, `tap 1.0.1`, `tokio 1.53.2`, `tokio-macros 2.7.2`, `tokio-native-tls 0.3.1`, `tokio-stream 0.1.19`, `tokio-util 0.7.19`, `tower 0.5.3`, `tower-http 0.6.11`, `tower-layer 0.3.3`, `tower-service 0.3.3`, `tracing 0.1.44`, `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `try-lock 0.2.5`, `vsimd 0.8.0`, `want 0.3.1`, `wyz 0.5.1`, `zip 8.6.0`, `zmij 1.0.23`, `zstd 0.13.3`

**Apache-2.0 OR MIT** (47)

`aes-gcm 0.10.3`, `atomic-waker 1.1.2`, `autocfg 1.5.1`, `base16ct 0.2.0`, `base64ct 1.8.3`, `bit-set 0.8.0`, `bit-vec 0.8.0`, `cmov 0.5.4`, `const-oid 0.10.2`, `const-oid 0.9.6`, `crypto-bigint 0.5.5`, `ctutils 0.4.2`, `der 0.7.10`, `ecdsa 0.16.9`, `ed25519 2.2.3`, `elliptic-curve 0.13.8`, `equivalent 1.0.2`, `event-listener 5.4.2`, `fastrand 2.5.0`, `flume 0.12.0`, `fnv 1.0.7`, `ghash 0.5.1`, `idna_adapter 1.2.2`, `indexmap 2.14.2`, `multiversion_no_op 1.0.0`, `p256 0.13.2`, `p384 0.13.1`, `parking 2.2.1`, `pem-rfc7468 0.7.0`, `phc 0.6.1`, `pin-project-lite 0.2.17`, `pkcs1 0.7.5`, `pkcs8 0.10.2`, `polyval 0.6.2`, `portable-atomic 1.15.0`, `portable-atomic-util 0.2.8`, `primeorder 0.13.6`, `rfc6979 0.4.0`, `rustc-hash 2.1.3`, `sec1 0.7.3`, `signature 2.2.0`, `simd_cesu8 1.2.0`, `spki 0.7.3`, `utf8_iter 1.0.4`, `utf8parse 0.2.2`, `uuid 1.27.0`, `zeroize 1.9.0`

**Unicode-3.0** (18)

`icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke 0.8.3`, `yoke-derive 0.8.3`, `zerofrom 0.1.8`, `zerofrom-derive 0.1.8`, `zerotrie 0.2.5`, `zerovec 0.11.8`, `zerovec-derive 0.11.6`

**BSD-3-Clause** (9)

`alloc-no-stdlib 2.0.4`, `alloc-stdlib 0.2.4`, `curve25519-dalek 4.1.3`, `deunicode 1.6.2`, `ed25519-dalek 2.2.0`, `sourcemap 9.3.2`, `subtle 2.6.1`, `zstd-safe 7.3.0`, `zstd-sys 2.1.0+zstd.1.5.7`

**Unlicense OR MIT** (7)

`aho-corasick 1.1.5`, `jiff 0.2.37`, `jiff-core 0.1.1`, `memchr 2.8.3`, `same-file 1.0.6`, `walkdir 2.5.0`, `winapi-util 0.1.11`

**Apache-2.0** (6)

`bytesize 2.7.0`, `debugid 0.8.0`, `memo-map 0.3.4`, `minijinja 2.24.0`, `openssl 0.10.81`, `sync_wrapper 1.0.2`

**Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT** (5)

`linux-raw-sys 0.12.1`, `rustix 1.1.5`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`

**Apache-2.0 OR ISC OR MIT** (3)

`hyper-rustls 0.27.10`, `rustls 0.23.45`, `rustls-native-certs 0.8.4`

**BSD-3-Clause OR MIT** (2)

`brotli 8.0.4`, `brotli-decompressor 5.0.3`

**CDLA-Permissive-2.0** (2)

`webpki-root-certs 1.0.9`, `webpki-roots 1.0.9`

**ISC** (2)

`rustls-webpki 0.103.15`, `untrusted 0.9.0`

**MIT OR Apache-2.0 OR LGPL-2.1-or-later** (2)

`r-efi 5.3.0`, `r-efi 6.0.0`

**MIT OR Apache-2.0 OR Unicode-3.0** (2)

`unicode-id-start 1.5.0`, `unicode-ident 1.0.26`

**0BSD** (1)

`quoted_printable 0.5.2`

**0BSD OR MIT OR Apache-2.0** (1)

`adler2 2.0.1`

**Apache-2.0 OR BSL-1.0** (1)

`ryu 1.0.23`

**Apache-2.0 OR ISC** (1)

`ring 0.17.14`

**Apache-2.0 OR MIT OR BSD-3-Clause** (1)

`encoding_rs 0.8.42`

**BSD-2-Clause OR Apache-2.0 OR MIT** (1)

`zerocopy 0.8.59`

**CC0-1.0 OR MIT-0 OR Apache-2.0** (1)

`dunce 1.0.5`

**ISC OR Apache-2.0** (1)

`aws-lc-rs 1.18.1`

**ISC OR Apache-2.0 OR MIT OR BSD-3-Clause OR MIT-0** (1)

`aws-lc-sys 0.45.0`

**MIT OR Apache-2.0 OR BSD-1-Clause** (1)

`fiat-crypto 0.2.9`

**MIT OR Zlib OR Apache-2.0** (1)

`miniz_oxide 0.9.1`

**Zlib** (1)

`foldhash 0.2.0`

### Rust: `packages/benchmarks` (crate `rustrak-benchmarks`)

258 crates.

**MIT OR Apache-2.0** (157)

`android_system_properties 0.1.6`, `anstream 1.0.0`, `anstyle 1.0.14`, `anstyle-parse 1.0.0`, `anstyle-query 1.1.5`, `anstyle-wincon 3.0.11`, `anyhow 1.0.104`, `async-compression 0.4.48`, `async-trait 0.1.92`, `base64 0.22.1`, `base64 0.23.1`, `bitflags 2.13.2`, `block-buffer 0.12.1`, `bumpalo 3.20.3`, `cc 1.5.1`, `cfg-if 1.0.5`, `chacha20 0.10.2`, `chrono 0.4.45`, `clap 4.6.7`, `clap_builder 4.6.7`, `clap_derive 4.6.7`, `clap_lex 1.1.1`, `cmake 0.1.58`, `colorchoice 1.0.5`, `compression-codecs 0.4.43`, `compression-core 0.4.33`, `core-foundation 0.10.1`, `core-foundation-sys 0.8.7`, `cpufeatures 0.3.1`, `crc32fast 1.5.2`, `crossbeam-channel 0.5.17`, `crossbeam-utils 0.8.23`, `crypto-common 0.2.2`, `digest 0.11.3`, `displaydoc 0.2.7`, `errno 0.3.14`, `fallible-iterator 0.2.0`, `find-msvc-tools 0.1.14`, `flate2 1.1.10`, `form_urlencoded 1.2.2`, `futures 0.3.34`, `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-executor 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`, `getrandom 0.4.3`, `hdrhistogram 7.6.0`, `heck 0.5.0`, `hex 0.4.3`, `hmac 0.13.0`, `http 1.5.0`, `httparse 1.10.1`, `httpdate 1.0.3`, `hybrid-array 0.4.15`, `iana-time-zone 0.1.65`, `iana-time-zone-haiku 0.1.2`, `idna 1.1.0`, `ipnet 2.12.2`, `is_terminal_polyfill 1.70.2`, `itoa 1.0.18`, `jni 0.22.4`, `jni-macros 0.22.4`, `jni-sys 0.4.1`, `jni-sys-macros 0.4.1`, `jobserver 0.1.35`, `js-sys 0.3.106`, `libc 0.2.189`, `lock_api 0.4.14`, `log 0.4.34`, `md-5 0.11.0`, `num-traits 0.2.19`, `once_cell 1.21.4`, `once_cell_polyfill 1.70.2`, `openssl-probe 0.2.1`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `percent-encoding 2.3.2`, `pkg-config 0.3.34`, `postgres-protocol 0.6.12`, `postgres-types 0.2.14`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rand 0.10.3`, `rand_core 0.10.1`, `reqwest 0.13.5`, `rustc_version 0.4.1`, `rustls-pki-types 1.15.1`, `rustls-platform-verifier 0.7.1`, `rustls-platform-verifier-android 0.2.0`, `rustversion 1.0.23`, `scopeguard 1.2.0`, `security-framework 3.7.0`, `security-framework-sys 2.17.0`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `serde_spanned 1.1.1`, `serde_urlencoded 0.7.1`, `sha2 0.11.0`, `shlex 2.0.1`, `signal-hook-registry 1.4.8`, `simdutf8 0.1.5`, `siphasher 1.0.4`, `smallvec 1.16.2`, `socket2 0.6.5`, `stable_deref_trait 1.2.1`, `stringprep 0.1.5`, `syn 2.0.119`, `syn 3.0.6`, `thiserror 2.0.21`, `thiserror-impl 2.0.21`, `tokio-postgres 0.7.18`, `tokio-rustls 0.26.5`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`, `typenum 1.20.1`, `unicode-bidi 0.3.18`, `unicode-normalization 0.1.25`, `unicode-properties 0.1.4`, `unicode-width 0.2.2`, `url 2.5.8`, `wasm-bindgen 0.2.129`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-shared 0.2.129`, `web-sys 0.3.106`, `web-time 1.1.0`, `winapi 0.3.9`, `winapi-i686-pc-windows-gnu 0.4.0`, `winapi-x86_64-pc-windows-gnu 0.4.0`, `windows-core 0.62.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.52.0`, `windows-sys 0.61.2`, `windows-targets 0.52.6`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_msvc 0.52.6`, `windows_i686_gnu 0.52.6`, `windows_i686_gnullvm 0.52.6`, `windows_i686_msvc 0.52.6`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_msvc 0.52.6`

**MIT** (35)

`bytes 1.12.1`, `combine 4.6.8`, `console 0.16.6`, `fs_extra 1.3.0`, `http-body 1.1.0`, `http-body-util 0.1.5`, `hyper 1.11.1`, `hyper-util 0.1.21`, `hyperlocal 0.9.1`, `indicatif 0.18.6`, `libredox 0.1.25`, `mio 1.2.3`, `nom 8.0.0`, `phf 0.13.1`, `phf_shared 0.13.1`, `redox_syscall 0.5.18`, `schannel 0.1.29`, `simd-adler32 0.3.10`, `slab 0.4.12`, `strsim 0.11.1`, `synstructure 0.14.0`, `tokio 1.53.2`, `tokio-macros 2.7.2`, `tokio-util 0.7.19`, `tower 0.5.3`, `tower-http 0.6.11`, `tower-layer 0.3.3`, `tower-service 0.3.3`, `tracing 0.1.44`, `tracing-core 0.1.36`, `try-lock 0.2.5`, `unit-prefix 0.5.2`, `want 0.3.1`, `winnow 1.0.4`, `zmij 1.0.23`

**Unicode-3.0** (18)

`icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke 0.8.3`, `yoke-derive 0.8.3`, `zerofrom 0.1.8`, `zerofrom-derive 0.1.8`, `zerotrie 0.2.5`, `zerovec 0.11.8`, `zerovec-derive 0.11.6`

**Apache-2.0 OR MIT** (14)

`atomic-waker 1.1.2`, `autocfg 1.5.1`, `cmov 0.5.4`, `const-oid 0.10.2`, `ctutils 0.4.2`, `encode_unicode 1.0.0`, `idna_adapter 1.2.2`, `pin-project-lite 0.2.17`, `portable-atomic 1.15.0`, `simd_cesu8 1.2.0`, `utf8_iter 1.0.4`, `utf8parse 0.2.2`, `uuid 1.27.0`, `zeroize 1.9.0`

**Unlicense OR MIT** (5)

`byteorder 1.5.0`, `memchr 2.8.3`, `same-file 1.0.6`, `walkdir 2.5.0`, `winapi-util 0.1.11`

**Apache-2.0** (4)

`bollard 0.21.1`, `bollard-stubs 1.53.1-rc.29.3.1`, `hyper-named-pipe 0.1.1`, `sync_wrapper 1.0.2`

**Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT** (4)

`wasi 0.11.1+wasi-snapshot-preview1`, `wasi 0.14.7+wasi-0.2.4`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`

**Apache-2.0 OR ISC OR MIT** (3)

`hyper-rustls 0.27.10`, `rustls 0.23.45`, `rustls-native-certs 0.8.4`

**Zlib OR Apache-2.0 OR MIT** (3)

`objc2-core-foundation 0.3.2`, `objc2-system-configuration 0.3.2`, `tinyvec 1.13.3`

**Apache-2.0 OR BSL-1.0 OR MIT** (2)

`wasite 1.0.2`, `whoami 2.1.3`

**ISC** (2)

`rustls-webpki 0.103.15`, `untrusted 0.9.0`

**0BSD OR MIT OR Apache-2.0** (1)

`adler2 2.0.1`

**Apache-2.0 OR BSL-1.0** (1)

`ryu 1.0.23`

**BSD-3-Clause** (1)

`subtle 2.6.1`

**CC0-1.0 OR MIT-0 OR Apache-2.0** (1)

`dunce 1.0.5`

**CDLA-Permissive-2.0** (1)

`webpki-root-certs 1.0.9`

**ISC OR Apache-2.0** (1)

`aws-lc-rs 1.18.1`

**ISC OR Apache-2.0 OR MIT OR BSD-3-Clause OR MIT-0** (1)

`aws-lc-sys 0.45.0`

**MIT OR Apache-2.0 OR LGPL-2.1-or-later** (1)

`r-efi 6.0.0`

**MIT OR Apache-2.0 OR Unicode-3.0** (1)

`unicode-ident 1.0.26`

**MIT OR Zlib OR Apache-2.0** (1)

`miniz_oxide 0.9.1`

**MPL-2.0** (1)

`colored 3.1.1`

## JavaScript

### JavaScript (workspace pnpm)

1270 pacotes (dependências de produção e de desenvolvimento).

**MIT** (1066)

`@adobe/css-tools 4.5.0`, `@alloc/quick-lru 5.2.0`, `@antfu/install-pkg 1.1.0`, `@asamuzakjp/css-color 5.1.11`, `@asamuzakjp/dom-selector 7.1.1`, `@asamuzakjp/generational-cache 1.0.1`, `@asamuzakjp/nwsapi 2.3.9`, `@astrojs/compiler 4.0.0`, `@babel/code-frame 7.29.7`, `@babel/compat-data 7.29.7`, `@babel/core 7.29.7`, `@babel/generator 7.29.8`, `@babel/helper-annotate-as-pure 7.29.7`, `@babel/helper-compilation-targets 7.29.7`, `@babel/helper-create-class-features-plugin 7.29.7`, `@babel/helper-globals 7.29.7`, `@babel/helper-member-expression-to-functions 7.29.7`, `@babel/helper-module-imports 7.29.7`, `@babel/helper-module-transforms 7.29.7`, `@babel/helper-optimise-call-expression 7.29.7`, `@babel/helper-plugin-utils 7.29.7`, `@babel/helper-replace-supers 7.29.7`, `@babel/helper-skip-transparent-expression-wrappers 7.29.7`, `@babel/helper-string-parser 7.29.7`, `@babel/helper-validator-identifier 7.29.7`, `@babel/helper-validator-option 7.29.7`, `@babel/helpers 7.29.7`, `@babel/parser 7.29.8`, `@babel/plugin-syntax-jsx 7.29.7`, `@babel/plugin-syntax-typescript 7.29.7`, `@babel/plugin-transform-modules-commonjs 7.29.7`, `@babel/plugin-transform-typescript 7.29.7`, `@babel/preset-typescript 7.29.7`, `@babel/runtime 7.29.7`, `@babel/template 7.29.7`, `@babel/traverse 7.29.7`, `@babel/types 7.29.8`, `@base-ui/react 1.8.0`, `@base-ui/utils 0.4.0`, `@bcoe/v8-coverage 1.0.2`, `@blazediff/core 1.10.0`, `@braintree/sanitize-url 7.1.1`, `@bramus/specificity 2.4.2`, `@changesets/apply-release-plan 8.1.1`, `@changesets/assemble-release-plan 7.0.0`, `@changesets/changelog-git 1.0.0`, `@changesets/changelog-github 1.0.1`, `@changesets/cli 3.0.3`, `@changesets/config 4.0.1`, `@changesets/errors 1.0.0`, `@changesets/format 0.1.2`, `@changesets/get-dependents-graph 3.0.0`, `@changesets/get-github-info 1.0.1`, `@changesets/git 4.0.1`, `@changesets/parse 1.0.0`, `@changesets/pre 3.0.0`, `@changesets/read 1.0.1`, `@changesets/should-skip-package 1.0.0`, `@changesets/types 7.0.0`, `@changesets/write 1.0.1`, `@clack/core 1.5.1`, `@clack/prompts 1.8.1`, `@codemirror/autocomplete 6.20.3`, `@codemirror/commands 6.11.1`, `@codemirror/lang-css 6.3.1`, `@codemirror/lang-html 6.4.11`, `@codemirror/lang-javascript 6.2.5`, `@codemirror/lang-json 6.0.2`, `@codemirror/lang-xml 6.1.0`, `@codemirror/lang-yaml 6.1.3`, `@codemirror/language 6.12.4`, `@codemirror/lint 6.9.7`, `@codemirror/state 6.7.6`, `@codemirror/view 6.43.13`, `@csstools/css-calc 3.3.0`, `@csstools/css-color-parser 4.1.10`, `@csstools/css-parser-algorithms 4.0.0`, `@csstools/css-tokenizer 4.0.0`, `@date-fns/tz 1.4.1`, `@esbuild/linux-x64 0.27.2, 0.28.2`, `@eslint-community/eslint-utils 4.10.1`, `@eslint-community/regexpp 4.12.2`, `@exodus/bytes 1.15.1`, `@floating-ui/core 1.8.0`, `@floating-ui/dom 1.8.0`, `@floating-ui/react 0.26.28`, `@floating-ui/react-dom 2.1.9`, `@floating-ui/utils 0.2.10, 0.2.12`, `@floating-ui/vue 1.1.9`, `@formatjs/fast-memoize 3.1.7`, `@formatjs/icu-messageformat-parser 3.5.16`, `@formatjs/icu-skeleton-parser 2.1.11`, `@formatjs/intl-localematcher 0.6.2`, `@headlessui/react 2.2.9`, `@headlessui/tailwindcss 0.2.2`, `@headlessui/vue 1.7.23`, `@hono/node-server 1.19.14`, `@hookform/resolvers 5.9.1`, `@iconify/types 2.0.0`, `@iconify/utils 3.1.0`, `@img/colour 1.1.0`, `@inquirer/ansi 2.0.5`, `@inquirer/confirm 6.0.12`, `@inquirer/core 11.1.9`, `@inquirer/figures 2.0.5`, `@inquirer/type 4.0.5`, `@joshwooding/vite-plugin-react-docgen-typescript 0.7.0`, `@jridgewell/gen-mapping 0.3.13`, `@jridgewell/remapping 2.3.5`, `@jridgewell/resolve-uri 3.1.2`, `@jridgewell/sourcemap-codec 1.6.0`, `@jridgewell/trace-mapping 0.3.31`, `@lezer/common 1.5.2`, `@lezer/css 1.3.3`, `@lezer/highlight 1.2.4, 1.2.5`, `@lezer/html 1.3.13`, `@lezer/javascript 1.5.4`, `@lezer/json 1.0.3`, `@lezer/lr 1.4.10`, `@lezer/xml 1.0.6`, `@lezer/yaml 1.0.4`, `@manypkg/find-root 3.1.0`, `@manypkg/get-packages 3.1.0`, `@manypkg/tools 2.1.2`, `@marijn/find-cluster-break 1.0.2`, `@mdx-js/mdx 3.1.1`, `@mdx-js/react 3.1.1`, `@mermaid-js/parser 0.6.3`, `@modelcontextprotocol/sdk 1.32.1`, `@msw/url 0.1.2`, `@mswjs/interceptors 0.41.3, 0.45.7`, `@napi-rs/simple-git 0.1.22`, `@napi-rs/simple-git-linux-x64-gnu 0.1.22`, `@next/env 16.3.8`, `@next/swc-linux-x64-gnu 16.3.8`, `@nodelib/fs.scandir 2.1.5`, `@nodelib/fs.stat 2.0.5`, `@nodelib/fs.walk 1.2.8`, `@open-draft/deferred-promise 2.2.0, 3.0.0`, `@open-draft/logger 0.3.0`, `@open-draft/until 2.1.0, 3.0.1`, `@oxc-parser/binding-linux-x64-gnu 0.127.0, 0.148.0, 0.152.0`, `@oxc-project/types 0.127.0, 0.148.0, 0.151.0, 0.152.0`, `@oxc-resolver/binding-linux-x64-gnu 11.21.2, 11.24.2`, `@oxlint/binding-linux-x64-gnu 1.81.0`, `@pagefind/linux-x64 1.5.2`, `@phosphor-icons/core 2.1.1`, `@pnpm/deps.graph-sequencer 1100.0.1`, `@polka/url 1.0.0-next.29`, `@quansync/fs 1.0.0`, `@radix-ui/primitive 1.1.7`, `@radix-ui/react-compose-refs 1.1.5`, `@radix-ui/react-context 1.2.2`, `@radix-ui/react-dialog 1.1.23`, `@radix-ui/react-dismissable-layer 1.1.19`, `@radix-ui/react-focus-guards 1.1.6`, `@radix-ui/react-focus-scope 1.1.16`, `@radix-ui/react-id 1.1.4`, `@radix-ui/react-portal 1.1.17`, `@radix-ui/react-presence 1.1.10`, `@radix-ui/react-primitive 2.1.10`, `@radix-ui/react-slot 1.3.3`, `@radix-ui/react-use-callback-ref 1.1.4`, `@radix-ui/react-use-controllable-state 1.2.6`, `@radix-ui/react-use-effect-event 0.0.5`, `@radix-ui/react-use-layout-effect 1.1.4`, `@reduxjs/toolkit 2.12.0`, `@replit/codemirror-css-color-picker 6.3.0`, `@rolldown/binding-linux-x64-gnu 1.2.11`, `@rolldown/pluginutils 1.0.1`, `@rollup/pluginutils 5.4.0`, `@rollup/rollup-linux-x64-gnu 4.55.2`, `@scalar/agent-chat 0.12.40`, `@scalar/api-client 3.21.4`, `@scalar/api-reference 1.72.4`, `@scalar/api-reference-react 0.9.77`, `@scalar/asyncapi-upgrader 0.1.14`, `@scalar/blocks 0.4.3`, `@scalar/code-highlight 0.4.8`, `@scalar/components 0.30.6`, `@scalar/helpers 0.16.0`, `@scalar/icons 0.7.6`, `@scalar/json-magic 0.15.4`, `@scalar/localization 0.2.4`, `@scalar/oas-utils 0.20.9`, `@scalar/openapi-to-markdown 1.5.1`, `@scalar/openapi-types 0.9.7`, `@scalar/openapi-upgrader 0.4.1`, `@scalar/schemas 0.12.3`, `@scalar/sidebar 0.11.11`, `@scalar/snippetz 0.10.5`, `@scalar/themes 0.18.1`, `@scalar/typebox 0.1.3`, `@scalar/types 0.22.4`, `@scalar/use-codemirror 0.14.15`, `@scalar/use-hooks 0.4.17`, `@scalar/use-toasts 0.10.5`, `@scalar/validation 0.6.6`, `@scalar/workspace-store 0.68.0`, `@schummar/icu-type-parser 1.21.5`, `@sec-ant/readable-stream 0.4.1`, `@sentry/bundler-plugins 10.76.0, 11.4.0`, `@sentry/conventions 0.16.0, 0.25.0`, `@sentry/core 10.74.0, 10.76.0, 11.4.0`, `@sentry/esbuild-plugin 5.4.0`, `@sentry/node 10.74.0, 11.4.0`, `@sentry/node-core 10.74.0`, `@sentry/opentelemetry 10.74.0, 11.4.0`, `@sentry/server-runtime-injection 11.4.0`, `@sentry/server-utils 10.74.0, 11.4.0`, `@shadcn/registry 0.1.0`, `@shikijs/core 3.21.0`, `@shikijs/engine-javascript 3.21.0`, `@shikijs/engine-oniguruma 3.21.0`, `@shikijs/langs 3.21.0`, `@shikijs/themes 3.21.0`, `@shikijs/twoslash 3.21.0`, `@shikijs/types 3.21.0`, `@shikijs/vscode-textmate 10.0.2`, `@sindresorhus/merge-streams 4.0.0`, `@standard-schema/spec 1.1.0`, `@standard-schema/utils 0.3.0`, `@storybook/addon-a11y 10.6.1`, `@storybook/addon-docs 10.6.1`, `@storybook/addon-themes 10.6.1`, `@storybook/addon-vitest 10.6.1`, `@storybook/builder-vite 10.6.1`, `@storybook/global 5.0.0`, `@storybook/icons 2.1.0`, `@storybook/react 10.6.1`, `@storybook/react-dom-shim 10.6.1`, `@storybook/react-vite 10.6.1`, `@tailwindcss/node 4.3.3`, `@tailwindcss/oxide 4.3.3`, `@tailwindcss/oxide-linux-x64-gnu 4.3.3`, `@tailwindcss/postcss 4.3.3`, `@tailwindcss/typography 0.5.20`, `@tailwindcss/vite 4.3.3`, `@tanstack/history 1.162.4`, `@tanstack/react-router 1.170.41`, `@tanstack/react-store 0.11.2`, `@tanstack/react-table 9.2.6`, `@tanstack/react-virtual 3.13.18`, `@tanstack/router-cli 1.167.40`, `@tanstack/router-core 1.171.34`, `@tanstack/router-generator 1.167.40`, `@tanstack/router-plugin 1.168.42`, `@tanstack/router-utils 1.162.3`, `@tanstack/store 0.11.2`, `@tanstack/table-core 9.2.6`, `@tanstack/virtual-core 3.13.18, 3.14.0`, `@tanstack/virtual-file-routes 1.162.0`, `@tanstack/vue-virtual 3.13.24`, `@testing-library/dom 10.4.1`, `@testing-library/jest-dom 6.9.1`, `@testing-library/user-event 14.6.6`, `@theguild/remark-mermaid 0.3.0`, `@theguild/remark-npm2yarn 0.3.3`, `@ts-morph/common 0.27.0, 0.28.1`, `@turbo/linux-64 2.11.7`, `@types/aria-query 5.0.4`, `@types/babel__core 7.20.5`, `@types/babel__generator 7.27.0`, `@types/babel__template 7.4.4`, `@types/babel__traverse 7.28.0`, `@types/chai 5.2.3`, `@types/d3 7.4.3`, `@types/d3-array 3.2.2`, `@types/d3-axis 3.0.6`, `@types/d3-brush 3.0.6`, `@types/d3-chord 3.0.6`, `@types/d3-color 3.1.3`, `@types/d3-contour 3.0.6`, `@types/d3-delaunay 6.0.4`, `@types/d3-dispatch 3.0.7`, `@types/d3-drag 3.0.7`, `@types/d3-dsv 3.0.7`, `@types/d3-ease 3.0.2`, `@types/d3-fetch 3.0.7`, `@types/d3-force 3.0.10`, `@types/d3-format 3.0.4`, `@types/d3-geo 3.1.0`, `@types/d3-hierarchy 3.1.7`, `@types/d3-interpolate 3.0.4`, `@types/d3-path 3.1.1`, `@types/d3-polygon 3.0.2`, `@types/d3-quadtree 3.0.6`, `@types/d3-random 3.0.3`, `@types/d3-scale 4.0.9`, `@types/d3-scale-chromatic 3.1.0`, `@types/d3-selection 3.0.11`, `@types/d3-shape 3.1.8`, `@types/d3-time 3.0.4`, `@types/d3-time-format 4.0.3`, `@types/d3-timer 3.0.2`, `@types/d3-transition 3.0.9`, `@types/d3-zoom 3.0.8`, `@types/debug 4.1.12, 4.1.13`, `@types/deep-eql 4.0.2`, `@types/doctrine 0.0.9`, `@types/esrecurse 4.3.1`, `@types/estree 1.0.8`, `@types/estree-jsx 1.0.5`, `@types/geojson 7946.0.16`, `@types/har-format 1.2.16`, `@types/hast 3.0.4`, `@types/json-schema 7.0.15`, `@types/katex 0.16.8`, `@types/mdast 4.0.4`, `@types/mdx 2.0.13`, `@types/ms 2.1.0`, `@types/nlcst 2.0.3`, `@types/node 24.12.2, 26.6.2, 26.6.4`, `@types/prismjs 1.26.5`, `@types/react 19.3.0`, `@types/react-dom 19.3.0`, `@types/react-syntax-highlighter 15.5.13`, `@types/resolve 1.20.6`, `@types/set-cookie-parser 2.4.10`, `@types/statuses 2.0.6`, `@types/trusted-types 2.0.7`, `@types/unist 2.0.11, 3.0.3`, `@types/use-sync-external-store 0.0.6`, `@types/validate-npm-package-name 4.0.2`, `@types/web-bluetooth 0.0.20, 0.0.21`, `@typescript-eslint/types 8.65.0`, `@typescript/vfs 1.6.2`, `@unhead/vue 2.1.13`, `@vitejs/plugin-react 6.1.2`, `@vitest/browser 5.0.3`, `@vitest/browser-playwright 5.0.3`, `@vitest/coverage-v8 5.0.3`, `@vitest/expect 3.2.4`, `@vitest/istanbul-lib-coverage 1.0.1`, `@vitest/istanbul-lib-report 1.0.1`, `@vitest/mocker 5.0.3`, `@vitest/pretty-format 3.2.4, 4.1.11, 5.0.3`, `@vitest/runner 4.1.11`, `@vitest/spy 3.2.4, 5.0.3`, `@vitest/ui 5.0.3`, `@vitest/utils 3.2.4, 4.1.11, 5.0.3`, `@vue/compiler-core 3.5.41`, `@vue/compiler-dom 3.5.41`, `@vue/compiler-sfc 3.5.41`, `@vue/compiler-ssr 3.5.41`, `@vue/reactivity 3.5.41`, `@vue/runtime-core 3.5.41`, `@vue/runtime-dom 3.5.41`, `@vue/server-renderer 3.5.41`, `@vue/shared 3.5.41`, `@vueuse/core 10.11.1, 13.9.0`, `@vueuse/integrations 13.9.0`, `@vueuse/metadata 10.11.1, 13.9.0`, `@vueuse/shared 10.11.1, 13.9.0`, `@webcontainer/env 1.1.1`, `@xmldom/xmldom 0.9.8`, `@yuku-codegen/binding-linux-x64-gnu 0.9.5`, `@yuku-parser/binding-linux-x64-gnu 0.9.5`, `@yuku-toolchain/types 0.9.5`, `@zerollup/ts-helpers 1.7.18`, `accepts 2.0.0`, `acorn 8.18.0`, `acorn-import-attributes 1.9.5`, `acorn-jsx 5.3.2`, `agent-base 6.0.2`, `agent-install 0.0.5`, `ajv 6.15.0, 8.20.0`, `ajv-formats 2.1.1, 3.0.1`, `ansi-colors 4.1.3`, `ansi-regex 5.0.1, 6.2.2`, `ansi-styles 3.2.1, 4.3.0, 5.2.0, 6.2.3`, `any-promise 1.3.0`, `archunit 2.5.4`, `arg 5.0.2`, `argparse 1.0.10`, `aria-hidden 1.2.6`, `array-iterate 2.0.1`, `assertion-error 2.0.1`, `ast-types 0.16.1`, `ast-v8-to-istanbul 1.0.6`, `astring 1.9.0`, `async 3.2.6`, `atomically 1.7.0, 2.1.1`, `babel-dead-code-elimination 1.0.12`, `bail 2.0.2`, `balanced-match 4.0.4`, `better-react-mathjax 2.3.0`, `bidi-js 1.0.3`, `bippy 0.6.1`, `body-parser 2.2.2`, `brace-expansion 5.0.5`, `braces 3.0.3`, `browserslist 4.28.2`, `bundle-name 4.1.0`, `bundle-require 5.1.0`, `bytes 3.1.2`, `cac 6.7.14, 7.0.0`, `call-bind-apply-helpers 1.0.2`, `call-bound 1.0.4`, `callsites 3.1.0`, `ccount 2.0.1`, `chai 5.3.3, 6.2.2`, `chalk 2.4.2, 5.6.2`, `character-entities 2.0.2`, `character-entities-html4 2.1.0`, `character-entities-legacy 3.0.0`, `character-reference-invalid 2.0.1`, `check-error 2.1.3`, `chevrotain-allstar 0.3.1`, `chokidar 4.0.3, 5.0.0`, `cjs-module-lexer 2.2.0`, `cli-cursor 5.0.0`, `cli-spinners 2.9.2`, `client-only 0.0.1`, `clipboardy 4.0.0`, `clsx 2.1.1`, `cmdk 1.1.1`, `cn 0.2.6`, `code-block-writer 13.0.3`, `collapse-white-space 2.1.0`, `color-convert 1.9.3, 2.0.1`, `color-name 1.1.3, 1.1.4`, `comma-separated-tokens 2.0.3`, `commander 4.1.1, 7.2.0, 8.3.0, 11.1.0, 13.1.0, 14.0.3`, `compute-scroll-into-view 3.1.1`, `conf 10.2.0, 15.1.0`, `confbox 0.1.8, 0.2.4`, `consola 3.4.2`, `content-disposition 1.1.0`, `content-type 1.0.5, 2.0.0`, `convert-source-map 2.0.0`, `cookie 0.7.2, 1.1.1, 2.0.1`, `cookie-es 3.1.1`, `cookie-signature 1.2.2`, `core-util-is 1.0.3`, `cors 2.8.6`, `cose-base 1.0.3, 2.2.0`, `cosmiconfig 9.0.2`, `crelt 1.0.6`, `cross-spawn 7.0.6`, `css-tree 3.2.1`, `css.escape 1.5.1`, `cssesc 3.0.0`, `csstype 3.2.3`, `cytoscape 3.33.1`, `cytoscape-cose-bilkent 4.1.0`, `cytoscape-fcose 2.2.0`, `dagre-d3-es 7.0.13`, `data-urls 7.0.0`, `dataloader 2.2.3`, `date-fns 4.4.0`, `dayjs 1.11.19`, `debounce-fn 4.0.0, 6.0.0`, `debug 4.4.3`, `decimal.js 10.6.0`, `decimal.js-light 2.5.1`, `decode-named-character-reference 1.3.0`, `dedent 1.7.2`, `deep-eql 5.0.2`, `deep-is 0.1.4`, `deepmerge 4.3.1`, `default-browser 5.5.0`, `default-browser-id 5.0.1`, `define-lazy-prop 2.0.0, 3.0.0`, `defu 6.1.7`, `depd 2.0.0`, `dequal 2.0.3`, `detect-node-es 1.1.0`, `devlop 1.1.0`, `dom-accessibility-api 0.5.16, 0.6.3`, `dot-prop 6.0.1, 10.2.0`, `dts-resolver 3.0.0`, `dunder-proto 1.0.1`, `ee-first 1.1.1`, `emoji-regex 8.0.0, 10.6.0`, `empathic 2.0.1`, `encodeurl 2.0.0`, `enhanced-resolve 5.24.4`, `enquirer 2.4.1`, `env-paths 2.2.1, 3.0.0`, `error-ex 1.3.4`, `es-define-property 1.0.1`, `es-errors 1.3.0`, `es-module-lexer 2.3.2`, `es-object-atoms 1.1.1`, `es-toolkit 1.49.0`, `esast-util-from-estree 2.0.0`, `esast-util-from-js 2.0.1`, `esbuild 0.27.2, 0.28.2`, `escalade 3.2.0`, `escape-html 1.0.3`, `escape-string-regexp 1.0.5, 4.0.0, 5.0.0`, `eslint 10.8.0`, `eslint-plugin-react-hooks 7.1.1`, `esm 3.2.25`, `estree-util-attach-comments 3.0.0`, `estree-util-build-jsx 3.0.1`, `estree-util-is-identifier-name 2.1.0, 3.0.0`, `estree-util-scope 1.0.0`, `estree-util-to-js 2.0.0`, `estree-util-value-to-estree 3.5.0`, `estree-util-visit 2.0.0`, `estree-walker 2.0.2, 3.0.3`, `etag 1.8.1`, `eventemitter3 5.0.4`, `eventsource 3.0.7`, `eventsource-parser 3.0.8`, `execa 5.1.1, 8.0.1, 9.6.1`, `express 5.2.1`, `express-rate-limit 8.5.2`, `extend 3.0.2`, `extend-shallow 2.0.1`, `fast-deep-equal 3.1.3`, `fast-glob 3.3.3`, `fast-json-stable-stringify 2.1.0`, `fast-levenshtein 2.0.6`, `fast-string-truncated-width 3.0.3`, `fast-string-width 3.0.2`, `fast-wrap-ansi 0.2.0`, `fault 1.0.4, 2.0.1`, `fdir 6.5.0`, `fflate 0.8.3`, `figures 6.1.0`, `file-entry-cache 8.0.0`, `fill-range 7.1.1`, `finalhandler 2.1.1`, `find-up 3.0.0, 5.0.0`, `fix-dts-default-cjs-exports 1.0.1`, `flat-cache 4.0.1`, `focus-trap 7.8.0`, `format 0.2.2`, `forwarded 0.2.0`, `framer-motion 14.0.0`, `fresh 2.0.0`, `fs-extra 11.4.0`, `function-bind 1.1.2`, `fuzzysort 3.1.0`, `gensync 1.0.0-beta.2`, `get-east-asian-width 1.6.0`, `get-intrinsic 1.3.0`, `get-nonce 1.0.1`, `get-own-enumerable-keys 1.0.0`, `get-proto 1.0.1`, `get-stdin 8.0.0`, `get-stream 6.0.1, 8.0.1, 9.0.1`, `get-tsconfig 5.0.0-beta.6`, `gopd 1.2.0`, `graphql 16.13.2`, `gray-matter 4.0.3`, `guess-json-indent 3.0.1`, `hachure-fill 0.5.2`, `has-flag 3.0.0, 4.0.0`, `has-symbols 1.1.0`, `hasown 2.0.3`, `hast-util-embedded 3.0.0`, `hast-util-format 1.1.0`, `hast-util-from-html 2.0.3`, `hast-util-from-html-isomorphic 2.0.0`, `hast-util-from-parse5 8.0.3`, `hast-util-has-property 3.0.0`, `hast-util-is-body-ok-link 3.0.1`, `hast-util-is-element 3.0.0`, `hast-util-minify-whitespace 1.0.1`, `hast-util-parse-selector 4.0.0`, `hast-util-phrasing 3.0.1`, `hast-util-raw 9.1.0`, `hast-util-sanitize 5.0.2`, `hast-util-to-estree 3.1.3`, `hast-util-to-html 9.0.5`, `hast-util-to-jsx-runtime 2.3.6`, `hast-util-to-mdast 10.1.2`, `hast-util-to-parse5 8.0.1`, `hast-util-to-string 3.0.1`, `hast-util-to-text 4.0.2`, `hast-util-whitespace 3.0.0`, `hastscript 9.0.1`, `headers-polyfill 5.0.1`, `hermes-estree 0.25.1`, `hermes-parser 0.25.1`, `hono 4.12.19`, `hookable 6.1.1`, `html-encoding-sniffer 6.0.0`, `html-void-elements 3.0.0`, `html-whitespace-sensitive-tag-names 3.0.1`, `http-errors 2.0.1`, `https-proxy-agent 5.0.1`, `human-id 4.2.1`, `iconv-lite 0.6.3, 0.7.2`, `icu-minify 4.14.9`, `ignore 5.3.2`, `immer 11.1.8`, `import-fresh 3.3.1`, `import-meta-resolve 4.2.0`, `import-without-cache 0.4.0`, `imurmurhash 0.1.4`, `indent-string 4.0.0`, `inline-style-parser 0.2.7`, `ip-address 10.2.0`, `ipaddr.js 1.9.1`, `is-absolute-url 4.0.1`, `is-alphabetical 2.0.1`, `is-alphanumerical 2.0.1`, `is-arrayish 0.2.1`, `is-core-module 2.16.2`, `is-decimal 2.0.1`, `is-docker 2.2.1, 3.0.0`, `is-extendable 0.1.1`, `is-extglob 2.1.1`, `is-fullwidth-code-point 3.0.0`, `is-glob 4.0.3`, `is-hexadecimal 2.0.1`, `is-in-ssh 1.0.0`, `is-inside-container 1.0.0`, `is-interactive 2.0.0`, `is-node-process 1.2.0`, `is-number 7.0.0`, `is-obj 2.0.0, 3.0.0`, `is-plain-obj 4.1.0`, `is-potential-custom-element-name 1.0.1`, `is-promise 4.0.0`, `is-regexp 3.1.0`, `is-stream 2.0.1, 3.0.0, 4.0.1`, `is-unicode-supported 1.3.0, 2.1.0`, `is-wsl 2.2.0, 3.1.0`, `is64bit 2.0.0`, `isarray 1.0.0`, `jiti 2.7.0`, `jju 1.4.0`, `jose 6.2.3`, `joycon 3.1.1`, `js-tokens 4.0.0, 10.0.0`, `js-yaml 3.14.2, 4.3.2`, `jsdom 29.1.1`, `jsesc 3.1.0`, `json-buffer 3.0.1`, `json-colorizer 2.2.2`, `json-parse-even-better-errors 2.3.1`, `json-schema-traverse 0.4.1, 1.0.0`, `json-stable-stringify-without-jsonify 1.0.1`, `json5 2.2.3`, `jsonc-parser 3.3.1`, `jsonfile 6.2.1`, `katex 0.16.27`, `keyv 4.5.4`, `kind-of 6.0.3`, `kleur 3.0.3, 4.1.5`, `ky 2.1.0`, `langium 3.3.1`, `launch-editor 2.14.1`, `layout-base 1.0.2, 2.0.1`, `lenis 1.3.26`, `levn 0.4.1`, `lilconfig 3.1.3`, `lines-and-columns 1.2.4`, `load-tsconfig 0.2.5`, `locate-path 3.0.0, 6.0.0`, `lodash 4.18.1`, `lodash-es 4.17.21, 4.17.22`, `lodash.get 4.4.2`, `log-symbols 6.0.0`, `longest-streak 3.1.0`, `loupe 3.2.1`, `lowlight 1.20.0, 3.3.0`, `lz-string 1.5.0`, `magic-string 0.30.21, 1.3.1`, `magicast 0.5.5`, `markdown-extensions 2.0.0`, `markdown-table 3.0.4`, `marked 16.4.2, 18.0.14`, `math-intrinsics 1.1.0`, `mdast-util-find-and-replace 3.0.2`, `mdast-util-from-markdown 2.0.2`, `mdast-util-frontmatter 2.0.1`, `mdast-util-gfm 3.1.0`, `mdast-util-gfm-autolink-literal 2.0.1`, `mdast-util-gfm-footnote 2.1.0`, `mdast-util-gfm-strikethrough 2.0.0`, `mdast-util-gfm-table 2.0.0`, `mdast-util-gfm-task-list-item 2.0.0`, `mdast-util-math 3.0.0`, `mdast-util-mdx 3.0.0`, `mdast-util-mdx-expression 2.0.1`, `mdast-util-mdx-jsx 3.2.0`, `mdast-util-mdxjs-esm 2.0.1`, `mdast-util-phrasing 4.1.0`, `mdast-util-to-hast 13.2.1`, `mdast-util-to-markdown 2.1.2`, `mdast-util-to-string 4.0.0`, `media-typer 1.1.0`, `merge-descriptors 2.0.0`, `merge-stream 2.0.0`, `merge2 1.4.1`, `mermaid 11.12.2`, `microdiff 1.5.0`, `micromark 4.0.2`, `micromark-core-commonmark 2.0.3`, `micromark-extension-frontmatter 2.0.0`, `micromark-extension-gfm 3.0.0`, `micromark-extension-gfm-autolink-literal 2.1.0`, `micromark-extension-gfm-footnote 2.1.0`, `micromark-extension-gfm-strikethrough 2.1.0`, `micromark-extension-gfm-table 2.1.1`, `micromark-extension-gfm-tagfilter 2.0.0`, `micromark-extension-gfm-task-list-item 2.1.0`, `micromark-extension-math 3.1.0`, `micromark-extension-mdx-expression 3.0.1`, `micromark-extension-mdx-jsx 3.0.2`, `micromark-extension-mdx-md 2.0.0`, `micromark-extension-mdxjs 3.0.0`, `micromark-extension-mdxjs-esm 3.0.0`, `micromark-factory-destination 2.0.1`, `micromark-factory-label 2.0.1`, `micromark-factory-mdx-expression 2.0.3`, `micromark-factory-space 2.0.1`, `micromark-factory-title 2.0.1`, `micromark-factory-whitespace 2.0.1`, `micromark-util-character 2.1.1`, `micromark-util-chunked 2.0.1`, `micromark-util-classify-character 2.0.1`, `micromark-util-combine-extensions 2.0.1`, `micromark-util-decode-numeric-character-reference 2.0.2`, `micromark-util-decode-string 2.0.1`, `micromark-util-encode 2.0.1`, `micromark-util-events-to-acorn 2.0.3`, `micromark-util-html-tag-name 2.0.1`, `micromark-util-normalize-identifier 2.0.1`, `micromark-util-resolve-all 2.0.1`, `micromark-util-sanitize-uri 2.0.1`, `micromark-util-subtokenize 2.1.0`, `micromark-util-symbol 2.0.1`, `micromark-util-types 2.0.2`, `micromatch 4.0.8`, `mime-db 1.54.0`, `mime-types 3.0.2`, `mimic-fn 2.1.0, 3.1.0, 4.0.0`, `mimic-function 5.0.1`, `min-indent 1.0.1`, `minimist 1.2.8`, `mlly 1.8.0`, `module-details-from-path 1.0.4`, `motion 14.0.0`, `motion-dom 14.0.0`, `motion-utils 14.0.0`, `mrmime 2.0.1`, `ms 2.1.3`, `msw 2.15.0, 3.0.2`, `mz 2.7.0`, `nanoid 3.3.18, 5.1.9`, `natural-compare 1.4.0`, `negotiator 1.0.0`, `neverpanic 0.0.8`, `next 16.3.8`, `next-themes 0.4.6`, `nextra 4.6.1`, `nextra-theme-docs 4.6.1`, `nlcst-to-string 4.0.0`, `node-fetch 2.7.0`, `node-releases 2.0.46`, `node-stream 1.7.0`, `npm-run-path 4.0.1, 5.3.0, 6.0.0`, `npm-to-yarn 3.0.1`, `object-assign 4.1.1`, `object-inspect 1.13.4`, `object-treeify 1.1.33`, `obug 2.1.4`, `on-finished 2.4.1`, `onetime 5.1.2, 6.0.0, 7.0.0`, `oniguruma-parser 0.12.1`, `oniguruma-to-es 4.3.4`, `open 8.4.2, 10.2.0, 11.0.0`, `optionator 0.9.4`, `ora 8.2.0`, `outvariant 1.4.3`, `oxc-parser 0.127.0, 0.148.0, 0.152.0`, `oxc-resolver 11.21.2, 11.24.2`, `oxlint 1.81.0`, `oxlint-plugin-react-doctor 0.9.17`, `p-limit 2.3.0, 3.1.0`, `p-locate 3.0.0, 5.0.0`, `p-try 2.2.0`, `package-manager-detector 1.8.0`, `pagefind 1.5.2`, `parent-module 1.0.1`, `parse-entities 4.0.2`, `parse-json 5.2.0`, `parse-latin 7.0.0`, `parse-ms 4.0.0`, `parse5 7.3.0, 8.0.1`, `parseurl 1.3.3`, `path-browserify 1.0.1`, `path-data-parser 0.1.0`, `path-exists 3.0.0, 4.0.0`, `path-key 3.1.1, 4.0.0`, `path-parse 1.0.7`, `path-to-regexp 6.3.0, 8.4.2`, `pathe 2.0.3`, `pathval 2.0.1`, `picomatch 2.3.1, 4.0.7`, `pirates 4.0.7`, `pkce-challenge 5.0.1`, `pkg-types 1.3.1`, `pkg-up 3.1.0`, `pngjs 7.0.0`, `points-on-curve 0.2.0`, `points-on-path 0.2.1`, `postcss 8.5.23, 8.5.28`, `postcss-load-config 6.0.1`, `postcss-selector-parser 6.0.10, 7.1.4`, `powershell-utils 0.1.0`, `prelude-ls 1.2.1`, `prettier 3.9.6`, `pretty-format 27.5.1`, `pretty-ms 9.3.0`, `prismjs 1.30.0`, `process-nextick-args 2.0.1`, `progress 2.0.3`, `prompts 2.4.2`, `property-information 7.1.0`, `proxy-addr 2.0.7`, `proxy-from-env 1.1.0`, `punycode 2.3.1`, `quansync 1.0.0`, `queue-microtask 1.2.3`, `radix-vue 1.9.17`, `range-parser 1.2.1`, `raw-body 3.0.2`, `react 19.3.0`, `react-compiler-runtime 19.1.0-rc.3`, `react-docgen 8.0.3`, `react-docgen-typescript 2.4.0`, `react-doctor 0.9.17`, `react-dom 19.3.0`, `react-hook-form 7.89.0`, `react-is 17.0.2`, `react-redux 9.3.0`, `react-remove-scroll 2.7.2`, `react-remove-scroll-bar 2.3.8`, `react-style-singleton 2.2.3`, `react-syntax-highlighter 16.1.1`, `readable-stream 2.3.8`, `readdirp 4.1.2, 5.1.1`, `reading-time 1.5.0`, `recast 0.23.12`, `recharts 3.10.1`, `recma-build-jsx 1.0.0`, `recma-jsx 1.0.1`, `recma-parse 1.0.0`, `recma-stringify 1.0.0`, `redent 3.0.0`, `redux 5.0.1`, `redux-thunk 3.1.0`, `refractor 5.0.0`, `regex 6.1.0`, `regex-recursion 6.0.2`, `regex-utilities 2.3.0`, `rehype-external-links 3.0.0`, `rehype-format 5.0.1`, `rehype-katex 7.0.1`, `rehype-minify-whitespace 6.0.2`, `rehype-parse 9.0.1`, `rehype-pretty-code 0.14.1`, `rehype-raw 7.0.0`, `rehype-recma 1.0.0`, `rehype-remark 10.0.1`, `rehype-sanitize 6.0.0`, `rehype-stringify 10.0.1`, `remark-frontmatter 5.0.0`, `remark-gfm 4.0.1`, `remark-math 6.0.0`, `remark-mdx 3.1.1`, `remark-parse 11.0.0`, `remark-rehype 11.1.2`, `remark-smartypants 3.0.2`, `remark-stringify 11.0.0`, `require-dir 1.2.0`, `require-directory 2.1.1`, `require-from-string 2.0.2`, `require-in-the-middle 8.0.1`, `reselect 5.2.0`, `resolve 1.22.12`, `resolve-from 4.0.0, 5.0.0`, `resolve-pkg-maps 1.0.0`, `restore-cursor 5.1.0`, `retext 9.0.0`, `retext-latin 4.0.0`, `retext-smartypants 6.2.0`, `retext-stringify 4.0.0`, `rettime 0.11.11, 0.11.12`, `reusify 1.1.0`, `rolldown 1.2.11`, `rolldown-plugin-dts 0.28.5`, `rollup 4.55.2`, `roughjs 4.6.6`, `router 2.2.0`, `run-applescript 7.1.0`, `run-parallel 1.2.0`, `safe-buffer 5.1.2`, `safer-buffer 2.1.2`, `scheduler 0.28.0`, `scroll-into-view-if-needed 3.1.0`, `section-matter 1.0.0`, `send 1.2.1`, `serialize-error 7.0.1`, `seroval 1.6.7`, `seroval-plugins 1.6.7`, `serve-static 2.2.1`, `server-only 0.0.1`, `set-cookie-parser 3.1.0`, `shadcn 4.21.1`, `shebang-command 2.0.0`, `shebang-regex 3.0.0`, `shell-quote 1.10.0`, `shiki 3.21.0`, `side-channel 1.1.0`, `side-channel-list 1.0.1`, `side-channel-map 1.0.1`, `side-channel-weakmap 1.0.2`, `sirv 3.0.2`, `sisteransi 1.0.5`, `slash 5.1.0`, `smart-buffer 4.2.0`, `socks 2.8.9`, `sonner 2.0.8`, `space-separated-tokens 2.0.2`, `statuses 2.0.2`, `std-env 4.2.0`, `stdin-discarder 0.2.2`, `storybook 10.6.1`, `stream-combiner2 1.1.1`, `strict-event-emitter 0.5.1`, `string-byte-length 3.0.1`, `string-byte-slice 3.0.1`, `string-width 4.2.3, 7.2.0, 8.3.0`, `string_decoder 1.1.1`, `stringify-entities 4.0.4`, `strip-ansi 6.0.1, 7.2.0`, `strip-bom 3.0.0`, `strip-bom-string 1.0.0`, `strip-final-newline 2.0.0, 3.0.0, 4.0.0`, `strip-indent 3.0.0, 4.1.1`, `stubborn-fs 2.0.0`, `stubborn-utils 1.0.2`, `style-mod 4.1.3`, `style-to-js 1.1.21`, `style-to-object 1.0.14`, `styled-jsx 5.1.6`, `stylis 4.3.6`, `sucrase 3.35.1`, `supports-color 5.5.0, 8.1.1`, `supports-preserve-symlinks-flag 1.0.0`, `symbol-tree 3.2.4`, `system-architecture 0.1.0`, `systeminformation 5.33.1`, `tabbable 6.4.0`, `tagged-tag 1.0.0`, `tailwind-merge 3.5.0, 3.7.0`, `tailwind-variants 3.3.1`, `tailwindcss 4.3.3`, `tapable 2.3.3`, `thenify 3.3.1`, `thenify-all 1.6.0`, `through2 2.0.5`, `tiny-invariant 1.3.3`, `tinybench 6.1.4`, `tinyexec 0.3.2, 1.3.1`, `tinyglobby 0.2.17`, `tinyrainbow 2.0.0, 3.1.1`, `tinyspy 4.0.4`, `title 4.0.1`, `tldts 7.0.19`, `tldts-core 7.0.19`, `to-regex-range 5.0.1`, `toidentifier 1.0.1`, `totalist 3.0.1`, `tr46 0.0.3, 6.0.0`, `tree-kill 1.2.2`, `trim-lines 3.0.1`, `trim-trailing-lines 2.1.0`, `trough 2.2.0`, `truncate-json 3.0.1`, `ts-dedent 2.2.0`, `ts-morph 26.0.0, 27.0.2`, `tsconfig-paths 4.2.0`, `tsdown 0.23.0`, `tsup 8.5.1`, `tsx 4.23.15`, `turbo 2.11.7`, `tw-animate-css 1.4.0`, `twoslash 0.3.6`, `twoslash-protocol 0.3.6`, `type-check 0.4.0`, `type-is 2.1.0`, `ufo 1.6.3`, `uint8array-extras 1.5.0`, `unconfig-core 7.5.0`, `undici 7.29.1`, `undici-types 7.16.0, 8.9.0`, `unhead 2.1.13`, `unicorn-magic 0.3.0`, `unified 11.0.5`, `unist-util-find-after 5.0.0`, `unist-util-is 5.2.1, 6.0.1`, `unist-util-modify-children 4.0.0`, `unist-util-position 5.0.0`, `unist-util-position-from-estree 2.0.0`, `unist-util-remove 4.0.0`, `unist-util-remove-position 5.0.0`, `unist-util-stringify-position 4.0.0`, `unist-util-visit 3.1.0, 5.1.0`, `unist-util-visit-children 3.0.0`, `unist-util-visit-parents 4.1.1, 6.0.2`, `universalify 2.0.1`, `unpipe 1.0.0`, `unplugin 2.3.11, 3.3.0`, `until-async 3.0.2`, `update-browserslist-db 1.2.3`, `use-callback-ref 1.3.3`, `use-intl 4.14.9`, `use-sidecar 1.1.3`, `use-sync-external-store 1.6.0`, `util-deprecate 1.0.2`, `uuid 11.1.0`, `vary 1.1.2`, `verkit 0.4.0`, `vfile 6.0.3`, `vfile-location 5.0.3`, `vfile-message 4.0.3`, `vite 8.3.3`, `vitest 5.0.3`, `vscode-jsonrpc 8.2.0`, `vscode-languageserver 9.0.1`, `vscode-languageserver-protocol 3.17.5`, `vscode-languageserver-textdocument 1.0.12`, `vscode-languageserver-types 3.17.5`, `vscode-uri 3.0.8`, `vue 3.5.41`, `vue-component-type-helpers 3.2.7`, `vue-demi 0.14.10`, `vue-sonner 1.3.2`, `w3c-keyname 2.2.8`, `w3c-xmlserializer 5.0.0`, `web-namespaces 2.0.1`, `webpack-virtual-modules 0.6.2`, `whatwg-mimetype 5.0.0`, `whatwg-url 5.0.0, 16.0.1`, `when-exit 2.1.5`, `why-is-node-running 3.2.1`, `wicked-good-xpath 1.3.0`, `word-wrap 1.2.5`, `wrap-ansi 7.0.0, 9.0.2`, `ws 8.21.3`, `wsl-utils 0.1.0, 0.3.1`, `xmlchars 2.2.0`, `xtend 4.0.2`, `yargs 16.2.2, 17.7.2, 18.2.0`, `yocto-queue 0.1.0`, `yocto-spinner 1.2.2`, `yoctocolors 2.1.2`, `yoga-layout 3.2.1`, `yuku-ast 0.9.5`, `yuku-codegen 0.9.5`, `yuku-parser 0.9.5`, `zod 3.25.76, 4.3.5, 4.6.5`, `zod-validation-error 4.0.2`, `zustand 5.0.10`, `zwitch 2.0.4`

**ISC** (70)

`@iarna/toml 2.2.5`, `@shaderfrog/glsl-parser 7.0.1`, `@ungap/structured-clone 1.3.0`, `ansis 4.4.0`, `cli-width 4.1.0`, `cliui 7.0.4, 8.0.1, 9.0.1`, `d3 7.9.0`, `d3-array 3.2.4`, `d3-axis 3.0.0`, `d3-brush 3.0.0`, `d3-chord 3.0.1`, `d3-color 3.1.0`, `d3-contour 4.0.2`, `d3-delaunay 6.0.4`, `d3-dispatch 3.0.1`, `d3-drag 3.0.0`, `d3-dsv 3.0.1`, `d3-fetch 3.0.1`, `d3-force 3.0.0`, `d3-format 3.1.2`, `d3-geo 3.1.1`, `d3-hierarchy 3.1.2`, `d3-interpolate 3.0.1`, `d3-path 3.1.0`, `d3-polygon 3.0.1`, `d3-quadtree 3.0.1`, `d3-random 3.0.1`, `d3-scale 4.0.2`, `d3-scale-chromatic 3.1.0`, `d3-selection 3.0.0`, `d3-shape 3.2.0`, `d3-time 3.1.0`, `d3-time-format 4.1.0`, `d3-timer 3.0.1`, `d3-transition 3.0.1`, `d3-zoom 3.0.0`, `delaunator 5.0.1`, `electron-to-chromium 1.5.361`, `fastq 1.20.1`, `flatted 3.4.4`, `get-caller-file 2.0.5`, `github-slugger 2.0.0`, `glob-parent 5.1.2, 6.0.2`, `graceful-fs 4.2.11`, `hast-util-from-dom 5.0.1`, `inherits 2.0.4`, `internmap 1.0.1, 2.0.3`, `isexe 2.0.0`, `lru-cache 5.1.1`, `lucide-react 1.52.0`, `mute-stream 3.0.0`, `once 1.4.0`, `parse-numeric-range 1.3.0`, `pegjs-backtrace 0.2.1`, `picocolors 1.1.1`, `read-vinyl-file-stream 2.0.3`, `remark-reading-time 2.0.2`, `saxes 6.0.0`, `semver 6.3.1, 7.8.5`, `setprototypeof 1.2.0`, `signal-exit 3.0.7, 4.1.0`, `split2 2.2.0`, `validate-npm-package-name 7.0.2`, `which 2.0.2, 4.0.0`, `wrappy 1.0.2`, `y18n 5.0.8`, `yallist 3.1.1`, `yaml 2.9.0`, `yargs-parser 20.2.9, 21.1.1, 22.0.0`, `zod-to-json-schema 3.25.2`

**Apache-2.0** (68)

`@ai-sdk/gateway 3.0.13, 4.0.103`, `@ai-sdk/provider 3.0.2, 4.0.21`, `@ai-sdk/provider-utils 4.0.5, 5.0.53`, `@ai-sdk/vue 3.0.33`, `@chevrotain/cst-dts-gen 11.0.3`, `@chevrotain/gast 11.0.3`, `@chevrotain/regexp-to-ast 11.0.3`, `@chevrotain/types 11.0.3`, `@chevrotain/utils 11.0.3`, `@eslint/config-array 0.23.5`, `@eslint/config-helpers 0.7.0`, `@eslint/core 1.2.1`, `@eslint/object-schema 3.0.5`, `@eslint/plugin-kit 0.7.2`, `@humanfs/core 0.19.2`, `@humanfs/node 0.16.8`, `@humanfs/types 0.15.0`, `@humanwhocodes/module-importer 1.0.1`, `@humanwhocodes/retry 0.4.3`, `@img/sharp-linux-x64 0.35.4`, `@internationalized/date 3.12.1`, `@internationalized/number 3.6.6`, `@opentelemetry/api 1.9.0, 1.9.1`, `@opentelemetry/api-logs 0.220.0`, `@opentelemetry/core 2.9.0`, `@opentelemetry/instrumentation 0.220.0`, `@opentelemetry/resources 2.9.0`, `@opentelemetry/sdk-trace 2.9.0`, `@opentelemetry/sdk-trace-base 2.9.0`, `@opentelemetry/semantic-conventions 1.40.0`, `@react-aria/focus 3.21.3`, `@react-aria/interactions 3.26.0`, `@react-aria/ssr 3.9.10`, `@react-aria/utils 3.32.0`, `@react-stately/flags 3.1.2`, `@react-stately/utils 3.11.0`, `@react-types/shared 3.32.1`, `@swc/core 1.16.2`, `@swc/counter 0.1.3`, `@swc/helpers 0.5.15, 0.5.23`, `@swc/types 0.1.28`, `@vercel/oidc 3.1.0, 3.2.0`, `@workflow/serde 4.1.0`, `ai 6.0.33, 7.0.127`, `aria-query 5.3.0, 5.3.2`, `baseline-browser-mapping 2.10.21`, `chevrotain 11.0.3`, `class-variance-authority 0.7.1`, `cva 1.0.0-beta.4`, `detect-libc 2.1.2`, `doctrine 3.0.0`, `eslint-visitor-keys 3.4.3, 5.0.1`, `expect-type 1.4.0`, `fuse.js 7.5.0`, `human-signals 2.1.0, 5.0.0, 8.0.1`, `import-in-the-middle 3.0.1`, `mathjax-full 3.2.2`, `mhchemparser 4.2.1`, `mj-context-menu 0.6.1`, `plantuml-parser 0.4.0`, `playwright 1.63.0`, `playwright-core 1.63.0`, `sharp 0.35.4`, `speech-rule-engine 4.1.2`, `swrv 1.2.0`, `ts-interface-checker 0.1.13`, `typescript 5.9.3, 6.0.3`, `xml-name-validator 5.0.0`

**BSD-3-Clause** (21)

`@dotenvx/dotenvx 1.75.1`, `@dotenvx/primitives 0.8.0`, `d3-array 2.12.1`, `d3-ease 3.0.1`, `d3-path 1.0.9`, `d3-sankey 0.12.3`, `d3-shape 1.3.7`, `diff 8.0.4`, `duplexer2 0.1.4`, `esquery 1.7.0`, `fast-uri 3.1.2`, `highlight.js 10.7.3, 11.11.1`, `intl-messageformat 11.2.13`, `js-base64 3.9.3`, `qs 6.15.2`, `react-medium-image-zoom 5.4.0`, `rw 1.3.3`, `source-map 0.6.1, 0.7.6`, `source-map-js 1.2.1`, `sprintf-js 1.0.3`, `tough-cookie 6.0.1, 6.0.2`

**BSD-2-Clause** (12)

`dotenv 17.4.2`, `entities 6.0.1, 7.0.1, 8.0.0`, `eslint-scope 9.1.2`, `espree 11.2.0`, `esprima 4.0.1`, `esrecurse 4.3.0`, `estraverse 5.3.0`, `esutils 2.0.3`, `json-schema-typed 7.0.3, 8.0.2`, `stringify-object 5.0.0`, `uri-js 4.4.1`, `webidl-conversions 3.0.1, 8.0.1`

**BlueOak-1.0.0** (6)

`glob 13.0.6`, `isexe 3.1.5`, `lru-cache 11.5.2`, `minimatch 10.2.5`, `minipass 7.1.3`, `path-scurry 2.0.2`

**MPL-2.0** (3)

`axe-core 4.13.0`, `lightningcss 1.32.0, 1.33.0`, `lightningcss-linux-x64-gnu 1.32.0, 1.33.0`

**OFL-1.1** (3)

`@fontsource-variable/geist 5.3.0`, `@fontsource-variable/geist-mono 5.3.0`, `platformicons 9.8.0`

**CC0-1.0** (2)

`highlightjs-vue 1.0.0`, `mdn-data 2.27.1`

**FSL-1.1-MIT** (2)

`@sentry/cli 2.58.6`, `@sentry/cli-linux-x64 2.58.6`

**MIT OR Apache-2.0** (2)

`@biomejs/biome 2.5.15`, `@biomejs/cli-linux-x64 2.5.15`

**MIT-0** (2)

`@csstools/color-helpers 6.1.0`, `@csstools/css-syntax-patches-for-csstree 1.1.7`

**Unlicense** (2)

`isbot 5.2.2`, `robust-predicates 3.0.2`

**(AFL-2.1 OR BSD-3-Clause)** (1)

`json-schema 0.4.0`

**(MIT OR CC0-1.0)** (1)

`type-fest 0.13.1, 4.41.0, 5.8.0`

**(MPL-2.0 OR Apache-2.0)** (1)

`dompurify 3.3.1`

**0BSD** (1)

`tslib 2.8.1`

**Apache-2.0 AND MIT** (1)

`@swc/core-linux-x64-gnu 1.16.2`

**CC-BY-4.0** (1)

`caniuse-lite 1.0.30001793`

**FSL-1.1-Apache-2.0** (1)

`sentry 0.45.0`

**LGPL-3.0-or-later** (1)

`@img/sharp-libvips-linux-x64 1.3.3`

**MIT AND ISC** (1)

`victory-vendor 37.3.6`

**Python-2.0** (1)

`argparse 2.0.1`

**Unknown** (1)

`khroma 2.1.0`
