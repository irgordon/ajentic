---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

## v0.0.3 - 2026-05-02

**Status:** Phase 3 - CI and Structure Drift Gates

### Changed

- Normalized `checklists/current-phase.md` to Phase 3 procedural scope, boundaries, task checklist, validation checklist, deferred items, and validation log table.
- Hardened `.github/workflows/structure-lint.yml` to reject root `PHASE_MAP.md` in addition to root governance and architecture anchors.
- Hardened `.github/workflows/docs-gate.yml` to reject root `PHASE_MAP.md` and exclude `.github/instructions/*.instructions.md` from governed documentation boundary checks.

### Notes

- Phase 3 is a CI and drift-gate hardening phase only.
- No runtime harness behavior was implemented.
- Workflows remain enforcement wiring and do not create standalone governance.
- Updated .github/workflows/docs-gate.yml to narrow pattern checks and avoid false positives on valid governance and README boundary language.

## v0.0.2 - 2026-05-02

**Status:** Phase 2 - Phase Execution Loop and Active Checklist

### Changed

- Audited and normalized `docs/roadmap/phase-map.md` so phase structure remains agent-executable and planned-only.
- Clarified `docs/governance/phase-execution-contract.md` execution surfaces and changelog handoff boundaries.
- Replaced `checklists/current-phase.md` with Phase 2 procedural execution surface content and validation tracking tables.
- Updated `AGENTS.md` navigation links to include the phase execution contract as an authoritative source pointer.

### Notes

- Phase 2 is a planning and execution-discipline phase only.
- No runtime harness behavior was implemented.
- `docs/roadmap/phase-map.md` remains planned truth and must not record completed work.
- `checklists/current-phase.md` remains the only active phase execution surface.
- Updated .github/workflows/structure-lint.yml so GitHub Copilot instruction files use applyTo metadata without requiring governed artifact frontmatter.
- Updated .gitignore to exclude Rust, Node.js, Python, editor, environment, and ephemeral memory artifacts from repository tracking.

## v0.0.1 - 2026-05-02

**Status:** Phase 1 - Governance and Architecture Baseline

### Added

- No new files were added in this phase.

### Changed

- Audited and normalized the Phase 1 execution surface in `checklists/current-phase.md`.
- Audited governance anchor and governance subdocuments under `docs/governance/` for authoritative frontmatter, scope, and subordination.
- Audited architecture anchor and architecture subdocuments under `docs/architecture/` for structural scope and non-implementation wording.
- Verified `AGENTS.md` remains navigation-only and non-authoritative.
- Verified `README.md` remains orientation-only and non-authoritative.

### Notes

- Phase 1 is a documentation and boundary-hardening phase only.
- No runtime harness behavior was implemented.
- Governance and architecture subdocuments remain subordinate to their docs-owned anchors.

## v0.0.0 - 2026-05-02

**Status:** Phase 0 - Initial Repo Setup

### Added

- Created the initial repository skeleton for AJENTIC.
- Added top-level navigation, orientation, and historical anchors.
- Added canonical directories for Rust core, browser UI, scripts, tests, memory, checklists, docs, schemas, and workflows.
- Added minimal Rust core compile skeleton under `core/`.
- Added minimal TypeScript browser UI placeholder structure under `ui/`.
- Added initial governed documentation directories under `docs/`.
- Added placeholder JSON Schema files under `schemas/`.

### Changed

- Moved `GOVERNANCE.md` from repository root to `docs/governance/GOVERNANCE.md` per structure-lint contract.
- Moved `ARCHITECTURE.md` from repository root to `docs/architecture/ARCHITECTURE.md` per structure-lint contract.
- Normalized `AGENTS.md` as a navigation-only contract using docs-owned governance, architecture, and phase-map paths.
- Updated `checklists/current-phase.md` to reference canonical paths for governance and architecture anchors.
- Updated `docs/roadmap/phases.md` and `docs/roadmap/sequencing.md` to reference `docs/roadmap/phase-map.md` instead of `docs/PHASE_MAP.md`.
- Updated docs subdocuments in `docs/governance/`, `docs/architecture/`, and `docs/examples/` to reference moved anchor paths.
- Created missing UI placeholder directories: `screens/`, `components/`, `api/`, `types/generated/`, `validation/generated/`, `validation/adapters/`, `hooks/`.

### Notes

- Phase 0 remains a skeleton validation phase.
- No runtime harness behavior was implemented.
- Placeholder files establish repository shape only.
