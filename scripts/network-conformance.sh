#!/usr/bin/env bash
# Network conformance for the BugLenz fork (CONSTITUTION I3, ADR-0004).
#
# The instance may open connections only to the database, the configured OIDC
# issuer, the configured SMTP server and the configured alert channels. This
# script checks that nothing else is even attempted, in two layers:
#
#   static   tokens from scripts/egress-denylist.txt must not appear in the
#            source, in the compiled server binary or in the dashboard bundle
#   runtime  the server runs as a dedicated user whose outbound traffic is
#            logged and rejected by iptables; it is exercised over HTTP for a
#            window and must not have tried a single connection
#
# Usage:
#   scripts/network-conformance.sh static [SERVER_BINARY] [DASHBOARD_DIST]
#   sudo -E scripts/network-conformance.sh runtime SERVER_BINARY
#
# Runtime knobs (environment): WINDOW_SECONDS (default 900), PORT (8080),
# CT_USER (rustrak-ct), CT_DIR (a fresh temp dir).
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
DENYLIST="$ROOT/scripts/egress-denylist.txt"

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*"; }
fail() { log "FAIL: $*"; exit 1; }

tokens() { grep -vE '^\s*(#|$)' "$DENYLIST"; }

# ---------------------------------------------------------------------------
# static
# ---------------------------------------------------------------------------
static_check() {
  local binary="${1:-}" dist="${2:-}" bad=0
  log "static: scanning sources against $(tokens | wc -l | tr -d ' ') denylisted tokens"
  # Documentation of the upstream is not shipped by the instance and keeps
  # describing the removed feature; governance explains the removal.
  local paths=(apps/server/src apps/server/tests apps/server/Dockerfile apps/server/.env.example
    apps/dashboard/src apps/dashboard/index.html packages/client/src packages/mcp/src
    packages/ui/src docker-compose.yml docker-compose.postgres.yml docker-compose.dev.yml
    .github/workflows)
  local existing=()
  for p in "${paths[@]}"; do [ -e "$ROOT/$p" ] && existing+=("$ROOT/$p"); done
  while IFS= read -r token; do
    if grep -rIFn --exclude='network-conformance.yml' -- "$token" "${existing[@]}" 2>/dev/null; then
      log "static: token '$token' found in sources"
      bad=1
    fi
  done < <(tokens)

  if [ -n "$binary" ]; then
    [ -f "$binary" ] || fail "static: binary not found: $binary"
    log "static: scanning binary $binary"
    if strings "$binary" | grep -E 'posthog|versions\.json' | head -5; then
      log "static: binary carries a denylisted host"
      bad=1
    fi
  fi

  if [ -n "$dist" ]; then
    [ -d "$dist" ] || fail "static: dashboard dist not found: $dist"
    log "static: scanning dashboard bundle $dist"
    if grep -rIlE 'posthog|versions\.json' "$dist" | head -5; then
      log "static: dashboard bundle carries a denylisted host"
      bad=1
    fi
  fi

  [ "$bad" -eq 0 ] || fail "static layer"
  log "static: clean"
}

# ---------------------------------------------------------------------------
# runtime
# ---------------------------------------------------------------------------
CT_USER="${CT_USER:-rustrak-ct}"
PORT="${PORT:-8080}"
WINDOW_SECONDS="${WINDOW_SECONDS:-900}"
BASE="http://127.0.0.1:$PORT"
ADMIN_EMAIL="ct-admin@example.com"
ADMIN_PASSWORD="conformance-password-123"

egress_count() { dmesg 2>/dev/null | grep -c 'EGRESS ' || true; }

install_rules() {
  local uid="$1"
  for ipt in iptables ip6tables; do
    # Order matters: DNS over loopback is logged and rejected before the
    # loopback accept, because a name lookup is the first sign of a
    # connection that should not exist (systemd-resolved lives on lo).
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -j REJECT
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -j LOG --log-prefix "EGRESS " --log-level 4
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -o lo -j ACCEPT
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -o lo -p tcp --dport 53 -j REJECT
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -o lo -p tcp --dport 53 -j LOG --log-prefix "EGRESS " --log-level 4
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -o lo -p udp --dport 53 -j REJECT
    "$ipt" -I OUTPUT 1 -m owner --uid-owner "$uid" -o lo -p udp --dport 53 -j LOG --log-prefix "EGRESS " --log-level 4
  done
}

