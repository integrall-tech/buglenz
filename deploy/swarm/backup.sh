#!/usr/bin/env bash
# Backup and restore of a BugLenz stack on Swarm (ADR-0012).
#
# Two things hold state: the PostgreSQL database and the /data volume (source
# maps, the ingest spool). Both are taken from the node that runs the
# containers; run this there, or point DOCKER_HOST at it.
#
#   deploy/swarm/backup.sh backup  [DEST_DIR]            -> DEST_DIR/buglenz-<stamp>.sql.gz and -data.tar.gz
#   deploy/swarm/backup.sh restore DUMP.sql.gz DATA.tar.gz
#
# STACK (default buglenz) names the stack; the service and volume names
# follow Swarm's <stack>_<name> convention. How BackupLenz picks these files
# up is still to be defined (see governance/openspec/003-build-e-implantacao).
set -euo pipefail

STACK="${STACK:-buglenz}"
PG_SERVICE="${STACK}_postgres"
SERVER_SERVICE="${STACK}_server"
DATA_VOLUME="${STACK}_buglenz_data"

container_of() {
  docker ps --filter "label=com.docker.swarm.service.name=$1" --format '{{.ID}}' | head -1
}

backup() {
  local dest="${1:-.}" stamp pg
  stamp=$(date -u +%Y%m%dT%H%M%SZ)
  mkdir -p "$dest"
  pg=$(container_of "$PG_SERVICE"); [ -n "$pg" ] || { echo "backup: no running container for $PG_SERVICE" >&2; exit 1; }
  docker exec "$pg" pg_dump -U buglenz -d buglenz --no-owner | gzip -9 > "$dest/buglenz-$stamp.sql.gz"
  docker run --rm -v "$DATA_VOLUME:/data:ro" -v "$(cd "$dest" && pwd):/out" alpine:3.20 \
    tar -C /data -czf "/out/buglenz-$stamp-data.tar.gz" .
  ls -la "$dest/buglenz-$stamp.sql.gz" "$dest/buglenz-$stamp-data.tar.gz"
}

restore() {
  local dump="$1" data="$2" pg
  [ -f "$dump" ] && [ -f "$data" ] || { echo "restore: need DUMP.sql.gz and DATA.tar.gz" >&2; exit 2; }
  echo "restore: stopping $SERVER_SERVICE"
  docker service scale "$SERVER_SERVICE=0" >/dev/null
  pg=$(container_of "$PG_SERVICE"); [ -n "$pg" ] || { echo "restore: no running container for $PG_SERVICE" >&2; exit 1; }
  docker exec "$pg" psql -U buglenz -d postgres -v ON_ERROR_STOP=1 \
    -c "DROP DATABASE IF EXISTS buglenz;" -c "CREATE DATABASE buglenz OWNER buglenz;"
  gunzip -c "$dump" | docker exec -i "$pg" psql -U buglenz -d buglenz -v ON_ERROR_STOP=1 -q
  docker run --rm -v "$DATA_VOLUME:/data" -v "$(cd "$(dirname "$data")" && pwd):/in:ro" alpine:3.20 \
    sh -c "rm -rf /data/* && tar -C /data -xzf /in/$(basename "$data")"
  docker service scale "$SERVER_SERVICE=1" >/dev/null
  echo "restore: done; the server is starting"
}

case "${1:-}" in
  backup)  shift; backup "$@" ;;
  restore) shift; restore "$@" ;;
  *) echo "usage: $0 backup [DEST_DIR] | restore DUMP.sql.gz DATA.tar.gz" >&2; exit 2 ;;
esac
