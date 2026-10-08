#!/usr/bin/env bash
# First-run provisioning of a BugLenz instance, over its API (ADR-0012).
#
# The upstream's RUSTRAK_BOOTSTRAP_TOKEN ignores its value and prints a random
# token to stderr (upstream issue #356), which no automation can read back
# from a Swarm service. This script does what an operator would do by hand:
#
#   1. log in as the superuser that CREATE_SUPERUSER made on the first boot
#   2. create an API token for automation (POST /api/tokens)
#   3. optionally create the first project and print its DSN
#
# Usage:
#   BUGLENZ_URL=https://errors.example.com \
#   BUGLENZ_ADMIN_EMAIL=admin@example.com BUGLENZ_ADMIN_PASSWORD=... \
#   deploy/swarm/provision.sh [project-name]
#
# Prints the token once. Store it in the vault; remove CREATE_SUPERUSER from
# the stack environment afterwards.
set -euo pipefail

: "${BUGLENZ_URL:?instance URL, e.g. https://errors.example.com}"
: "${BUGLENZ_ADMIN_EMAIL:?the CREATE_SUPERUSER email}"
: "${BUGLENZ_ADMIN_PASSWORD:?the CREATE_SUPERUSER password}"
PROJECT="${1:-}"

jar=$(mktemp)
trap 'rm -f "$jar"' EXIT

json() { python3 -c 'import json,sys; d=json.load(sys.stdin); print(d'"$1"')'; }

for i in $(seq 1 30); do
  curl -sf -o /dev/null "$BUGLENZ_URL/health" && break
  sleep 2
done
curl -sf -o /dev/null "$BUGLENZ_URL/health" || { echo "provision: $BUGLENZ_URL/health not answering" >&2; exit 1; }

code=$(curl -sS -o /dev/null -w '%{http_code}' -c "$jar" -H 'Content-Type: application/json' \
  -d "$(printf '{"email":"%s","password":"%s"}' "$BUGLENZ_ADMIN_EMAIL" "$BUGLENZ_ADMIN_PASSWORD")" \
  "$BUGLENZ_URL/auth/login")
[ "$code" = 200 ] || { echo "provision: login answered $code" >&2; exit 1; }

token=$(curl -sS -b "$jar" -H 'Content-Type: application/json' \
  -d '{"description":"automation (provision.sh)"}' "$BUGLENZ_URL/api/tokens" | json '["token"]')
[ -n "$token" ] || { echo "provision: no token in the response" >&2; exit 1; }
echo "BUGLENZ_API_TOKEN=$token"

if [ -n "$PROJECT" ]; then
  project=$(curl -sS -b "$jar" -H 'Content-Type: application/json' \
    -d "$(printf '{"name":"%s"}' "$PROJECT")" "$BUGLENZ_URL/api/projects")
  echo "PROJECT_ID=$(printf '%s' "$project" | json '["id"]')"
  echo "DSN=$(printf '%s' "$project" | json '["dsn"]')"
fi
