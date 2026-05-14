---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.15 - Out-of-Band Local Operator Shell Helper Extraction

## Phase goal
- [x] Continue behavior-preserving decomposition of `core/src/api/local_operator_shell.rs` before Phase 171 begins.
- [x] Move one coherent production helper family into a smaller sibling Rust module.
- [x] Do not implement Phase 171 release-candidate preparation behavior.

## Working-tree hygiene gate
- [x] Started from an inspected working tree with `git status --short`.
- [x] Kept changes limited to allowed Phase 170.15 surfaces.
- [x] Cleaned generated artifacts and build-only outputs from the repository working tree.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_boundary_markers.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [x] No UI, schema, roadmap, governance, architecture, package, lockfile, CI, release, or deployment surfaces changed.

## Extraction target checklist
- [x] Required extraction target remained `core/src/api/local_operator_shell.rs`.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Created a non-empty sibling Rust module owning moved production helper code.
- [x] Avoided circular module dependencies.
- [x] Did not repeat transport routing extraction.

## Helper movement checklist
- [x] Moved repeated boundary-status helper lists.
- [x] Moved repeated absence-marker helper constructors.
- [x] Moved repeated capability-surface helper constructors.
- [x] Moved directly related stop-condition helper list.
- [x] Moved code mechanically without redesign.

## Boundary marker preservation checklist
- [x] Preserved existing boundary-status helper outputs.
- [x] Preserved boundary marker strings through unchanged enum `code()` mappings.
- [x] Preserved deterministic ordering of boundary-status vectors.
- [x] Did not remove boundary markers.

## Capability preservation checklist
- [x] Preserved local-only and non-production capability booleans.
- [x] Preserved no-authority and no-effect capability booleans.
- [x] Preserved capability summary wording.
- [x] Did not add provider execution, persistence authority, release, deployment, signing, publishing, public-use, or readiness capabilities.

## Absence marker preservation checklist
- [x] Preserved absence-marker booleans.
- [x] Preserved absence-marker summary strings.
- [x] Preserved absence-marker ordering.
- [x] Did not collapse no-authority or absence markers.

## Behavior-preservation checklist
- [x] Preserved runtime semantics.
- [x] Preserved validation outcomes.
- [x] Preserved reason strings and status-code strings.
- [x] Preserved deterministic ordering.
- [x] Preserved serialized formats.
- [x] Preserved UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-exported moved public helper functions through `local_operator_shell.rs`.
- [x] Kept `core/src/api/mod.rs` stable.
- [x] Did not rename public types, functions, or enum variants.
- [x] Used narrow `pub(super)` visibility for moved helpers that were private to the local shell surface.

## Test preservation checklist
- [x] Kept `core/src/api/local_operator_shell_tests.rs` intact.
- [x] Did not delete tests.
- [x] Did not weaken assertions.
- [x] Did not mark tests ignored.
- [x] Did not relax snapshot text.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-15-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-15-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-15-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Helper moved-code scan.
- [x] Moved-helper location scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining local shell helper families and projection code remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, controlled-human-use, or production approval is introduced.

## Deferred items
- [x] Broader local shell projection decomposition is deferred.
- [x] Additional serialization/helper-family extraction is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.15 surfaces.
- [x] At least one coherent helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Boundary marker strings are unchanged.
- [x] Capability flags are unchanged.
- [x] Absence marker ordering is unchanged.
- [x] Reason strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, or public-use behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
