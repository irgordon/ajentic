# Phase 179.2 - OOB Dry-Run Rehearsal UI Forbidden-Label Fix

- Phase name: Phase 179.2 - OOB Dry-Run Rehearsal UI Forbidden-Label Fix
- Phase goal: Repair the Phase 179.1 UI forbidden-label collision so denial markers remain allowed while forbidden positive labels remain blocked.
- Working-tree hygiene gate: start from clean tree intent, run all required validation commands, and finish with no unintended drift.
- Allowed surfaces: ui/src/api/submissionBoundary.behavior.test.ts, optional shell/view fixture touch only if needed, CHANGELOG.md, checklists/current-phase.md.

## Forbidden-label collision fix checklist
- [x] Identify the exact forbidden-label assertion collision for no_signature_created vs signature_created.
- [x] Replace naive substring matching with token-aware forbidden-label detection.
- [x] Keep forbidden positive label detection for signature_created, artifact_signed, signed_release, signing_enabled, and public_signing_enabled.
- [x] Explicitly allow denial markers: no_signature_created, no_signing, no_public_signing, release_artifact_not_created, public_artifact_not_created.
- [x] Keep forbidden-label coverage active; do not remove the check.

## UI test repair checklist
- [x] Add or update behavior coverage proving no_signature_created is allowed.
- [x] Add or update behavior coverage proving signature_created alone is rejected.
- [x] Add or update behavior coverage proving signed_release is rejected.
- [x] Add or update behavior coverage proving signing_enabled is rejected.
- [x] Preserve required UI wording for no-authority rehearsal boundaries.

## No-authority boundary checklist
- [x] No Release Candidate approval behavior.
- [x] No release readiness approval behavior.
- [x] No signing or signature creation behavior.
- [x] No publishing, deployment, release, or distribution behavior.
- [x] No release artifact or public artifact creation behavior.
- [x] No provider trust, action authorization, replay repair, or recovery promotion behavior.

## Validation checklist
- [ ] cd ui && npm run test:api
- [ ] cd ui && npm run typecheck
- [ ] cd ui && npm run lint
- [ ] cd ui && npm run build && rm -rf dist
- [ ] cargo fmt --manifest-path core/Cargo.toml -- --check
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-2-target cargo test --manifest-path core/Cargo.toml --all-targets
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-2-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
- [ ] cd ui && timeout 10 npm run dev
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-2-target ./scripts/check.sh
- [ ] git diff --check
- [ ] git status --short

## Zero-drift checklist
- [ ] forbidden-label scan run and reviewed.
- [ ] no-Phase-180 scan run and reviewed.
- [ ] no-roadmap-drift guard run and reviewed.
- [ ] No changes outside allowed surfaces.

## Phase 180 handoff checklist
- [x] Phase 179.2 is an out-of-band validation fix only.
- [x] No Phase 180 implementation introduced.
- [x] Phase 180 remains the next decision gate.
