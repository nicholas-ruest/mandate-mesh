#!/usr/bin/env bash
set -euo pipefail
for f in /tmp/mandate-mesh-backend.pid /tmp/mandate-mesh-frontend.pid;do if test -f "$f";then kill "$(cat "$f")" 2>/dev/null || true;rm -f "$f";fi;done
