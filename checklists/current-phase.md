---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist - Phase 96 Local Startup Command Boundary

## Phase name
Phase 96 - Local Startup Command Boundary.

## Phase goal
Define the minimal local startup command/path for operator testing after the Phase 95 hardening evidence gates.

Phase 96 is a local startup boundary candidate only. It is a usability boundary, not production readiness, packaging approval, public usability approval, release-candidate readiness, or Production Candidate approval.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified the initial working tree as clean.
- [x] Found no pre-existing uncommitted generated artifact drift requiring cleanup before edits.
- [x] Reviewed generated artifact hygiene after validation.
- [x] Confirmed no generated compiler metadata, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, `scripts/__pycache__`, or unrelated tool output were staged.

## Allowed surfaces
- [x] `checklists/current-phase.md` may be updated for Phase 96 procedural truth.
- [x] `CHANGELOG.md` may be updated with `v0.0.96` historical truth.
- [x] `docs/operations/local-startup-boundary-phase-96.md` may be created.
- [x] `core/src/main.rs` was available only if CLI behavior needed to change; no runtime edit was required.
- [x] `core/src/api/local_workflow.rs` was available only if an existing bounded helper needed CLI exposure; no edit was required.
- [x] `tests/integration_smoke.rs` was available only if behavior changed; no test edit was required.
- [x] `checklists/release.md` was not changed because evidence posture did not change release posture.
- [x] `README.md` was not changed because operations documentation was sufficient.
- [x] `scripts/check.sh` was not changed because no validation gate change was needed.
- [x] `ui/package.json` was not changed because no UI validation/startup script change was needed.

## Boundary rules
- [x] Phase 96 remains bounded, deterministic, non-authoritative, and non-executing.
- [x] No server was added.
- [x] No daemon was added.
- [x] No background process was added.
- [x] No socket was added.
- [x] No network behavior was added.
- [x] No async runtime was added.
- [x] No thread/process spawn was added.
- [x] No browser launch was added.
- [x] No installer or package artifact was added.
- [x] No file watcher was added.
- [x] No live UI/Rust transport was added.
- [x] No provider/model call was added.
- [x] No provider execution expansion was added.
- [x] No persistence write was added.
- [x] No durable append was added.
- [x] No export write was added.
- [x] No import behavior was added.
- [x] No replay repair was added.
- [x] No recovery promotion was added.
- [x] No global state replacement was added.
- [x] No action execution was added.
- [x] No new action authority was added.
- [x] No dependency change was added.
- [x] No startup/package approval, public-usability approval, release-candidate readiness, production-readiness, or Production Candidate approval claim was added.

## Task checklist
- [x] Read roadmap files and confirmed Phase 96 title/scope.
- [x] Read `CHANGELOG.md`, `checklists/current-phase.md`, `checklists/release.md`, `README.md`, and `AGENTS.md`.
- [x] Read Phase 95.4 and related operations evidence docs.
- [x] Inspected current startup/validation surfaces.
- [x] Evaluated startup command Options A, B, and C.
- [x] Chose Option A: existing dry-run remains the local startup boundary.
- [x] Created `docs/operations/local-startup-boundary-phase-96.md`.
- [x] Updated this checklist to Phase 96 procedural truth.
- [x] Updated `CHANGELOG.md` with `v0.0.96`.
- [x] Added no new runtime behavior.
- [x] Added no new tests because behavior did not change.

