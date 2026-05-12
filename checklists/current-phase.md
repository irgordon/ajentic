---
truth_dimension: procedural
phase: 153
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 153

## Phase name
- [x] Phase 153 - Real Local Provider Adapter Contract.

## Phase goal
- [x] Add the Rust-owned real local provider adapter contract surface before any Phase 154 dry-run harness.
- [x] Make adapter registry/configuration visible in the local UI as contract-only, non-executing state.

## Working-tree hygiene gate
- [x] Review allowed Phase 153 surfaces before editing.
- [x] Keep roadmap, governance, architecture, release, installer, update-channel, publishing, and production persistence surfaces unchanged.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Add Rust-owned local provider adapter contract types.
- [x] Add TypeScript Rust-shaped adapter contract/projection types.
- [x] Extend local shell state and transport with adapter registry/configuration projection.
- [x] Add visible local UI adapter registry/configuration panel.

## Local provider adapter contract checklist
- [x] Define adapter kind, declaration candidate, declaration, contract, capability, validation, execution, trust, and boundary types.
- [x] Accept `deterministic_fake_adapter` as contract-only and non-executing.
- [x] Accept `local_model_adapter_contract` as declaration-only when no unsafe fields are present.

## Adapter registry projection checklist
- [x] Initial registry projection is deterministic.
- [x] Projection lists supported and rejected adapter declarations.
- [x] Projection includes capability surface, validation result, execution status, trust status, and boundary statuses.

## Adapter validation checklist
- [x] Unsupported and unknown adapter kinds reject fail-closed.
- [x] Cloud, network, shell, filesystem, executable-path, endpoint, command, path, secret, execution, trust, readiness, release, deployment, public-use, signing, and publishing declarations reject fail-closed.
- [x] Rejected declarations preserve prior accepted registry state.

## UI adapter panel checklist
- [x] Render `Local provider adapter contract` panel.
- [x] Render `Adapter registry` and `Adapter configuration` labels.
- [x] Render supported, accepted, and rejected declaration status.
- [x] Render validation error/reason, capability surface, execution status, trust status, and boundary markers.
- [x] Render required Phase 153 no-model-execution wording.

## No-execution/no-trust checklist
- [x] Accepted declarations record `contract_only` and `no_execution`.
- [x] Accepted declarations record `no_provider_trust`.
- [x] Accepted declarations record `no_network`, `no_shell`, `no_secrets`, and `no_production_persistence`.
- [x] Accepted declarations record no readiness, release, deployment, or public-use effect.

## Local-only/non-production boundary checklist
- [x] No provider execution expansion is added.
- [x] No cloud calls, network sockets, shell commands, executable paths, or secret handling are added.
- [x] No production persistence, readiness approval, release, deployment, signing, publishing, or public-use approval is added.

## Rust test checklist
- [x] Deterministic initial adapter registry projection.
- [x] Accepted non-executing deterministic fake/local adapter declarations.
- [x] Unsupported and unsafe declaration rejection.
- [x] No-execution and no-mutation boundaries.
- [x] Rejected declaration preserves prior registry.

## TypeScript test checklist
- [x] Visible adapter registry/configuration panel.
- [x] Accepted adapter declaration rendering.
- [x] Rejected unsafe declaration behavior.
- [x] Deterministic projection/validation and no-authority wording.

## Phase 154 handoff checklist
- [x] Phase 154 remains the next code-production phase for controlled adapter dry-run harness.
- [x] Phase 153 does not implement a dry-run harness.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-153-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-153-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Adapter contract and boundary scans complete.

## Deferred items
- [x] Provider execution remains deferred.
- [x] Controlled adapter dry-run harness remains deferred to Phase 154.
- [x] Production persistence, replay repair, recovery promotion, release/deployment/signing/publishing, and readiness approval remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-153-target ./scripts/check.sh` passed after commit with a clean worktree.
- Full Rust tests passed with `CARGO_TARGET_DIR=/tmp/ajentic-phase-153-target cargo test --manifest-path core/Cargo.toml --all-targets`.
- TypeScript typecheck, lint, build, and API behavior tests passed.
- Local dev smoke test printed `http://127.0.0.1:5173`; timeout stopped the long-running server.
- `git diff --check`, `git status --short`, adapter scans, boundary scans, forbidden-label scan, unsafe-execution scan, persistence/release scan, changed-file guard, and no-roadmap-drift guard completed.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 153 changes.
- [x] Roadmap files are not modified.
- [x] Forbidden approval/enablement labels are not introduced for adapter contract state.
- [x] Generated UI artifacts are not kept.
