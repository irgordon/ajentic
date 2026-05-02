---
truth_dimension: planned
authority_level: authoritative
mutation_path: roadmap_update
---
# Phase map
This document defines the planned phase sequence from repository bootstrap to a production-ready harness application.

This document is planned truth.

It does not record completed work.

It does not define governance rules.

It does not define architecture authority.

Active phase execution occurs in:

```text
checklists/current-phase.md
```

Completed accepted work is recorded in:

```text
CHANGELOG.md
```
## Production outcome

The production outcome is a human-usable harness application with a working browser UI that can constrain, inspect, validate, and record local or cloud model-driven work.

The app must support:

* Rust-owned runtime authority
* browser-based TypeScript UI
* usable operator review flow
* typed operator intents
* bounded context inspection
* controlled local/cloud model-provider boundaries
* governed memory
* deterministic validation
* authoritative ledger events
* deterministic replay
* audit projections
* drift-prevention CI
* integration path for local LLMs, Codex Desktop, Antigravity, or similar IDE/model environments

### Global phase execution rules

Each phase must follow the phase execution contract in:

```text
docs/governance/phase-execution-contract.md
```

For every phase:

* Work one task surface at a time.
* Do not expand scope beyond the phase.
* Do not record completion in this roadmap.
* Do not modify governance unless the phase explicitly allows governance work.
* Do not modify architecture unless the phase explicitly allows architecture work.
* Do not add runtime authority outside Rust.
* Do not add UI authority.
* Do not add Bash policy.
* Do not add schemas outside schemas/.
* Do not add memory policy inside memory/.

### Milestone groups

| Milestone group	| Purpose |
| -------- | -------- |
| Milestone 0	| Repository foundation and drift prevention |
| Milestone 1 | Rust authority core |
| Milestone 2	| Context, memory, ledger, replay, and audit |
| Milestone 3	| Provider boundary and model-run control |
| Milestone 4	| Browser UI and human operator workflow |
| Milestone 5	| Local/IDE integration and production hardening |
| Milestone N	| Production-ready harness application |

⸻

## Phase 0: Repository bootstrap

Milestone group: Milestone 0

Primary goal: Create the minimum governed repository structure needed for future work.

Scope:

* Create top-level anchors.
* Create canonical directories.
* Create minimal Rust compile skeleton.
* Create minimal browser UI placeholder structure.
* Create minimal scripts.
* Create minimal schema placeholders.
* Create initial GitHub workflow stubs.
* Create initial changelog entry.

Allowed:

* docs/governance/GOVERNANCE.md
* docs/architecture/ARCHITECTURE.md
* AGENTS.md
* README.md
* CHANGELOG.md
* canonical directories
* scripts/bootstrap_repo.py
* placeholder Rust modules
* placeholder UI files
* placeholder schemas
* placeholder workflows

Not allowed:

* runtime harness behavior
* model provider adapters
* real policy logic
* real validation logic
* ledger persistence
* replay logic
* UI state mutation
* generated TypeScript bindings
* hand-written Zod contracts
* non-placeholder provider integration

Validation gate:

Required commands:

```python
python3 scripts/bootstrap_repo.py
python3 scripts/bootstrap_repo.py
python3 -m compileall scripts/bootstrap_repo.py
bash -n scripts/*.sh
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
cargo check --manifest-path core/Cargo.toml
```

UI validation, when Node dependencies are available:

```bash
cd ui
npm install
npm run typecheck
npm run lint
npm run build
```

Required checks:

* bootstrap is idempotent
* governed Markdown files have frontmatter
* placeholder schemas are valid JSON
* Rust skeleton compiles
* shell scripts parse
* UI placeholder typechecks when dependencies are installed

Exit criteria:

Phase 0 is complete when the repository has the canonical structure, top-level anchors, minimal compile skeleton, and valid bootstrap path without claiming runtime harness capability.

Boundary note:

Phase 0 creates structure only. It must not imply that the harness can run, validate, replay, or control model output.

⸻

## Phase 1: Governance and architecture baseline

Milestone group: Milestone 0

Primary goal: Establish the smallest durable governance and architecture baseline.

Scope:

* Define top-level governance truth.
* Define top-level architecture truth.
* Define truth dimensions.
* Define authority boundaries.
* Define phase execution contract.
* Define browser UI structural target.
* Define artifact placement and mutation path boundaries.

Allowed:

