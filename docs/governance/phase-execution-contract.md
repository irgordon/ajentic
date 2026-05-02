---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Phase execution contract

This document defines how agents and humans execute phases without boundary drift.

This document is subordinate to `docs/governance/GOVERNANCE.md`.

It does not define architecture, roadmap sequencing, implementation status, or historical completion.


## 0. Execution surfaces and handoff

Durable phase planning lives in:

```text
docs/roadmap/phase-map.md
```

The only active execution checklist lives in:

```text
checklists/current-phase.md
```

Completed accepted work is recorded in:

```text
CHANGELOG.md
```

No other document may become the active phase tracker.

## 1. One active execution surface

Each phase must execute through one active checklist.

The active checklist is:

```text
checklists/current-phase.md
```

Agents must not create additional phase task files unless explicitly requested through the roadmap or governance mutation path.
## 2. One task, one surface

An agent may work on one task surface at a time.
A task surface is the smallest bounded area needed to complete the current checklist item.

Examples:
* one governance document
* one architecture document
* one schema
* one Rust module
* one UI screen
* one workflow
* one script
* one test group
An agent must not modify unrelated surfaces while completing a checklist item.
## 3. No boundary escapes

An agent must not bypass the declared truth dimension of the work item.

Governance work must remain in governance surfaces.

Architecture work must remain in architecture surfaces.

Roadmap work must remain in roadmap surfaces.

Checklist work must remain in checklist surfaces.

Schema work must remain in schemas.

Runtime behavior must remain in code and tests.

UI behavior must remain in UI surfaces and must not create authority.

Scripts must remain wrappers around Rust CLI behavior.

## 4. Completion rule
A checklist item is complete only when:

* the intended surface was changed
* no unrelated surface was changed
* required validation passed or the failure is recorded
* the result does not contradict GOVERNANCE.md
* the result does not contradict ARCHITECTURE.md
* the result does not introduce a new truth-dimension location
