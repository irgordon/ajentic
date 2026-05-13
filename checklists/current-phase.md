---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.7 - Out-of-Band Rust Module Extraction Pass

## Phase name
- [x] Phase 170.7 - Out-of-Band Rust Module Extraction Pass.

## Phase goal
- [x] Reduce the highest-risk Rust monolith surface before Phase 171.
- [x] Move code mechanically into smaller modules while preserving behavior exactly.
- [x] Keep Phase 171 as the next planned code-production phase.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Read `docs/operations/rust-maintainability-audit-phase-170-5.md` before extraction.
- [x] Inspect the current Rust module layout before extraction.
- [x] Keep edits limited to allowed Phase 170.7 surfaces.

## Allowed surfaces
- [x] `core/src/**/*.rs`
- [x] `tests/**/*.rs`
- [x] `core/tests/**/*.rs` when present; no `core/tests` directory is present.
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [ ] Optional concise `docs/operations/rust-maintainability-audit-phase-170-7.md` if needed.

## Extraction target checklist
- [x] Start with `core/src/api/local_operator_shell.rs`.
- [x] Split at least one high-risk monolith surface.
- [x] Extract module-local local operator shell tests to `core/src/api/local_operator_shell_tests.rs`.
- [x] Keep `core/src/api/local_operator_shell.rs` as the local shell API surface.
- [x] Avoid circular module dependencies.

## Behavior-preservation checklist
- [x] Move code without redesigning it.
- [x] Do not change runtime semantics.
- [x] Do not change serialized formats.
- [x] Do not change deterministic ID, digest, or ordering inputs.
- [x] Do not change validation outcomes or reason strings.
- [x] Do not change UI behavior, TypeScript behavior, or schemas.

## Public API preservation checklist
- [x] Preserve the existing `local_operator_shell` public API surface.
- [x] Keep `core/src/api/mod.rs` as the API index.
- [x] Do not rename public types or enum variants.
- [x] Do not add Phase 171 release-candidate preparation APIs.

## Test preservation checklist
- [x] Do not delete tests.
- [x] Do not weaken assertions.
- [x] Do not remove adversarial coverage.
- [x] Keep moved local shell tests compiled under the same parent module boundary.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-7-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-7-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-7-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] Rust file-size scan.
- [x] Extraction scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after test extraction.
- [x] `core/src/execution/mod.rs` remains oversized.
- [x] `core/src/api/observability.rs` remains oversized.
- [x] `core/src/api/persistence.rs` remains oversized.
- [x] `core/src/api/local_workflow.rs` remains oversized.
- [x] `tests/integration_smoke.rs` remains a test-accumulation risk.
- [x] `tests/adversarial_corpus.rs` remains a test-accumulation risk.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 should avoid adding large release-candidate preparation logic directly to `local_operator_shell.rs`.
- [x] New Phase 171 release-candidate preparation behavior remains unimplemented.

## Deferred items
- [x] Provider output pipeline helper extraction is deferred.
- [x] Trial projection helper extraction is deferred.
- [x] Transport handler extraction is deferred.
- [x] Candidate materialization validation helper extraction is deferred.
- [x] Persistence transaction codec extraction is deferred.
- [x] Observability export encoder extraction is deferred.
- [x] Root integration/adversarial test reorganization is deferred.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.7 surfaces.
- [x] At least one high-risk Rust monolith surface is split.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Serialized formats are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, or publishing behavior is introduced.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
