#!/usr/bin/env python3
"""Checks DELTA-MANIFEST.md against the real divergence from the upstream tag.

    python3 governance/tools/check-manifest.py [BASE_TAG]     # default: the base read from the manifest

Exits 1 when a file that differs from the base is not covered by a manifest row, or when a row names a
file that is no longer a divergence. Rows may use `{a,b}` and `*`/`**` globs.
"""
import re
import subprocess
import sys

base = sys.argv[1] if len(sys.argv) > 1 else None
manifest = open('DELTA-MANIFEST.md', encoding='utf-8').read()
if base is None:
    # "Base atual: tag `vX.Y.Z`" in the manifest header: the tag the divergence is measured from.
    m = re.search(r'Base atual: tag `(v\d+\.\d+\.\d+)`', manifest)
    if not m:
        sys.exit('check-manifest: could not read the base tag from the manifest header ("Base atual: tag `vX.Y.Z`")')
    base = m.group(1)

changed = {}
for line in subprocess.check_output(['git', 'diff', '--name-status', base, 'HEAD'], text=True).splitlines():
    parts = line.split('\t')
    changed[parts[-1]] = parts[0][0]

tokens = set()
for line in manifest.splitlines():
    if line.startswith('|'):
        tokens.update(re.findall(r'`([^`]+)`', line.split('|')[1]))


def expand(t):
    m = re.search(r'\{([^}]*)\}', t)
    if not m:
        return [t]
    return [x for opt in m.group(1).split(',') for x in expand(t[:m.start()] + opt + t[m.end():])]


patterns = [p for t in tokens for p in expand(t) if '/' in p or '.' in p]


def covered(path):
    for p in patterns:
        if p == path or (p.endswith('/') and path.startswith(p)):
            return True
        if '*' in p:
            rx = '^' + re.escape(p).replace(r'\*\*', '.*').replace(r'\*', '[^/]*') + '$'
            if re.match(rx, path):
                return True
    return False


uncovered = sorted(p for p in changed if not covered(p))
stale = sorted(
    p for p in {q for t in tokens for q in expand(t) if '*' not in q and '/' in q and not q.endswith('/')}
    if p not in changed
)
print(f"base {base}: {len(changed)} files differ, {len(changed) - len(uncovered)} covered, {len(uncovered)} not covered")
for p in uncovered:
    print(f"  not in the manifest: {changed[p]} {p}")
print(f"{len(stale)} manifest row(s) name a file that no longer differs")
for p in stale:
    print(f"  stale: {p}")
sys.exit(1 if uncovered or stale else 0)
