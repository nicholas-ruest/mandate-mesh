# Architecture

## Contexts
Objective Graph owns DAGs. Agent Registry owns identity and trust zones. Capability Assurance owns contextual samples and abstention. Team Formation owns coverage, budgets and optimization. Governance owns Regorus decisions. Contract Lifecycle owns least-authority state. Intelligence/Evidence owns RuVector recall, RVF durability and audit. Delivery owns HTTP, middleware and React.

## Runtime path
mm-cli -> mm-api -> mm-auth/mm-middleware -> mm-orchestrator -> objective/task/agent/capability/calibration/team/optimizer/budget/contract/policy/recall -> regorus/ruvector/embedding -> repository/audit/rvf.

## Crate map
Every crate has a typed API, caller or executable role, design link and focused or journey evidence.

| # | Crate | Responsibility | Context | Caller | Acceptance |
|---:|---|---|---|---|---|
| 1 | mm-types | shared value objects and DTOs | Shared Kernel | all domain crates | focused test + journey |
| 2 | mm-errors | typed errors | Shared Kernel | API/application | focused test + journey |
| 3 | mm-config | validated runtime configuration | Platform | mm-cli | focused test + journey |
| 4 | mm-objective | objective aggregate | Objective Graph | mm-orchestrator | focused test + journey |
| 5 | mm-task-graph | dependency-safe subtask DAG | Objective Graph | mm-orchestrator | focused test + journey |
| 6 | mm-agent | agent identity and trust zone | Agent Registry | mm-team | focused test + journey |
| 7 | mm-capability | contextual skill evidence | Capability Assurance | mm-calibration | focused test + journey |
| 8 | mm-calibration | selective conformal lower bound | Capability Assurance | mm-team | focused test + journey |
| 9 | mm-team | multi-role team formation | Team Formation | mm-orchestrator | focused test + journey |
| 10 | mm-optimizer | constrained utility search | Team Formation | mm-team | focused test + journey |
| 11 | mm-budget | hard cost/concurrency limits | Team Formation | mm-optimizer | focused test + journey |
| 12 | mm-contract | least-authority mandate contract | Contract Lifecycle | mm-orchestrator | focused test + journey |
| 13 | mm-policy | typed policy decision | Governance | mm-orchestrator | focused test + journey |
| 14 | mm-regorus | Microsoft Regorus adapter | Governance | mm-policy | focused test + journey |
| 15 | mm-embedding | task/team embedding | Intelligence | mm-recall | focused test + journey |
| 16 | mm-ruvector | RuVector adapter | Intelligence | mm-recall | focused test + journey |
| 17 | mm-recall | analogue team recall | Intelligence | mm-orchestrator | focused test + journey |
| 18 | mm-rvf | RVF append/reopen/read | Evidence | mm-repository | focused test + journey |
| 19 | mm-audit | hash-chained evidence | Evidence | mm-repository | focused test + journey |
| 20 | mm-repository | single persistence boundary | Evidence | mm-orchestrator | focused test + journey |
| 21 | mm-auth | prototype principal | Delivery | mm-api | focused test + journey |
| 22 | mm-middleware | idempotency/correlation | Delivery | mm-api | focused test + journey |
| 23 | mm-orchestrator | delegation use cases | Application | mm-api/mm-cli | focused test + journey |
| 24 | mm-api | versioned Axum API | Delivery | mm-cli | focused test + journey |
| 25 | mm-seed | synthetic workload | Evaluation | mm-cli | focused test + journey |
| 26 | mm-evaluation | benchmarks/ablations | Evaluation | mm-cli | focused test + journey |
| 27 | mm-cli | service executable | Delivery | operator | focused test + journey |

## Topology
React/Vite proxies to stateless Axum. Only the API owns upstream adapters. RVF is the sole durable store. Browser code has no credentials. Production Cloud Run requires separately authorized shared RuVector/Cloud SQL.

## Failure order
Auth and validation precede orchestration. Budgets and calibration precede policy. Regorus denial precedes vector or RVF writes. Append failure prevents success. Timeouts and shutdown bound execution.
