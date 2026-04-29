````markdown id="spec-md"
# AJENTIC Specification

## 1. Overview

AJENTIC is a governed harness for machine-generated candidate solutions.

AJENTIC treats generated output as untrusted, records candidate and evaluation data under Rust-owned control, and promotes outputs only after explicit governance checks in later phases.

The harness is not primarily an LLM caller. It is a control layer around model output, adapter output, evaluator evidence, lifecycle state, governance decisions, audit records, and replay.

## 2. Current implementation status

Current repository status: Phase 6 complete pending Owner acceptance and documentation/version alignment.

Implemented:

- contracts and schemas
- candidate lifecycle state machine
- static CLI validation and inspection
- adapter protocol and deterministic Python mock adapter boundary
- Rust-owned candidate creation from validated adapter responses
- Rust-owned evaluation result ingestion

Not implemented:

- full objective validation
- full constraint validation
- domain compatibility validation
- governance approval
- promotion eligibility
- Tier-1 promotion decisions
- append-only ledger
- audit record emission
- replay
- real local or cloud provider adapters
- TypeScript review UI
- multi-domain runtime capability

## 3. Architecture rule

```text
Rust = authority
Python = adaptation
TypeScript = visibility
Bash = glue
Go = optional service wrapper, not part of the current implementation path
````

Rust owns authoritative lifecycle, governance, policy enforcement, ledger, replay, audit, validation, and promotion decisions.

Python adapters and evaluators are non-authoritative. They may produce output, evidence, or observations, but they must not approve, promote, mutate lifecycle state, write ledger entries, emit audit records, or decide final pass/fail semantics.

TypeScript is a future visibility and review surface. It may display Rust-generated state and request actions, but it must not compute authoritative promotion eligibility.

Bash is thin local and CI glue. It must not encode policy, lifecycle, governance, replay, audit, or promotion decisions.

Go is optional future service infrastructure only. If added, it must wrap the Rust authority rather than duplicate it.

## 4. Core trust model

Generated output is untrusted by default.

Adapter output is untrusted by default.

Evaluator output is structured evidence, not governance approval.

Candidate creation is not validation.

Required evaluator satisfaction is not promotion eligibility.

`UNKNOWN` is not `PASS`.

Only Rust may make authoritative lifecycle, governance, ledger, replay, audit, and promotion decisions.

No unsafe bypass flags are allowed:

```text
--force-promote
--skip-policy
--trust-adapter-output
--ignore-unknown
--promote-anyway
```

## 5. Implemented surfaces

### 5.1 Contracts and schemas

Schemas under `schemas/` define boundary object shapes.

Rust contract types under `core/src/*/contract.rs` define Rust-side contract shapes.

Contracts and schemas define structure. They do not by themselves enforce runtime behavior.

Schema validation, full YAML parsing, full contract deserialization, and complete run validation remain future work unless explicitly implemented in later phases.

### 5.2 Candidate lifecycle

The Rust core defines the candidate lifecycle state machine.

Lifecycle states include:

* `CREATED`
* `EVALUATING`
* `FAILED`
* `BLOCKED`
* `PASSED`
* `PROMOTED_TIER_1`
* `REJECTED`
* `UNKNOWN`

Lifecycle transition checks are typed and Rust-owned.

`PROMOTED_TIER_1` is currently a lifecycle state shape. Authorization to enter this state belongs to future Rust governance promotion logic.

### 5.3 Static CLI validation and inspection

The CLI supports:

```sh
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
```

The current validation command checks static run-directory shape only:

* required file presence
* non-empty file content
* expected plain-text markers

The inspect command reports static file presence and byte lengths.

Static CLI validation does not parse YAML, validate schemas, deserialize contracts, evaluate candidates, apply governance, or approve promotion.

### 5.4 Adapter protocol and mock adapter

The Rust core defines a Phase 4 adapter boundary.

Rust can invoke a deterministic Python mock adapter through a subprocess. Rust validates response linkage, required metadata, status vocabulary, and output-size limits.

A completed adapter response is untrusted generated output only.

A completed adapter response is not a passing candidate, not governance approval, and not a promotion decision.

The current line protocol is an early deterministic mock boundary. It is not provider authority and should not be treated as the final real-provider adapter contract.

### 5.5 Candidate creation

Rust can create a candidate record from a validated completed adapter response.

Candidate IDs are deterministic and Rust-assigned.

Created candidates start in lifecycle state `CREATED`.

Candidate creation does not evaluate, govern, promote, persist, replay, or audit the candidate.

Created candidates remain untrusted.

### 5.6 Evaluation result ingestion

Rust can ingest structured evaluation results and link them to candidate records.

Evaluation statuses include:

* `PASS`
* `FAIL`
* `BLOCKED`
* `UNKNOWN`

Rust can check required evaluator presence and required evaluator satisfaction.

Only `PASS` satisfies a required evaluator.

`FAIL`, `BLOCKED`, `UNKNOWN`, missing, malformed, or incomplete evaluator results do not satisfy required evaluators.

Evaluation result ingestion does not mutate candidate lifecycle state. It does not approve candidates and does not promote candidates.

## 6. Not-yet-implemented surfaces

The following are intentionally not implemented yet:

* governance precheck
* runtime governance review
* policy enforcement engine
* promotion eligibility engine
* promotion decision creation
* Rust-only authorization into `PROMOTED_TIER_1`
* append-only ledger
* audit record emission
* replay from recorded facts
* real local model adapter
* real cloud model adapter
* TypeScript review UI
* multi-domain runtime capability
* reuse and bounded improvement records
* hardening fixture suite

These surfaces must be implemented only through later scoped phases.

## 7. Boundary definitions

### Candidate solution

A Rust-owned candidate record derived from a validated completed adapter response.

The candidate may contain or reference generated content, but that content remains untrusted until evaluated and governed.

### Evaluation result

A structured result linked to a candidate and evaluator.

An evaluation result may record `PASS`, `FAIL`, `BLOCKED`, or `UNKNOWN`.

An evaluation result is evidence. It is not promotion authority.

### Required evaluator satisfaction

A Rust-owned check that determines whether required evaluators have recorded satisfying results.

Only `PASS` satisfies a required evaluator.

Required evaluator satisfaction is not promotion eligibility.

### Governance result

A future Rust-owned governance record.

Governance will consume candidate data, evaluation evidence, policy checks, and required references.

Governance is not implemented in the current repository state.

### Promotion decision

A future Rust-owned decision that may authorize transition to `PROMOTED_TIER_1`.

Promotion decisions must have one authoritative implementation path in Rust governance code.

Promotion is not implemented in the current repository state.

### Tier-1 output

A future governed output suitable for downstream review.

Tier-1 does not mean automatic production approval.

Tier-1 output creation is not implemented in the current repository state.

### Domain

A distinct problem class described by domain contract fields.

Domains may later configure objective types, constraint types, evaluators, thresholds, failure modes, and evidence requirements.

Domains must not alter lifecycle authority, promotion semantics, ledger rules, replay rules, unknown handling, or Rust authority.

### Reuse event

A future Rust-owned record representing reuse of earlier patterns.

Reuse is advisory until validated in the new context.

Reuse must not bypass validation or alter governance rules.

## 8. Phase boundary constraints

Current implemented surfaces must not be reinterpreted as later-phase behavior.

Specifically:

* static CLI validation must not be treated as full run validation
* adapter completion must not be treated as candidate approval
* candidate creation must not be treated as candidate validation
* evaluator satisfaction must not be treated as governance approval
* lifecycle state shape must not be treated as promotion authorization
* Python output must not be treated as authoritative
* UI state must not be treated as authoritative
* Bash scripts must not encode governance policy

## 9. Future implementation direction

### Governance and promotion

The next major implementation phase is Rust-only governance and promotion.

Promotion authority must live in one Rust governance path. It must not be duplicated in candidate lifecycle code, evaluation ingestion code, CLI wrappers, Python adapters, TypeScript UI, or Bash scripts.

A candidate may be promoted only when future governance logic verifies:

* objective validity
* constraint validity
* domain validity
* required evaluator completion
* absence of required `FAIL`
* absence of required `BLOCKED`
* absence of unresolved `UNKNOWN`
* required policy checks
* required evidence references

### Ledger, audit, and replay

Later phases must add append-only ledger records, audit records, and replay.

Replay must reconstruct decisions from recorded facts. It must not regenerate model output or call Python adapters, model providers, TypeScript UI, Bash scripts, or remote services.

### Provider adapters

Real local and cloud adapters must use the same non-authoritative adapter boundary.

Provider source must not affect trust semantics.

Provider-specific behavior must not enter candidate lifecycle or promotion logic.

### UI

The TypeScript UI must display Rust-generated state.

It must not compute authoritative promotion eligibility or hide failed, blocked, unknown, or missing checks.

## 10. Scope constraints

The current repository state does not implement:

* full run validation
* provider calls
* evaluator execution
* governance engines
* promotion logic
* ledger persistence
* replay
* API server
* dashboard behavior
* production deployment approval

Any future work that adds these capabilities must be scoped through the roadmap and must preserve Rust authority, fail-closed behavior, and the non-authoritative status of adapters, UI, and scripts.

```
```
