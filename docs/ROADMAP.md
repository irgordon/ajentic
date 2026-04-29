# AJENTIC Roadmap

## 1. Purpose

This roadmap defines the phased implementation path for AJENTIC from a blank repository to early production capability. Through each phase, slight deviations may occur as it moves, the CHANGELOG should be considered the authoritative state, this document reflect the overarching planned implemention and serve as a strong guide to keep the project from adopting inappropriate expansion.

AJENTIC is a governed harness for local and cloud-based large language models (LLMs). It treats generated output as untrusted, records candidate and evaluation data under Rust-owned control, and promotes only governed outputs for downstream review after explicit governance checks.

The implementation must progress incrementally. Each phase has a narrow scope, explicit validation gates, and a defined stopping point. Later phases must not backfill hidden behavior into earlier phases.

The primary goal is not feature volume. The primary goal is a usable, maintainable, auditable harness whose control layer remains understandable to first-time developers and extensible by experienced developers.

This roadmap is an anchor document. It provides a forecast from the full v0 implementation path to production capability. Historical release notes are owned by `CHANGELOG.md`.

---

## 2. Architecture rule

The language ownership model is fixed for this roadmap:

```text
Rust = authority
Python = adaptation
TypeScript = visibility
Bash = glue
Go = optional service wrapper, not part of the current implementation path
````

Rust owns authoritative lifecycle, governance, ledger, replay, validation, promotion, and audit decisions.

Python adapters may call local or cloud models in later milestones, but their outputs remain untrusted.

TypeScript may display harness state and request actions, but it must not compute authoritative promotion eligibility.

Bash may invoke commands, but it must not encode policy, lifecycle, governance, replay, audit, or promotion decisions.

Go is not used before early production unless a service wrapper becomes necessary. If added later, it must wrap the Rust authority rather than duplicate it.

---

## 3. Current implementation status

Current repository status: Phase 6 complete pending Owner acceptance and documentation/version alignment.

Implemented through the current repository state:

* Phase 0: repository skeleton and governance placeholders
* Phase 1: contracts and schemas
* Phase 2: candidate lifecycle state machine
* Phase 3: static CLI validation and inspection
* Phase 4: adapter protocol and deterministic mock adapter boundary
* Phase 5: Rust-owned candidate creation and runtime adapter checks
* Phase 6: Rust-owned evaluation result ingestion

Not implemented yet:

* Phase 7: governance and promotion gates
* Phase 8: append-only ledger and audit records
* Phase 9: replay
* Phase 10: local model adapter capability
* Phase 11: cloud model adapter capability
* Phase 12: TypeScript UI review surface
* Phase 13: multi-domain capability
* Phase 14: reuse and bounded improvement records
* Phase 15: hardening and negative testing
* Phase N: early production capability

Current boundary statement:

Candidate creation is not validation. Evaluation result ingestion is not governance approval. Required evaluator satisfaction is not promotion eligibility. `PromotedTier1` is a lifecycle state shape only until Rust-owned governance promotion is implemented.

---

## 4. Versioning guide

AJENTIC uses pre-production milestone versioning.

Version numbers are gates, not marketing labels. A version must not be tagged until its validation criteria pass and the Owner accepts the milestone.

```text
0.0.x  = bootstrap, documentation alignment, skeleton, audit corrections, and pre-milestone cleanup
0.1.0  = core boundary milestone complete
0.1.x  = iterative fixes and hardening within the core boundary milestone
0.2.0  = governance and promotion milestone complete
0.2.x  = iterative fixes and hardening within governance and promotion
0.3.0  = ledger, audit, and replay milestone complete
0.3.x  = iterative fixes and hardening within ledger/audit/replay
0.4.0  = provider adapter milestone complete
0.4.x  = iterative fixes and hardening within provider adapters
0.5.0  = UI review surface milestone complete
0.5.x  = iterative fixes and hardening within UI/review
0.6.0  = multi-domain and reuse milestone complete
0.6.x  = iterative fixes and hardening within domain/reuse capability
0.7.0  = failure hardening milestone complete
0.7.x  = iterative fixes and hardening within negative testing and failure coverage
0.8.0  = early production candidate milestone complete
0.8.x  = iterative stabilization before production release
1.0.0  = production release line
```

Patch versions may include:

* documentation alignment
* small bug fixes
* test additions
* audit corrections
* small non-breaking contract refinements
* implementation work that does not complete the next milestone
* internal cleanup that preserves the trust model

Patch versions must not silently introduce a new authority surface, provider capability, promotion path, ledger behavior, replay behavior, or UI decision behavior.

A milestone version such as `0.2.0` may be tagged only when the whole milestone is complete, tested, documented, and accepted.

---

## 5. Global invariants

These apply to every milestone and every implementation phase.

* Generated output is untrusted by default.
* Adapter output is untrusted by default.
* Candidate creation is not validation.
* Evaluation result ingestion is not governance approval.
* Required evaluator satisfaction is not promotion eligibility.
* Only Rust may promote a candidate to Tier-1.
* `UNKNOWN` is not `PASS`.
* Missing validation is not `PASS`.
* Malformed evidence is not `PASS`.
* Failed, blocked, missing, malformed, incomplete, or indeterminate results must fail closed.
* Adapters do not decide.
* UI does not decide.
* Scripts do not decide.
* Replay does not regenerate.
* Every promoted output must be traceable.
* No unsafe bypass flags are allowed.

Disallowed flags:

```text
--force-promote
--skip-policy
--trust-adapter-output
--ignore-unknown
--promote-anyway
```

Experimental modes may exist later, but they must remain non-authoritative and must not emit Tier-1 outputs.

---

## 6. Gate model

Each implementation phase and each milestone must end with four checks.

Build gate:

* The repository builds or checks successfully.

Contract gate:

* Public file shapes, schemas, command surfaces, and documented records match the scoped work.

Behavior gate:

* Only the behavior allowed in the phase or milestone exists.

Documentation gate:

* README, roadmap, changelog, and relevant docs reflect current reality.

A phase is not complete if documentation claims behavior that does not exist.

A phase is not complete if code implements behavior reserved for a later phase.

A milestone is not complete until all included implementation phases are complete, tests pass, and documentation reflects the accepted state.

---

## 7. Milestone forecast

### Milestone 0: Bootstrap and alignment

Version range: `0.0.x`

Status: current line

Purpose: Establish the repository skeleton, coding discipline, documentation alignment, and pre-governance control surfaces.

Includes:

* repository skeleton
* contracts and schemas
* candidate lifecycle state machine
* static CLI validation and inspection
* adapter protocol and deterministic mock adapter
* Rust-owned candidate creation
* Rust-owned evaluation result ingestion
* documentation alignment after audit
* roadmap, changelog, and versioning normalization

Required capabilities:

* Rust workspace builds
* core and CLI crates exist
* Python adapter scaffold exists
* TypeScript UI scaffold exists
* Bash scripts remain thin
* schemas and examples exist
* static CLI validate/inspect commands exist
* deterministic mock adapter boundary exists
* candidate creation remains Rust-owned
* evaluation ingestion remains Rust-owned
* no governance or promotion behavior exists
* no ledger or replay behavior exists
* no real provider integration exists
* no UI decision behavior exists

Exit criteria:

* `cargo fmt --all -- --check`
* `cargo test --workspace`
* `cargo check --workspace`
* `./scripts/check.sh`
* documentation reflects actual implemented state
* versioning policy is normalized
* no unsafe bypass flags exist
* no authority drift into Python, TypeScript, or Bash
* Phase 7 can begin without ambiguity

---

### Milestone 1: Core boundary milestone

Target version: `0.1.0`

Status: forecast / near-complete pending Owner acceptance

Purpose: Complete the pre-governance Rust authority boundary so the system can validate static run inputs, handle adapter output, create candidates, and record evaluation evidence without promotion authority.

Required capabilities:

* explicit schemas
* Rust contract shapes
* static CLI validation and inspection
* candidate lifecycle state machine
* deterministic mock adapter boundary
* Rust-owned candidate creation
* Rust-owned evaluation result ingestion
* `UNKNOWN` is not `PASS`
* no governance or promotion behavior
* no ledger or replay behavior
* no real provider integrations
* documentation accurately states the boundary

Required invariants:

* adapter completion is not candidate approval
* candidate creation is not validation
* evaluation satisfaction is not promotion eligibility
* `PromotedTier1` is a lifecycle state shape only until governance promotion exists
* Python, TypeScript, and Bash remain non-authoritative

Exit criteria:

* `cargo fmt --all -- --check`
* `cargo test --workspace`
* `cargo check --workspace`
* `./scripts/check.sh`
* documentation reflects actual implemented state
* no unsafe bypass flags exist
* no authority drift into Python, TypeScript, or Bash

---

### Milestone 2: Governance and promotion milestone

Target version: `0.2.0`

Status: not started

Purpose: Implement Rust-only governance review and Tier-1 promotion decisions.

This is the first milestone where promotion authority becomes real.

Required capabilities:

* governance precheck
* runtime governance result
* policy check result model
* promotion eligibility check
* promotion decision record
* Rust-only transition into `PROMOTED_TIER_1`
* explicit failure behavior for `FAIL`, `BLOCKED`, `UNKNOWN`, missing evaluator, missing evidence, and malformed governance result
* one authoritative promotion path

Required invariants:

* promotion authority must live in one Rust governance path
* promotion authority must not be duplicated in candidate lifecycle code
* promotion authority must not be duplicated in evaluation ingestion code
* promotion authority must not be duplicated in CLI wrappers
* promotion authority must not be duplicated in Python adapters
* promotion authority must not be duplicated in TypeScript UI
* promotion authority must not be duplicated in Bash scripts

Exit criteria:

* all required checks pass permits promotion
* `FAIL` blocks promotion
* `BLOCKED` blocks promotion
* unresolved `UNKNOWN` blocks promotion
* missing evaluator blocks promotion
* missing evidence blocks promotion
* malformed governance result blocks promotion
* Python adapter approval fields are ignored or rejected
* UI cannot promote directly
* CLI cannot bypass governance
* promotion logic has one authoritative Rust implementation

---

### Milestone 3: Ledger, audit, and replay milestone

Target version: `0.3.0`

Status: not started

Purpose: Preserve and replay the authoritative decision path.

Required capabilities:

* append-only ledger
* typed ledger entries
* candidate traceability
* evaluation traceability
* governance traceability
* promotion traceability
* audit record emission
* replay from recorded facts
* replay mismatch detection
* replay does not call adapters, providers, UI, scripts, or remote services

Required invariants:

* ledger is Rust-owned
* audit records are derived from recorded facts
* audit must not invent missing evidence
* replay reconstructs decisions from recorded facts
* replay must not regenerate model output
* replay must not call Python adapters
* replay must not call model providers
* replay must not use UI-derived truth

Exit criteria:

* promoted candidates have full audit trace
* failed and blocked candidates preserve reasons
* invalid ledger append fails
* replay reconstructs candidate state and promotion decision
* replay detects missing or invalid ledger ordering
* replay does not invoke Python or model providers

---

### Milestone 4: Provider adapter milestone

Target version: `0.4.0`

Status: not started

Purpose: Add real local and cloud model adapter capability without weakening trust boundaries.

Required capabilities:

* one local model adapter
* one cloud model adapter
* same adapter protocol path for both
* provider error mapping
* timeout or failure behavior
* missing credential failure for cloud provider
* no credential leakage
* successful provider generation still requires evaluation and governance

Required invariants:

* provider source does not affect trust semantics
* local output remains untrusted adapter output
* cloud output remains untrusted adapter output
* provider-specific logic must not enter candidate lifecycle
* provider-specific logic must not enter promotion
* provider token/cost metadata is operational metadata, not proof of quality unless explicitly configured

Exit criteria:

* local adapter unavailable fails safely
* cloud credentials missing fail safely
* malformed provider output is rejected
* successful generation still requires evaluation
* adapter cannot promote candidates
* provider-specific logic does not enter lifecycle or promotion

---

### Milestone 5: UI review surface milestone

Target version: `0.5.0`

Status: not started

Purpose: Provide a TypeScript review surface that makes governance state inspectable without becoming an authority source.

Required capabilities:

* run overview
* candidate list
* candidate detail
* evaluation result detail
* governance result detail
* promotion decision detail
* audit trail
* artifact or raw record view
* status labels for all lifecycle states

Required invariants:

* UI displays Rust-generated state
* UI does not compute authoritative promotion eligibility
* UI does not define lifecycle truth
* UI does not hide failed checks
* UI does not hide blocked checks
* UI does not hide unknown checks
* UI does not collapse blocked and failed states into a generic error

Exit criteria:

* UI displays failed checks
* UI displays blocked checks
* UI displays unknown checks
* UI does not hide missing evidence
* UI does not compute promotion result independently

---

### Milestone 6: Multi-domain and reuse milestone

Target version: `0.6.0`

Status: not started

Purpose: Support materially different domains and bounded reuse without changing the core lifecycle or promotion model.

Required capabilities:

* at least two domain profiles
* domain compatibility checks
* evaluator/domain matching
* reuse event records
* source and target reuse references
* reuse validation status
* mismatch notes

Required invariants:

* domains configure evaluation
* domains do not alter lifecycle
* domains do not alter promotion
* domains do not alter ledger rules
* domains do not alter replay rules
* domains do not alter unknown handling
* domains do not alter Rust authority
* reuse is advisory until validated in the new context
* reuse must not skip validation
* reuse must not change promotion rules

Exit criteria:

* two distinct domains run through the same core path
* missing domain fails
* evaluator mismatch fails
* domain cannot override lifecycle
* domain cannot override promotion
* reuse event does not skip validation
* reuse cannot change promotion rules

---

### Milestone 7: Failure hardening milestone

Target version: `0.7.0`

Status: not started

Purpose: Prove fail-closed behavior across invalid inputs, malformed records, boundary violations, and misuse.

Required capabilities:

* negative unit tests
* negative integration tests
* malformed schema tests
* adapter failure tests
* evaluator failure tests
* ledger corruption tests
* replay mismatch tests
* attempted UI authority tests
* attempted Python authority tests
* script misuse tests

Required invariants:

* invalid inputs fail closed
* missing inputs fail closed
* malformed records fail closed
* blocked results fail closed
* failed results fail closed
* unknown results fail closed
* test-only bypasses are prohibited
* hidden permissive modes are prohibited

Exit criteria:

* bad adapter output cannot promote
* bad evaluator output cannot promote
* corrupt ledger cannot replay as valid
* UI cannot hide governance failure
* CLI cannot bypass governance
* failure paths are tested, not only success paths

---

### Milestone 8: Early production candidate milestone

Target version: `0.8.0`

Status: not started

Purpose: Make AJENTIC usable for controlled early production review workflows with explicit limitations.

Required capabilities:

* static run validation
* local model adapter
* cloud model adapter
* candidate generation
* runtime governance checks
* structured evaluation results
* Rust-only Tier-1 promotion
* append-only ledger
* replay
* audit records
* UI review surface
* at least two domains
* reuse event logging
* negative test coverage

Required limitations:

AJENTIC must still declare that it is:

* not a production deployment approval system
* not a replacement for human review
* not a model safety guarantee
* not a training system
* not a fully autonomous self-improving agent
* not a guarantee that Tier-1 output is correct

Exit criteria:

* full build and tests pass
* mock workflow runs
* local workflow runs if configured
* cloud workflow runs if configured
* failed candidates do not promote
* `UNKNOWN` results do not promote
* replay does not call providers
* UI displays full audit state

---

## 8. Implementation phases

The phases below are implementation slices. They may produce `0.x.y` patch releases. A `0.x.0` milestone release is tagged only when the full milestone group is accepted.

---

## Phase 0: Repository bootstrap

Milestone group: Milestone 0 / Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Create the blank repository skeleton with clear language ownership and no runtime behavior.

### Scope

Create the repository structure:

```text
core/       Rust trusted core placeholder
cli/        Rust CLI placeholder
adapters/   Python adapter placeholder
schemas/    JSON Schema placeholders
examples/   minimal example shape
ui/         TypeScript visibility placeholder
scripts/    thin shell scripts
docs/       architecture and governance docs
```

### Allowed

* Rust workspace setup
* core crate skeleton
* cli crate skeleton
* Python adapter package skeleton
* UI placeholder
* schema placeholder files
* example YAML placeholders
* governance documents
* simple shell scripts

### Not allowed

* runtime governance
* model calls
* provider integrations
* evaluators
* ledger persistence
* replay logic
* UI dashboard implementation
* API server
* third-party dependencies

### Validation gate

Required commands:

```sh
cargo check --workspace
./scripts/check.sh
```

Expected result:

* Rust workspace compiles.
* Scripts are executable.
* No placeholder file is empty except `.gitkeep`.
* No third-party dependencies are added.
* Documentation states skeleton-only status for Phase 0.

### Exit criteria

Phase 0 is complete when a first-time developer can clone the repo, run the check script, and understand the intended architecture from README.md and AGENTS.md.

---

## Phase 1: Contracts and schemas

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Define stable data contracts before implementing behavior that depends on those contracts.

### Scope

Add concrete schema definitions for:

* objective
* constraints
* domain
* candidate solution
* evaluation result
* governance result
* promotion decision
* reuse event
* audit record
* adapter request
* adapter response
* policy

Add matching Rust contract structures in the trusted core.

### Allowed

* concrete Draft 2020-12 schemas
* Rust contract structs and enums
* contract-shaped example YAML files
* documentation explaining contract ownership
* minimal module wiring required for compilation

### Not allowed

* runtime validation engines
* YAML parsing
* schema validation commands unless explicitly scoped
* lifecycle transitions
* adapter execution
* evaluator execution
* governance approval
* promotion logic
* ledger persistence
* replay
* UI behavior

### Validation gate

Required commands:

```sh
cargo check --workspace
./scripts/check.sh
```

Expected result:

* Every schema has `$schema`, `title`, `type`, `required`, `properties`, and `additionalProperties: false`.
* Every schema has a matching Rust contract or documented owner.
* Contract names align with schema file names.
* No provider-specific fields leak into core contracts.
* No UI-specific authority leaks into core contracts.
* Documentation states contract-shape scope only.

### Exit criteria

Phase 1 is complete when contract shapes are explicit enough to support lifecycle, evaluation, governance, audit, and replay work without guessing.

---

## Phase 2: Candidate lifecycle state machine

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Implement authoritative candidate lifecycle transitions in Rust.

### Scope

Implement candidate lifecycle states:

* `CREATED`
* `EVALUATING`
* `FAILED`
* `BLOCKED`
* `PASSED`
* `PROMOTED_TIER_1`
* `REJECTED`
* `UNKNOWN`

Implement legal transitions and typed lifecycle errors.

### Allowed

* lifecycle state enum
* legal transition checks
* typed lifecycle errors
* lifecycle unit tests
* documentation stating lifecycle boundary

### Not allowed

* model generation
* evaluator execution
* governance approval
* promotion authorization
* ledger persistence
* replay
* UI state mutation
* Python lifecycle decisions

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* valid transitions pass
* invalid transitions fail
* `CREATED -> PROMOTED_TIER_1` fails
* `UNKNOWN -> PASS` fails
* terminal states remain terminal
* Python cannot set lifecycle state

### Exit criteria

Phase 2 is complete when lifecycle transitions are typed, tested, and impossible to bypass through ordinary Rust APIs.

### Boundary note

`PROMOTED_TIER_1` is a lifecycle state shape. Authorization to enter this state belongs to future Rust governance promotion logic.

---

## Phase 3: CLI static validation surface

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Provide a usable command-line surface for static run-directory checks.

### Scope

Add initial CLI commands:

```sh
ajentic validate <run-dir>
ajentic inspect <run-dir>
```

Static validation checks only:

* required file presence
* non-empty file contents
* expected plain-text markers

Required files:

* `objective.yaml`
* `constraints.yaml`
* `domain.yaml`
* `policy.yaml`

### Allowed

* minimal CLI dispatch
* static run-directory checks
* clear command errors
* inspect output showing file presence and byte lengths
* tests for success and expected failure paths

### Not allowed

* YAML parsing
* JSON Schema validation
* contract deserialization
* model calls
* candidate generation
* lifecycle mutation
* governance approval
* promotion logic
* replay
* ledger writes
* audit emission

### Validation gate

Required commands:

```sh
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
cargo test --workspace
./scripts/check.sh
```

Expected result:

* Valid static example directory passes.
* Missing files fail.
* Empty files fail.
* Missing expected markers fail.
* CLI errors are deterministic and clear.

### Exit criteria

Phase 3 is complete when a first-time developer can validate and inspect the example run directory and receive clear static-shape feedback.

### Boundary note

Static validation is not full run validation. It does not prove objective validity, constraint validity, policy approval, candidate quality, governance readiness, or promotion eligibility.

---

## Phase 4: Adapter protocol and mock adapter

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Establish the Rust-to-Python adapter boundary without calling real models.

### Scope

Add:

* adapter request shape
* adapter response shape
* deterministic Python mock adapter
* Rust subprocess invocation
* adapter response validation
* malformed response handling
* linkage checks
* output-size checks

### Allowed

* dependency-free mock adapter protocol
* deterministic Python mock adapter
* Rust subprocess runner
* response parsing
* response linkage validation
* typed adapter errors
* tests for malformed and mismatched responses

### Not allowed

* real provider calls
* API keys or credential loading
* candidate creation, except in later Phase 5 code
* lifecycle mutation
* evaluator execution
* governance approval
* promotion logic
* ledger writes
* replay
* audit emission
* UI behavior

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Recommended manual check:

```sh
python adapters/python/ajentic_adapter/providers/mock_adapter.py < valid-mock-request-input
```

Expected result:

* Rust can call the mock Python adapter.
* Malformed adapter output is rejected.
* Mismatched run or candidate request IDs are rejected.
* Over-limit output is rejected.
* Adapter output cannot promote a candidate.
* Adapter output cannot set authoritative lifecycle state.

### Exit criteria

Phase 4 is complete when Rust can call a deterministic mock Python adapter and reject malformed or mismatched responses.

### Boundary note

A completed adapter response is untrusted generated output only. It is not a passing candidate, not governance approval, and not a promotion decision.

The current line protocol is an early deterministic mock boundary. It is not necessarily the final provider adapter contract.

---

## Phase 5: Candidate creation and runtime adapter checks

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Convert validated adapter output into Rust-owned candidate records.

### Scope

Implement candidate creation after adapter response validation.

Runtime checks include:

* response linkage
* completed adapter status
* required content presence
* run ID consistency
* domain ID consistency
* objective ID consistency
* constraints ID consistency
* deterministic candidate ID assignment
* initial lifecycle state `CREATED`

### Allowed

* Rust-owned candidate creation API
* deterministic candidate ID derivation
* candidate record shape
* checks for missing or empty creation fields
* typed candidate creation errors
* tests for success and expected failure paths

### Not allowed

* real provider calls
* evaluator execution
* evaluation result ingestion, except in later Phase 6 code
* governance approval
* promotion logic
* lifecycle transition beyond initial `CREATED`
* ledger persistence
* replay
* audit emission
* UI behavior

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* valid completed adapter output creates a candidate in `CREATED`
* malformed adapter output does not create a candidate
* non-completed adapter statuses do not create a candidate
* candidate ID is assigned by Rust
* lifecycle state is assigned by Rust
* empty required fields fail

### Exit criteria

Phase 5 is complete when a candidate can be created only through validated Rust-controlled flow.

### Boundary note

Candidate creation is not validation. Created candidates remain untrusted. They are not evaluated, governed, promoted, persisted, replayed, or audited by this phase.

---

## Phase 6: Evaluation result ingestion

Milestone group: Milestone 1

Status: complete pending final Owner acceptance, if not already accepted

Primary goal: Record structured evaluation results without implementing governance or promotion.

### Scope

Add evaluation result handling for:

* `PASS`
* `FAIL`
* `BLOCKED`
* `UNKNOWN`

Implement:

* evaluation result record shape
* candidate-to-result linking
* in-memory evaluation result set
* required evaluator presence checks
* required evaluator satisfaction checks
* failure reason recording
* evidence reference preservation
* `UNKNOWN` handling

### Allowed

* Rust-owned evaluation ingestion
* result linkage checks
* required evaluator presence checks
* required evaluator satisfaction checks
* typed evaluation ingestion errors
* tests for pass, fail, blocked, unknown, missing, malformed, and duplicate results

### Not allowed

* evaluator execution
* Python evaluator invocation
* model-assisted evaluation
* governance approval
* promotion eligibility
* lifecycle mutation
* ledger persistence
* replay
* audit emission
* UI behavior

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* `PASS` result is recorded
* `FAIL` result is recorded
* `BLOCKED` result is recorded
* `UNKNOWN` result is recorded
* `UNKNOWN` does not satisfy required evaluator
* `FAIL` does not satisfy required evaluator
* `BLOCKED` does not satisfy required evaluator
* missing evaluator does not satisfy required evaluator
* duplicate result IDs fail
* duplicate evaluator IDs fail
* evaluation ingestion does not mutate lifecycle state

### Exit criteria

Phase 6 is complete when evaluation results can be recorded and linked to candidates, and required evaluator satisfaction can be computed without promoting candidates.

### Boundary note

Required evaluator satisfaction is not promotion eligibility. It is only an input to future governance.

---

## Phase 7: Governance and promotion gates

Milestone group: Milestone 2

Status: not started

Primary goal: Implement Rust-only governance review and Tier-1 promotion decision.

### Scope

Implement:

* governance precheck
* runtime governance result
* policy check result model
* promotion eligibility check
* promotion decision record
* Rust-only transition into `PROMOTED_TIER_1`

A candidate may become Tier-1 only if:

* objective is valid
* constraints are valid
* domain is valid
* required evaluators completed
* no required evaluator returned `FAIL`
* no required evaluator returned `BLOCKED`
* no required evaluator returned unresolved `UNKNOWN`
* policy checks passed
* required evidence references exist

### Allowed

* explicit Rust governance checks
* explicit Rust policy check surface
* promotion eligibility function
* promotion decision record creation
* typed governance and promotion errors
* lifecycle transition to `PROMOTED_TIER_1` only through governance promotion logic
* tests for pass and fail promotion paths

### Not allowed

* promotion logic in Python
* promotion logic in TypeScript
* promotion logic in Bash
* promotion logic in CLI wrappers
* duplicate promotion checks outside Rust governance
* ledger persistence
* replay
* audit emission beyond any record shape explicitly scoped here
* real provider integrations
* UI implementation

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* all required checks pass permits promotion
* `FAIL` blocks promotion
* `BLOCKED` blocks promotion
* unresolved `UNKNOWN` blocks promotion
* missing evaluator blocks promotion
* missing evidence blocks promotion
* malformed governance result blocks promotion
* Python adapter approval field is ignored or rejected
* UI cannot promote directly
* CLI cannot bypass governance
* promotion logic has one authoritative Rust implementation

### Exit criteria

Phase 7 is complete when the harness can promote a candidate to Tier-1 through Rust-only governance and deny promotion for every required failure case.

---

## Phase 8: Append-only ledger and audit records

Milestone group: Milestone 3

Status: not started

Primary goal: Preserve the decision path from objective to promotion.

### Scope

Implement append-only ledger entries for:

* run started
* objective validated
* constraints validated
* domain validated
* governance precheck
* adapter requested
* adapter response accepted or rejected
* candidate created
* evaluation recorded
* governance reviewed
* promotion decided
* audit emitted

Implement audit records that include:

* run ID
* objective ID
* constraints ID
* domain ID
* candidate solution IDs
* evaluation result IDs
* governance result IDs
* promotion decision IDs
* final status
* failure or blocked reasons when applicable

### Allowed

* append-only in-memory or file-backed ledger, depending on scoped implementation choice
* typed ledger entries
* typed append errors
* audit record shape
* audit emission from recorded facts
* ordering checks
* traceability tests

### Not allowed

* ledger mutation
* editing previous entries
* replay regeneration
* model calls during audit
* UI authority
* audit records that invent missing evidence

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* ledger append preserves order
* invalid append fails
* promoted candidate has full audit trace
* failed candidate has failure reason
* blocked candidate has blocked reason
* audit does not invent missing evidence
* audit trace links objective, constraints, domain, candidate, evaluations, governance, and promotion decision

### Exit criteria

Phase 8 is complete when every governed run can produce an audit trail sufficient to explain the final result.

---

## Phase 9: Replay

Milestone group: Milestone 3

Status: not started

Primary goal: Reconstruct the authoritative decision path from ledger records without regenerating.

### Scope

Implement:

* replay from ledger records
* replay comparison output
* replay mismatch errors
* replay CLI command if scoped
* no adapter calls during replay
* no provider calls during replay

Replay must not call:

* LLM providers
* Python adapters
* TypeScript UI
* Bash scripts
* remote services

Replay operates on recorded facts only.

### Allowed

* Rust replay engine
* ledger reconstruction
* replay mismatch detection
* tests for valid and corrupted replay paths
* CLI replay surface if scoped

### Not allowed

* model regeneration
* adapter invocation
* provider calls
* UI-derived truth
* script-derived truth
* network access
* silent repair of corrupt ledger records

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

If replay CLI exists:

```sh
cargo run -p ajentic-cli -- replay <fixture-run>
```

Required tests:

* replay reconstructs candidate state
* replay reconstructs promotion decision
* replay detects missing ledger entries
* replay detects invalid ordering
* replay does not invoke adapter
* replay does not regenerate candidate content

### Exit criteria

Phase 9 is complete when a run can be replayed and verified without model access.

---

## Phase 10: Local model adapter capability

Milestone group: Milestone 4

Status: not started

Primary goal: Add the first real local model adapter while preserving non-authority.

### Scope

Add one local provider adapter through the existing adapter boundary.

Candidate options:

* Ollama
* llama.cpp server
* vLLM local endpoint

The Owner must choose one provider for this phase.

### Allowed

* one local model provider adapter
* local provider availability checks
* provider timeout/error mapping
* adapter output through the same untrusted protocol
* tests using mockable provider behavior

### Not allowed

* provider-specific promotion logic
* provider-specific lifecycle logic
* provider-specific governance shortcuts
* direct candidate approval from local provider output
* weakening adapter validation
* cloud provider integration

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Optional manual command if configured:

```sh
cargo run -p ajentic-cli -- run examples/minimal_run --adapter local
```

Required tests:

* provider unavailable returns clear adapter failure
* provider timeout blocks or fails candidate
* malformed provider output is rejected
* successful generation still requires evaluation
* local adapter cannot promote

### Exit criteria

Phase 10 is complete when AJENTIC can run against one local model adapter without weakening the governance path.

---

## Phase 11: Cloud model adapter capability

Milestone group: Milestone 4

Status: not started

Primary goal: Add the first cloud model adapter while preserving the same adapter boundary.

### Scope

Add one cloud provider adapter through the existing adapter protocol.

Candidate options:

* OpenAI
* Anthropic
* Gemini

The Owner must choose one provider for this phase.

### Allowed

* one cloud model provider adapter
* environment-variable credential lookup
* missing credential failure
* provider error mapping
* provider timeout handling
* token/cost metadata as operational metadata only

### Not allowed

* credential logging
* committed secrets
* provider-specific promotion logic
* provider-specific lifecycle logic
* provider-specific governance shortcuts
* treating token or cost metadata as quality evidence unless explicitly configured
* weakening adapter validation

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Optional manual command if configured:

```sh
cargo run -p ajentic-cli -- run examples/minimal_run --adapter cloud
```

Required tests:

* missing API key fails safely
* provider error maps to adapter failure
* timeout blocks or fails candidate
* malformed cloud response is rejected
* successful cloud response still requires evaluation

### Exit criteria

Phase 11 is complete when AJENTIC can run against one cloud provider through the same non-authoritative adapter boundary.

---

## Phase 12: TypeScript UI review surface

Milestone group: Milestone 5

Status: not started

Primary goal: Implement the initial UI for visibility, review, and audit inspection.

### Scope

Create UI views for:

* run overview
* candidate list
* candidate detail
* evaluation result detail
* governance result detail
* promotion decision detail
* audit trail
* artifact or raw record view
* status labels for all lifecycle states

The UI must display Rust-generated state. It must not compute authoritative promotion eligibility.

### Allowed

* TypeScript UI views
* generated or hand-maintained read-only types, as scoped
* API client surfaces that request actions
* clear display of status distinctions
* tests for display and request behavior

### Not allowed

* UI-owned lifecycle transitions
* UI-owned governance decisions
* UI-owned promotion eligibility
* hiding failed, blocked, unknown, or missing checks
* collapsing blocked and failed into a generic error
* local UI-only truth

### Required UI status labels

The UI must distinguish:

* `CREATED`
* `EVALUATING`
* `FAILED`
* `BLOCKED`
* `PASSED`
* `PROMOTED_TIER_1`
* `REJECTED`
* `UNKNOWN`

### Validation gate

Required commands:

```sh
cargo test --workspace
npm test --prefix ui
./scripts/check.sh
```

Required tests:

* UI displays failed checks
* UI displays blocked checks
* UI displays unknown checks
* UI does not hide missing evidence
* UI does not compute promotion result independently

### Exit criteria

Phase 12 is complete when a developer can inspect a run and understand why it promoted, failed, or blocked without the UI becoming an authority source.

---

## Phase 13: Multi-domain capability

Milestone group: Milestone 6

Status: not started

Primary goal: Support materially different domains without changing the core lifecycle.

### Scope

Add at least two domain profiles.

Example domains:

* document summary
* code patch review
* security analysis
* financial research

Domains may configure:

* objective types
* constraint types
* evaluator lists
* known failure modes
* thresholds
* prohibited outputs
* evidence requirements

Domains may not change:

* candidate lifecycle
* promotion semantics
* ledger rules
* replay rules
* unknown-is-not-pass rule
* Rust authority boundary
* adapter trust boundary

### Allowed

* domain profiles
* domain compatibility checks
* domain-specific evaluator wiring
* domain-specific failure mode declarations
* tests across at least two domains

### Not allowed

* domain-owned lifecycle overrides
* domain-owned promotion overrides
* domain-owned ledger semantics
* domain-owned replay semantics
* domain-specific bypasses around governance

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* domain A validates with its own evaluator set
* domain B validates with its own evaluator set
* missing domain fails
* evaluator mismatch fails
* domain cannot override lifecycle
* domain cannot override promotion semantics

### Exit criteria

Phase 13 is complete when AJENTIC can run at least two distinct domains through the same core control path.

---

## Phase 14: Reuse event and bounded improvement records

Milestone group: Milestone 6

Status: not started

Primary goal: Record reuse and bounded improvement without allowing self-modifying governance.

### Scope

Implement reuse event records for:

* strategy reuse
* prompt pattern reuse
* evaluator ordering reuse
* repair pattern reuse
* candidate structure reuse

Reuse is advisory until validated in the new context.

### Allowed

* reuse event records
* source and target references
* validation status for reuse
* mismatch notes
* tests proving reuse does not bypass validation

### Not allowed

* automatic promotion based on prior success
* self-modifying governance rules
* silent threshold changes
* policy relaxation from historical performance
* reuse-based lifecycle overrides
* reuse-based promotion shortcuts

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
```