* GOVERNANCE.md
* ARCHITECTURE.md
* docs/governance/
* docs/architecture/
* AGENTS.md navigation updates only

Not allowed:

* implementation status in governance
* roadmap progress in architecture
* runtime implementation
* schema contract expansion unless needed for frontmatter validation
* UI implementation beyond structural description
* changelog entries except after accepted completion

Validation gate:

Required commands:

```python
python3 -m compileall scripts/bootstrap_repo.py
bash -n scripts/*.sh
cargo check --manifest-path core/Cargo.toml
```

Required review checks:

* governance contains timeless rules only
* architecture contains structural truth only
* subordinate governance docs do not contradict GOVERNANCE.md
* subordinate architecture docs do not contradict ARCHITECTURE.md
* AGENTS.md remains a navigation contract

Exit criteria:

Phase 1 is complete when governance, architecture, truth-dimension, and phase-execution boundaries are explicit enough for agents to follow without relying on unstated assumptions.

Boundary note:

This phase defines control surfaces. It must not implement runtime control behavior.

⸻

## Phase 2: Phase execution loop and active checklist

Milestone group: Milestone 0

Primary goal: Establish the working loop agents use to move through phases.

Scope:

* Define durable phase map.
* Define active current-phase checklist.
* Define changelog handoff boundary.
* Ensure agents have one active execution surface.

Allowed:

* docs/roadmap/phase-map.md
* docs/roadmap/sequencing.md
* checklists/current-phase.md
* docs/governance/phase-execution-contract.md

Not allowed:

* completed-work claims in roadmap
* permanent rules in checklist
* implementation status in governance
* additional per-phase working files unless explicitly required
* runtime code implementation

Validation gate:

Required checks:

```python
python3 -m compileall scripts/bootstrap_repo.py
bash -n scripts/*.sh
cargo check --manifest-path core/Cargo.toml
```

Required review checks:

* phase-map.md is planned truth
* current-phase.md is procedural truth
* current phase checklist is the only active phase execution tracker
* changelog records only accepted completed work

Exit criteria:

Phase 2 is complete when agents can execute the next phase from checklists/current-phase.md without creating new planning, governance, or status documents.

Boundary note:

The phase map plans work. It does not record work.

⸻

## Phase 3: CI and structure drift gates

Milestone group: Milestone 0

Primary goal: Add deterministic repository checks before runtime behavior expands.

Scope:

* Implement structure lint.
* Implement CI baseline.
* Validate schema placement.
* Validate memory placement.
* Validate frontmatter placement.
* Validate shell scripts.
* Validate Rust skeleton.
* Validate UI placeholder behavior.

Allowed:

* .github/workflows/structure-lint.yml
* .github/workflows/ci.yml
* .github/workflows/docs-gate.yml
* .github/workflows/memory-lint.yml
* schemas/docs/artifact-frontmatter.schema.json
* scripts needed for validation

Not allowed:

* runtime harness behavior
* model provider integration
* UI authority
* workflow-defined governance not traceable to governed docs
* brittle file-list enforcement where directory and metadata checks are sufficient

Validation gate:

Required commands:

```python
python3 -m compileall scripts/bootstrap_repo.py
bash -n scripts/*.sh
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
cargo fmt --manifest-path core/Cargo.toml -- --check
cargo check --manifest-path core/Cargo.toml --all-targets
cargo test --manifest-path core/Cargo.toml --all-targets
cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
```

UI validation:

```bash
cd ui
npm install
npm run typecheck
npm run lint
npm run build
```

Required tests/checks:

* required top-level anchors exist
* no unexpected top-level directories
* schemas live only under schemas/
* memory contains no Markdown docs or schemas
* ephemeral memory commits only .gitignore
* governed Markdown frontmatter is present
* CI uses npm ci only when a lockfile exists

Exit criteria:

Phase 3 is complete when CI can block structure drift, schema drift, memory placement violations, and basic Rust/UI breakage.

Boundary note:

Workflows enforce rules defined elsewhere. Workflows must not become governance roots.

⸻

## Phase 4: Contract schema baseline

Milestone group: Milestone 1

Primary goal: Define the first useful shared contracts for the harness.

Scope:

* Artifact frontmatter schema.
* Operator intent schema.
* Ledger event schema.
* Context packet schema.
* Memory entry schema.
* Replay report schema.
* Validation report schema.

Allowed:

