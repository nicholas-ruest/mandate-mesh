# ADR-0001: Task graph before team selection

Status: Accepted for the 2026-10-25 prototype.

## Context
The real browser-to-storage path needs an explicit decision, ownership and failure contract. Structural presence alone cannot satisfy this decision.

## Decision
Represent each objective as an acyclic role-labelled DAG before considering agents. Reject cycles and missing dependencies.

## Consequences
The owning crate exposes typed behavior, has a real caller and returns an explicit error on failure. This increases focused-test and traceability obligations while preventing silent fallback.

## Verification
DAG test and journey.