Required tests:

* reuse event records source and target
* reuse event with missing source fails
* reuse event does not skip validation
* reuse event cannot change promotion rules
* reused pattern still requires full evaluation

### Exit criteria

Phase 14 is complete when reuse is visible, bounded, logged, and non-authoritative.

---

## Phase 15: Hardening and negative testing

Milestone group: Milestone 7

Status: not started

Primary goal: Test failure paths, invalid inputs, and boundary violations.

### Scope

Add negative test suites for:

* malformed schemas
* invalid lifecycle transitions
* adapter timeout
* adapter malformed output
* missing objective
* missing constraints
* missing domain
* missing evaluator
* unknown result
* ledger corruption
* replay mismatch
* attempted UI authority
* attempted Python authority
* script misuse

### Allowed

* negative unit tests
* negative integration tests
* fixture replay tests
* schema failure tests
* adapter failure tests
* CLI failure tests
* UI display failure tests
* authority-boundary violation tests

### Not allowed

* weakening fail-closed rules to pass tests
* test-only bypasses
* hidden permissive modes
* undocumented exception paths

### Validation gate

Required commands:

```sh
cargo test --workspace
./scripts/check.sh
npm test --prefix ui
```

Required fixture checks if replay fixtures exist:

```sh
cargo run -p ajentic-cli -- replay fixtures/runs/promoted
cargo run -p ajentic-cli -- replay fixtures/runs/blocked
cargo run -p ajentic-cli -- replay fixtures/runs/failed
```

