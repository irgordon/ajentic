---
truth_dimension: procedural
phase: 170.19
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 170.19 - Out-of-Band Local Operator Shell Constrained Invocation Extraction

## Phase goal
- [x] Extract the constrained local provider invocation helper family from `core/src/api/local_operator_shell.rs` into a smaller sibling Rust module.
- [x] Preserve behavior exactly.
- [x] Keep Phase 171 release-candidate preparation unimplemented.

## Working-tree hygiene gate
- [x] Started from a clean working tree.
- [x] Ran `git status --short` before editing.
- [x] Read `docs/operations/rust-maintainability-audit-phase-170-5.md`.
- [x] Inspected the Phase 170.18 end state.

## Allowed surfaces
- [x] Edited `core/src/api/local_operator_shell.rs`.
- [x] Created `core/src/api/local_operator_shell_constrained_invocation.rs`.
- [x] Edited `CHANGELOG.md`.
- [x] Edited `checklists/current-phase.md`.
- [x] Did not edit UI, schemas, roadmap, governance, architecture, package, lockfile, CI, release, or deployment surfaces.

## Extraction target checklist
- [x] Selected the constrained local provider invocation helper family.
- [x] Did not repeat prior extracted provider output pipeline helpers.
- [x] Did not repeat prior candidate materialization helpers.
- [x] Did not repeat prior trial, transport, boundary-marker, workflow, restore, or provider-adapter helper families.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Created a non-empty sibling Rust module owning moved production code.

## Constrained invocation movement checklist
- [x] Moved constrained local provider invocation request, result, and projection types.
- [x] Moved invocation status, error, boundary, trust, effect, and capability-surface types.
- [x] Moved allowlisted local provider kind helpers.
- [x] Moved invocation validation helpers.
- [x] Moved invocation rejection helpers.
- [x] Moved deterministic invocation checksum, result ID, and output summary helpers.
- [x] Moved invocation projection helpers.
- [x] Moved the apply/derive helper that updates invocation projection and connected provider output pipeline projection.
- [x] Moved code mechanically without redesign.

## Projection preservation checklist
- [x] Preserved constrained invocation projection output.
- [x] Preserved initial projection reason text.
- [x] Preserved rejection projection reason text.
- [x] Preserved executed projection reason text.
- [x] Preserved status-code strings.
- [x] Preserved deterministic ordering.

## Allowlisted provider behavior preservation checklist
- [x] Preserved `allowlisted_local_deterministic_provider` acceptance behavior.
- [x] Preserved deterministic fake adapter precondition behavior.
- [x] Preserved deterministic result ID and output summary behavior.
- [x] Preserved allowlisted provider kind code strings.

## Rejection behavior preservation checklist
- [x] Preserved fail-closed non-allowlisted provider rejection behavior.
- [x] Preserved unsupported cloud, network, shell, and local provider rejection behavior.
- [x] Preserved no-arbitrary-command, no-shell, no-network, no-cloud, and no-secret field rejection behavior.
- [x] Preserved trust, provider-output approval, readiness, release, deployment, public-use, candidate-materialization, action, and persistence claim rejection behavior.
- [x] Preserved error-code strings and ordering.

## Pipeline linkage preservation checklist
- [x] Preserved provider output pipeline re-derivation after invocation execution.
- [x] Preserved provider output pipeline rejection bridge behavior.
- [x] Preserved local provider output validation linkage.
- [x] Preserved local session evidence export attachment after successful invocation.

## Provider boundary preservation checklist
- [x] Preserved provider output remains untrusted/descriptive behavior.
- [x] Preserved no provider trust behavior.
- [x] Preserved candidate-materialization prohibition behavior.
- [x] Preserved action-authorization prohibition behavior.
- [x] Preserved replay-repair and recovery-promotion prohibition behavior.
- [x] Preserved no provider execution expansion.

## Behavior-preservation checklist
- [x] Preserved runtime semantics.
- [x] Preserved validation outcomes.
- [x] Preserved reason strings.
- [x] Preserved boundary marker strings.
- [x] Preserved capability flags.
- [x] Preserved deterministic ordering.
- [x] Preserved UI, TypeScript, and schema behavior.

## Public API preservation checklist
- [x] Re-exported moved public constrained invocation types and helpers through `local_operator_shell.rs`.
- [x] Kept `core/src/api/mod.rs` stable.
- [x] Did not rename public types, functions, or enum variants.
- [x] Avoided broad public module exports.

## Test preservation checklist
- [x] Kept `core/src/api/local_operator_shell_tests.rs` intact.
- [x] Did not delete tests.
- [x] Did not weaken assertions.
- [x] Did not mark tests ignored.
- [x] Did not relax snapshot text.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-19-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-19-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-19-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Remaining-shell-function scan.
- [x] Moved-code scan.
- [x] Helper location scan.
- [x] Provider-invocation boundary scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining local shell production-code families remain extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, controlled-human-use, or production approval is introduced.

## Deferred items
- [x] Broader local shell decomposition is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.
- [x] Release artifacts, deployment, signing, publishing, installer, update-channel, public-use, and production approval remain deferred.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.19 surfaces.
- [x] The constrained local provider invocation helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Constrained invocation projection output is unchanged.
- [x] Allowlisted provider behavior is unchanged.
- [x] Non-allowlisted provider rejection behavior is unchanged.
- [x] No-arbitrary-command, no-shell, no-network, no-cloud, and no-secret boundary behavior is unchanged.
- [x] Provider output remains untrusted/descriptive.
- [x] Candidate-materialization prohibition is unchanged.
- [x] Action-authorization prohibition is unchanged.
- [x] Replay-repair and recovery-promotion prohibition is unchanged.
- [x] Provider output pipeline linkage behavior is unchanged.
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
