#!/usr/bin/env python3
"""Gera THIRD-PARTY-LICENSES.md a partir de `cargo deny list` e `pnpm licenses list`.

Uso, na raiz do repositório:

    (cd apps/server && cargo deny list -f json -l crate) > /tmp/deny-server.json
    (cd packages/benchmarks && cargo deny list -f json -l crate) > /tmp/deny-bench.json
    corepack pnpm licenses list --json --long > /tmp/pnpm-licenses.json
    python3 -I governance/tools/third-party-licenses.py \
        /tmp/deny-server.json /tmp/deny-bench.json /tmp/pnpm-licenses.json > THIRD-PARTY-LICENSES.md

Pacotes do próprio repositório (`rustrak`, `rustrak-benchmarks`, `@rustrak/*`) ficam fora
da lista: a licença deles está em LICENSE e NOTICE.md.
"""

import json
import sys
from collections import defaultdict
from datetime import date

OWN_CRATES = {"rustrak", "rustrak-benchmarks"}
OWN_NPM_PREFIX = "@rustrak/"


def rust_section(title: str, path: str) -> tuple[list[str], int]:
    data = json.load(open(path))
    by_license: dict[str, list[str]] = defaultdict(list)
    count = 0
    for key, info in data.items():
        name, version, _source = key.split(" ", 2)
        if name in OWN_CRATES:
            continue
        count += 1
        expr = " OR ".join(info["licenses"]) if info["licenses"] else "(sem licença declarada)"
        by_license[expr].append(f"{name} {version}")
    out = [f"### {title}", "", f"{count} crates.", ""]
    for expr in sorted(by_license, key=lambda e: (-len(by_license[e]), e)):
        crates = sorted(by_license[expr])
        out.append(f"**{expr}** ({len(crates)})")
        out.append("")
        out.append(", ".join(f"`{c}`" for c in crates))
        out.append("")
    return out, count


def npm_section(path: str) -> tuple[list[str], int]:
    data = json.load(open(path))
    rows: dict[str, list[str]] = defaultdict(list)
    count = 0
    for expr, pkgs in data.items():
        for p in pkgs:
            if p["name"].startswith(OWN_NPM_PREFIX):
                continue
            count += 1
            versions = ", ".join(p["versions"])
            rows[expr.strip()].append(f"{p['name']} {versions}")
    out = ["### JavaScript (workspace pnpm)", "", f"{count} pacotes (dependências de produção e de desenvolvimento).", ""]
    for expr in sorted(rows, key=lambda e: (-len(rows[e]), e)):
        pkgs = sorted(rows[expr])
        out.append(f"**{expr}** ({len(pkgs)})")
        out.append("")
        out.append(", ".join(f"`{p}`" for p in pkgs))
        out.append("")
    return out, count


def main() -> None:
    server, bench, npm = sys.argv[1:4]
    s_out, s_n = rust_section("Rust: `apps/server` (crate `rustrak`)", server)
    b_out, b_n = rust_section("Rust: `packages/benchmarks` (crate `rustrak-benchmarks`)", bench)
    n_out, n_n = npm_section(npm)
    head = [
        "# Licenças de terceiros",
        "",
        "Inventário das dependências do BugLenz e das licenças que elas declaram. Gerado por",
        "`governance/tools/third-party-licenses.py` a partir de `cargo deny list` (Rust) e",
        "`pnpm licenses list` (JavaScript); o cabeçalho do script tem os comandos. Este arquivo é",
        "regenerado a cada sincronização com o upstream.",
        "",
        f"Data: {date.today().isoformat()}. Base: Rustrak `v0.15.2`.",
        "",
        "A allow-list de licenças que o upstream aceita está em `deny.toml` e é verificada pelo",
        "workflow `rust-security.yml` (`cargo deny check advisories licenses` em `apps/server`).",
        "Não há verificação equivalente para o lado JavaScript no upstream.",
        "",
        "O inventário JavaScript cobre dependências de produção e de desenvolvimento e inclui os",
        "pacotes opcionais da plataforma em que foi gerado (binários de `@sentry/cli`, `sharp`,",
        "`esbuild`, etc.). Em outra plataforma a lista de opcionais muda.",
        "",
        "| Inventário | Pacotes |",
        "|---|---|",
        f"| Rust, `apps/server` | {s_n} |",
        f"| Rust, `packages/benchmarks` | {b_n} |",
        f"| JavaScript, workspace pnpm | {n_n} |",
        "",
        "## Rust",
        "",
    ]
    print("\n".join(head + s_out + b_out + ["## JavaScript", ""] + n_out).rstrip())


if __name__ == "__main__":
    main()