* schemas/docs/
* schemas/intents/
* schemas/events/
* schemas/context/
* schemas/memory/
* schemas/traces/
* schema validation tests or scripts

Not allowed:

* schemas outside schemas/
* TypeScript-authored contract truth
* Zod-authored contract truth
* Rust structs that intentionally conflict with schemas
* provider-specific schema assumptions
* open-ended authority payloads without planned narrowing

Validation gate:

Required commands:

```bash
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
cargo check --manifest-path core/Cargo.toml
```

Required checks:

* all schema files are valid JSON
* schema files use Draft 2020-12
* schemas use explicit required fields where authority boundaries require them
* schema placeholders are replaced where the phase scope requires real contracts
* generated TypeScript and validator outputs are not hand-authored in this phase unless explicitly scoped

Exit criteria:

Phase 4 is complete when initial shared data contracts are strict enough to support Rust validation, UI typing, and CI checks without creating a second contract authority.

Boundary note:

schemas/ owns contract truth. TypeScript, Zod, and Rust may consume contracts but must not redefine them.

⸻

## Phase 5: Rust authority skeleton

Milestone group: Milestone 1

Primary goal: Establish the authoritative Rust module boundaries.

Scope:

* State module.
* Context module.
* Memory module.
* Policy module.
* Validation module.
* Execution module.
* Ledger module.
* Replay module.
* Audit module.
* API/CLI module.
* Error module.

Allowed:

* Rust module definitions
* typed placeholder structs/enums
* compile-shape tests
* minimal CLI routing shape

Not allowed:

* real model execution
* provider-specific behavior
* UI mutation logic
* ledger persistence beyond typed shape
* replay reconstruction beyond placeholder shape
* policy decisions not backed by tests
* broad abstractions not needed for current surfaces

Validation gate:

Required commands:

```bash
cargo fmt --manifest-path core/Cargo.toml -- --check
cargo check --manifest-path core/Cargo.toml --all-targets
cargo test --manifest-path core/Cargo.toml --all-targets
cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
./scripts/check.sh
```

Required tests:

* core module imports compile
* public module boundaries are stable enough for next phase
* placeholder CLI compiles
* no UI/Bash surface bypasses Rust authority

Exit criteria:

Phase 5 is complete when Rust has a coherent compile-safe authority skeleton that matches ARCHITECTURE.md.

Boundary note:

Skeleton types do not imply implemented runtime authority.

⸻

## Phase 6: Candidate lifecycle state machine

Milestone group: Milestone 1

Primary goal: Implement authoritative candidate lifecycle transitions in Rust.

Scope:

Implement candidate lifecycle states:

* CREATED
* EVALUATING
* FAILED
* BLOCKED
* PASSED
* PROMOTED_TIER_1
* REJECTED
* UNKNOWN

Implement legal transitions and typed lifecycle errors.

Allowed:

* lifecycle state enum
* legal transition checks
* typed lifecycle errors
* lifecycle unit tests
* documentation stating lifecycle boundary

Not allowed:

* model generation
* evaluator execution
* governance approval
* promotion authorization
* ledger persistence
* replay
* UI state mutation
* Python lifecycle decisions

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* valid transitions pass
* invalid transitions fail
* CREATED -> PROMOTED_TIER_1 fails
* UNKNOWN -> PASSED fails
* terminal states remain terminal
* non-Rust surfaces cannot set lifecycle state through ordinary APIs

Exit criteria:

Phase 6 is complete when lifecycle transitions are typed, tested, and impossible to bypass through ordinary Rust APIs.

Boundary note:

PROMOTED_TIER_1 is a lifecycle state shape. Authorization to enter this state belongs to future Rust governance or promotion logic.

⸻

## Phase 7: Policy and validation baseline

Milestone group: Milestone 1

Primary goal: Implement deterministic pass/fail boundaries over typed evidence.

Scope:

* Policy gate types.
* Validation result types.
* Evidence references.
* Missing evidence failures.
* Malformed evidence failures.
* Unknown-is-not-pass behavior.

Allowed:

* core/src/policy/
* core/src/validation/
* policy/validation tests
* schema validation helpers if scoped

Not allowed:

* model-based authority
* heuristic pass/fail gates
* UI-computed pass/fail
* Bash-computed pass/fail
* provider execution
* promotion authorization beyond typed shape

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* missing evidence fails
* malformed evidence fails
* unknown status fails
* policy pass requires required evidence
* validation output is deterministic
* UI cannot create validation authority
* Bash cannot create validation authority

