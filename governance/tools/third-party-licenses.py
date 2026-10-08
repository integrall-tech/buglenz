#!/usr/bin/env python3
"""Gera THIRD-PARTY-LICENSES.md a partir de `cargo deny list` e `pnpm licenses list`.

Uso, na raiz do repositório:

    (cd apps/server && cargo deny list -f json -l crate) > /tmp/deny-server.json
    (cd packages/benchmarks && cargo deny list -f json -l crate) > /tmp/deny-bench.json
    corepack pnpm -r licenses list --json --long > /tmp/pnpm-licenses.json   # -r: todo o workspace (pnpm >= 12.10 lista só a raiz sem ele)
    python3 -I governance/tools/third-party-licenses.py \
        /tmp/deny-server.json /tmp/deny-bench.json /tmp/pnpm-licenses.json > THIRD-PARTY-LICENSES.md

Pacotes do próprio repositório (`rustrak`, `rustrak-benchmarks`, `@rustrak/*`) ficam fora
da lista: a licença deles está em LICENSE e NOTICE.md.
"""

import json
import subprocess
import sys
from collections import defaultdict
from datetime import date

LOCKFILES = ["apps/server/Cargo.lock", "packages/benchmarks/Cargo.lock", "pnpm-lock.yaml"]


def git(*args: str) -> str | None:
    try:
        out = subprocess.run(["git", *args], capture_output=True, text=True, check=True).stdout.strip()
        return out or None
    except (OSError, subprocess.CalledProcessError):
        return None


def inventory_date() -> str:
    """Data do último commit que tocou um lockfile: determinística para o mesmo tree, para
    que a CI possa comparar o arquivo gerado com o commitado."""
    return git("log", "-1", "--format=%cs", "--", *LOCKFILES) or date.today().isoformat()


def upstream_base() -> str:
    """Tag estável do upstream mais próxima (`vX.Y.Z`, sem o sufixo `-itl.N` do fork)."""
    return git("describe", "--tags", "--abbrev=0", "--match", "v[0-9]*", "--exclude", "*-itl*") or "(desconhecida)"

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
        "`pnpm -r licenses list` (JavaScript); o cabeçalho do script tem os comandos. Este arquivo é",
        "regenerado a cada sincronização com o upstream.",
        "",
        f"Data: {inventory_date()} (último commit dos lockfiles). Base: Rustrak `{upstream_base()}`.",
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
