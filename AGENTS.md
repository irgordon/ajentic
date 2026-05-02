---
truth_dimension: navigation
authority_level: non_authoritative
mutation_path: agents_update
---

# AGENTS.md

This file is a short navigation contract for agents and tools working in this repository.

It is not the system of record.

It does not define governance rules, architecture authority, roadmap commitments, examples, or implementation details.

## Authoritative sources

| Source | Purpose |
| --- | --- |
| `docs/governance/GOVERNANCE.md` | Normative rules, authority model, prohibited patterns, and invariants. |
| `docs/architecture/ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/roadmap/phase-map.md` | Planned phase sequence and active phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts (JSON Schema). |

## Quick navigation

- **Rules and invariants**: `docs/governance/GOVERNANCE.md`, `docs/governance/`
- **System structure**: `docs/architecture/ARCHITECTURE.md`, `docs/architecture/`
- **Roadmap and phases**: `docs/roadmap/phase-map.md`, `docs/roadmap/`
- **Active phase tasks**: `checklists/current-phase.md`
- **Data contracts**: `schemas/`
- **Runtime source**: `core/`
- **Browser UI source**: `ui/`
- **Operator scripts**: `scripts/`

## Constraint reminder

Rust owns runtime authority.

TypeScript is a review and operator-intent surface only.

Bash scripts are operator wrappers only.

Model output is untrusted until validated through Rust-owned paths.

This file must remain short, stable, and non-authoritative.
