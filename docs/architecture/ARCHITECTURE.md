---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---
# ARCHITECTURE.md

This document defines the structural truth of AJENTIC.

It describes how the system is organized, how authority flows, how model-driven work moves through the harness, and how the browser UI supports human review.

It does not define governance rules, implementation status, roadmap progress, release history, operational procedures, or example usage.
## 1. System purpose

AJENTIC is a deterministic control layer for model-driven work.

The harness provides structure around local or cloud large language model output so that candidate work can be inspected, validated, recorded, replayed, and audited before it is treated as clean output.

The system provides:
- bounded context assembly
- controlled model invocation
- typed operator intents
- deterministic policy and validation boundaries
- governed memory
- authoritative ledger recording
- deterministic replay
- human-readable audit projections
- browser-based TypeScript review UI
- Bash wrappers around Rust CLI commands
- integration boundaries for local and cloud model workflows

Raw model output is candidate material.

Clean output is output that has passed through the harness boundary.
## 2. Architectural principle
AJENTIC separates authority from visibility.
Rust owns authority.

TypeScript owns browser visibility and operator intent submission.

Bash owns local operator convenience.

GitHub workflows own enforcement wiring.

Schemas own shared contract shape.

Memory owns governed data.

Tests and code own executable behavior.

Documentation describes truth within its declared artifact type.

## 3. High-level workflow

```text
┌──────────────────────────────────────────────────────────────────────┐
│                              User / Operator                         │
│                 human engineer, reviewer, or workflow owner          │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                │ task, review action, or operator request
                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                         UI / CLI Request Surface                      │
│                                                                      │
│   Browser UI: displays projections and submits typed intents          │
│   Rust CLI: accepts typed local commands                              │
│   Bash: wraps Rust CLI commands only                                  │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                │ typed request or typed intent
                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                              Rust Boundary                           │
│                                                                      │
│   API / CLI intake validates request shape and routes to core         │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                            Harness Engine                            │
│                                                                      │
│   ┌──────────┐   ┌──────────┐   ┌────────────┐   ┌──────────────┐   │
│   │  State   │ → │  Policy  │ → │ Validation │ → │  Execution   │   │
│   └──────────┘   └──────────┘   └────────────┘   └──────┬───────┘   │
│                                                          │           │
│   ┌──────────┐   ┌──────────┐   ┌────────────┐          │           │
│   │ Context  │ ← │  Memory  │ ← │  Schemas   │          │           │
│   └────┬─────┘   └──────────┘   └────────────┘          │           │
│        │                                                │           │
└────────┼────────────────────────────────────────────────┼───────────┘
         │ bounded context packet                         │ provider request
         ▼                                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                            Model Provider                            │
│                    local LLM or cloud LLM provider                    │
│                        output is untrusted                            │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                │ candidate model output
                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                        Rust Validation Boundary                      │
│                                                                      │
│   schema checks, policy checks, state checks, evidence checks         │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                    ┌───────────┴───────────┐
                    │                       │
                    ▼                       ▼
          ┌─────────────────┐     ┌─────────────────────┐
          │ Rejected result  │     │ Accepted result      │
          │ no authority     │     │ ledgered event       │
          └────────┬────────┘     └──────────┬──────────┘
                   │                         │
                   ▼                         ▼
          ┌─────────────────┐     ┌─────────────────────┐
          │ Retry / revise   │     │ Ledger               │
          │ via typed intent │     │ accepted events      │
          └─────────────────┘     └──────────┬──────────┘
                                             │
                                             ▼
┌──────────────────────────────────────────────────────────────────────┐
│                           Replay + Audit                             │
│       deterministic reconstruction and human-readable projections      │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────────┐
│                            Clean Output                              │
│      validated result, audit trail, replayable run, UI projection      │
└──────────────────────────────────────────────────────────────────────┘
```

4. Controlled cycle

AJENTIC operates as a controlled review and execution cycle.

```text
        ┌──────────────────────┐
        │  1. Operator intent   │
        │  task or review       │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  2. Rust intake       │
        │  API or CLI boundary  │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  3. Policy + state    │
        │  allowed path check   │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  4. Context builder   │
        │  bounded packet       │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  5. LLM invocation    │
        │  local or cloud       │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  6. Candidate output  │
        │  untrusted            │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │  7. Validation        │
        │  deterministic checks │
        └──────────┬───────────┘
                   │
          ┌────────┴────────┐
          ▼                 ▼
┌──────────────────┐ ┌──────────────────┐
│  8a. Rejected     │ │  8b. Accepted     │
│  no authority     │ │  ledgered event   │
└────────┬─────────┘ └────────┬─────────┘
         │                    │
         ▼                    ▼
┌──────────────────┐ ┌──────────────────┐
│  Retry / revise   │ │  Replay + audit   │
│  through intent   │ │  projection       │
└────────┬─────────┘ └────────┬─────────┘
         │                    │
         └──────────┬─────────┘
                    ▼
        ┌──────────────────────┐
        │  9. UI projection     │
        │  inspectable output   │
        └──────────────────────┘
```