Required tests:

* failure paths are tested, not only success paths
* bad adapter output cannot promote
* bad evaluator output cannot promote
* corrupt ledger cannot replay as valid
* UI cannot hide governance failure
* CLI cannot bypass governance

### Exit criteria

Phase 15 is complete when common misuse and failure classes are covered by tests and invalid, missing, malformed, blocked, and unknown states fail closed.

---

## Phase N: Early production capability

Milestone group: Milestone 8

Status: not started

Primary goal: AJENTIC is usable for controlled early production review workflows.

This is not full production maturity. It is the point at which AJENTIC can safely run governed review workflows with explicit limitations.

### Required capabilities

AJENTIC must support:

* static run validation
* local model adapter
* cloud model adapter
* candidate generation
* runtime governance checks
* structured evaluation results
* Rust-only Tier-1 promotion
* append-only ledger
* replay
* audit records
* UI review surface
* at least two domains
* reuse event logging
* negative test coverage

### Required operational properties

AJENTIC must provide:

* clear first-run documentation
* clear failure messages
* no secret leakage in logs
* no unsafe bypass flags
* reproducible replay from recorded facts
* explicit environment configuration
* versioned schemas
* versioned adapter protocol
* maintained changelog

### Validation gate

Required commands before tagging the milestone:

```sh
cargo test --workspace
./scripts/check.sh
npm test --prefix ui
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- run examples/minimal_run --adapter mock
cargo run -p ajentic-cli -- replay fixtures/runs/promoted
cargo run -p ajentic-cli -- audit fixtures/runs/promoted
```

