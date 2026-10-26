#!/usr/bin/env bash
set -euo pipefail
grep -q '/v1/mandates' openapi.yaml
grep -q '/v1/mandates' frontend/src/client.ts
echo CONTRACT_PASS
