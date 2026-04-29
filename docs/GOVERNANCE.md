# AJENTIC Governance Rules

AJENTIC treats machine-generated output as untrusted input. Generated content, adapter output, evaluator output, UI state, scripts, and service wrappers must not define candidate truth or promotion authority.

## Core principle

Generated output is untrusted by default.

Rust is authoritative for lifecycle, governance, policy enforcement, ledger, replay, audit, validation, and promotion decisions.

Python, TypeScript, Bash, and future Go wrappers are non-authoritative.

This means:

- adapters do not decide
- evaluators do not promote
- dashboards do not define truth
- scripts do not encode policy
- generated content does not approve itself

Only the Rust core may make authoritative lifecycle, governance, policy, ledger, replay, audit, validation, and promotion decisions.

## Result handling

`UNKNOWN` is not `PASS`.

Any `UNKNOWN`, missing, malformed, incomplete, blocked, failed, or indeterminate result must not be treated as passing.

Required evaluator satisfaction is not promotion eligibility. It is only an input to governance logic.

A candidate must not be promoted merely because:

- an adapter returned `COMPLETED`
- a candidate record was created
- one or more evaluation results were recorded
- all required evaluators are present
- all required evaluators are satisfied
- a UI or script requested promotion

Promotion requires a Rust-owned governance decision.

## Boundary rules

### Contracts and schemas

Schemas and Rust contract types define boundary shape. By themselves, they do not authorize behavior.

Schema validation, contract deserialization, policy enforcement, and run validation must remain owned by the Rust authority boundary.

### CLI validation

CLI validation and inspection surfaces may expose Rust-owned checks. They must not create independent governance, promotion, lifecycle, ledger, audit, or replay authority.

CLI convenience paths must not bypass governance rules.

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

A Rust-owned candidate record only means the harness accepted validated adapter output and recorded candidate data under Rust control.

Created candidates remain untrusted. Candidate creation alone must not evaluate, govern, promote, persist, replay, or audit the candidate.

### Evaluation ingestion

Evaluation ingestion records structured evidence.

Evaluation ingestion must not approve candidates or promote candidates.

Evaluation ingestion must not mutate authoritative lifecycle state unless explicitly routed through Rust-owned lifecycle rules.

`Pass` may satisfy a required evaluator. It does not satisfy governance by itself.

`Fail`, `Blocked`, `Unknown`, missing, malformed, or incomplete evaluator results do not satisfy required evaluators.

### Promotion

`PromotedTier1` is a lifecycle state, not standalone authorization.

Authorization to enter `PromotedTier1` must belong to Rust governance promotion logic only. It must not be implemented in adapters, evaluators, UI, scripts, or CLI convenience paths.

### Ledger and audit

Ledger and audit records must be Rust-owned.

Ledger entries must preserve the decision path and must not be mutated to alter prior facts.

Audit records must be derived from recorded facts. They must not invent missing evidence.

### Replay

Replay must reconstruct decisions from recorded facts.

Replay must not call model providers, Python adapters, UI code, scripts, remote services, or regenerate candidate output.

### Domain profiles

Domain profiles may define compatibility metadata, evaluator requirements, evidence requirements, known failure modes, and thresholds.

Domain profiles must not alter lifecycle authority, promotion authority, ledger rules, audit rules, replay rules, adapter trust, or `UNKNOWN` handling.

### UI boundary

The UI is a visibility and review surface.

The UI may display Rust-generated state and request actions through approved interfaces. It must not compute authoritative promotion eligibility, mutate lifecycle state, write ledger or audit records, perform replay, call providers, or hide failed, blocked, unknown, malformed, or missing results.

### Scripts

Scripts are glue only.

Scripts may invoke checks and commands. They must not encode policy, lifecycle, governance, promotion, ledger, audit, replay, provider, or credential-handling authority.

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

Experimental modes may exist only if they remain non-authoritative and cannot emit Tier-1 outputs.

## Required failure posture

AJENTIC must fail closed.

The system must reject or block progression when required information is missing, malformed, unknown, incomplete, failed, blocked, or indeterminate.

No component may silently convert uncertainty into success.