Manual validation:

* run a mock adapter workflow
* run one local model workflow if configured
* run one cloud model workflow if configured
* confirm failed candidates do not promote
* confirm `UNKNOWN` results do not promote
* confirm UI displays full audit state
* confirm replay does not call any model provider

### Early production limitations

At this milestone, AJENTIC must still declare that it is:

* not a production deployment approval system
* not a replacement for human review
* not a model safety guarantee
* not a training system
* not a fully autonomous self-improving agent
* not a guarantee that Tier-1 output is correct

A Tier-1 output means the candidate passed configured governance and quality thresholds for downstream review. It does not mean automatic production release.

### Exit criteria

Phase N is complete when AJENTIC can support controlled early production review workflows while preserving Rust authority, replayability, traceability, and fail-closed behavior.

---

## 9. Structural milestone summary

### Milestone A: Contract freeze

Occurs after Phase 1.

Validation question:

Can lifecycle, evaluation, governance, and audit implementation proceed without inventing missing contract fields?

Required result:

Yes. Contracts are sufficient for downstream implementation.

---

### Milestone B: Adapter boundary freeze

Occurs after Phase 4.

Validation question:

Can model providers change without changing candidate lifecycle or promotion logic?

Required result:

Yes. Providers are behind the adapter protocol.

---

