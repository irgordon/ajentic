---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

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
