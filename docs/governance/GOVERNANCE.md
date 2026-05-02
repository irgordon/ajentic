---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---
# GOVERNANCE.md
This document defines the normative truth of AJENTIC.

It defines what must always be true for the project, repository, and harness authority model.

It does not define implementation status, roadmap progress, release history, operational procedures, UI layout, or example usage.
## 1. System identity

AJENTIC is a deterministic control layer for model-driven work.

AJENTIC is not an autonomous agent.

AJENTIC is not a general chat application.

AJENTIC is not a UI-authoritative system.

AJENTIC is not a Bash-governed runtime.

AJENTIC exists to constrain, observe, validate, record, replay, and audit work performed with local or cloud large language models.

## 2. Authority model
Rust is the only authoritative runtime surface.
The Rust core owns runtime authority for:
- state
- policy
- validation
- context assembly
- memory access
- execution
- ledger recording
- replay
- audit projection generation
- API boundaries
- CLI boundaries
No TypeScript, Bash, workflow, documentation, memory entry, example, or model output may create runtime authority.
## 3. Model-output rule
Model output is untrusted input.
Model output may propose:
- actions
- text
- code
- plans
- explanations
- candidate artifacts
- review comments
Model output must not directly mutate:
- authoritative state
- memory
- policy
- ledger
- replay inputs
- execution results
- schemas
- tests
- repository files
Model output becomes usable only after validation.
Model output becomes authoritative only when accepted through Rust-owned authority paths and recorded where required.
## 4. Intent rule
External surfaces submit typed intents only.
An intent is a request.
An intent is not authority.
The UI, CLI, scripts, workflows, model adapters, and operator surfaces may submit requests only through typed boundaries.
Rust decides whether an intent is valid.
Rejected intents must not mutate authoritative state.
Accepted intents must produce auditable events where required.
## 5. Required mutation path
All mutation requests must follow the authoritative path:

```text
request or model output
  → typed intent or typed input
  → Rust API or CLI
  → policy
  → state
  → validation
  → execution or memory operation
  → ledger
  → replay
  → audit projection
  → UI/API display
```

No surface may bypass this path.

## 6. Rust rule

Rust owns the harness authority boundary.

Rust must:

* validate typed input at authoritative boundaries
* enforce policy and state constraints
* accept or reject intents
* control memory access through typed paths
* control execution paths
* record accepted authoritative events
* support deterministic replay
* produce audit projections
* expose API and CLI surfaces without bypassing policy, state, validation, memory, ledger, or replay

Rust must reject malformed, incomplete, unauthorized, unverifiable, or schema-invalid payloads.

## 7. UI rule

The TypeScript UI is a review and operator-intent surface.

The UI may:

* display state projections
* display context projections
* display memory projections
* display policy and validation results
* display run timelines
* display replay reports
* display audit projections
* submit typed operator intents

The UI must not:

* directly mutate authoritative state
* directly mutate memory
* write ledger events
* decide policy outcomes
* decide validation outcomes
* repair replay failures
* call model providers directly
* infer authority from displayed data

Validated UI data is renderable data, not authoritative state.

## 8. Browser UI rule

The browser UI exists for human operators who need to inspect and guide harness runs without living in the terminal.

The browser UI must keep core harness functions visible and understandable.

The browser UI must prefer readable cards, tables, timelines, diagrams, controls, and status indicators over raw structured text.

Raw JSON may be available for diagnostics, but raw JSON must not be the default human review surface.

The browser UI must remain responsive across desktop, tablet, and mobile views.

The browser UI may later be packaged as a desktop application, but packaging must not change runtime authority.

A desktop shell must remain a transport and display surface.

## 9. Bash rule

Bash scripts are operator glue.

Scripts may:

* call Rust CLI commands
* start local support processes
* run local checks
* invoke replay through Rust
* request memory snapshots through Rust
* clear ephemeral memory through Rust
* start the UI

Scripts must not:

* implement policy
* directly edit authoritative state
* write ledger entries
* mutate persistent memory directly
* bypass Rust validation
* create independent runtime decisions

Scripts must be replaceable by direct Rust CLI invocation without changing behavior.

## 10. Workflow rule

GitHub workflows are enforcement surfaces.

Workflows may:

* run deterministic checks
* validate schemas
* run tests
* detect artifact placement violations
* validate memory placement
* validate frontmatter
* attach advisory agent review evidence

Workflows must not:

* create standalone governance
* introduce rules absent from governance, architecture, schemas, tests, or code
* treat model or agent judgment as authoritative
* mutate runtime state
* rewrite truth dimensions outside normal review

If a workflow enforces a rule, the rule must be traceable to an authoritative source.

## 11. Contract rule

Contract truth lives in schemas/.

JSON Schema defines shared data contracts.

TypeScript types and runtime validators must be generated from JSON Schema where feasible.

Generated TypeScript types are compile-time derivatives.

Generated runtime validators are boundary-checking derivatives.

Hand-written TypeScript interfaces or Zod schemas must not redefine contract truth.

