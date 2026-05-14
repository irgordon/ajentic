---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
operator_notes: active_phase_checklist
---

# Current Phase Checklist: Phase 170.18

## Phase name
- [x] Phase 170.18 - Out-of-Band Local Operator Shell Provider Adapter Extraction.

## Phase goal
- [x] Extract the provider adapter projection/helper family from `core/src/api/local_operator_shell.rs` into a focused sibling Rust module.
- [x] Preserve behavior exactly before Phase 171 begins.

## Working-tree hygiene gate
- [x] Started from a clean working tree.
- [x] Reviewed the Phase 170.5 Rust maintainability audit.
- [x] Inspected the Phase 170.17 end state before editing.
- [x] Removed no generated artifacts from the repository working tree.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_provider_adapter.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [x] No UI, schema, roadmap, governance, architecture, package, lockfile, CI, release, or deployment surfaces changed.

## Extraction target checklist
- [x] Required extraction target remained `core/src/api/local_operator_shell.rs`.
- [x] Selected the provider adapter contract, registry, declaration validation, and dry-run family.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Created a non-empty sibling Rust module owning moved production code.
- [x] Avoided circular module dependencies.
- [x] Did not repeat previously extracted Phase 170 helper families.

## Provider adapter movement checklist
- [x] Moved local provider adapter kind, status, error, and contract types.
- [x] Moved adapter declaration and configuration candidate types.
- [x] Moved adapter registry projection and validation helpers.
- [x] Moved adapter rejection helpers.
- [x] Moved adapter dry-run request, result, projection, validation, and execution helpers.
- [x] Moved code mechanically without redesign.

## Adapter registry preservation checklist
- [x] Preserved registry projection output.
- [x] Preserved declaration ordering.
- [x] Preserved boundary status ordering.
- [x] Preserved capability surface content.

## Adapter declaration validation preservation checklist
- [x] Preserved `deterministic_fake_adapter` acceptance behavior.
- [x] Preserved `local_model_adapter_contract` declaration behavior.
- [x] Preserved unsupported/cloud/network/shell/filesystem/unknown rejection behavior.
- [x] Preserved endpoint, command, path, secret, execution, trust, readiness, release, deployment, public-use, signing, publishing, and unknown-field rejection behavior.
- [x] Preserved fail-closed status construction.

## Adapter dry-run preservation checklist
- [x] Preserved dry-run status and projection behavior.
- [x] Preserved dry-run precondition and rejection behavior.
- [x] Preserved deterministic fake adapter dry-run output construction.
- [x] Preserved output trust, boundary, effect, and capability flags.

## Provider boundary preservation checklist
- [x] Preserved provider execution boundaries.
- [x] Preserved no provider trust, network, shell, secret, persistence, readiness, release, deployment, and public-use boundaries.
- [x] Preserved no candidate materialization and no action execution dry-run boundaries.
- [x] Introduced no provider execution expansion.

## Behavior-preservation checklist
- [x] Preserved runtime semantics.
- [x] Preserved validation outcomes.
- [x] Preserved reason strings and status-code strings.
- [x] Preserved boundary marker strings.
- [x] Preserved deterministic ordering.
- [x] Preserved UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-exported moved provider adapter public types and helpers through `local_operator_shell.rs`.
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
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-18-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-18-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-18-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Remaining-shell-function scan.
- [x] Moved-code scan.
- [x] Helper location scan.
- [x] Provider-adapter boundary scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining constrained local provider invocation and controlled internal trial execution projection code remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, controlled-human-use, or production approval is introduced.

## Deferred items
- [x] Broader local shell decomposition is deferred.
- [x] Constrained invocation and controlled internal trial execution extraction is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.18 surfaces.
- [x] The provider adapter projection/helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Adapter registry projection output is unchanged.
- [x] Adapter declaration validation behavior is unchanged.
- [x] Adapter dry-run behavior is unchanged.
- [x] Provider execution boundaries are unchanged.
- [x] Unsupported/unsafe adapter rejection behavior is unchanged.
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
