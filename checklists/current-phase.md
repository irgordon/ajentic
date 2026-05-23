---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 181.1 - Release Candidate Label and Evidence Manifest

- Phase name: Phase 181.1 - Release Candidate Label and Evidence Manifest
- Phase goal: add Rust-owned Release Candidate supportability label + evidence manifest projection and visible UI panel with non-authority boundaries.
- Working-tree hygiene gate: clean start, allowed-surface edits only, required validation before completion.
- Phase 181.0 shape-map use checklist: read `docs/operations/release-candidate-manifest-shape-map-phase-181-0.md`; wire only current Rust/TypeScript projection shapes.
- File-size discipline checklist: all new `.rs/.ts/.tsx` files under 1,000 LOC.
- Targeted cleanup carry-forward checklist: typed statuses, isolated adapters, deterministic ordering, stronger typed tests, no broad status substring heuristics.
- Allowed surfaces: `core/src/api/release_candidate_evidence_manifest*.rs`, `core/src/api/mod.rs`, `core/src/api/local_operator_shell_state.rs`, `ui/src/api/localOperatorShell.ts`, `ui/src/api/localOperatorShellView.ts`, `ui/src/api/submissionBoundary.behavior.test.ts`, `CHANGELOG.md`, `checklists/current-phase.md`.

## Code-production deliverable checklist
- [x] Dedicated manifest module added.
- [x] Typed supportability label status and manifest status added.
- [x] Thin shell integration added.
- [x] Evidence manifest projection carried into UI model.
- [x] Visible UI manifest panel rendering added.
- [x] No-release/no-production/no-public-use boundaries shown.

## Evidence manifest checklist
- [x] Deterministic manifest ordering for items/blockers/caveats.
- [x] Deterministic manifest ID generation.
- [x] Upstream linkage summary represented.
- [x] Missing/rejected/blocked evidence converted into blockers.
- [x] Phase 180.2 targeted-cleanup caveats included.

## Validation checklist
- [ ] `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-1-target ./scripts/check.sh`
- [ ] `git diff --check`
- [ ] `git status --short`
- [ ] file-size scan
- [ ] manifest scan
- [ ] shape-map-use scan
- [ ] typed-hardening scan
- [ ] dedicated module scan
- [ ] boundary scan
- [ ] forbidden-label scan
- [ ] unsafe execution scan
- [ ] release/deployment scan
- [ ] changed-file guard
- [ ] no-roadmap-drift guard
- [ ] UI typecheck/lint/build/test
- [ ] Rust tests
- [ ] UI dev smoke test

## Deferred items
- Phase 182 remains next code-production phase for Release Candidate Review UI.

## Validation log
- Pending final run.

## Zero-drift checklist
- [ ] Roadmap files unchanged.
- [ ] No signing/publishing/deployment/release/public-distribution behavior added.
- [ ] Manifest remains local-only and non-authoritative.
