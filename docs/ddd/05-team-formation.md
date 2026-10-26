# Team formation

## Model
TeamPlan requires role coverage, budget, calibrated floor and distinct planner/verifier.

## Invariants
This context owns validation and transitions. Cross-context work uses typed values and explicit ports. Failed invariants return domain errors and cause no hidden durable effect.

## Events and verification
Events carry objective or contract id plus correlation id. Focused tests verify invariants; the acceptance map verifies the context end to end.