### Milestone C: Promotion authority freeze

Occurs after Phase 7.

Validation question:

Is there exactly one authoritative promotion path?

Required result:

Yes. Rust governance/promotion owns Tier-1 promotion.

---

### Milestone D: Replay integrity freeze

Occurs after Phase 9.

Validation question:

Can the harness reconstruct decisions without regenerating model output?

Required result:

Yes. Replay uses recorded facts only.

---

### Milestone E: Provider parity

Occurs after Phase 11.

Validation question:

Do local and cloud models use the same governance path?

Required result:

Yes. Provider source does not affect trust semantics.

---

### Milestone F: Cross-domain stability

Occurs after Phase 13.

Validation question:

Can materially different domains run without changing the control layer?

Required result:

Yes. Domains configure evaluation, not lifecycle authority.

---

### Milestone G: Failure hardening

Occurs after Phase 15.

Validation question:

Do invalid, missing, malformed, blocked, and unknown states fail closed?

Required result:

Yes. Failure paths are tested and visible.

---

## 10. Roadmap control rule

Future phases may extend capability only if they preserve the existing trust model.

A phase must not be accepted if it makes the system more impressive while making it harder to audit, harder to explain, or easier to bypass.

For AJENTIC, progress means stricter control with better usability, not more automation at the expense of governance.
