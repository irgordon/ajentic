---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Artifact placement rules

This document defines where artifact types live.

This document is subordinate to `GOVERNANCE.md`.

## Directory mapping

| Truth dimension | Directory |
| --- | --- |
| Normative truth | `GOVERNANCE.md`, `docs/governance/` |
| Structural truth | `ARCHITECTURE.md`, `docs/architecture/` |
| Planned truth | `docs/roadmap/` |
| Historical truth | `CHANGELOG.md` |
| Procedural truth | `checklists/` |
| Executable truth | `core/`, `tests/` |
| Contract truth | `schemas/` |
| Data truth | `memory/` |
| Orientation truth | `README.md`, `docs/operations/` |
| Navigation truth | `AGENTS.md` |
| Illustrative truth | `docs/examples/` |

## Placement prohibitions

The following must not occur:

- governance artifacts outside governance roots
- architecture artifacts outside architecture roots
- schemas outside `schemas/`
- memory entries inside `docs/`
- Markdown documentation inside `memory/`
- examples inside governance, architecture, roadmap, or checklists
- changelog entries outside `CHANGELOG.md`
- procedural checklists outside `checklists/`
