# AJENTIC Specification

## Overview

AJENTIC is a governed harness for machine-generated candidate solutions.

## Phase 1 status (v0.1.0)

Phase 1 defines contract shapes only across JSON Schemas, Rust contract types,
and minimal examples. Validation behavior will be added in later phases.

## Contract boundaries

- Schemas under `schemas/` define boundary object shapes.
- Rust contract types under `core/src/*/contract.rs` are authoritative shapes.
- Adapter input/output remains non-authoritative and untrusted.

## Definitions

**Candidate solution**
A machine-generated option represented by the candidate contract shape.

**Tier-1 output**
A contract-level promotion target shape. Promotion behavior is not implemented in Phase 1.

**Domain**
A distinct problem class described by domain contract fields.

**Reuse event**
A contract shape representing later reuse of earlier patterns.

## Scope constraints

Phase 1 does not implement governance engines, validation execution, lifecycle enforcement,
provider calls, evaluator execution, ledger persistence, replay, API, or dashboard behavior.
