---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 181.2 - OOB Release Candidate Evidence Manifest Completion Fix

- Phase name: Phase 181.2 - OOB Release Candidate Evidence Manifest Completion Fix
- Phase goal: complete the Phase 181.1 Release Candidate label + evidence manifest contract without adding Phase 182 behavior.
- Working-tree hygiene gate: start clean, edit only allowed files, run required validations, and finish with clean diff checks.
- Phase 181.0 shape-map use checklist: `docs/operations/release-candidate-manifest-shape-map-phase-181-0.md` reviewed and used for shape-safe wiring.

## TypeScript missing-helper fix checklist
- [x] Add `initialReleaseCandidateEvidenceManifestProjection` helper.
- [x] Use helper in both local shell state construction paths.
- [x] Add behavior coverage for initial state and rendering.

## Manifest completion checklist
- [x] Manifest projection remains deterministic and non-authoritative.
- [x] Manifest panel remains visible with linkage/blocker/caveat summaries.
- [x] Supportability label remains evidence-only and non-readiness.

## Rust test completion checklist
- [x] Valid deterministic manifest projection test added.
- [x] Missing/rejected evidence blocker tests added.
- [x] Blocked rehearsal and blocking-gap blocker tests added.
- [x] Deterministic manifest id/item/blocker/caveat ordering tests added.
- [x] Targeted cleanup caveat coverage added.
- [x] No-authority boundary coverage added.

## TypeScript test completion checklist
- [x] Initial state includes releaseCandidateEvidenceManifest.
- [x] Manifest defaults to initial projection values.
- [x] Manifest panel/supportability label/manifest id/linkage/blocker/caveat rendering checks added.
- [x] Deterministic rendering and forbidden-label absence checks retained.

## Typed-hardening checklist
- [x] No broad substring authority inference in manifest readiness logic.
- [x] String status conversion remains isolated in named manifest helper with exact matches.

## No-authority checklist
- [x] evidence_manifest_only and release_candidate_label_only boundaries retained.
- [x] No signing/publishing/deployment/public distribution/public-use/release-readiness approvals added.

## Validation checklist
- [ ] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [ ] `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-2-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [ ] `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-2-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [ ] `cd ui && npm run typecheck`
- [ ] `cd ui && npm run lint`
- [ ] `cd ui && npm run build && rm -rf dist`
- [ ] `cd ui && npm run test:api`
- [ ] `cd ui && timeout 10 npm run dev`
- [ ] `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-2-target ./scripts/check.sh`
- [ ] `git diff --check`
- [ ] `git status --short`

## Zero-drift checklist
- [ ] No roadmap/governance/architecture/schema/package/lockfile drift.
- [ ] No Phase 182 implementation introduced.
- [ ] No release/public/deployment/signing/publishing/provider-trust/action-authorization/replay-repair/recovery-promotion behavior introduced.

## Phase 182 handoff checklist
- [ ] Phase 182 remains next code-production phase.
- [ ] No Release Candidate Review UI implementation added in Phase 181.2.