self_test() {
  # The rules must bite before anything else is trusted: one attempt, one line.
  dmesg -C
  if sudo -u "$CT_USER" curl -sS -m 5 -o /dev/null https://example.com 2>/dev/null; then
    fail "self-test: the blocked user reached the internet"
  fi
  local n; n=$(egress_count)
  [ "$n" -ge 1 ] || fail "self-test: the blocked attempt left no EGRESS log line (got $n)"
  log "self-test: blocked and logged ($n line(s))"
}

json_field() { python3 -c 'import json,sys; d=json.load(sys.stdin); print(d'"$1"')'; }

send_envelope() {
  # $1 = project id, $2 = sentry key, $3 = kind (event|session)
  local pid="$1" key="$2" kind="$3" now id payload item env_file
  now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  id=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
  if [ "$kind" = event ]; then
    payload=$(printf '{"event_id":"%s","timestamp":"%s","platform":"other","level":"error","release":"ct@1.0.0","environment":"conformance","exception":{"values":[{"type":"ConformanceError","value":"network conformance probe"}]}}' "$id" "$now")
    item=$(printf '{"type":"event","length":%d}' "${#payload}")
    env_file=$(printf '{"event_id":"%s","sent_at":"%s"}\n%s\n%s\n' "$id" "$now" "$item" "$payload")
  else
    payload=$(printf '{"sid":"%s","did":"ct","init":true,"started":"%s","timestamp":"%s","status":"ok","errors":0,"attrs":{"release":"ct@1.0.0","environment":"conformance"}}' "$id" "$now" "$now")
    item=$(printf '{"type":"session","length":%d}' "${#payload}")
    env_file=$(printf '{"sent_at":"%s"}\n%s\n%s\n' "$now" "$item" "$payload")
  fi
  curl -sS -o /dev/null -w '%{http_code}' -X POST "$BASE/api/$pid/envelope/" \
    -H "X-Sentry-Auth: Sentry sentry_version=7, sentry_key=$key, sentry_client=buglenz-ct/1.0" \
    -H 'Content-Type: application/x-sentry-envelope' --data-binary "$env_file"
}