5. Authority flow

Authority flows through Rust-owned boundaries.

```text
schemas/ define shared contract shape
        ↓
Rust validates typed input
        ↓
policy and state determine allowed paths
        ↓
validation evaluates typed evidence
        ↓
execution invokes controlled work
        ↓
ledger records accepted events
        ↓
replay reconstructs recorded runs
        ↓
audit projects readable history
        ↓
UI displays projections
```

The UI does not send authority upstream.

The UI sends typed requests upstream.

Rust decides whether those requests are accepted.

6. Repository structure

The intended top-level repository structure is:

```text
agentic-harness/
├── GOVERNANCE.md
├── ARCHITECTURE.md
├── AGENTS.md
├── README.md
├── CHANGELOG.md
├── core/
├── ui/
├── scripts/
├── tests/
├── memory/
├── checklists/
├── docs/
├── schemas/
└── .github/
```

Top-level anchors:

* GOVERNANCE.md defines normative truth.
* ARCHITECTURE.md defines structural truth.
* AGENTS.md provides navigation truth.
* README.md provides human orientation truth.
* CHANGELOG.md records historical truth.

7. Rust core

Location:

```text
core/
```

Rust is the authoritative runtime engine.

Rust owns:

* state machine
* context assembly
* memory access
* policy
* validation
* execution
* ledger
* replay
* audit projection generation
* API boundaries
* CLI boundaries
* typed errors

The Rust core rejects invalid, malformed, incomplete, unauthorized, or unverifiable requests.

8. Rust module layout

```text
core/src/
├── lib.rs
├── main.rs
├── state/
├── context/
├── memory/
├── policy/
├── validation/
├── execution/
├── ledger/
├── replay/
├── audit/
├── api/
└── errors/
```

9. `state/`

The state module defines lifecycle states and valid transitions.

It owns:

* state types
* transition rules
* transition validation inputs
* state transition errors

The state module does not depend on UI display state, Bash script behavior, provider preference, or model confidence.

10. `context/`

The context module assembles model-visible context.

It owns:

* context packet construction
* context slices
* context views
* budget metadata
* source selection
* provenance for included context

The context module prefers minimal, relevant, auditable context slices over maximal context accumulation.

The context module does not include arbitrary repository knowledge by default.

11. `memory/`

The memory module governs runtime access to memory/.

It owns:

* memory entry types
* memory loading
* memory validation
* memory retrieval
* memory snapshotting
* memory persistence
* provenance enforcement

Persistent memory is long-lived, versioned, and provenance-bearing.

Ephemeral memory is run-scoped.

Memory does not define policy.

12. `policy/`

The policy module defines allowed and prohibited runtime behavior.

It owns:

* policy rules
* gates
* authority checks
* decision errors

Policy is deterministic.

Policy does not depend on UI preference, Bash script behavior, provider preference, model confidence, or undocumented repository text.

13. `validation/`

The validation module evaluates typed evidence.

It owns:

* evidence types
* schema validation boundaries
* context validation
* memory validation
* policy evidence validation
* deterministic pass/fail results

Validation does not directly mutate state.

Validation does not accept missing evidence as pass.

14. `execution/`

The execution module invokes harness workflows.

It owns:

* run orchestration
* planner invocation boundaries
* tool invocation boundaries
* provider invocation boundaries
* execution errors
* execution event production

Execution passes through policy, state, and validation.

Execution does not directly trust provider output.

## 15. `ledger/`

The ledger module records accepted authoritative events.

It owns:

* ledger entries
* append logic
* sequence checks
* ledger queries
* ledger integrity errors

Ledger append is authoritative.

Ledger read is observational.

The ledger does not decide validity.

## 16. `replay/`

The replay module reconstructs runs from ledger entries and recorded inputs.

It owns:

* replay engine
* replay integrity checks
* replay reports
* deterministic reconstruction errors

Replay fails closed on missing, malformed, reordered, conflicting, or unverifiable inputs.

Replay does not repair ledger history.

## 17. `audit/`

The audit module produces human-readable projections.

It owns:

