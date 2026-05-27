---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 184.2 - OOB Security Audit - Remaining Heuristic Triage

- Phase name: Phase 184.2 - OOB Security Audit - Remaining Heuristic Triage.
- Phase goal: classify all remaining heuristic scan hits by authority risk and narrowly repair any authority-sensitive heuristic exposure.
- Working-tree hygiene gate:
  - [x] Run `git status --short` before work.
- Phase 184.S carry-forward checklist:
  - [x] Review `docs/operations/security-audit-phase-184-s.md`.
  - [x] Preserve pause-for-security-repairs context until triage decision is finalized.
- Phase 184.1 carry-forward checklist:
  - [x] Confirm exact-match authority classifier remains present.
  - [x] Confirm positive authority markers remain rejected and denial markers remain exact-match allowed.
  - [x] Confirm unknown authority-shaped tokens remain fail-closed in authority-sensitive contexts.
- Remaining heuristic inventory checklist:
  - [x] Run remaining-heuristic discovery scan.
  - [x] Run authority-surface scan.
  - [x] Run positive-authority marker scan.
  - [x] Run denial-marker scan.
- Triage classification checklist:
  - [x] Classify each remaining hit into one required category.
  - [x] Record classifications and rationale in `docs/operations/remaining-heuristic-triage-phase-184-2.md`.
- Authority-sensitive repair checklist:
  - [x] Evaluate whether any unrepaired authority-sensitive heuristics remain.
  - [x] Apply only narrow/safe repairs where required.
- Accepted-risk checklist:
  - [x] Record non-authority accepted, test-only accepted, documentation/copy accepted, deferred refactor, and false-positive classifications.
- Rebuild-trigger checklist:
  - [x] Assess and record rebuild-trigger state.
- Release-path decision checklist:
  - [x] Select one required release-path decision and document it.
- Validation checklist:
  - [x] `cargo fmt --manifest-path core/Cargo.toml`
  - [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
  - [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-184-2-triage-target cargo test --manifest-path core/Cargo.toml --all-targets`
  - [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-184-2-triage-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
  - [x] `cd ui && npm run typecheck`
  - [x] `cd ui && npm run lint`
  - [x] `cd ui && npm run build && rm -rf dist`
  - [x] `cd ui && npm run test:api`
  - [x] `cd ui && timeout 10 npm run dev`
  - [x] `git diff --check`
- Zero-drift checklist:
  - [x] No roadmap edits.
  - [x] No governance edits.
  - [x] No architecture edits.
  - [x] No schema edits.
  - [x] No release/publish/deploy workflow edits.
- Phase 185 non-implementation checklist:
  - [x] No Phase 185 code or UI implementation introduced.
- Validation log:
  - [x] Validation and scan commands executed in phase terminal history.
