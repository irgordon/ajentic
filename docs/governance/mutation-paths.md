---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Mutation paths

This document defines how artifact types may be changed.

This document is subordinate to `GOVERNANCE.md`.

## Mutation path table

| Artifact type | Mutation path | Requirement |
| --- | --- | --- |
| Governance | `governance_pr` | Human review and deterministic validation. |
| Architecture | `architecture_pr` | Must not contradict governance. |
| Roadmap | `roadmap_update` | Must not imply completion. |
| Changelog | `changelog_entry` | Completed work only. |
| Checklists | `checklist_revision` | Must remain procedural. |
| Code/tests | `code_change` | CI and tests required. |
| Schemas | `schema_change` | Version handling and schema validation required. |
| Memory | `memory_update` | Provenance and schema validation required. |
| Examples | `example_update` | Must remain non-authoritative. |
| README | `readme_update` | Must remain non-authoritative. |
| AGENTS.md | `agents_update` | Must remain navigational. |

## Prohibited mutations

The following must not occur:

- governance changes through roadmap updates
- architecture changes through changelog entries
- schema changes without validation
- memory changes without provenance
- AGENTS.md gaining authoritative content
- examples gaining implicit policy
- workflows introducing standalone governance