Rust structs must not redefine shared contract truth in conflict with schemas.

Any contract change requires the schema_change mutation path and required version handling.

## 12. TypeScript binding rule

TypeScript bindings are non-authoritative derivatives of schema contracts.

Generated TypeScript files must not be hand-edited.

Generated validators must not weaken schema constraints.

Hand-written adapters may exist only as temporary non-authoritative wrappers.

UI components must receive already-validated typed data where feasible.

UI components must not perform contract validation internally as a substitute for boundary validation.

## 13. Memory rule

Memory is governed data.

Memory is not documentation.

Memory is not policy.

Memory must not contain:

* normative rules
* roadmap commitments
* changelog entries
* architecture authority
* procedural checklists
* schema definitions

Persistent memory must be typed, versioned, and provenance-bearing.

Ephemeral memory must be run-scoped and must not be committed.

Memory content may be used only after validation through Rust-owned paths.

## 14. Context rule

Context is the bounded set of material made visible to a model.

The harness must prefer minimal, relevant, auditable context slices over maximal context accumulation.

Context packets must be inspectable.

Context slices must carry provenance where required.

Context does not create authority by itself.

Model-visible context must not silently bypass governance, architecture, memory, schema, or validation boundaries.

## 15. Provider rule

Model providers are adapters.

Providers may be local or cloud-based.

Provider output is untrusted.

Provider adapters must not own policy.

Provider adapters must not own state.

Provider adapters must not write ledger entries directly.

Provider adapters must not mutate memory directly.

Provider responses must return through Rust validation before use.

Integration with local large language models, Codex Desktop, Antigravity, or similar model environments must not reduce Rust-owned authority.

## 16. Ledger rule

The ledger records accepted authoritative events.

Ledger append is authoritative.

Ledger read is observational.

The ledger does not decide validity.

The ledger must not repair, reinterpret, reorder, or silently discard accepted events.

Accepted state transitions must be represented by typed ledger events where required.

No UI, Bash script, workflow, provider adapter, documentation file, memory entry, or model output may write ledger events directly.

## 17. Replay rule

Replay reconstructs runs from ledger entries and recorded inputs.

Replay must be deterministic.

Replay must fail closed on missing, malformed, reordered, conflicting, or unverifiable inputs.

Replay must not repair ledger history.

Replay must not infer missing authority.

Replay must not reinterpret invalid event ordering as valid.

## 18. Audit rule

Audit surfaces are projections.

Audit may render, summarize, export, or explain ledger and replay outputs.

Audit must not create authority.

Audit must not mutate:

* state
* memory
* policy
* ledger
* replay
* execution results

Audit output may be displayed by the UI or exported for human review.

## 19. Clean output rule

Raw model output is not clean output.

Clean output is output that has passed through the harness boundary.

Clean output must have, where applicable:

* originating task or intent
* bounded context provenance
* validation result
* policy result
* ledger event
* replay support
* audit projection

The UI must visually distinguish raw model output from clean output.

## 20. Truth-dimension rule

Each artifact type has one truth dimension.

Artifact	Truth dimension	Meaning
GOVERNANCE.md	Normative truth	What must always be true.
ARCHITECTURE.md	Structural truth	How the system is organized.
docs/roadmap/	Planned truth	What may be attempted next.
CHANGELOG.md	Historical truth	What has already changed.
checklists/	Procedural truth	Steps for bounded execution events.
core/ and tests/	Executable truth	What the system actually does.
schemas/	Contract truth	What shared data must satisfy.
memory/	Data truth	Governed facts and run-scoped memory.
README.md	Orientation truth	Human overview.
AGENTS.md	Navigation truth	Agent entrypoint and source index.
docs/examples/	Illustrative truth	Non-authoritative examples.

Truth dimensions must not collapse into each other.

A document that mixes truth dimensions must be split before it becomes authoritative.

## 21. Artifact placement rule

Artifacts must live in the directory that matches their declared truth dimension.

Governance belongs in:

GOVERNANCE.md
docs/governance/

Architecture belongs in:

ARCHITECTURE.md
docs/architecture/

Roadmap planning belongs in:

docs/roadmap/

Historical change records belong in:

CHANGELOG.md

Procedural checklists belong in:

checklists/

Contract schemas belong in:

schemas/

Governed memory belongs in:

memory/

Examples belong in:

docs/examples/

README must remain orientation only.

AGENTS.md must remain navigation only.

## 22. Mutation-path rule

Each artifact type must use the correct mutation path.

Artifact type	Mutation path
Governance	governance_pr
Architecture	architecture_pr
Roadmap	roadmap_update
Changelog	changelog_entry
Checklists	checklist_revision
Code and tests	code_change
Schemas	schema_change
Memory	memory_update
Examples	example_update
README	readme_update
AGENTS.md	agents_update

Changing an artifact through the wrong mutation path is a governance violation.

## 23. Phase execution rule

Phases must execute through one active checklist.

The active checklist is:

checklists/current-phase.md

The durable phase plan lives in:

docs/roadmap/phase-map.md

Completed accepted work is recorded in:

