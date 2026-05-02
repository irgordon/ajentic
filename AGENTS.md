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
| `GOVERNANCE.md` | Normative rules, authority model, prohibited patterns, and invariants. |
| `ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/PHASE_MAP.md` | Planned phase sequence and active phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts (JSON Schema). |

## Quick navigation

- **Rules and invariants**: `GOVERNANCE.md`, `docs/governance/`
- **System structure**: `ARCHITECTURE.md`, `docs/architecture/`
- **Roadmap and phases**: `docs/PHASE_MAP.md`, `docs/roadmap/`
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
