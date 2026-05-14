---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
category: active_phase
status: active
phase: 170.14
---

# Current Phase Checklist - Phase 170.14

## Phase name
- [x] Phase 170.14 - Out-of-Band Local Operator Shell Transport Extraction.

## Phase goal
- [x] Extract the local shell transport request/response routing production-code family from `core/src/api/local_operator_shell.rs` into a smaller sibling Rust module.
- [x] Preserve behavior exactly while reducing the local shell monolith before Phase 171.
- [x] Do not implement Phase 171 release-candidate preparation behavior.

## Working-tree hygiene gate
- [x] Run `git status --short` before editing.
- [x] Read `docs/operations/rust-maintainability-audit-phase-170-5.md`.
- [x] Inspect the Phase 170.13 local shell extraction end state.
- [x] Keep generated artifacts and temporary build output outside the repository.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_transport.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [ ] `core/src/api/local_operator_shell_tests.rs` only if direct transport test movement is required.
- [ ] Existing extracted local shell sibling modules only if import visibility requires narrow adjustment.
- [ ] `core/src/api/mod.rs` only if a public module export is strictly required.
- [ ] `tests/**/*.rs` only if import paths must be corrected.

## Extraction target checklist
- [x] Required target: `core/src/api/local_operator_shell.rs`.
- [x] Created `core/src/api/local_operator_shell_transport.rs` as a non-empty sibling module owning moved production transport code.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Avoided circular module dependencies.
- [x] Did not repeat provider pipeline, candidate materialization, trial verification, trial observability, trial review, or codec extraction families.

## Transport movement checklist
- [x] Moved the local shell transport status enum.
- [x] Moved the forbidden request enum and rejection-code mapping.
- [x] Moved the local shell request enum without changing variants.
- [x] Moved the capabilities and response types.
- [x] Moved `LocalOperatorShellTransport` and its stateful `step` surface.
- [x] Moved transport convenience helper functions.
- [x] Moved `local_operator_shell_transport_step` and variant-specific routing arms.
- [x] Moved accepted/rejected response helpers.
- [x] Moved code mechanically without redesign.

## Request/response preservation checklist
- [x] Preserve every existing request variant name and payload type.
- [x] Preserve the response shape and public fields.
- [x] Preserve response status values.
- [x] Preserve reason strings.
- [x] Preserve capabilities values.
- [x] Preserve local session evidence export attachment behavior.

## State-transition preservation checklist
- [x] Preserve accepted-request state updates in `LocalOperatorShellTransport::step`.
- [x] Preserve rejected-request prior-state behavior.
- [x] Preserve deterministic stub-run state transition behavior.
- [x] Preserve provider configuration and adapter declaration routing behavior.
- [x] Preserve adapter dry-run and constrained invocation rejection-state behavior.
- [x] Preserve staged proposal, validation, decision, and materialization routing behavior.
- [x] Preserve controlled internal trial start/step routing behavior.

## Fail-closed preservation checklist
- [x] Preserve fail-closed forbidden request handling.
- [x] Preserve invalid/rejected request response status.
- [x] Preserve authority, provider execution, readiness, production mutation, release artifact, deployment, signing, and publishing rejection codes.
- [x] Preserve no release artifact, deployment, signing, publishing, readiness, or production approval behavior.

## Behavior-preservation checklist
- [x] Preserve runtime semantics.
- [x] Preserve deterministic ordering.
- [x] Preserve validation outcomes.
- [x] Preserve boundary markers and no-authority markers.
- [x] Preserve serialized formats.
- [x] Preserve UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-export moved public transport types and functions through `local_operator_shell.rs`.
- [x] Do not rename public types, enum variants, or functions.
- [x] Keep `core/src/api/mod.rs` stable.
- [x] Do not add Phase 171 release-candidate preparation APIs.

## Test preservation checklist
- [x] Kept `core/src/api/local_operator_shell_tests.rs` intact.
- [x] Did not delete tests.
- [x] Did not weaken assertions.
- [x] Did not mark tests ignored.
- [x] Did not relax snapshot text.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-14-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-14-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-14-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Transport moved-code scan.
- [x] Transport behavior scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining local shell helper families remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation logic to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, or production approval is introduced.

## Deferred items
- [x] Boundary marker helper extraction is deferred.
- [x] Remaining serialization/helper-family extraction is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.14 surfaces.
- [x] The local shell transport request/response routing family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Request variants are unchanged.
- [x] Response shape is unchanged.
- [x] State-transition behavior is unchanged.
- [x] Rejected-request prior-state behavior is unchanged.
- [x] Fail-closed behavior is unchanged.
- [x] Reason strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, or public-use behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
