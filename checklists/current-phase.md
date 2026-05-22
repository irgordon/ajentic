# Phase 179.1 - OOB Release Candidate Dry-Run Rehearsal Completion Fix

- Phase name: Phase 179.1 - OOB Release Candidate Dry-Run Rehearsal Completion Fix
- Phase goal: Complete unfinished Phase 179 rehearsal validation without introducing Phase 180 decision behavior.
- Working-tree hygiene gate: start from clean tree, run full validation, end with clean tracked outputs.
- Allowed surfaces: rehearsal module/tests, local shell/view behavior tests, changelog, current-phase checklist.

## Heuristic replacement checklist
- [x] Remove inline loose substring status inference from rehearsal derivation path.
- [x] Add explicit upstream rehearsal classification helpers for preparation, dry package, checksum/provenance, installer/distribution, signing/key-custody, evidence assembly, and gap review.
- [x] Keep any string-only upstream projection handling isolated to named helper functions with closed classifications.

## Rust test completion checklist
- [x] initial not_rehearsed projection.
- [x] valid upstream evidence deterministic rehearsal projection.
- [x] missing upstream evidence permutations block rehearsal (171/172/173/174/176/177/178).
- [x] rejected upstream evidence permutations block/reject rehearsal (171/172/173/174/176/177).
- [x] blocking gap review blocks rehearsal.
- [x] informational-only gap review does not block rehearsal.
- [x] deterministic rehearsal ID, linkage ordering, missing-ordering, blocker-ordering.
- [x] no-authority boundary markers remain enforced.

## TypeScript test completion checklist
- [x] Release Candidate dry-run rehearsal panel visibility.
- [x] rehearsal status and rehearsal ID rendering.
- [x] upstream linkage, missing summary, blocker summary, gap review summary, artifact summary rendering.
- [x] blocked/rejected rehearsal projection rendering.
- [x] deterministic rendering for identical projection.
- [x] no-authority wording and forbidden-label absence coverage.

## No-authority boundary checklist
- [x] no Release Candidate approval.
- [x] no release readiness approval.
- [x] no signing/publishing/deployment/release/distribution behavior.
- [x] no release/public artifact creation.
- [x] no GitHub release/tag/public download/installer/update channel behavior.
- [x] no provider trust/action authorization/replay repair/recovery promotion.

## Validation checklist
- [ ] cargo fmt --manifest-path core/Cargo.toml -- --check
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-1-target cargo test --manifest-path core/Cargo.toml --all-targets
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-1-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
- [ ] cd ui && npm run typecheck
- [ ] cd ui && npm run lint
- [ ] cd ui && npm run build && rm -rf dist
- [ ] cd ui && npm run test:api
- [ ] cd ui && timeout 10 npm run dev
- [ ] CARGO_TARGET_DIR=/tmp/ajentic-phase-179-1-target ./scripts/check.sh
- [ ] git diff --check
- [ ] git status --short

## Zero-drift checklist
- [ ] heuristic scan clean for inline loose status inference in rehearsal derivation path.
- [ ] rehearsal scan confirms coverage surfaces.
- [ ] forbidden-label scan clean (outside explicit tests/checklist strings).
- [ ] no-Phase-180 scan clean.
- [ ] no-roadmap-drift guard clean.

## Phase 180 handoff checklist
- [x] Phase 179.1 is an out-of-band completion fix only.
- [x] Phase 180 remains next decision gate.
- [x] No decision-gate or approval behavior added in this phase.