* timelines
* run projections
* replay projections
* exportable audit reports

Audit does not create authority.

Audit does not mutate state, memory, ledger, replay, policy, or execution outputs.

## 18. `api/`

The API module exposes typed request and projection surfaces.

It owns:

* HTTP or IPC surfaces
* CLI surfaces
* event streams
* typed intent submission
* projection endpoints

The API does not expose mutation paths that bypass policy, state, validation, memory, ledger, or replay.

## 19. `errors/`

The errors module defines typed error surfaces.

Errors are explicit enough for deterministic tests and operator inspection.

Errors do not hide authority failures behind generic failure states.

## 20. Browser UI

Location:

```text
ui/
```

The browser UI is a non-authoritative review and operator-intent surface.

The browser UI is for human operators who need to inspect and guide harness runs without living in the terminal.

The UI displays Rust projections and submits typed intents.

The UI does not directly mutate authoritative state.

## 21. Browser UI layout target

The UI uses a responsive browser-first application shell.

```text
┌───────────────────────────────────────────────┐
│ Top bar                                       │
│ Run selector | State badge | API status        │
├───────────────┬───────────────────────────────┤
│ Navigation    │ Main review surface            │
│               │                               │
│ - Overview    │ State, context, policy,        │
│ - Context     │ validation, memory, replay     │
│ - Memory      │                               │
│ - Policy      │                               │
│ - Timeline    │                               │
│ - Replay      │                               │
│ - Output      │                               │
├───────────────┴───────────────────────────────┤
│ Intent/action tray                            │
└───────────────────────────────────────────────┘
```

On mobile, navigation collapses into a bottom tab bar or drawer.

The current state and required operator attention remain visible or quickly reachable on desktop, tablet, and mobile layouts.

## 22. Browser UI screens

The planned UI screens are:

```text
OverviewScreen
RunOverviewScreen
StateMachineScreen
ContextInspectorScreen
MemoryInspectorScreen
PolicyValidationScreen
RunTimelineScreen
ReplayScreen
CleanOutputScreen
```

Each screen consumes Rust projections or submits typed intents.

No screen owns authority.

## 23. Browser UI components

The planned reusable UI components are:

```text
StateBadge
AuthorityBadge
StatusCard
Timeline
EvidenceList
ContextSliceCard
MemoryEntryCard
IntentButton
RawDataDisclosure
```

Components render structured data into human-readable cards, tables, badges, timelines, diagrams, and controls.

Raw JSON may be available through explicit diagnostic disclosure.

Raw JSON is not the default human review interface.

## 24. Browser UI routes

The initial route model is:

```bash
/
  Overview
/runs
  Run list
/runs/:runId
  Run overview
/runs/:runId/state
  State machine view
/runs/:runId/context
  Context inspector
/runs/:runId/memory
  Memory inspector
/runs/:runId/policy
  Policy and validation console
/runs/:runId/timeline
  Run timeline
/runs/:runId/replay
  Replay view
/runs/:runId/output
  Clean output view
/settings
  Non-authoritative UI preferences only
```
Settings do not define policy or runtime authority.

## 25. UI mutation path

All UI-originated mutation requests follow this path:

```text
UI component
  → typed UI client
  → operator intent schema
  → Rust API
  → policy/state/validation
  → accepted ledger event
  → projected UI update
```
No UI-originated mutation bypasses this path.

## 26. UI data rendering

The UI renders structured data as:

* cards
* tables
* timelines
* status badges
* diagrams
* filterable lists
* detail drawers
* readable summaries

The UI visually distinguishes:

* raw model output
* candidate output
* rejected output
* accepted clean output
* advisory evidence
* authoritative decisions
* replayable runs
* non-replayable runs

## 27. TypeScript contract usage

TypeScript types are generated from JSON Schema where feasible.

Runtime validators are generated from JSON Schema where feasible.

Generated types live under:

```text
ui/src/types/generated/
```

Generated validators live under:

```text
ui/src/validation/generated/
```

Temporary non-authoritative adapters may live under:

```text
ui/src/validation/adapters/
```

TypeScript does not define shared contract truth.

## 28. Bash scripts

Location:

```
scripts/
```

Bash scripts are wrappers around Rust CLI commands.

Scripts may:

* run local checks
* start local development flows
* invoke replay through Rust
* request memory snapshots through Rust
* clear ephemeral memory through Rust
* start the browser UI

Scripts do not implement policy.

Scripts do not directly mutate authoritative runtime state.

Scripts are replaceable by direct Rust CLI invocation without changing behavior.

