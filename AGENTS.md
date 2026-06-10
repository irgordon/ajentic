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
| `docs/governance/phase-execution-contract.md` | Normative phase execution loop contract and checklist/changelog boundaries. |
| `docs/architecture/ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/roadmap/phase-map.md` | Planned phase sequence and active phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts (JSON Schema). |

## Quick navigation

- **Rules and invariants**: `docs/governance/GOVERNANCE.md`, `docs/governance/phase-execution-contract.md`, `docs/governance/`
- **System structure**: `docs/architecture/ARCHITECTURE.md`, `docs/architecture/`
- **Roadmap and phases**: `docs/roadmap/phase-map.md`, `docs/roadmap/`
- **Active phase tasks**: `checklists/current-phase.md`
- **Data contracts**: `schemas/`
- **Runtime source**: `core/`
- **Browser UI source**: `ui/`
- **Operator scripts**: `scripts/`

## Release discipline reminders

These reminders point back to roadmap and governance authority; they are not standalone release authority.

- Do not create tags or GitHub Releases unless the active phase explicitly authorizes release execution.
- Do not add signing, publishing, deployment, installer, or update-channel behavior unless explicitly scoped.
- Validation workflows may produce evidence; they do not approve releases.
- Do not infer release readiness from passing tests, passing CI, or clean validation.
- Do not claim production readiness unless a roadmap decision gate approves it.
- Do not claim v1.0 readiness from package metadata.
- Do not infer licensing or add a license without an Owner-selected license decision.
- Do not normalize versions to `1.0.0` before the v1.0 decision gate.
- Do not introduce publishing, signing, deployment, installer, or update-channel mechanics in identity-alignment phases.
- Keep GitHub Actions release authority behind explicit release-platform phases.
- Keep workflow permissions least-privilege unless a later release-execution phase explicitly requires broader permissions.
- Keep release approval tied to explicit roadmap decision gates.

## Constraint reminder

Rust owns runtime authority.

TypeScript is a review and operator-intent surface only.

Bash scripts are operator wrappers only.

Model output is untrusted until validated through Rust-owned paths.

This file must remain short, stable, and non-authoritative.
