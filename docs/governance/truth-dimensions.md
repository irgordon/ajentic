---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Truth dimensions

This document defines the repository artifact taxonomy used to prevent truth-dimension collapse.

This document is subordinate to `GOVERNANCE.md`.

This document does not redefine system identity, runtime authority, architecture, implementation status, roadmap progress, release history, or operational procedure.

## 1. Purpose

Each artifact type represents one kind of truth.

A repository remains governable when each artifact stays inside its assigned truth dimension.

A repository becomes difficult to audit when one artifact type starts carrying another artifact type's role.

## 2. Truth-dimension table

| Truth dimension | Artifact location | Role |
| --- | --- | --- |
| Normative truth | `GOVERNANCE.md`, `docs/governance/` | Defines rules, invariants, authority boundaries, prohibitions, and non-goals. |
| Structural truth | `ARCHITECTURE.md`, `docs/architecture/` | Describes system structure, boundaries, data flow, and component responsibilities. |
| Planned truth | `docs/roadmap/` | Describes possible future work, sequencing, dependencies, and acceptance targets. |
| Historical truth | `CHANGELOG.md` | Records completed changes. |
| Procedural truth | `checklists/` | Defines bounded execution steps for releases, audits, migrations, or similar events. |
| Executable truth | `core/`, `tests/` | Defines system behavior through code and tests. |
| Contract truth | `schemas/` | Defines machine-checkable shared data contracts. |
| Data truth | `memory/` | Stores governed facts and run-scoped memory data. |
| Orientation truth | `README.md`, `docs/operations/` | Explains the project or operator concepts for humans. |
| Navigation truth | `AGENTS.md` | Directs agents to authoritative sources. |
| Illustrative truth | `docs/examples/` | Shows examples without creating authority. |

## 3. Required artifact metadata

Governed Markdown artifacts must declare frontmatter.

```yaml
truth_dimension: normative | structural | planned | historical | procedural | executable | contract | data | orientation | navigation | example
authority_level: authoritative | advisory | non_authoritative
mutation_path: governance_pr | architecture_pr | roadmap_update | changelog_entry | checklist_revision | code_change | schema_change | memory_update | readme_update | agents_update | example_update
```

Frontmatter identifies an artifact’s declared role.

Frontmatter does not make an artifact authoritative outside its allowed location.
## 4. Collision handling
When an artifact contains content from more than one truth dimension, the content must be split.

The target location is determined by the content’s role, not by the file where it was first written.

If an artifact’s declared truth dimension conflicts with its directory, the artifact placement is invalid.

If a document conflicts with GOVERNANCE.md, GOVERNANCE.md controls.

If a structural subdocument conflicts with ARCHITECTURE.md, ARCHITECTURE.md controls until the conflict is resolved through the architecture mutation path.