Exit criteria:

Phase 7 is complete when policy and validation can deterministically accept or reject typed evidence without relying on model confidence.

Boundary note:

Validation evaluates evidence. It does not directly mutate state.

⸻

## Phase 8: Context packet assembly

Milestone group: Milestone 2

Primary goal: Create bounded, inspectable, provenance-bearing context packets.

Scope:

* Context packet type.
* Context slice type.
* Source references.
* Provenance.
* Budget metadata.
* Deterministic assembly for fixed inputs.

Allowed:

* core/src/context/
* schemas/context/
* tests/context/
* context projection shape

Not allowed:

* provider invocation
* model-generated context authority
* maximal context accumulation
* memory mutation
* UI context editing
* hidden source inclusion

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* fixed inputs produce stable packet shape
* every slice has provenance
* included sources have truth dimensions
* used budget is tracked
* irrelevant sources are excluded when not selected
* packet validation rejects missing provenance

Exit criteria:

Phase 8 is complete when a context packet can be assembled, inspected, and validated for fixed inputs.

Boundary note:

Context is what the model sees. It is not authority by itself.

⸻

## Phase 9: Governed memory baseline

Milestone group: Milestone 2

Primary goal: Make memory usable as governed data without becoming policy.

Scope:

* Persistent memory entries.
* Ephemeral memory behavior.
* Memory provenance.
* Memory loading through Rust.
* Memory validation through Rust.
* Memory snapshot command.
* Ephemeral clear command.

Allowed:

* memory/
* schemas/memory/
* core/src/memory/
* tests/memory/
* Rust CLI memory commands

Not allowed:

* memory policy
* normative rules in memory
* roadmap commitments in memory
* direct UI memory file edits
* Bash memory mutation outside Rust CLI
* schemas inside memory

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* persistent memory validates
* memory entry without provenance fails
* ephemeral memory is not committed
* memory snapshot uses Rust path
* clear-ephemeral uses Rust path
* memory cannot define policy

Exit criteria:

Phase 9 is complete when memory can be read, validated, snapshotted, and cleared through Rust-owned paths.

Boundary note:

Memory stores facts and context material. It does not define rules.

⸻

## Phase 10: Ledger event recording

Milestone group: Milestone 2

Primary goal: Record accepted authoritative events.

Scope:

* Ledger event type.
* Ledger entry type.
* Append path.
* Query path.
* Revision sequence.
* Event integrity checks.

Allowed:

* core/src/ledger/
* schemas/events/
* tests/ledger/

Not allowed:

* ledger deciding validity
* direct UI ledger writes
* direct Bash ledger writes
* replay repair
* provider execution
* untyped event payloads at authority boundaries

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* valid append succeeds
* missing event ID fails
* invalid revision sequence fails
* accepted state transition is recorded
* direct non-core append path is unavailable
* ledger query does not mutate state

Exit criteria:

Phase 10 is complete when accepted events can be recorded and queried with deterministic sequencing.

Boundary note:

Ledger records accepted authority. It does not create validity.

⸻

## Phase 11: Replay and audit baseline

Milestone group: Milestone 2

Primary goal: Reconstruct and explain recorded runs.

Scope:

* Replay engine.
* Replay integrity checks.
* Replay report type.
* Audit timeline projection.
* Audit export projection.

Allowed:

* core/src/replay/
* core/src/audit/
* schemas/traces/
* tests/replay/

Not allowed:

* replay repair
* replay inference of missing authority
* audit mutation
* audit authority
* UI replay repair
* ledger rewriting

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* valid ledger reconstructs
* missing event fails
* reordered event fails
* conflicting event fails
* replay does not repair history
* audit projection does not mutate state

Exit criteria:

Phase 11 is complete when valid ledger history can be replayed and projected for human audit.

Boundary note:

Replay reconstructs. Audit explains. Neither creates authority.

⸻

## Phase 12: API and CLI boundary

Milestone group: Milestone 2

Primary goal: Expose controlled request and projection surfaces.

Scope:

* CLI command shape.
* Typed intent submission.
* State projection.
* Context projection.
* Memory projection.
* Timeline projection.
* Replay projection.
* Audit projection.

Allowed:

* core/src/api/
* scripts/
* tests/api/

Not allowed:

