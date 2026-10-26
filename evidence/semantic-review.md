# Semantic evidence review

Candidate: `CI_REBIND_REQUIRED`

Review method: direct inspection of every workspace manifest and Rust source file; `cargo metadata`/dependency edges; focused tests; the composed `mm-orchestrator -> mm-api -> mm-cli` path; real Regorus, RuVector and RVF adapter tests; Playwright and restart evidence. A directory count was not accepted as evidence.

## Result: PASS

All 27 members expose typed behavior or a justified executable role, appear on a real caller path, and have focused or whole-system evidence. None is empty, generated-only, mock-only or an unused adapter. The lowest-line-count members are role-justified: `mm-cli` is the service entrypoint, `mm-seed` owns deterministic registry fixtures entering the real orchestrator, and `mm-recall` is the typed boundary coupling embeddings to RuVector search. The remaining domain crates implement validation, DAG ordering, capability coverage, calibration/abstention, team formation, optimization, budgets, contract hashing/scopes, policy, append-only audit, repository recovery, authentication, correlation, orchestration and HTTP delivery.

## Call-path findings

- Primary: React control -> POST /v1/mandates -> auth/correlation -> objective/task graph -> capability/calibration -> team/optimizer/budget -> Regorus -> RuVector recall -> RVF audit -> rendered team.
- Denial: the same path reaches Regorus before RuVector insert or RVF append; browser evidence proves no durable row appears.
- Recovery: a real process termination/restart reopens RVF and returns the exact saved mandate ID.
- Evaluation: the ablation executable calls actual Regorus, RuVector and RVF APIs and demonstrates the missing capability for each removed ingredient.

## Counts inspected

27 workspace crates; 26 focused Rust tests plus the executable-role crate; 25 decision ADRs; 12 DDD documents; 6 browser journeys across desktop/mobile; one restart process journey. Exact commands and artifact hashes are recorded in the prototype manifest generated for this candidate.
