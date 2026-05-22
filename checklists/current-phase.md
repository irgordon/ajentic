---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 178.2 - OOB Core Purpose and Drift Audit

- Phase goal: determine whether AJENTIC has drifted from deterministic, bounded, auditable, non-authoritative operation before Phase 179.
- Working-tree hygiene gate: run `git status --short` and `git diff --check` before and after audit edits.
- Allowed surfaces: `docs/operations/core-purpose-drift-audit-phase-178-2.md`, `CHANGELOG.md`, `checklists/current-phase.md` only.
- Audit scope checklist: root governance/roadmap/changelog/checklist, Rust API/shell/provider/replay/restore/release-path surfaces, UI state/render/help, tests, and validation scripts.
- Determinism checklist: deterministic IDs/digests/orderings, replay/status derivation from recorded facts, no authoritative nondeterminism.
- Bounded-control checklist: typed request/response boundaries, local operator shell control surface, dry-run vs authority separation.
- Non-authority checklist: no readiness/release/public-use/production/provider-trust/action/replay-repair/recovery-promotion approval semantics.
- Provider/execution guardrail checklist: arbitrary execution blocked; no shell/network/cloud/secrets authority expansion.
- Release-path guardrail checklist: no real signing/publishing/deployment/distribution/release artifacts; dry-run evidence only.
- UI honesty checklist: copy remains local-only/non-authoritative/untrusted where required; no authority-implying controls.
- Documentation truth-dimension checklist: governance normative, roadmap planned, changelog historical, checklist procedural, code/tests executable truth.
- Architecture/modularity checklist: identify monolith risk (>1,000 LOC) and boundary-marker duplication risk.
- Test/validation integrity checklist: adversarial/fail-closed checks present; no failure masking; `scripts/check.sh` remains integration gate.
- Rebuild trigger assessment checklist: explicitly assess unbounded execution, default trust, unauthorized action, release/public/prod authority, signing/publishing/deployment behavior, replay repair mutation, hidden validation failure, authoritative nondeterminism.
- Phase 179 recommendation checklist: choose one required decision option and record caveats.
- Validation checklist: run required `rg` scans, file-size scan, changelog-size scan, `CARGO_TARGET_DIR=/tmp/ajentic-phase-178-2-target ./scripts/check.sh`, `git diff --check`, and `git status --short`.
- Zero-drift checklist: audit doc present, required finding sections complete, severity/disposition recorded, rebuild trigger explicit, recommendation explicit, changelog/checklist aligned with actual diff.
