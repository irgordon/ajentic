# AJENTIC Specification

## Overview

AJENTIC is a governed harness for machine-generated candidate solutions.

## Phase 3 status (v0.3.0)

This phase adds static run directory checks.
The CLI supports `validate <run-dir>` and `inspect <run-dir>`.
The validate command checks required file presence and minimal static markers.
The inspect command reports static file presence and byte lengths.
YAML parsing and schema validation are reserved for later phases.
Static validation does not prove governance validity or candidate correctness.

## Contract boundaries

- Schemas under `schemas/` define boundary object shapes.
- Rust contract types under `core/src/*/contract.rs` are authoritative shapes.
- Adapter input/output remains non-authoritative and untrusted.

## Definitions

**Candidate solution**
A machine-generated option represented by the candidate contract shape.

**Tier-1 output**
A contract-level promotion target shape. Promotion behavior is not implemented in Phase 3.

**Domain**
A distinct problem class described by domain contract fields.

**Reuse event**
A contract shape representing later reuse of earlier patterns.

## Scope constraints

Phase 3 does not implement governance engines, objective/constraint validation execution,
provider calls, evaluator execution, ledger persistence, replay, API, or dashboard behavior.
