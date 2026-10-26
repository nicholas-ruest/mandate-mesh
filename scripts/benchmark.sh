#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/tmp/mandate-mesh-target
mkdir -p evidence
cargo run --locked -p mm-evaluation --example ablation > evidence/benchmark.json
node -e 'const x=require("./evidence/benchmark.json");if(!x.find(r=>r.mode==="combined"&&r.success)||x.find(r=>r.mode==="without_rvf").restart_retrieval)process.exit(1);console.log("ABLATION_PASS rows="+x.length)'
