---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 177

- Phase name: Phase 177 - Release Candidate Evidence Assembly UI.
- Phase goal: add Rust-owned release-candidate evidence assembly projection and visible review-only UI panel.
- Working-tree hygiene gate: run pre/post `git status --short`.
- File-size discipline checklist: new Rust/TypeScript files remain under 1,000 LOC.
- Allowed surfaces: dedicated Rust module, thin shell integration, UI projection/panel, tests, changelog/checklist.
- Code-production deliverable checklist: deterministic assembly projection + deterministic ordering + visible UI evidence assembly panel.
- Dedicated module checklist: `core/src/api/release_candidate_evidence_assembly.rs` owns projection/types/derivation.
- Thin shell integration checklist: state field + helper invocation only.
- Evidence assembly checklist: status, ID, categories, linkage, missing evidence, blockers, validation summary, boundaries.
- Category/status checklist: closed enums and deterministic status mapping.
- Upstream linkage checklist: phase 171/172/173/174/176 linkages represented.
- Missing evidence checklist: fail-closed missing status and deterministic ordering.
- Blocker checklist: fail-closed blocked/rejected status and deterministic ordering.
- UI evidence assembly panel checklist: status, ID, counts, category sections, linkage/missing/blocker/validation summaries.
- No-approval/no-release-readiness boundary checklist: explicit no-approval/no-sign/no-publish/no-deploy/no-public-distribution wording.
- Rust test checklist: complete/missing/rejected/deterministic/no-authority coverage.
- TypeScript test checklist: panel/sections/linkage/missing-blocker/deterministic/forbidden-label checks.
- Phase 178 handoff checklist: Phase 178 remains next code-production phase.
- Validation checklist: check.sh + lint/typecheck/tests + scans + diff/status guards.
- Deferred items: approval/signing/publishing/deployment/public distribution remain deferred.
- Validation log: commands and outcomes captured in task report.
- Zero-drift checklist: no roadmap/governance/architecture drift.
