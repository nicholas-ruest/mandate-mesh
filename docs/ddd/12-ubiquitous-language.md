# Ubiquitous language

## Model
Objective, Subtask, Role, Agent, CapabilityWindow, Mandate, TeamPlan, Contract, Abstention and Receipt have one precise meaning.

## Invariants
This context owns validation and transitions. Cross-context work uses typed values and explicit ports. Failed invariants return domain errors and cause no hidden durable effect.

## Events and verification
Events carry objective or contract id plus correlation id. Focused tests verify invariants; the acceptance map verifies the context end to end.