CHANGELOG.md

Agents must work one task surface at a time.

Agents must not create additional phase task files unless explicitly requested through the roadmap or governance mutation path.

Agents must not record phase completion in the roadmap.

Agents must not place historical claims in checklists.

## 24. AGENTS.md rule

AGENTS.md is a navigation contract.

It is not the system of record.

It must remain short, stable, and non-authoritative.

It must point to authoritative sources rather than restating them.

AGENTS.md must not contain:

* full governance
* architecture definitions
* roadmap content
* changelog entries
* examples
* implementation history
* operational checklists
* provider-specific instructions
* long prompt libraries

## 25. README rule

README.md is human orientation.

README.md is non-authoritative.

README.md may explain:

* project purpose
* project status
* intended users
* repository model
* high-level concept
* human-readable overview

README.md must not define:

* runtime authority
* governance rules
* architecture authority
* roadmap commitments
* executable behavior
* schema contracts

## 26. Changelog rule

CHANGELOG.md records historical truth.

CHANGELOG.md may contain completed accepted work only.

CHANGELOG.md must not contain:

* future plans
* proposed phases
* speculative capability
* roadmap commitments
* procedural instructions
* claims of runtime capability that does not exist

Versioning follows:

v0.0.0 = Initial repo
v0.0.x = Phase task increment
v0.x.0 = Major milestone
vX.0.0 = Release version

## 27. Roadmap rule

Roadmap documents describe planned truth.

Roadmap documents may contain:

* planned phases
* sequencing
* dependencies
* intended scope
* validation gates
* exit criteria
* production-readiness targets

Roadmap documents must not contain:

* completed-work claims
* changelog entries
* immutable governance rules
* permanent architecture authority
* bounded procedural checklists

## 28. Checklist rule

Checklists define procedural truth.

Checklists guide bounded execution events.

Checklists may contain:

* active phase tasks
* release steps
* audit steps
* migration steps
* validation steps
* deferred items
* validation logs

Checklists must not define:

* timeless rules
* permanent authority boundaries
* architecture definitions
* roadmap phases
* changelog entries

## 29. Example rule

Examples are illustrative only.

Examples may show:

* sample prompts
* sample workflows
* sample inputs
* sample outputs

Examples must not define:

* policy
* required behavior
* acceptance criteria
* hidden tests
* architecture rules
* governance rules

If an example describes required behavior, that behavior must move to governance, architecture, schemas, tests, or code.

## 30. Testing rule

Tests and code define executable truth.

Tests must cover deterministic behavior where behavior exists.

Tests must include negative paths where authority boundaries are involved.

Tests must avoid interpretive ambiguity.

A feature should not be treated as implemented merely because it is described in documentation.

When documentation and executable behavior conflict, the conflict must be resolved by changing one or both surfaces so the repository returns to a coherent authority model.

## 31. Validation rule

Validation must be deterministic where it creates authority.

Validation must not accept missing evidence.

Validation must not accept malformed evidence.

Validation must not treat heuristic confidence as pass.

Validation must not treat model agreement as pass.

Validation must distinguish advisory evidence from authoritative decisions.

## 32. Failure rule

Unknown is not pass.

Missing evidence is not pass.

Malformed evidence is not pass.

Heuristic confidence is not pass.

Model agreement is not pass.

Provider output is not pass.

UI display state is not pass.

Authority requires deterministic acceptance through Rust-owned paths.

## 33. Negative patterns

The repository must not develop any of the following patterns:

* monolithic AGENTS.md
* governance hidden in README
* architecture hidden in roadmap
* future plans in changelog
* completed work in roadmap
* schemas outside schemas/
* memory as documentation
* memory as policy
* UI authority leakage
* Bash authority leakage
* workflow-only governance
* examples as implicit rules
* model output as accepted work
* replay repair
* audit authority
* hand-written TypeScript or Zod contract truth
* provider-owned policy
* provider-owned state
* terminal-only review as the production UI model

## 34. Non-goals

AJENTIC is not:

* an autonomous agent
* a general chat application
* a UI-authoritative system
* a Bash-governed runtime
* a workflow-governed runtime
* a memory-driven policy engine
* an examples-as-contracts system
* a maximal-context assembly system
* a provider-trusting infrastructure
* a prose-hidden governance system
* a raw JSON dashboard as the primary human interface

## 35. Conflict resolution

If two artifacts claim the same authority, the artifact with the correct truth dimension and placement controls.

If both artifacts are correctly placed but conflict, the top-level anchor controls:

* GOVERNANCE.md controls normative conflicts.
* ARCHITECTURE.md controls structural conflicts.
* schemas/ controls contract conflicts.
* core/ and tests/ control executable behavior.

The conflicting artifact must be corrected through its declared mutation path.

## 36. Drift rule

Documentation drift is governance debt.

Behavior drift is architecture risk.

Truth-dimension collapse is an audit failure.

Authority leakage is a system failure.

A document, workflow, script, UI surface, memory entry, or model output that violates these rules must be corrected before it becomes authoritative or operationally relied upon.
