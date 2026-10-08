#!/usr/bin/env bash
# Assertions of the React end-to-end check (BugLenz fork; ADR-0005 step 6,
# OpenSpec 006), run after e2e/react-app/run.mjs fired its four errors.
#
#   BASE=http://127.0.0.1:8080 TOKEN=<api token> PROJECT_ID=1 scripts/e2e-react-assert.sh
#
# Checks, against the server's API:
#   - 3 issues; the click-handler one has event_count 2
#   - the latest event's application frame is source-mapped: original file,
#     original line, context line present
#   - release health counts the `unhandled` session as errored, not crashed
# Writes the function-name (G7) and in_app (G8) readings to the step summary
# as measurements for package 010; they never fail the run.
set -euo pipefail

: "${BASE:?server url}" "${TOKEN:?api token}" "${PROJECT_ID:?project id}"
SUMMARY="${GITHUB_STEP_SUMMARY:-/dev/null}"
api() { curl -sS -f -H "Authorization: Bearer $TOKEN" "$BASE$1"; }
log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*"; }
fail() { log "FAIL: $*"; exit 1; }

# The session aggregator flushes on an interval; wait for release health to
# show the session before judging it.
for i in $(seq 1 30); do
  if api "/api/projects/$PROJECT_ID/sessions/stats?period=24h" | python3 -c '
import json,sys
d=json.load(sys.stdin)
rows=d if isinstance(d,list) else d.get("items") or d.get("releases") or d.get("data") or [d]
sys.exit(0 if any((r.get("total") or 0)>0 for r in rows if isinstance(r,dict)) else 1)
' 2>/dev/null; then break; fi
  sleep 2
done

# Digest is asynchronous: give the last event a moment to be grouped.
for i in $(seq 1 20); do
  issues_json=$(api "/api/projects/$PROJECT_ID/issues?per_page=50")
  printf '%s' "$issues_json" | python3 -c '
import json,sys
d=json.load(sys.stdin); items=d if isinstance(d,list) else d.get("items",[])
sys.exit(0 if len(items)>=3 and sum(int(i.get("event_count",0)) for i in items)>=4 else 1)' 2>/dev/null && break
  sleep 1
done
read -r issue_count click_issue click_count < <(printf '%s' "$issues_json" | python3 -c '
import json,sys
d=json.load(sys.stdin)
items=d if isinstance(d,list) else d.get("items",[])
click=[i for i in items if "pedido sem itens" in json.dumps(i, ensure_ascii=False)]
print(len(items), click[0]["id"] if click else "-", click[0].get("event_count",0) if click else 0)
')
log "issues: $issue_count (click issue $click_issue, events $click_count)"
[ "$issue_count" = 3 ] || fail "expected 3 issues, got $issue_count"
[ "$click_count" = 2 ] || fail "the click-handler issue should have 2 events, got $click_count"

event_id=$(api "/api/projects/$PROJECT_ID/issues/$click_issue/events?per_page=1" | python3 -c '
import json,sys
d=json.load(sys.stdin); items=d if isinstance(d,list) else d.get("items",[])
print(items[0]["id"] if items else "")')
[ -n "$event_id" ] || fail "no event for the click issue"
# The detail route takes the server's own event id (`id`), not the SDK's `event_id`.
event_file=$(mktemp)
api "/api/projects/$PROJECT_ID/issues/$click_issue/events/$event_id" > "$event_file"

python3 - "$SUMMARY" "$event_file" <<'EOF'
import json, sys
d = json.load(open(sys.argv[2]))
summary = open(sys.argv[1], "a")

def frames(obj):
    if isinstance(obj, dict):
        if "frames" in obj and isinstance(obj["frames"], list):
            yield from obj["frames"]
        for v in obj.values():
            yield from frames(v)
    elif isinstance(obj, list):
        for v in obj:
            yield from frames(v)

fs = list(frames(d))
if not fs:
    print("FAIL: no stack frames in the event"); sys.exit(1)
app = [f for f in fs if str(f.get("filename") or f.get("abs_path") or "").endswith("pedido.ts")]
if not app:
    print("FAIL: no frame resolved to src/pedido.ts; frames:",
          [str(f.get("filename") or f.get("abs_path")) for f in fs][:8]); sys.exit(1)
f = app[-1]
line = f.get("lineno")
ctx = f.get("context_line")
print(f"source-mapped frame: {f.get('filename')}:{line} function={f.get('function')} context={ctx!r}")
if line != 12:
    print(f"FAIL: expected the throw at pedido.ts:12, got line {line}"); sys.exit(1)
if not ctx or "TypeError" not in ctx:
    print(f"FAIL: context_line missing or wrong: {ctx!r}"); sys.exit(1)

# Personal data (package 005, ADR-0009): the id survives, the e-mail and
# the denied extra do not, and the document in the message is masked.
user = d.get("data", {}).get("user") or d.get("user") or {}
if user.get("id") != "u-1":
    print(f"FAIL: user.id should be kept, got {user!r}"); sys.exit(1)
if user.get("email") != "[email]":
    print(f"FAIL: user.email should be masked, got {user.get('email')!r}"); sys.exit(1)
extra = d.get("data", {}).get("extra") or d.get("extra") or {}
if extra.get("password") != "[Filtered]":
    print(f"FAIL: extra.password should be filtered, got {extra.get('password')!r}"); sys.exit(1)
blob = json.dumps(d, ensure_ascii=False)
for leaked in ("ana@example.com", "hunter2", "529.982.247-25", "52998224725"):
    if leaked in blob:
        print(f"FAIL: {leaked!r} is still in the stored event"); sys.exit(1)
if "[cpf]" not in blob:
    print("FAIL: the document in the message was not masked to [cpf]"); sys.exit(1)
print("personal data: user.id kept, e-mail masked, extra filtered, cpf masked")

# Measurements, not assertions (packages 010: G7 function names, G8 in_app).
nm = [f for f in fs if "node_modules" in str(f.get("abs_path") or f.get("filename") or "")]
nm_in_app = sorted({str(f.get("in_app")) for f in nm})
summary.write("### e2e-react measurements (G7/G8, package 010)\n\n")
summary.write(f"- application frame: `{f.get('filename')}:{line}` function `{f.get('function')}` (expected `calcularTotalPedido`)\n")
summary.write(f"- frames under node_modules: {len(nm)}, in_app values: {nm_in_app or ['(none)']}\n")
summary.write(f"- culprit: `{(d.get('data') or {}).get('culprit')}`\n\n")
EOF

stats_file=$(mktemp)
api "/api/projects/$PROJECT_ID/sessions/stats?period=24h" > "$stats_file"
python3 - "$SUMMARY" "$stats_file" <<'EOF'
import json, sys
d = json.load(open(sys.argv[2]))
rows = d if isinstance(d, list) else d.get("items") or d.get("releases") or d.get("data") or [d]
rows = [r for r in rows if isinstance(r, dict) and "errored" in r]
if not rows:
    print("FAIL: no release health rows:", json.dumps(d)[:300]); sys.exit(1)
errored = sum(int(r.get("errored") or 0) for r in rows)
crashed = sum(int(r.get("crashed") or 0) for r in rows)
total = sum(int(r.get("total") or 0) for r in rows)
print(f"release health: total={total} errored={errored} crashed={crashed}")
open(sys.argv[1], "a").write(f"- release health: total {total}, errored {errored}, crashed {crashed}\n")
if errored < 1:
    print("FAIL: the unhandled session was not counted as errored"); sys.exit(1)
if crashed != 0:
    print("FAIL: an unhandled session must not count as a crash"); sys.exit(1)
EOF
log "e2e-react: all assertions passed"
