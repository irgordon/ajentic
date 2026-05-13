---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 170.10 Out-of-Band Local Operator Shell Extraction Continuation II

## Phase name
- [x] Phase 170.10 - Out-of-Band Local Operator Shell Extraction Continuation II.

## Phase goal
- [x] Move another coherent production-code family out of `core/src/api/local_operator_shell.rs`.
- [x] Preserve behavior exactly while reducing local shell monolith risk before Phase 171.
- [x] Keep Phase 171 as the next planned code-production phase.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Read `docs/operations/rust-maintainability-audit-phase-170-5.md` before extraction.
- [x] Inspect the Phase 170.9 end state with provider pipeline and candidate materialization already split.
- [x] Keep edits limited to allowed Phase 170.10 surfaces.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_*.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [ ] `core/src/api/local_operator_shell_tests.rs` only if direct test movement is required.
- [ ] `core/src/api/local_operator_shell_provider_pipeline.rs` only if import visibility requires adjustment.
- [ ] `core/src/api/local_operator_shell_candidate.rs` only if import visibility requires adjustment.
- [ ] `core/src/api/mod.rs` only if a public sibling export is strictly required.
- [ ] `tests/**/*.rs` only if import paths must be corrected.

## Extraction target checklist
- [x] Required target: `core/src/api/local_operator_shell.rs`.
- [x] Selected target: trial replay/restore verification types, projection helpers, mismatch/error model, and boundary helpers.
- [x] Create a non-empty sibling module owning moved production code.
- [x] Keep `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Avoid circular module dependencies.

## Production-code movement checklist
- [x] Move trial replay/restore verification status, mismatch, boundary status, comparison summary, projection, boundary list, initial projection, digest/mismatch helpers, claim-marker comparison, and projection derivation into `core/src/api/local_operator_shell_trial_verification.rs`.
- [x] Do not repeat Phase 170.8 provider output pipeline extraction.
- [x] Do not repeat Phase 170.9 local candidate materialization extraction.
- [x] Do not count Phase 170.7 test extraction as sufficient.
- [x] Do not redesign code.

## Behavior-preservation checklist
- [x] Preserve runtime semantics.
- [x] Preserve serialized formats.
- [x] Preserve deterministic ordering and digest/ID inputs.
- [x] Preserve reason strings and validation outcomes.
- [x] Preserve boundary markers and no-authority markers.
- [x] Preserve UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-export moved trial verification items through `local_operator_shell.rs`.
- [x] Do not rename public types, enum variants, or functions.
- [x] Keep `core/src/api/mod.rs` stable.
- [x] Do not add Phase 171 release-candidate preparation APIs.

## Test preservation checklist
- [x] Keep `core/src/api/local_operator_shell_tests.rs` intact.
- [x] Do not delete tests.
- [x] Do not weaken assertions.
- [x] Do not mark tests ignored.
- [x] Do not relax snapshot text.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-10-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-10-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-10-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Moved-code scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Trial observability, error reporting, evidence review, evidence codecs, boundary marker, and transport helpers remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation logic to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, or production approval is introduced.

## Deferred items
- [x] Trial observability/error-reporting extraction is deferred.
- [x] Trial evidence review extraction is deferred.
- [x] Trial package/session evidence codec extraction is deferred.
- [x] Boundary marker helper extraction is deferred.
- [x] Local shell transport extraction is deferred.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.10 surfaces.
- [x] At least one additional coherent production-code family moved out of `local_operator_shell.rs`.
- [x] Moved family is not the provider output pipeline extracted in Phase 170.8.
- [x] Moved family is not the local candidate materialization family extracted in Phase 170.9.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Serialized formats are unchanged.
- [x] Reason strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, or publishing behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
