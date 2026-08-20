---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 197.2 - Task Outcome Evidence Binding

- Starting baseline:
  - [x] Verify Phase 197.1 head `73315721ce4986dfb93c0673c174cfdb6dcfd0de`.
  - [x] Branch `mothra/phase-197-2-outcome-evidence-binding` from that exact head.
  - [x] Confirm the starting worktree was clean.
- TaskContract mapping:
  - [x] Add `required_postcondition_ids` to every success criterion.
  - [x] Reject duplicate criterion and postcondition IDs.
  - [x] Reject empty, duplicate, and unknown postcondition mappings.
  - [x] Require every required criterion to map only required postconditions.
  - [x] Require every required postcondition to be mapped by a required criterion.
  - [x] Preserve deterministic TaskContract validation.
- Outcome authority removal:
  - [x] Remove caller-controlled postcondition requiredness.
  - [x] Remove caller-supplied satisfied criterion IDs.
  - [x] Replace checks with `PostconditionObservation` source records.
  - [x] Align Rust with action-outcome evidence references and evidence digests.
- Rust derivation:
  - [x] Validate exact TaskContract postcondition identity.
  - [x] Reject unknown and duplicate postcondition observations.
  - [x] Require evidence references, evidence digests, and observed-value digest binding for observed success.
  - [x] Derive private postcondition results.
  - [x] Derive private criterion outcomes using all mapped postconditions.
  - [x] Aggregate evidence across multiple actions and retries deterministically.
  - [x] Preserve unresolved same-retry conflicts as `UNKNOWN`.
  - [x] Preserve prior errors and partial side effects after later success.
- Schema alignment:
  - [x] Add criterion-to-postcondition mapping to `task-contract.schema.json`.
  - [x] Remove caller requiredness from `action-outcome.schema.json`.
  - [x] Add postcondition observation states and observation evidence digests.
  - [x] Confirm no generated TypeScript contract artifacts require regeneration.
- Regression coverage:
  - [x] Add TaskContract structural negative cases.
  - [x] Add action observation, evidence, requiredness, and criterion derivation cases.
  - [x] Add cross-module multi-action completion tests.
  - [x] Add source-lint tripwires for caller-authored criterion IDs and `PostconditionCheck`.
  - [x] Confirm Rust boundary lint self-tests pass (`28/28`).
- Scope boundaries:
  - [x] Phase 197.1 validation and policy verifier semantics remain unchanged.
  - [x] ClaimEvidence and ClaimReport semantics remain unchanged.
  - [x] No provider, promotion, approval, memory, context, persistence, replay, UI, workflow, package, deployment, or release authority expansion.
- Validation:
  - [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`.
  - [x] `cargo test --manifest-path core/Cargo.toml --all-targets --no-run`.
  - [x] `cargo test --manifest-path core/Cargo.toml --all-targets` (`1,264` tests).
  - [x] `cargo test --manifest-path core/Cargo.toml --test outcome_evidence_binding` (`7/7`).
  - [x] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`.
  - [x] Structure, contract-map, documentation, Rust/UI boundary, help, and schema validation.
  - [x] `cd ui && npm run typecheck && npm run test:api` (`141/141`).
  - [x] Deterministic artifact and release-integrity checks.
  - [x] `git diff --check`.
  - [x] Full clean-tree `./scripts/check.sh` after commit (`All checks passed.`).
