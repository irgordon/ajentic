---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.20

## Phase name
- [x] Phase 170.20 - Out-of-Band Local Operator Shell Controlled Trial Execution Extraction.

## Phase goal
- [x] Extract the controlled internal trial execution helper family from `core/src/api/local_operator_shell.rs` into a smaller sibling Rust module.
- [x] Preserve behavior exactly before Phase 171 begins.

## Working-tree hygiene gate
- [x] Start from `git status --short` inspection.
- [x] Keep edits limited to allowed Phase 170.20 surfaces.
- [x] Commit only the behavior-preserving extraction and required procedural/history updates.

## Allowed surfaces
- [x] `core/src/api/local_operator_shell.rs`
- [x] `core/src/api/local_operator_shell_trial_execution.rs`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`
- [x] No UI, schema, roadmap, governance, architecture, package, lockfile, CI, release, or deployment changes.

## Extraction target checklist
- [x] Selected controlled internal trial execution helper family.
- [x] Created non-empty sibling module `core/src/api/local_operator_shell_trial_execution.rs`.
- [x] Kept `core/src/api/local_operator_shell.rs` as the public local shell surface.
- [x] Avoided circular module dependencies.
- [x] Did not repeat prior extracted helper families.

## Controlled trial execution movement checklist
- [x] Moved controlled internal trial execution projection types.
- [x] Moved trial execution request and run projection types.
- [x] Moved trial run status, step, step status, error, manual-step status, stop-observation, and evidence-linkage types.
- [x] Moved trial execution capability and boundary projection helpers.
- [x] Moved initial projection, precondition validation, blocker, evidence linkage, deterministic run ID, start, step, and rejection helper logic.
- [x] Kept local shell state and local session evidence export helpers in the shell surface.

## Projection preservation checklist
- [x] Controlled internal trial execution projection output is unchanged.
- [x] Boundary status ordering is unchanged.
- [x] Capability surface content is unchanged.
- [x] Current blocker behavior is unchanged.
- [x] Precondition validation behavior is unchanged.

## Trial run lifecycle preservation checklist
- [x] Trial run status behavior is unchanged.
- [x] Trial run step ordering is unchanged.
- [x] Deterministic trial run ID behavior is unchanged.
- [x] Accepted and rejected controlled trial start behavior is unchanged.
- [x] Controlled trial step behavior is unchanged.

## Stop-condition observation preservation checklist
- [x] Stop-condition observation status behavior is unchanged.
- [x] Stop-condition marker projection is unchanged.
- [x] Stop-condition automation remains disabled.

## Manual operator step preservation checklist
- [x] Manual action required behavior is unchanged.
- [x] Manual operator recorded behavior is unchanged.
- [x] Manual step missing blocker behavior is unchanged.

## Evidence linkage preservation checklist
- [x] Trial package linkage behavior is unchanged.
- [x] Runbook linkage behavior is unchanged.
- [x] Failure drill linkage behavior is unchanged.
- [x] Trial session evidence linkage behavior is unchanged.
- [x] Replay/restore and local workflow linkage behavior is unchanged.

## Controlled-trial boundary preservation checklist
- [x] No controlled-human-use approval boundary is unchanged.
- [x] No readiness, release, deployment, public-use, and production approval boundaries are unchanged.
- [x] No provider trust and no action authorization boundaries are unchanged.
- [x] No replay repair and no recovery promotion boundaries are unchanged.
- [x] No stop-condition automation and no automated escalation boundaries are unchanged.

## Behavior-preservation checklist
- [x] Runtime semantics are unchanged.
- [x] Request and response behavior is unchanged.
- [x] Reason strings and status-code strings are unchanged.
- [x] Boundary marker strings are unchanged.
- [x] Deterministic ordering is unchanged.
- [x] Validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.

## Public API preservation checklist
- [x] Moved public controlled trial execution types and helpers are re-exported through `local_operator_shell.rs`.
- [x] Public type names and enum variants are unchanged.
- [x] `core/src/api/mod.rs` remains stable.
- [x] No broad public module export was added.

## Test preservation checklist
- [x] Existing tests remain in place.
- [x] No tests were deleted.
- [x] No assertions were weakened.
- [x] No tests were marked ignored.
- [x] No snapshot text was relaxed.

## Validation checklist
- [x] `cargo fmt --manifest-path core/Cargo.toml`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-20-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-20-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-20-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] File-size scan.
- [x] Extraction scan.
- [x] Remaining-shell-function scan.
- [x] Moved-code scan.
- [x] Helper location scan.
- [x] Controlled-trial boundary scan.
- [x] Behavior-boundary scan.
- [x] No-Phase-171 scan.
- [x] No-UI-drift guard.

## Remaining monolith risk checklist
- [x] `core/src/api/local_operator_shell.rs` remains oversized after this focused extraction.
- [x] Remaining local shell helper families remain future extraction candidates.
- [x] Phase 171 should avoid adding release-candidate preparation behavior to the monolith.

## Phase 171 handoff checklist
- [x] Phase 171 remains the next planned code-production phase.
- [x] Phase 171 release-candidate preparation behavior remains unimplemented.
- [x] No readiness, release, deployment, public-use, controlled-human-use, or production approval is introduced.

## Deferred items
- [x] Broader local shell decomposition is deferred.
- [x] Release-candidate preparation remains deferred to Phase 171.
- [x] Release artifacts, signing, publishing, deployment, update-channel, installer, and public-use behavior remain out of scope.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.20 surfaces.
- [x] The controlled internal trial execution helper family moved out of `local_operator_shell.rs`.
- [x] Test extraction alone is not counted as sufficient.
- [x] Moved code remains behavior-preserving.
- [x] Existing tests pass.
- [x] UI behavior tests pass.
- [x] No tests are removed or weakened.
- [x] Public API compatibility is preserved.
- [x] Controlled trial execution projection output is unchanged.
- [x] Trial run lifecycle behavior is unchanged.
- [x] Stop-condition observation behavior is unchanged.
- [x] Manual operator step behavior is unchanged.
- [x] Evidence linkage behavior is unchanged.
- [x] Current blocker and precondition validation behavior are unchanged.
- [x] Deterministic trial run ID behavior is unchanged.
- [x] No approval, no authorization, no replay repair, and no recovery promotion boundaries are unchanged.
- [x] Boundary marker strings, capability flags, reason strings, deterministic ordering, and validation outcomes are unchanged.
- [x] No Phase 171 release-candidate preparation behavior is implemented.
- [x] No release artifact, deployment, signing, publishing, installer, update-channel, public-use, or production behavior is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 171 remains the next planned code-production phase.
