# AJENTIC Governance Rules

AJENTIC treats machine-generated output as untrusted input. The project is designed so generated content, adapter output, evaluator output, UI state, and scripts cannot define candidate truth or promotion authority.

## Core principle

Generated output is untrusted by default.

Rust is authoritative for lifecycle, governance, policy enforcement, ledger, replay, audit, and promotion decisions.

Python, TypeScript, Bash, and future Go wrappers are non-authoritative.

This means:

- adapters do not decide
- evaluators do not promote
- dashboards do not define truth
- scripts do not encode policy
- generated content does not approve itself

Only the Rust core may make authoritative state, governance, and promotion decisions.

## Current implementation boundary

The current repository state implements lower-level control surfaces only:

- contracts and schemas
- candidate lifecycle state machine
- static CLI validation and inspection
- adapter protocol and deterministic mock adapter boundary
- Rust-owned candidate creation
- Rust-owned evaluation result ingestion

The following are not implemented yet:

- governance approval
- promotion eligibility
- Tier-1 promotion decisions
- append-only ledger
- audit record emission
- replay
- real local or cloud provider adapters
- TypeScript review UI

Documentation, code, and tests must not imply these features exist until they are implemented and validated.

## Result handling

`UNKNOWN` is not `PASS`.

Any `UNKNOWN`, missing, malformed, incomplete, blocked, failed, or indeterminate result must not be treated as passing.

Required evaluator satisfaction is not promotion eligibility. It is only an input that future governance logic may consume.

A candidate must not be promoted merely because:

- an adapter returned `COMPLETED`
- a candidate record was created
- one or more evaluation results were recorded
- all required evaluators are present
- all required evaluators are satisfied
- a UI or script requested promotion

Promotion requires a future Rust-owned governance decision.

## Phase boundary notes

### Contracts and schemas

Schemas and Rust contract types define boundary shape only. By themselves, they do not enforce behavior.

Schema validation, contract deserialization, policy enforcement, and full run validation are separate implementation work.

### Static CLI validation

The current CLI validation surface checks static run-directory shape only. It may check required files, non-empty contents, and expected markers.

It does not prove objective validity, constraint validity, domain compatibility, policy approval, candidate quality, governance readiness, or promotion eligibility.

### Adapter boundary

Adapter output is untrusted generated content.

A completed adapter response is not a passing candidate. It is only adapter output accepted into the Rust boundary for later handling.

Python adapters must not:

- approve candidates
- promote candidates
- assign lifecycle authority
- write ledger entries
- emit audit records
- interpret governance policy
- decide final pass/fail semantics

### Candidate creation

Candidate creation is not validation.

A Rust-owned candidate record only means the harness accepted a validated adapter response and recorded candidate data under Rust control.

Created candidates remain untrusted. They are not evaluated, governed, promoted, persisted, replayed, or audited by candidate creation alone.

### Evaluation ingestion

Evaluation ingestion records structured evidence.

Evaluation ingestion does not mutate candidate lifecycle state. It does not approve candidates and does not promote candidates.

`Pass` may satisfy a required evaluator. It does not satisfy governance by itself.

`Fail`, `Blocked`, `Unknown`, missing, malformed, or incomplete evaluator results do not satisfy required evaluators.

### Promotion

`PromotedTier1` is a lifecycle state shape until governance promotion is implemented.

Authorization to enter `PromotedTier1` must belong to Rust governance promotion logic only. It must not be implemented in adapters, evaluators, UI, scripts, or CLI convenience paths.

## One-rule-one-home rule

Each governance rule must have one authoritative implementation.

Promotion authority must live in one Rust governance path. It must not be duplicated across candidate lifecycle code, evaluation ingestion code, Python adapters, TypeScript UI, Bash scripts, or CLI wrappers.

Other layers may display, request, or test governance outcomes. They must not redefine them.

## Prohibited bypass flags

The following flags are prohibited:

- `--force-promote`
- `--skip-policy`
- `--trust-adapter-output`
- `--ignore-unknown`
- `--promote-anyway`

Future experimental modes may exist only if they remain non-authoritative and cannot emit Tier-1 outputs.

## Required failure posture

AJENTIC must fail closed.

The system must reject or block progression when required information is missing, malformed, unknown, incomplete, failed, blocked, or indeterminate.

No component may silently convert uncertainty into success.
