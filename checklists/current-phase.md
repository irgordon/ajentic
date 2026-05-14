---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 170.16

## Phase name
- [x] Phase 170.16 - Out-of-Band Local Operator Shell Projection Extraction.

## Phase goal
- [x] Continue behavior-preserving decomposition of `core/src/api/local_operator_shell.rs` before Phase 171 begins.
- [x] Move one coherent production projection/helper family into a smaller sibling Rust module.
- [x] Do not implement Phase 171 release-candidate preparation behavior.

## Working-tree hygiene gate
- [x] Started from an inspected working tree with `git status --short`.
- [x] Kept changes limited to allowed Phase 170.16 surfaces.
- [x] Cleaned generated artifacts and build-only outputs from the repository working tree.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_workflow.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [x] No UI, schema, roadmap, governance, architecture, package, lockfile, CI, release, or deployment surfaces changed.

## Extraction target checklist
- [x] Required extraction target remained `core/src/api/local_operator_shell.rs`.
- [x] Selected the complete local operator workflow projection family.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Created a non-empty sibling Rust module owning moved production projection/helper code.
- [x] Avoided circular module dependencies.
- [x] Did not repeat previously extracted Phase 170 helper families.

## Projection/helper movement checklist
- [x] Moved complete workflow projection status, step status, step kind, and error enums.
- [x] Moved complete workflow projection structs and capability surface type.
- [x] Moved workflow step ordering, step construction, classification, evidence summary, current blocker, and projection derivation helpers.
- [x] Moved code mechanically without redesign.

## Projection output preservation checklist
- [x] Preserved complete workflow projection fields and field values.
- [x] Preserved evidence summary strings and ordering.
- [x] Preserved local-only and no-authority projection notes.
- [x] Preserved boundary status projection output.

## Status classification preservation checklist
- [x] Preserved workflow status-code strings.
- [x] Preserved workflow step status-code strings.
- [x] Preserved current blocker selection logic.
- [x] Preserved rejection reason construction and ordering.

## Behavior-preservation checklist
- [x] Preserved runtime semantics.
- [x] Preserved validation outcomes.
- [x] Preserved reason strings and status-code strings.
- [x] Preserved deterministic ordering.
- [x] Preserved serialized formats.
- [x] Preserved UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-exported moved public workflow projection types and helpers through `local_operator_shell.rs`.
- [x] Kept `core/src/api/mod.rs` stable.
- [x] Did not rename public types, functions, or enum variants.
- [x] Avoided broad new public module exports.

## Test preservation checklist
- [x] Kept `core/src/api/local_operator_shell_tests.rs` intact.
- [x] Did not delete tests.
- [x] Did not weaken assertions.
- [x] Did not mark tests ignored.
- [x] Did not relax snapshot text.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-16-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-16-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-16-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Remaining-shell-function scan.
- [x] Moved-code scan.
- [x] Helper location scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining local shell session, provider adapter, and trial execution projection code remains extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, controlled-human-use, or production approval is introduced.

## Deferred items
- [x] Broader local shell projection decomposition is deferred.
- [x] Session history / restore projection extraction is deferred.
- [x] Provider adapter and controlled internal trial execution projection extraction is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.16 surfaces.
- [x] At least one coherent projection/helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Projection output is unchanged.
- [x] Status classification is unchanged.
- [x] Boundary marker strings are unchanged.
- [x] Capability flags are unchanged.
- [x] Reason strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, or public-use behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