* direct state mutation endpoint
* direct ledger append endpoint
* direct memory file edit endpoint
* direct policy override endpoint
* UI-only authority path
* Bash-only authority path

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
bash -n scripts/*.sh
```

Required tests:

* malformed intent rejected
* direct state mutation unavailable
* direct ledger append unavailable
* projection endpoints are read-only
* script wrappers call Rust CLI behavior
* scripts are replaceable by direct CLI invocation

Exit criteria:

Phase 12 is complete when external surfaces can request and inspect through controlled Rust-owned boundaries.

Boundary note:

API and CLI expose authority paths; they do not bypass them.

⸻

## Phase 13: Provider adapter boundary

Milestone group: Milestone 3

Primary goal: Connect local and cloud model providers without trusting provider output.

Scope:

* Provider request type.
* Provider response type.
* Local provider boundary.
* Cloud provider boundary.
* Candidate output path.
* Provider output validation path.
* Recorded provider boundary inputs where needed for replay.

Allowed:

* core/src/execution/
* core/src/api/
* schemas/events/
* provider boundary tests

Not allowed:

* provider-owned policy
* provider-owned state
* direct provider-to-ledger writes
* direct provider-to-memory writes
* trusting model output
* UI calling provider APIs directly

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* provider output enters candidate path
* provider output cannot mutate state directly
* malformed provider response fails
* provider adapter does not own policy
* provider boundary records replay-required data where scoped
* local/cloud path shares same trust boundary model

Exit criteria:

Phase 13 is complete when local and cloud provider outputs can enter the harness as untrusted candidate material.

Boundary note:

Provider integration does not make model output trusted.

⸻

## Phase 14: End-to-end model run loop

Milestone group: Milestone 3

Primary goal: Connect task input, context assembly, provider invocation, validation, ledger, replay, and audit into a controlled run loop.

Scope:

* Task input.
* Context packet creation.
* Provider invocation.
* Candidate output validation.
* Accepted/rejected result path.
* Ledger event recording.
* Replay and audit projection.

Allowed:

* core/
* schemas/
* tests/
* limited script wrapper updates

Not allowed:

* browser UI dependence
* direct model output acceptance
* skipping validation
* skipping ledger recording for accepted events
* hidden manual steps
* broad provider-specific branching

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
```

Required tests:

* full run accepts valid candidate output
* invalid candidate output rejects
* accepted run records ledger events
* accepted run can replay
* rejected output does not become clean output
* audit projection explains run outcome

Exit criteria:

Phase 14 is complete when a model-driven run can complete through the Rust harness boundary and produce accepted or rejected output with replay/audit support.

Boundary note:

This phase creates the first controlled run loop. It does not require full UI usability.

⸻

## Phase 15: Browser UI shell

Milestone group: Milestone 4

Primary goal: Create the browser UI foundation.

Scope:

* App shell.
* Routes.
* Navigation.
* Responsive layout foundation.
* API client.
* Event stream client shape.
* Generated types location.
* Generated validators location.

Allowed:

* ui/
* ui/src/app/
* ui/src/api/
* ui/src/styles/
* ui/src/types/generated/
* ui/src/validation/generated/

Not allowed:

* UI authority
* direct file writes
* direct provider calls
* direct memory edits
* direct ledger edits
* hand-authored contract truth
* UI-computed pass/fail decisions

Validation gate:

Required commands:

```bash
cd ui
npm install
npm run typecheck
npm run lint
npm run build
```

Required checks:

* UI builds
* UI typechecks
* responsive shell exists
* no direct authority path exists
* API client is typed or prepared for generated types
* raw JSON is not the intended default UI model

Exit criteria:

Phase 15 is complete when the browser UI shell can build and route between planned review surfaces without mutating authority.

Boundary note:

The UI is a review and operator-intent surface only.

⸻

## Phase 16: State and run overview UI

Milestone group: Milestone 4

Primary goal: Make current harness state visually obvious to a human operator.

Scope:

* Overview screen.
* Run overview screen.
* State machine screen.
* State badge.
* Status cards.
* Available actions display.

Allowed:

* ui/src/screens/
* ui/src/components/
* ui/src/api/
* UI contract tests where scoped

Not allowed:

* UI-computed allowed transitions
* UI-computed authority
* direct state mutation
* local-only pass/fail decisions
* hidden JSON-only display

Validation gate:

Required commands:

```bash
cd ui
npm run typecheck
npm run lint
npm run build
```

Required tests/checks:

* state comes from Rust projection shape
* allowed actions come from projection shape
* current state is visible on desktop/tablet/mobile layout
* rejected or pending states are readable
* UI does not mutate state directly

Exit criteria:

Phase 16 is complete when an operator can immediately understand the current run and lifecycle state through the browser UI.

Boundary note:

The UI may display state. Rust owns state.

⸻

## Phase 17: Context, memory, policy, and validation UI

Milestone group: Milestone 4

Primary goal: Make model-visible inputs and acceptance decisions inspectable.

Scope:

* Context Inspector.
* Memory Inspector.
* Policy and Validation Console.
* Context slice cards.
* Memory entry cards.
* Evidence list.
* Raw data disclosure.

Allowed:

* ui/src/screens/
* ui/src/components/
* ui/src/api/
* generated types/validators if available
* UI contract tests

Not allowed:

* raw JSON as default UI
* UI memory file edits
* UI policy decisions
* UI validation decisions
* hiding advisory vs authoritative distinction
* treating model confidence as pass

Validation gate:

Required commands:

```bash
cd ui
npm run typecheck
npm run lint
npm run build
```

Required tests/checks:

* operator can see what the model saw
* every context slice displays provenance when available
* memory entries display provenance
* validation failures are visible
* advisory evidence is visually distinct from authoritative decisions
* raw data is behind explicit disclosure

Exit criteria:

Phase 17 is complete when a human operator can inspect context, memory, policy, and validation without reading raw JSON by default.

Boundary note:

Visibility does not create authority.

⸻

## Phase 18: Timeline, replay, and clean output UI

Milestone group: Milestone 4

Primary goal: Make run history, replay status, and accepted output usable.

Scope:

* Run Timeline screen.
* Replay screen.
* Clean Output screen.
* Timeline component.
* Replay report display.
* Clean output display.
* Raw model output distinction.

Allowed:

* ui/src/screens/
* ui/src/components/
* ui/src/api/
* UI contract tests

Not allowed:

* UI replay repair
* UI audit authority
* UI ledger mutation
* treating raw model output as clean output
* hiding replay failure details

Validation gate:

Required commands:

```bash
cd ui
npm run typecheck
npm run lint
npm run build
```

Required tests/checks:

* timeline derives from projection shape
* replay readiness is visible
* replay failure details are visible
* raw output and clean output are visually distinct
* accepted output references validation/policy/ledger context where available

Exit criteria:

Phase 18 is complete when an operator can inspect run history, replay status, and clean output through the UI.

Boundary note:

Clean output is accepted harness output. Raw model output is candidate material.

⸻

## Phase 19: Operator intent controls

Milestone group: Milestone 4

Primary goal: Allow safe human action through typed operator intents.

Scope:

* Operator intent tray.
* Intent buttons.
* Approve request.
* Reject request.
* Retry request.
* Replay request.
* Context rebuild request.
* Memory request actions.
* Rejection reason display.

Allowed:

* ui/src/components/
* ui/src/api/
* schemas/intents/
* tests/ui-contracts/
* Rust API intent handling if scoped

Not allowed:

* direct UI mutation
* force approve
* promote anyway
* skip policy
* skip validation
* edit ledger
* repair replay
* trust output

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Required tests/checks:

* every UI action emits typed intent
* malformed intent rejects
* accepted intent appears through ledger-derived projection
* rejected intent displays Rust-provided reason
* no UI path directly mutates state, memory, ledger, replay, or policy

Exit criteria:

Phase 19 is complete when operators can request actions through the browser UI without authority leakage.

Boundary note:

Intent is a request. Rust decides.

⸻

## Phase 20: Responsive UI and operator usability hardening

Milestone group: Milestone 4

Primary goal: Make the browser UI usable on desktop, tablet, and mobile.

Scope:

* Responsive layout.
* Navigation refinement.
* Readable status hierarchy.
* Empty states.
* Error states.
* Keyboard navigation.
* Touch target review.
* Raw-data disclosure refinement.
* Visual distinction for authority states.

Allowed:

* ui/
* UI tests
* accessibility checks where scoped

Not allowed:

* new runtime authority
* new policy logic in UI
* UI-only validation
* terminal-only critical review flows
* raw JSON as primary workflow

Validation gate:

Required commands:

```bash
cd ui
npm run typecheck
npm run lint
npm run build
```

Required checks:

* desktop layout usable
* tablet layout usable
* mobile layout usable
* current state visible or quickly reachable
* required operator attention visible
* core review flow does not require terminal-only inspection
* status does not rely on color alone

Exit criteria:

Phase 20 is complete when the UI can support real human review across desktop, tablet, and mobile layouts.

Boundary note:

Human usability is part of production readiness.

⸻

## Phase 21: Local LLM and IDE integration boundary

Milestone group: Milestone 5

Primary goal: Make the harness applicable to local LLM and IDE-style model workflows.

Scope:

* Local model connection path.
* IDE adapter request path.
* Codex Desktop-style integration boundary.
* Antigravity-style integration boundary.
* Local operator startup path.
* Provider boundary documentation.
* UI projection support for integrated runs.

Allowed:

* core/src/api/
* core/src/execution/
* scripts/
* docs/operations/
* integration tests where scoped

Not allowed:

* IDE-owned authority
* provider-owned authority
* direct IDE-to-ledger writes
* direct IDE-to-memory writes
* bypassing validation for IDE convenience
* terminal-only review requirement

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Required tests/checks:

* local model output enters candidate path
* IDE-style output enters candidate path
* provider output remains untrusted
* UI can display integrated run projections
* controlled run does not require terminal-only review
* integration boundary can be disabled or rejected deterministically

Exit criteria:

Phase 21 is complete when the harness can attach to local or IDE-style model workflows without surrendering authority.

Boundary note:

Integration increases reach. It must not reduce control.

⸻

## Phase 22: Production hardening

Milestone group: Milestone 5

Primary goal: Harden the harness for controlled production use.

Scope:

* Negative-path tests.
* Malformed input tests.
* Replay integrity tests.
* Memory provenance tests.
* UI intent contract tests.
* Provider boundary tests.
* CI hardening.
* Documentation link checks where scoped.
* Packaging review.

Allowed:

* core/
* ui/
* tests/
* schemas/
* .github/workflows/
* scripts/
* docs/operations/
* checklists/release.md

Not allowed:

* new unplanned feature surfaces
* governance drift
* architecture drift
* UI authority shortcuts
* provider trust shortcuts
* skipped validation for release convenience

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
```

Required tests/checks:

* invalid external input fails closed
* malformed provider output fails
* missing evidence fails
* replay integrity violations fail
* UI actions remain intent-only
* memory provenance required
* CI gates block drift
* release checklist is usable

Exit criteria:

Phase 22 is complete when known authority leakage, replay integrity, provider trust, UI mutation, and drift-prevention paths have negative-path coverage.

Boundary note:

Hardening should reduce risk without adding new scope.

## Phase N: Production-ready harness application

Milestone group: Milestone N

Primary goal: Release a production-ready harness application for human-operated model work.

Scope:

* Rust authority core.
* Browser UI.
* Local/cloud provider boundaries.
* Local/IDE integration boundary.
* Governed memory.
* Context inspection.
* Deterministic validation.
* Ledger.
* Replay.
* Audit.
* CI gates.
* Operator workflows.

Allowed:

* release packaging
* release checklist
* changelog release entry
* production readiness validation
* final documentation alignment

Not allowed:

* unvalidated provider output
* UI authority
* Bash authority
* workflow-only governance
* memory policy
* unreplayable accepted runs where replay is required
* raw JSON-only human review flow
* undocumented production limitations

Validation gate:

Required commands:

```bash
cargo test --workspace
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
```

Required production checks:

* operator can run and inspect a model workflow without living in the terminal
* operator can see current state, context, memory, policy, validation, timeline, replay, and clean output
* raw model output is visually distinct from clean output
* local/cloud model output enters the harness as untrusted candidate material
* accepted output is ledgered
* accepted run can replay where required
* audit projection is usable
* UI works on desktop, tablet, and mobile
* CI blocks structure, schema, memory, and truth-dimension drift

Exit criteria:

Phase N is complete when the harness is usable by a human operator for controlled model-driven work and can attach to local/cloud/IDE-style model workflows without losing Rust-owned authority.

Boundary note:

Production-ready means usable, inspectable, replayable, and governed. It does not mean autonomous.

This map is agent-usable because every phase has the same control structure:

```text
Phase
Milestone group
Primary goal
Scope
Allowed
Not allowed
Validation gate
Required tests/checks
Exit criteria
Boundary note
```

It also avoids the main failure modes: agents will not need to infer scope, invent validation, or decide where to record progress.
