# AJENTIC Specification

## Overview

AJENTIC is a governed harness for machine-generated candidate solutions.

## Phase 2 status (v0.2.0)

This phase defines the candidate lifecycle transition surface.
The transition API rejects illegal state moves.
Promotion governance is not implemented in Phase 2.
Passed -> PromotedTier1 is a legal transition shape, not a promotion authorization engine.
Validation of objectives, constraints, evaluators, policies, and evidence is reserved for later phases.

## Contract boundaries

- Schemas under `schemas/` define boundary object shapes.
- Rust contract types under `core/src/*/contract.rs` are authoritative shapes.
- Adapter input/output remains non-authoritative and untrusted.

## Definitions

**Candidate solution**
A machine-generated option represented by the candidate contract shape.

**Tier-1 output**
A contract-level promotion target shape. Promotion behavior is not implemented in Phase 2.

**Domain**
A distinct problem class described by domain contract fields.

**Reuse event**
A contract shape representing later reuse of earlier patterns.

## Scope constraints

Phase 2 does not implement governance engines, objective/constraint validation execution,
provider calls, evaluator execution, ledger persistence, replay, API, or dashboard behavior.
