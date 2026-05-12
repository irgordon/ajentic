---
truth_dimension: procedural
scope: current_phase
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist

Phase 156 - Constrained Real Local Provider Invocation.

## Phase goal
- [x] Add one Rust-owned constrained real local provider invocation path.
- [x] Keep invocation local-only, descriptive, untrusted, non-production, and allowlisted.

## Working-tree hygiene gate
- [x] Keep changes on allowed Phase 156 code, UI, changelog, and checklist surfaces.
- [x] Do not modify roadmap, governance, architecture, release, installer, update-channel, publishing, or production persistence surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned constrained invocation request, result, projection, status, error, boundary, trust, effect, and capability types.
- [x] Local shell state and transport response include constrained invocation projection.
- [x] TypeScript Rust-shaped invocation types and local shell transport behavior.
- [x] Visible UI panel/control for constrained local provider invocation.

## Constrained invocation checklist
- [x] Initial projection is `not_invoked`.
- [x] Invocation requires an accepted compatible adapter declaration.
- [x] Invocation request is validated before execution.
- [x] Invocation output is `untrusted_descriptive`.
- [x] Invocation result ID and output summary are deterministic for identical input.

## Allowlisted provider checklist
- [x] Exactly one provider path is allowed: `allowlisted_local_deterministic_provider`.
- [x] The path is implemented in Rust without process spawning.
- [x] Projection links to adapter declaration and registry state.
- [x] Capability surface records `constrained_local_invocation_only` and `allowlisted_provider_only`.

## Rejected unsafe invocation checklist
- [x] Missing adapter declaration rejects.
- [x] Rejected or unsupported adapter declaration rejects.
- [x] Non-allowlisted, cloud, network, and shell provider requests reject.
- [x] Command, process, args, endpoint, URL, host, port, path, secret, token, API key, credential, trust, provider-output approval, readiness, release, deployment, public-use, candidate materialization, action, and persistence claims reject.
- [x] Rejected request preserves prior accepted invocation state when present.

## No-arbitrary-command/no-shell/no-network/no-secret checklist
- [x] Record `no_arbitrary_command`.
- [x] Record `no_shell`.
- [x] Record `no_network` and `no_cloud`.
- [x] Record `no_secrets`.

## UI invocation panel checklist
- [x] Render `Constrained local provider invocation`.
- [x] Render invocation status, allowlisted provider kind, adapter declaration status, result ID, output summary, capability surface, boundary markers, output trust status, and rejection reasons.
- [x] Enable invocation control only for accepted `deterministic_fake_adapter` declaration.
- [x] Show required no-arbitrary-command, no-shell/network/cloud/secrets, untrusted descriptive, no-candidate, and no-readiness/release/deployment/public-use wording.
- [x] Avoid command, path, endpoint, and secret input fields.

## No-effect boundary checklist
- [x] No provider trust approval.
- [x] No decision ledger mutation.
- [x] No replay decision mutation.
- [x] No export evidence promotion.
- [x] No candidate output creation or materialization.
- [x] No action execution.
- [x] No production persistence or durable invocation storage.
- [x] No readiness, release, deployment, or public-use effect.

## Rust test checklist
- [x] Initial `not_invoked` projection.
- [x] Accepted allowlisted invocation and deterministic result/output.
- [x] Rejected preconditions and forbidden fields.
- [x] No-effect boundaries for ledger, replay, candidate, export, package, and restore projections.

## TypeScript test checklist
- [x] Visible invocation panel and accepted result rendering.
- [x] Rejected invocation state rendering.
- [x] No-authority wording and forbidden label avoidance.
- [x] Deterministic shell projection behavior.

## Phase 157 handoff checklist
- [x] Phase 157 remains the next code-production phase for real provider output pipeline integration.
- [x] General provider execution, cloud/network providers, arbitrary commands, candidate materialization, replay repair, recovery promotion, release/deployment/signing/publishing, public-use, and readiness approval remain deferred.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-156-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-156-target cargo test --manifest-path core/Cargo.toml --all-targets phase_156 -- --nocapture`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-156-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Invocation, boundary, forbidden-label, unsafe-execution, persistence/release, changed-file, and no-roadmap-drift scans.

## Deferred items
- [x] General provider execution.
- [x] Cloud provider execution.
- [x] Shell command execution and local binary invocation.
- [x] Network sockets and secret handling.
- [x] Durable invocation/provider configuration storage.
- [x] Provider output trust approval, candidate materialization, action execution, production persistence, readiness, release, deployment, public-use, signing, and publishing.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-156-target ./scripts/check.sh` was attempted before commit and reported the expected initial dirty working-tree guard; it is rerun after commit because the script requires initial repository cleanliness.
- `git diff --check` passed.
- `git status --short` showed only allowed Phase 156 files before commit.
- UI typecheck, lint, build, and API behavior tests passed.
- Rust tests passed with `CARGO_TARGET_DIR=/tmp/ajentic-phase-156-target cargo test --manifest-path core/Cargo.toml --all-targets`.
- Local dev smoke test printed `http://127.0.0.1:5173`; timeout stopped the long-running server.
- Invocation scan, boundary scan, forbidden-label scan, unsafe-execution scan, persistence/release scan, changed-file guard, and no-roadmap-drift guard completed; existing historical/test prohibition matches remain.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 156 scope.
- [x] Roadmap files are not modified.
- [x] Forbidden enablement/trust/candidate/readiness labels are not introduced as API or UI authority.
- [x] Generated UI artifacts are not kept.
