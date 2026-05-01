# AJENTIC Specification

## 1. Overview

AJENTIC is a governed harness for machine-generated candidate solutions.
AJENTIC treats generated output as untrusted, records candidate and evaluation data under Rust-owned control, and promotes outputs only after explicit governance checks.
The harness is not primarily an LLM caller. It is a control layer around model output, adapter output, evaluator evidence, lifecycle state, governance decisions, audit records, and replay.
This specification defines architecture and capability boundaries.  
It does not record operational status or phase completion.

## 2. Implemented capability surfaces

The repository contains foundational control-layer surfaces that establish deterministic behavior and Rust-owned authority boundaries.
These surfaces describe architectural capability, not operational readiness.
Implemented capability surfaces include:
- contracts and schemas
- candidate lifecycle state machine
- static CLI validation and inspection
- adapter protocol and deterministic Python mock adapter boundary
- Rust-owned candidate creation from validated adapter responses
- Rust-owned evaluation result ingestion
- deterministic domain registry construction and validation
- fail-closed domain resolution behavior
- deterministic verification and replay classification surfaces
- deterministic registry identity and ordering enforcement
- domain boundary and isolation regression tests
- registry inventory lock and extension safety enforcement
- domain resolution determinism and non-mutation validation
These capabilities define structural correctness and boundary enforcement.
They do not imply production readiness, governance authorization, or promotion eligibility.
Operational state, readiness, and implementation acceptance are determined exclusively by:
- executable code
- passing tests
- accepted CHANGELOG entries
If documentation and implementation diverge, implementation artifacts take precedence.

## 3. Architecture rule

```text
Rust = authority
Python = adaptation
TypeScript = visibility
Bash = glue
Go = optional service wrapper, not part of the current implementation path
```

Rust owns authoritative lifecycle, governance, policy enforcement, ledger, replay, audit, validation, and promotion decisions.

Python adapters and evaluators are non-authoritative. They may produce output, evidence, or observations, but they must not approve, promote, mutate lifecycle state, write ledger entries, emit audit records, or decide final pass/fail semantics.

TypeScript is a visibility and review surface. It may display Rust-generated state and request actions, but it must not compute authoritative promotion eligibility.

Bash is thin local and CI glue. It must not encode policy, lifecycle, governance, replay, audit, or promotion decisions.

Go is optional future service infrastructure only. If added, it must wrap the Rust authority rather than duplicate 

##4. Core trust model

Generated output is untrusted by default.

Adapter output is untrusted by default.

Evaluator output is structured evidence, not governance approval.

Candidate creation is not validation.

Required evaluator satisfaction is not promotion eligibility.

UNKNOWN is not PASS.

Only Rust may make authoritative lifecycle, governance, ledger, replay, audit, validation, and promotion decisions.

No unsafe bypass flags are allowed:

--force-promote
--skip-policy
--trust-adapter-output
--ignore-unknown
--promote-anyway

## 5. Implemented architectural surfaces

### 5.1 Contracts and schemas

Schemas under schemas/ define boundary object shapes.

