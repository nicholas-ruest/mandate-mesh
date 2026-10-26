# Prototype acceptance

## Journey 1 - issue a governed team contract

- UI: home form, **Form governed team**
- API/middleware: POST `/v1/mandates`; bearer auth; validated JSON; generated correlation/idempotency key; 10-second timeout; structured error
- Application/crates: `mm-api -> mm-orchestrator -> mm-objective/mm-task-graph -> mm-capability/mm-calibration -> mm-team/mm-optimizer/mm-budget -> mm-contract/mm-policy`
- Upstream/persistence: Regorus policy evaluation, RuVector recall/insert, RVF append
- Visible evidence: issued status, risk, three roles and contract digest; desktop/mobile screenshot and trace

## Journey 2 - deny without effect, then recover

- UI: choose `restricted`, submit, inspect alert, refresh audit, choose `internal`, retry
- API/middleware: 403 error retains correlation ID; authorization runs before vector insert/RVF append
- Application/crates: policy anti-corruption boundary calls real Regorus; repository is not invoked on denial
- Proof: Playwright records audit row count before/after denial and requires no change, then requires one new row after recovery

## Journey 3 - retrieve after refresh and service restart

- UI: create, reload page, refresh durable audit
- API/middleware: GET `/v1/mandates` with bearer authorization
- Application/crates: `mm-repository -> mm-audit -> mm-rvf`
- Proof: Playwright validates refresh; `scripts/restart-journey.sh` creates a mandate, terminates the actual Rust process, restarts it against the same RVF path, and retrieves the exact saved ID

No browser acceptance route is intercepted or mocked. Seeded agents are deterministic and labelled synthetic. Test effects are isolated to `data/mandates.rvf` and removed by `scripts/reset.sh`.