## 29. GitHub workflows

Location:

```text
.github/workflows/
```

GitHub workflows are enforcement wiring.

Workflows may:

* run deterministic build checks
* run Rust checks
* run UI checks
* validate JSON syntax
* validate schema placement
* validate memory placement
* validate frontmatter placement
* run advisory agent review

Workflows do not create standalone governance.

Workflow-enforced rules trace back to governance, architecture, schemas, tests, or code.

## 30. Schemas

Location:

```text
schemas/
```

Schemas define shared contract shape.

Schema categories include:

```text
schemas/docs/
schemas/context/
schemas/memory/
schemas/events/
schemas/intents/
schemas/traces/
```

Schemas are consumed by:

* Rust validation
* TypeScript generated types
* TypeScript generated validators
* CI checks
* memory linting
* docs gates
* tests

The canonical schema root is schemas/.

Schema authority is not duplicated elsewhere.

## 31. Memory directory

Location:

memory/

Memory is governed data.

Persistent memory lives under:

memory/persistent/

Ephemeral memory lives under:

memory/ephemeral/

Persistent memory contains committed, versioned, provenance-bearing facts.

Ephemeral memory contains run-scoped data and is not committed.

Memory is accessed through Rust-owned paths.

Memory is not documentation.

Memory is not policy.

## 32. Tests

Location:

tests/

Harness-level integration and contract tests live in tests/.

Rust unit tests may live near Rust source files.

Tests define executable truth.

Tests cover:

* state transitions
* context assembly
* memory validation
* policy gates
* validation behavior
* ledger append behavior
* replay integrity
* API intent handling
* UI contract behavior
* schema compatibility

Tests avoid interpretive ambiguity.

## 33. Context architecture

A context packet is the bounded set of material made visible to a model for a task.

A context packet may include:

* task instruction slice
* system or harness instruction slice
* relevant governance slice
* relevant architecture slice
* relevant schema slice
* relevant code slice
* relevant diff slice
* relevant memory slice
* relevant tool output slice
* budget metadata
* provenance metadata

Context packets are inspectable.

Context packets are reproducible where replay requires it.

## 34. Model provider architecture

Providers are adapters.

Providers may be local or cloud-based.

Provider output is untrusted.

Provider adapters do not own policy.

Provider adapters do not own state.

Provider adapters do not write ledger entries directly.

Provider responses return through Rust validation before use.

Integration boundaries may include:

* local LLM processes
* cloud LLM providers
* Codex Desktop-style environments
* Antigravity-style environments
* similar IDE or model-run environments

Integration does not change authority ownership.

## 35. Intent architecture

An intent is a typed request.

An intent is not authority.

Intent types may include:

* approve
* reject
* retry
* request replay
* request context rebuild
* request memory snapshot
* request memory write
* request memory disable
* request run cancellation
* request tool execution

Every intent identifies:

* intent type
* target
* requester
* reason where required
* schema version

Rust accepts or rejects intents.

## 36. Ledger and replay architecture

The ledger records accepted events.

Replay reconstructs behavior from ledger entries and recorded inputs.

Replay validates:

* event ordering
* completeness
* integrity
* consistency

Replay fails closed on violations.

Replay output may be projected through audit and UI.

## 37. Audit architecture

Audit produces human-readable projections over ledger and replay output.

Audit projections may include:

* run timeline
* validation summary
* policy summary
* replay summary
* memory usage summary
* clean output summary
* exportable review report

Audit does not create authority.

## 38. Clean output architecture

Clean output is output that has passed through the harness boundary.

Clean output has, where applicable:

* originating task or intent
* context packet reference
* validation result
* policy result
* ledger reference
* replay status
* audit projection

Raw model output is not clean output.

The UI displays raw model output and clean output as different states.

## 39. Trust boundaries

The following are untrusted by default:

* model output
* provider responses
* UI input
* CLI arguments
* Bash script arguments
* external files
* memory entries before validation
* documentation snippets before artifact classification
* workflow-generated agent review
* examples
* README content

The following may become trusted only through deterministic acceptance:

* typed intents
* typed evidence
* validated context packets
* validated memory entries
* accepted policy decisions
* accepted state transitions
* ledgered events

## 40. Production structural target

The production structure supports:

* usable browser UI review flow
* typed operator intents
* controlled local/cloud model invocation
* bounded context inspection
* governed memory inspection
* policy and validation inspection
* ledgered event history
* deterministic replay
* audit projection
* reproducible CI enforcement
* local/IDE model-run integration boundaries
* clear authority boundaries

This target does not imply implementation status.
