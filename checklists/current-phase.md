---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 171.1

## Phase name
- [x] Phase 171.1 - OOB Missing Constrained Invocation Module Restore.

## Phase goal
- [x] Restore the missing constrained local provider invocation module expected from the Phase 170.19 extraction.
- [x] Reconnect the module through `core/src/api/local_operator_shell.rs` without redesigning constrained invocation behavior.

## Working-tree hygiene gate
- [x] Start from `git status --short`.
- [x] Restrict edits to the constrained invocation module, local shell module declaration/re-export, checklist, and changelog surfaces.
- [x] Do not edit UI, schemas, roadmap, governance, architecture, package, lockfile, CI, release, or deployment infrastructure.

## Missing module verification checklist
- [x] Check whether `core/src/api/local_operator_shell_constrained_invocation.rs` exists.
- [x] Check whether `core/src/api/local_operator_shell.rs` declares and re-exports `local_operator_shell_constrained_invocation`.
- [x] Check whether constrained invocation definitions still exist inline in `local_operator_shell.rs`.
- [x] Determine the applicable repair path before editing.

## Repair path checklist
- [x] Use Repair path B because the constrained invocation code existed inline and the module file was missing.
- [x] Move the constrained invocation helper family into `core/src/api/local_operator_shell_constrained_invocation.rs`.
- [x] Add the sibling module declaration and public re-export in `core/src/api/local_operator_shell.rs`.
- [x] Remove duplicate inline constrained invocation definitions from `core/src/api/local_operator_shell.rs`.

## Behavior-preservation checklist
- [x] Preserve allowlisted provider code strings.
- [x] Preserve constrained invocation status and error code strings.
- [x] Preserve boundary, trust, and effect status code strings.
- [x] Preserve the default `allowlisted_default` input summary.
- [x] Preserve initial projection and rejection reason strings.
- [x] Preserve forbidden field rejection behavior and fail-closed coverage.
- [x] Preserve deterministic checksum basis and result ID format.
- [x] Preserve `untrusted_descriptive` output trust behavior.
- [x] Preserve provider output pipeline bridge behavior and provider output approval rejection mapping.
- [x] Preserve `attach_local_session_evidence_export` behavior.

## Public API preservation checklist
- [x] Keep constrained invocation symbols publicly reachable through `local_operator_shell.rs`.
- [x] Keep local shell state and provider pipeline references compiling through the re-exported module symbols.
- [x] Keep definitions owned by the sibling constrained invocation module exactly once.

## Validation checklist
- [x] Run Rust formatting.
- [x] Run Rust formatting check.
- [x] Run Rust tests for all core targets.
- [x] Run Rust clippy for all core targets with warnings denied.
- [x] Run repository check script with the required target directory.
- [x] Run whitespace diff check.
- [x] Run constrained invocation scan.
- [x] Run duplicate definition scan.
- [x] Run no-authority scan and confirm no new authority behavior.
- [x] Record final validation commands in the agent final response.

## Zero-drift checklist
- [x] No UI changes.
- [x] No TypeScript changes.
- [x] No schema changes.
- [x] No roadmap changes.
- [x] No governance or architecture changes.
- [x] No package, lockfile, CI, release, or deployment infrastructure changes.
- [x] No release-candidate preparation expansion.
- [x] No new provider execution semantics, provider trust, action authorization, persistence authority, readiness approval, release approval, deployment approval, or public-use approval.

## Phase 172 handoff checklist
- [x] Phase 172 remains the next planned code-production phase.
- [x] Phase 171.1 creates no release artifacts.
- [x] Phase 171.1 does not add signing, publishing, installer, update-channel, deployment, public-use, readiness, or production-use behavior.