Rust contract types under core/src/*/contract.rs define Rust-side contract shapes.

Contracts and schemas define structure. They do not authorize behavior.

Schema validation, full YAML parsing, full contract deserialization, and complete run validation remain governed by explicit Rust logic.

### 5.2 Candidate lifecycle

The Rust core defines the candidate lifecycle state machine.

Lifecycle states include:

* CREATED
* EVALUATING
* FAILED
* BLOCKED
* PASSED
* PROMOTED_TIER_1
* REJECTED
* UNKNOWN

Lifecycle transition checks are typed and Rust-owned.

PROMOTED_TIER_1 is a lifecycle state shape.

Authorization to enter this state belongs exclusively to Rust governance promotion logic.

### 5.3 Static CLI validation and inspection

The CLI supports:

cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run

The validation command checks static run-directory structure only:

* required file presence
* non-empty file content
* expected structure markers

The inspect command reports parsed structure and validation status.

These commands do not:

* perform governance decisions
* promote candidates
* execute replay
* mutate runtime state

### 5.4 Adapter protocol and mock adapter

The Rust core defines a deterministic adapter boundary.

Rust invokes a Python mock adapter through a subprocess boundary.

Rust validates:

* protocol structure
* response linkage
* status vocabulary
* output size limits
* required metadata

A completed adapter response is untrusted generated output only.

It is not:

* candidate approval
* governance authorization
* promotion eligibility
* audit evidence

The current adapter protocol is a deterministic boundary mechanism.
It is not provider authority.

### 5.5 Candidate creation

Rust creates candidate records from validated adapter responses.

Candidate IDs are deterministic and Rust-assigned.

Created candidates begin in lifecycle state:

CREATED

Candidate creation:

* records untrusted output
* preserves traceability
* does not evaluate
* does not govern
* does not promote
* does not persist
* does not replay
* does not audit

### 5.6 Evaluation result ingestion

Rust ingests structured evaluation results and links them to candidates.

Evaluation statuses include:

* PASS
* FAIL
* BLOCKED
* UNKNOWN

Rust verifies required evaluator presence and required evaluator satisfaction.

Only PASS satisfies a required evaluator.

FAIL, BLOCKED, UNKNOWN, missing, malformed, or incomplete results do not satisfy required evaluators.

Evaluation ingestion:

* records evidence
* preserves determinism
* does not authorize promotion
* does not mutate lifecycle state outside explicit rules

### 5.7 Domain registry and resolution

Domain profiles are defined as deterministic metadata structures.

The domain registry:

* is constructed in fixed literal order
* has deterministic identity
* has deterministic lookup behavior
* rejects invalid references
* fails closed on malformed input

Registry ordering is part of identity.

Domain resolution is:

* deterministic
* explicit
* side-effect free
* non-mutating

Domain configuration must not alter:

* lifecycle authority
* governance semantics
* replay behavior
* verification logic
* promotion eligibility

### 5.8 Verification and replay classification

Verification and replay modules operate as deterministic classification surfaces.

They:

* analyze recorded state
* classify system condition
* produce descriptive results

They do not:

* mutate runtime state
* execute governance
* perform promotion
* regenerate external outputs

Replay reconstructs decisions from recorded facts.

Replay must not:

* call adapters
* call providers
* call external services
* regenerate model output
* mutate ledger records

## 6. Boundary definitions

Candidate solution

A Rust-owned record derived from validated adapter output.

Candidate content remains untrusted until governed.

⸻

Evaluation result

Structured evidence linked to a candidate.

Evaluation results do not authorize promotion.

⸻

Required evaluator satisfaction

A deterministic check verifying evaluator completion.

It is an input to governance logic.

It is not promotion eligibility.

⸻

Governance result

A Rust-owned decision record.

Governance consumes candidate state, evaluation evidence, policy checks, and required references.

Only governance logic may authorize promotion.

⸻

Promotion decision

A Rust-owned authorization event.

Promotion requires:

* valid objective
* valid constraints
* valid domain
* completed required evaluators
* absence of FAIL
* absence of BLOCKED
* absence of unresolved UNKNOWN
* required evidence presence

Promotion authority must exist in exactly one Rust implementation path.

⸻

Tier-1 output

A governed output suitable for downstream review.

Tier-1 does not imply production deployment.

⸻

Domain

A metadata-defined problem classification.

Domains configure compatibility.

Domains do not alter authority.

⸻

Reuse event

A recorded reuse of prior patterns.

Reuse is advisory.

Reuse does not bypass validation.

## 7. Scope constraints

The current architecture intentionally restricts capability expansion.

The system does not assume:

* automatic promotion
* implicit success
* hidden defaults
* silent recovery
* nondeterministic execution
* external authority

All transitions require explicit validation.

All decisions require deterministic evidence.

All failures must fail closed.


## 8. Architectural invariants

The following invariants are permanent:

Rust authority invariant

Only Rust may authorize lifecycle transitions, governance decisions, ledger operations, replay execution, audit generation, and promotion.

Adapter non-authority invariant

Adapters may generate output. They must not approve candidates.

UI non-authority invariant

User interfaces may display state. They must not define truth.

Script non-authority invariant

Scripts may invoke commands. They must not encode policy.

Unknown-is-not-pass invariant

Unknown, missing, malformed, incomplete, blocked, or failed results must not be treated as passing.

One-rule-one-home invariant

Each governance rule must have one authoritative implementation.

Replay independence invariant

Replay must reconstruct decisions from recorded facts without regeneration.

Determinism invariant

Repeated operations must produce identical results under identical inputs.

Failure-closed invariant

Missing or invalid information must halt progression.


## 9. Specification governance rule

This document defines architecture and invariant behavior.

It does not:

* record operational readiness
* declare implementation completion
* assign phase status
* authorize deployment
* override tests
* override code
* override the CHANGELOG

Behavior is determined by:

* executable code
* deterministic tests
* accepted CHANGELOG entries

These artifacts are the final authority.