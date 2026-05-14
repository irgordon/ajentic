---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.13 - Out-of-Band Local Operator Shell Codec Extraction

## Phase name
- [x] Phase 170.13 - Out-of-Band Local Operator Shell Codec Extraction.

## Phase goal
- [x] Continue behavior-preserving decomposition of `core/src/api/local_operator_shell.rs` before Phase 171.
- [x] Extract the trial package / trial session evidence codec production-code family into a smaller sibling Rust module.
- [x] Keep Phase 171 release-candidate preparation behavior unimplemented.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Read `docs/operations/rust-maintainability-audit-phase-170-5.md` before extraction.
- [x] Inspect the Phase 170.12 end state and existing extracted local shell sibling modules.
- [x] Keep edits limited to allowed Phase 170.13 surfaces.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_codecs.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [ ] `core/src/api/local_operator_shell_tests.rs` only if direct codec test movement is required.
- [ ] Existing extracted local shell modules only if import visibility requires narrow adjustment.
- [ ] `core/src/api/mod.rs` only if a public module export is strictly required.
- [ ] `tests/**/*.rs` only if import paths must be corrected.

## Extraction target checklist
- [x] Required target: `core/src/api/local_operator_shell.rs`.
- [x] Created `core/src/api/local_operator_shell_codecs.rs` as a non-empty sibling module owning moved production code.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Avoided circular module dependencies.
- [x] Did not repeat provider pipeline, candidate materialization, trial verification, trial observability, or trial review extraction families.

## Codec movement checklist
- [x] Moved controlled internal trial package deterministic content-basis and digest helpers.
- [x] Moved controlled internal trial package serialization, parsing, caller-path write/read, and read-back validation helpers.
- [x] Moved trial session evidence deterministic payload-basis and digest helpers.
- [x] Moved trial session evidence serialization, parsing, malformed-input handling, key/value decoding, list splitting, and read-back validation helpers.
- [x] Moved code mechanically without redesign.

## Serialization preservation checklist
- [x] Preserve serialized section headers.
- [x] Preserve serialized field names.
- [x] Preserve serialized field order.
- [x] Preserve caller-provided path write/read behavior for controlled internal trial packages.
- [x] Preserve deterministic digest inputs.

## Parsing preservation checklist
- [x] Preserve key/value parsing behavior.
- [x] Preserve malformed package input rejection.
- [x] Preserve malformed evidence input rejection.
- [x] Preserve status and stop-condition parser behavior.
- [x] Preserve digest mismatch rejection behavior.

## Read-back validation preservation checklist
- [x] Preserve controlled internal trial package read-back validation projection behavior.
- [x] Preserve trial session evidence read-back validation projection behavior.
- [x] Preserve validation error mapping and validation outcomes.
- [x] Preserve structure-only read-back semantics.

## Behavior-preservation checklist
- [x] Preserve runtime semantics.
- [x] Preserve reason strings and boundary notes.
- [x] Preserve deterministic ordering.
- [x] Preserve boundary markers and no-authority markers.
- [x] Preserve UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-export moved public codec functions through `local_operator_shell.rs`.
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
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-13-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-13-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-13-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Codec moved-code scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Local shell transport and remaining non-codec helper families remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation logic to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, or production approval is introduced.

## Deferred items
- [x] Local shell transport extraction is deferred.
- [x] Boundary marker helper extraction is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.13 surfaces.
- [x] The trial package / trial session evidence codec family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Serialized formats are unchanged.
- [x] Serialized field names are unchanged.
- [x] Serialized field ordering is unchanged.
- [x] Reason strings are unchanged.
- [x] Digest inputs are unchanged.
- [x] Parse/rejection behavior is unchanged.
- [x] Read-back validation behavior is unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, or public-use behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
