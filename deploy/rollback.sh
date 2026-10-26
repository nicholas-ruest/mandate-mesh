#!/usr/bin/env bash
set -euo pipefail
: "${REVISION:?set authorized previous revision}"
gcloud run services update-traffic mandate-mesh --to-revisions "$REVISION=100" --region "${REGION:?}"
