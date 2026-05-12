---
truth_dimension: procedural
phase: 154
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 154

## Phase name
- [x] Phase 154 - Controlled Adapter Dry-Run Harness.

## Phase goal
- [x] Prove the Phase 153 adapter contract path with a Rust-owned deterministic fake adapter dry run.
- [x] Keep dry-run output descriptive, untrusted, local-only, and non-production.

## Working-tree hygiene gate
- [x] Keep changes on allowed Phase 154 code, UI, changelog, and checklist surfaces.
- [x] Do not modify roadmap, governance, architecture, release, installer, update-channel, publishing, or production persistence surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned adapter dry-run request, result, projection, status, error, boundary, trust, effect, and capability types.
- [x] Local shell state and transport response include adapter dry-run projection.
- [x] TypeScript Rust-shaped dry-run types and local shell transport behavior.
- [x] Visible UI panel/control for controlled adapter dry run.

## Adapter dry-run harness checklist
- [x] Initial projection is `not_run`.
- [x] Accepted `deterministic_fake_adapter` declaration is required.
- [x] Dry-run request is validated before execution.
- [x] Dry-run result links to adapter declaration and registry projection.
- [x] Dry-run output trust status is `untrusted_descriptive`.

## Deterministic fake adapter checklist
- [x] Only `deterministic_fake_adapter` can execute in Phase 154.
- [x] Dry-run output summary is deterministic for identical input.
- [x] Dry-run result ID is deterministic for identical input.
- [x] Execution is in-process and descriptive only.

## Rejected dry-run precondition checklist
- [x] Missing adapter declaration rejects.
- [x] Rejected or unsupported adapter declaration rejects.
- [x] `local_model_adapter_contract` rejects for Phase 154 execution.
- [x] Cloud, network, shell, filesystem, executable-path, endpoint, command, path, secret, execution, trust, readiness, release, deployment, public-use, signing, publishing, action, and persistence claims reject.
- [x] Rejected request preserves prior accepted dry-run state when present.

## No-real-model/no-process/no-network/no-secret checklist
- [x] Record `no_real_model_execution`.
- [x] Record `no_process_spawn`.
- [x] Record `no_network`.
- [x] Record `no_shell`.
- [x] Record `no_secrets`.

## UI dry-run panel checklist
- [x] Render `Controlled adapter dry run`.
- [x] Render status, adapter kind, declaration status, result ID, output summary, linkage, boundaries, trust, capabilities, and rejection reasons.
- [x] Enable run control only for accepted `deterministic_fake_adapter`.
- [x] Show required no-real-model, untrusted descriptive, no-candidate, no-readiness/release/deployment/public-use wording.
- [x] Avoid arbitrary command, path, endpoint, and secret inputs.

## No-effect boundary checklist
- [x] No provider trust approval.
- [x] No decision ledger mutation.
- [x] No replay decision mutation.
- [x] No export evidence promotion.
- [x] No candidate output creation or materialization.
- [x] No action execution.
- [x] No production persistence or durable dry-run storage.
- [x] No readiness, release, deployment, or public-use effect.

## Rust test checklist
- [x] Initial `not_run` projection.
- [x] Accepted deterministic dry run and deterministic result/output.
- [x] Rejected preconditions and forbidden fields.
- [x] No-effect boundaries for ledger, replay, candidate, export, package, and restore projections.

## TypeScript test checklist
- [x] Visible dry-run panel and accepted result rendering.
- [x] Rejected dry-run state rendering.
- [x] No-authority wording and forbidden label avoidance.
- [x] Deterministic shell projection behavior.

## Phase 155 handoff checklist
- [x] Phase 155 remains the next code-production alignment checkpoint.
- [x] Real model/provider execution remains deferred.
- [x] Candidate materialization, replay repair, recovery promotion, release/deployment/signing/publishing, public-use, and readiness approval remain deferred.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-154-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-154-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Adapter dry-run, boundary, forbidden-label, unsafe-execution, persistence/release, changed-file, and no-roadmap-drift scans.

## Deferred items
- [x] Real local model execution.
- [x] Cloud provider execution.
- [x] Shell command execution and local binary invocation.
- [x] Network sockets and secret handling.
- [x] Durable adapter dry-run/provider configuration storage.
- [x] Provider output trust approval, candidate materialization, action execution, production persistence, readiness, release, deployment, public-use, signing, and publishing.

## Validation log
- `git diff --check` passed.
- UI typecheck, lint, build, and API behavior tests passed.
- Rust tests passed with `CARGO_TARGET_DIR=/tmp/ajentic-phase-154-target cargo test --manifest-path core/Cargo.toml --all-targets`.
- Local dev smoke test printed `http://127.0.0.1:5173`; timeout stopped the long-running server.
- Adapter dry-run scan, boundary scan, forbidden-label scan, unsafe-execution scan, persistence/release scan, changed-file guard, and no-roadmap-drift guard completed; historical/test prohibition matches remain.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-154-target ./scripts/check.sh` is run after commit because the script requires initial repository cleanliness.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 154 scope.
- [x] Roadmap files are not modified.
- [x] Forbidden enablement/trust/candidate/readiness labels are not introduced as API or UI authority.
- [x] Generated UI artifacts are not kept.
