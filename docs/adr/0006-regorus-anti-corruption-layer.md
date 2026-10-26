# ADR-0006: Regorus anti-corruption layer

Status: Accepted for the 2026-10-25 prototype.

## Context
The real browser-to-storage path needs an explicit decision, ownership and failure contract. Structural presence alone cannot satisfy this decision.

## Decision
Only mm-regorus imports engine types; domain uses typed inputs and decisions.

## Consequences
The owning crate exposes typed behavior, has a real caller and returns an explicit error on failure. This increases focused-test and traceability obligations while preventing silent fallback.

## Verification
Integration test.
