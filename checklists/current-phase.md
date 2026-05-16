---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.21

## Phase name
- [x] Phase 170.21 - Out-of-Band Local Operator Shell Final Extraction.

## Phase goal
- [x] Complete the final behavior-preserving decomposition pass on `core/src/api/local_operator_shell.rs` before Phase 171.
- [x] Move one last safe coherent shell initialization, state projection, or tightly scoped glue helper family into a sibling Rust module.
- [x] Keep Phase 171 release-candidate preparation unimplemented.

## Working-tree hygiene gate
- [x] Start from `git status --short`.
- [x] Restrict changes to allowed Phase 170.21 surfaces.
- [x] Do not leave generated artifacts or unrelated drift.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`.
- [x] `core/src/api/local_operator_shell_state.rs`.
- [x] Existing `core/src/api/local_operator_shell_*.rs` only for narrow import visibility if required.
- [x] `CHANGELOG.md`.
- [x] `checklists/current-phase.md`.

## Extraction target checklist
- [x] Select shell state initialization and state projection glue as the final extraction target.
- [x] Move production code; do not count test movement as sufficient.
- [x] Do not repeat prior extracted provider, candidate, trial, codec, transport, boundary marker, workflow, restore, adapter, constrained invocation, or trial execution families.
- [x] Avoid circular module dependencies.

## Final extraction checklist
- [x] Create `core/src/api/local_operator_shell_state.rs` with moved production code.
- [x] Keep `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Re-export moved public types and functions through `local_operator_shell.rs`.
- [x] Use narrow crate visibility only where existing sibling glue requires it.

## Initial-state preservation checklist
- [x] Initial harness status is unchanged.
- [x] Initial non-production flag is unchanged.
- [x] Initial run projection is unchanged.
- [x] Initial decision ledger and replay projection are unchanged.
- [x] Initial provider, candidate, session, workflow, trial, observability, and review projections are unchanged.
- [x] Initial refresh/recompute order is unchanged.

## Projection output preservation checklist
- [x] Local run projection fields and values are unchanged.
- [x] Local validation projection fields and values are unchanged.
- [x] Local session evidence export projection is unchanged.
- [x] Local workflow, trial failure drill, trial runbook, and controlled trial execution recomputation calls are unchanged.
- [x] Deterministic ordering is unchanged.

## Status classification preservation checklist
- [x] Local run status classification is unchanged.
- [x] Local decision replay status classification is unchanged.
- [x] Local session evidence export status classification is unchanged.
- [x] Existing status-code strings are unchanged.

## Behavior-preservation checklist
- [x] Runtime semantics are unchanged.
- [x] Request and response behavior is unchanged.
- [x] Reason strings are unchanged.
- [x] Boundary marker strings are unchanged.
- [x] Capability flags and capability surface behavior are unchanged.
- [x] Validation outcomes are unchanged.
- [x] Fail-closed behavior is unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.

## Public API preservation checklist
- [x] Public shell state, run projection, validation projection, candidate output, intent, initialization, stub-run, intent-application, and evidence-export helpers remain available through `local_operator_shell.rs`.
- [x] Public type names and enum variants are unchanged.
- [x] `core/src/api/mod.rs` remains stable.
- [x] No broad public module export was added.

## Test preservation checklist
- [x] Existing tests remain in place.
- [x] No tests were deleted.
- [x] No assertions were weakened.
- [x] No tests were marked ignored.
- [x] Snapshot text was not relaxed.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-21-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-21-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-21-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Remaining-shell-function scan.
- [x] Moved-code scan.
- [x] Helper location scan.
- [x] Boundary scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused final extraction.
- [x] Remaining local shell helper families remain future maintenance candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] Phase 171 should use a dedicated release-candidate preparation module.
- [x] Phase 171 local shell integration should remain thin.

## Deferred items
- [x] Broader local shell decomposition is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.
- [x] Release artifacts, signing, publishing, deployment, update-channel, installer, public-use, controlled-human-use, readiness, and production approval behavior remain out of scope.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.21 surfaces.
- [x] One final coherent shell initialization/projection helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Initial shell state behavior is unchanged.
- [x] Projection output is unchanged.
- [x] Status classification is unchanged.
- [x] Boundary marker strings are unchanged.
- [x] Capability flags are unchanged.
- [x] Reason strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, public-use, controlled-human-use, readiness, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase with a dedicated release-candidate preparation module and thin local shell integration.