runtime_check() {
  local binary="${1:-}"
  [ -n "$binary" ] && [ -x "$binary" ] || fail "runtime: server binary required (got '$binary')"
  [ "$(id -u)" -eq 0 ] || fail "runtime: must run as root (iptables, dmesg, su)"
  command -v iptables >/dev/null || fail "runtime: iptables missing"
  command -v python3 >/dev/null || fail "runtime: python3 missing"

  id "$CT_USER" >/dev/null 2>&1 || useradd --system --no-create-home --shell /usr/sbin/nologin "$CT_USER"
  local uid; uid=$(id -u "$CT_USER")
  CT_DIR="${CT_DIR:-$(mktemp -d /tmp/buglenz-ct.XXXXXX)}"
  mkdir -p "$CT_DIR/ingest" "$CT_DIR/sourcemaps"
  cp "$binary" "$CT_DIR/rustrak"
  chown -R "$CT_USER" "$CT_DIR"
  log "runtime: user $CT_USER (uid $uid), dir $CT_DIR, window ${WINDOW_SECONDS}s"

  install_rules "$uid"
  self_test
  dmesg -C

  local secret; secret=$(python3 -c 'import secrets; print(secrets.token_hex(32))')
  sudo -u "$CT_USER" env \
    HOST=127.0.0.1 PORT="$PORT" \
    DATABASE_URL="sqlite://$CT_DIR/rustrak.db?mode=rwc" \
    INGEST_DIR="$CT_DIR/ingest" SOURCEMAP_STORAGE_PATH="$CT_DIR/sourcemaps" \
    SESSION_SECRET_KEY="$secret" CREATE_SUPERUSER="$ADMIN_EMAIL:$ADMIN_PASSWORD" \
    RUSTRAK_DASHBOARD=off RUSTRAK_METRICS=on RUSTRAK_TELEMETRY=off DO_NOT_TRACK=1 \
    RUST_LOG=info \
    "$CT_DIR/rustrak" >"$CT_DIR/server.log" 2>&1 &
  local server_pid=$!
  trap 'kill "$server_pid" 2>/dev/null || true' EXIT

  local i
  for i in $(seq 1 60); do
    curl -sf -o /dev/null "$BASE/health" && break
    kill -0 "$server_pid" 2>/dev/null || { cat "$CT_DIR/server.log"; fail "runtime: server died during startup"; }
    sleep 1
  done
  curl -sf -o /dev/null "$BASE/health" || { cat "$CT_DIR/server.log"; fail "runtime: server not healthy after 60s"; }
  log "runtime: server up"

  local jar="$CT_DIR/cookies"
  local code
  code=$(curl -sS -o /dev/null -w '%{http_code}' -c "$jar" -H 'Content-Type: application/json' \
    -d "{\"email\":\"$ADMIN_EMAIL\",\"password\":\"$ADMIN_PASSWORD\"}" "$BASE/auth/login")
  [ "$code" = 200 ] || fail "runtime: login answered $code"

  local project pid key
  project=$(curl -sS -b "$jar" -H 'Content-Type: application/json' -d '{"name":"conformance"}' "$BASE/api/projects")
  pid=$(printf '%s' "$project" | json_field '["id"]')
  key=$(printf '%s' "$project" | json_field '["sentry_key"]' | tr -d '-')
  [ -n "$pid" ] && [ -n "$key" ] || fail "runtime: project creation returned no id/key: $project"
  log "runtime: project $pid"

  code=$(send_envelope "$pid" "$key" event);   [ "$code" = 200 ] || fail "runtime: event envelope answered $code"
  code=$(send_envelope "$pid" "$key" session); [ "$code" = 200 ] || fail "runtime: session envelope answered $code"
  log "runtime: envelopes accepted"

  code=$(curl -sS -o /dev/null -w '%{http_code}' -b "$jar" "$BASE/api/telemetry/preview")
  [ "$code" = 404 ] || fail "runtime: /api/telemetry/preview answered $code, expected 404"

  sleep 2
  curl -sf "$BASE/metrics" | grep -q 'rustrak_ingest_accepted_total' || fail "runtime: /metrics lacks rustrak_ingest_accepted_total"
  log "runtime: /metrics serves counters; preview route is gone"

  local deadline=$(( $(date +%s) + WINDOW_SECONDS ))
  while [ "$(date +%s)" -lt "$deadline" ]; do
    sleep 60
    [ "$(date +%s)" -lt "$deadline" ] || break
    kill -0 "$server_pid" 2>/dev/null || { cat "$CT_DIR/server.log"; fail "runtime: server died during the window"; }
    code=$(send_envelope "$pid" "$key" event) || true
    log "runtime: probe event -> $code, EGRESS lines so far: $(egress_count)"
  done

  local n; n=$(egress_count)
  kill -0 "$server_pid" 2>/dev/null || fail "runtime: server is not alive at the end of the window"
  if grep -qi 'telemetry' "$CT_DIR/server.log"; then
    grep -i 'telemetry' "$CT_DIR/server.log" | head -3
    fail "runtime: the server log mentions telemetry"
  fi
  if [ "$n" -ne 0 ]; then
    dmesg | grep 'EGRESS ' | head -20
    fail "runtime: $n outbound attempt(s) logged"
  fi
  log "runtime: clean — no outbound attempt in ${WINDOW_SECONDS}s"
}

case "${1:-}" in
  static)  shift; static_check "$@" ;;
  runtime) shift; runtime_check "$@" ;;
  *) echo "usage: $0 static [BINARY] [DIST] | runtime BINARY" >&2; exit 2 ;;
esac