## Validation checklist
- [x] `./scripts/check.sh` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml golden --all-targets` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` passed after final edits.
- [x] `cd ui && npm run test:api` passed after final edits.
- [x] `node scripts/test_rust_boundary_lint.mjs` passed after final edits.
- [x] `node scripts/rust_boundary_lint.mjs` passed after final edits.
- [x] `node scripts/test_lint_ui_boundaries.mjs` passed after final edits.
- [x] `node scripts/lint_ui_boundaries.mjs` passed after final edits.
- [x] `cd ui && npm run typecheck && npm run lint && npm run build` passed after final edits.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` passed after final edits.
- [x] Startup boundary scans completed.
- [x] No-live-startup scans completed.
- [x] No-authority scans completed.
- [x] Source guard completed with no prohibited diffs.
- [x] Rust source guard completed with no prohibited diffs.
- [x] Readiness scan completed with only explicit non-approval/prohibition language.
- [x] Roadmap/changelog scans completed.
- [x] Lint wiring scan completed.
- [x] Final `git status --short` reviewed before commit.
- [x] Final `git diff --name-only --cached` reviewed before commit.

## Startup command decision checklist
- [x] Option A was evaluated.
- [x] Option B was evaluated and rejected as unnecessary surface expansion.
- [x] Option C was evaluated and treated as the documentation posture for Option A, not a replacement command.
- [x] Existing command selected: `cargo run --manifest-path core/Cargo.toml -- dry-run`.
- [x] Existing dry-run already reports provider output remains untrusted.
- [x] Existing dry-run already reports no provider/model call.
- [x] Existing dry-run already reports no persistence occurred.
- [x] Existing dry-run already reports no files were read or written.
- [x] Existing dry-run already reports no release-candidate readiness claim.
- [x] Existing dry-run already reports no production readiness claim.
- [x] No new CLI alias/report was added.

## Local startup non-authority checklist
- [x] `startup_status` documented as `bounded_local_startup_reported`.
- [x] `provider_call_performed=false` documented.
- [x] `model_call_performed=false` documented.
- [x] `live_transport_started=false` documented.
- [x] `ui_bridge_started=false` documented.
- [x] `server_started=false` documented.
- [x] `background_process_started=false` documented.
- [x] `persistence_written=false` documented.
- [x] `ledger_appended=false` documented.
- [x] `audit_appended=false` documented.
- [x] `export_written=false` documented.
- [x] `recovery_promoted=false` documented.
- [x] `replay_repaired=false` documented.
- [x] `action_executed=false` documented.
- [x] `mutates_authority=false` documented.
- [x] `production_candidate_approved=false` documented.
- [x] `release_candidate_ready=false` documented.
- [x] `public_usability_approved=false` documented.
- [x] `startup_package_approved=false` documented.

## CLI/test coverage checklist
- [x] No new CLI command was added.
- [x] No CLI behavior changed.
- [x] No new tests were required because Phase 96 is documentation-only.
- [x] Existing CLI unit and integration coverage remained part of the all-target Rust test suite.
- [x] Existing `cargo run --manifest-path core/Cargo.toml -- dry-run` was run explicitly.

## Operator documentation checklist
- [x] Created operations doc with required frontmatter.
- [x] Added Scope section.
- [x] Added Startup boundary decision section.
- [x] Added Supported local startup command section.
- [x] Added What this command does section.
- [x] Added What happens when it fails section.
- [x] Added What this command does not do section.
- [x] Added Operator expectations section.
- [x] Added Non-authority guarantees section.
- [x] Added Relationship to Phase 95.4 evidence gate section.
- [x] Added Relationship to Phase 97 packaging boundary section.
- [x] Added Validation evidence section.
- [x] Added AST/boundary lint parity section.
- [x] Added Test fidelity section.
- [x] Added Confirmed vs suspected section.
- [x] Added Deferred items section.
- [x] Added Non-readiness statement section.
- [x] Stated roadmap remains planned truth.
- [x] Stated `CHANGELOG.md` remains historical truth.

## AST/boundary lint parity checklist
- [x] Confirmed `rg` scans are discovery/evidence only.
- [x] Confirmed blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.
- [x] Confirmed Phase 96 did not rely on `rg` as enforcement.
- [x] Confirmed no new prohibited source pattern was found.
- [x] Confirmed no lint behavior changed.
- [x] Confirmed no lint self-test changes were required.

## Test fidelity checklist
- [x] Confirmed new behavior requires tests in the same phase.
- [x] Confirmed no runtime behavior changed.
- [x] Confirmed no new tests were required.
- [x] Confirmed full existing validation passed after final edits.
- [x] Confirmed cross-boundary behavior was not expanded.
- [x] Confirmed no tests were skipped after final edits.

## Zero-drift checklist
- [x] Confirmed generated compiler metadata was not staged.
- [x] Confirmed UI build artifacts were not staged.
- [x] Confirmed test temp files were not staged.
- [x] Confirmed export temp files were not staged.
- [x] Confirmed node build artifacts were not staged.
- [x] Confirmed coverage output was not staged.
- [x] Confirmed `scripts/__pycache__` was not staged.
- [x] Confirmed no unrelated tool output was staged.
- [x] Confirmed staged files match the allowed Phase 96 surfaces.

## Findings table
| Question | Status | Finding |
| --- | --- | --- |
| Working-tree hygiene | Clean | Initial `git status --short` was clean; no pre-edit generated drift cleanup was required. |
| Phase 96 title/scope | Confirmed | Roadmap planned truth identifies Phase 96 as Local Startup Command Boundary for the minimal local startup command surface for operator testing. |
| Startup command decision | Option A | Existing `cargo run --manifest-path core/Cargo.toml -- dry-run` remains the supported local startup boundary. |
| Runtime behavior | Unchanged | No new runtime behavior, CLI alias, helper export, tests, script gate, UI path, package entry, or README surface was added. |
| Non-authority posture | Sufficient | The documented command remains bounded, deterministic, non-authoritative, and non-executing. |
| Test coverage | Sufficient | No new tests were required because no behavior changed; full existing validation passed. |
| AST/boundary lint parity | Sufficient | Existing blocking lint/test gates remain the enforcement surfaces; scans were evidence only. |
| Readiness posture | Not approved | Phase 96 does not approve startup/package readiness, public usability, release-candidate readiness, production readiness, or Production Candidate status. |

## Deferred items table
| Item | Status | Reason |
| --- | --- | --- |
| New local-startup CLI alias/report | Deferred | Existing dry-run is sufficient and avoids extra surface. |
| Runtime startup behavior | Deferred | Phase 96 is documentation-only and non-executing. |
| README local startup section | Deferred | Operations documentation is sufficient for this boundary. |
| `scripts/check.sh` gate change | Deferred | No validation gate change was required. |
| UI startup or bridge path | Deferred | Phase 96 does not start UI, browser, bridge, transport, server, daemon, or background process. |
| Packaging artifacts/installers | Deferred | Phase 97 owns packaging boundary work if Phase 96 evidence permits it. |
| Production Candidate approval | Deferred | Phase 100 is the planned decision gate; Phase 96 does not approve it. |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | Pass | Initial tree clean before edits. |
| `./scripts/check.sh` | Pass | Full repository gate passed after final edits. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust all-target tests passed. |
| `cargo test --manifest-path core/Cargo.toml golden --all-targets` | Pass | Golden-filter Rust tests passed. |
| `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` | Pass | Adversarial-filter Rust tests passed. |
| `cd ui && npm run test:api` | Pass | UI API behavior tests passed. |
| `node scripts/test_rust_boundary_lint.mjs` | Pass | Rust boundary lint self-tests passed. |
| `node scripts/rust_boundary_lint.mjs` | Pass | Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | Pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | Pass | UI AST lint passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI typecheck, lint, and build passed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | Pass | Existing local startup boundary command completed successfully. |
| Startup boundary scan | Pass | Discovery/evidence scan completed. |
| No-live-startup scan | Pass | Matches were documentation/checklist prohibitions or pre-existing references; no new live startup behavior was added. |
| No-authority scan | Pass | Matches were documentation/checklist posture or pre-existing tests/source references; no new authority mutation was added. |
| Source guard | Pass | No TypeScript, UI source, script, workflow, AGENTS, package, lockfile, or roadmap diffs. |
| Rust source guard | Pass | No protected Rust source diffs. |
| Readiness scan | Pass | Matches are explicit non-approval/prohibition language only. |
| Roadmap/changelog scan | Pass | Planned/historical truth posture present. |
| Lint wiring scan | Pass | Lint and UI behavior wiring remained intact. |
| `git status --short` | Pass | Reviewed before staging and commit. |
| `git diff --name-only --cached` | Pass | Staged files matched allowed surfaces. |
