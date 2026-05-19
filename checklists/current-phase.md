---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 176

Phase 176 - Signing and Key-Custody Dry Run.

- Phase goal: add local-only signing/key-custody dry-run evidence and UI with fail-closed boundaries.
- Working-tree hygiene gate: run pre/post `git status --short`.
- Allowed surfaces: dedicated Rust module, thin shell integration, UI projection/panel, tests, changelog/checklist.
- Code-production deliverables: Rust derivation + validation, deterministic evidence ID, visible UI panel.
- Dedicated module checklist: signing/key-custody types, enums, helpers, derivation/validation in dedicated module.
- Thin shell integration checklist: only add projection field and helper invocation wiring.
- Signing/key-custody dry-run checklist: status, classification, linkage, placeholder metadata, boundaries rendered.
- Placeholder key metadata checklist: deterministic placeholder-only, no real key/private/cert/KMS/secret material.
- Upstream evidence linkage checklist: preparation, dry package, checksum/provenance, installer/distribution linked.
- Missing evidence checklist: deterministic missing evidence list.
- Blocker checklist: deterministic blocker list with fail-closed rejection.
- Validation/fail-closed checklist: missing/rejected upstream and forbidden claims reject.
- UI signing/key-custody panel checklist: panel with status, ID, linkage, metadata, validation, missing/blockers.
- No-real-signing/no-release boundary checklist: explicit no-key/no-sign/no-publish/no-approval wording visible.
- Rust test checklist: valid mapping + deterministic ID + missing evidence rejection covered.
- TypeScript test checklist: panel text/state shape coverage and deterministic rendering checks.
- Phase 177 handoff checklist: Phase 177 remains next code-production phase for Release Candidate Evidence Assembly UI.
- Validation checklist: run check.sh + targeted UI/Rust/test/scans + diff/status guards.
- Deferred items: real signing, publishing, deployment, release, approvals remain deferred.
- Validation log: record executed commands and results below.
- Zero-drift checklist: no roadmap/governance/architecture drift; changes limited to allowed files.
