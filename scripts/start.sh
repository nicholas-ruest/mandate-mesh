#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/tmp/mandate-mesh-target
mkdir -p evidence/prototype data
MM_BIND=127.0.0.1:18080 MM_DATA="${MM_DATA:-data/mandates.rvf}" MM_TOKEN=demo-token RUST_LOG=info cargo run -p mm-cli >evidence/prototype/backend.log 2>&1 &
echo $! > /tmp/mandate-mesh-backend.pid
(cd frontend && exec ./node_modules/.bin/vite --host 127.0.0.1 --port 5173) >evidence/prototype/frontend.log 2>&1 &
echo $! > /tmp/mandate-mesh-frontend.pid
for _ in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && curl -fsS http://127.0.0.1:5173 >/dev/null 2>&1; then exit 0; fi
  sleep 1
done
echo readiness timeout >&2
exit 1
