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

Historical implementation through v0.0.19 followed a more granular Rust authority sequence than this roadmap originally projected. Completed work is recorded only in CHANGELOG.md. This roadmap resumes from the current repository state and defines planned work from the next phase forward.
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

Required tests/checks:

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
* checklists/current-phase.md
* docs/governance/phase-execution-contract.md

Not allowed:

* completed-work claims in roadmap
* permanent rules in checklist
* implementation status in governance
* additional per-phase working files unless explicitly required
* runtime code implementation

Validation gate:

Required tests/checks:

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

Required tests/checks:

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

Required tests/checks:

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

## Planned sequence from current state

The phase entries below resume planning from the post-v0.0.19 repository state. Earlier phase numbers may appear in historical records, but completed work remains authoritative only in CHANGELOG.md. Future implementation planning resumes at Phase 20.

## Phase 20: Roadmap Alignment Check and UI Entry Reset

Milestone group: Milestone 4

Primary goal: Verify roadmap/changelog alignment after reconciliation and prepare the repository for UI work without implementing UI behavior.

Scope:

* Compare `docs/roadmap/phase-map.md` to `CHANGELOG.md`.
* Correct planned future sequencing drift only.
* Reset `checklists/current-phase.md` entry framing for UI planning handoff.

Allowed:

* `docs/roadmap/phase-map.md`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* UI implementation
* runtime behavior
* provider adapter work
* schema changes

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Roadmap and changelog alignment is verified, future planned sequencing is corrected if needed, and no implementation behavior is changed.

Boundary note:

This is a planning reconciliation phase only.

## Phase 21: Browser UI Shell Baseline

Milestone group: Milestone 4

Primary goal: Create the browser UI shell and layout structure without runtime authority, data mutation, or API integration.

Scope:

* App shell layout.
* Primary navigation layout.
* Non-authoritative route scaffolding.

Allowed:

* `ui/` layout shell files
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* runtime authority paths
* state mutation
* provider calls
* direct API integration

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

The UI shell builds and provides non-authoritative layout scaffolding only.

Boundary note:

UI visibility is non-authoritative and intent-free in this phase.

## Phase 22: Read-Only API Projection Surface for UI

Milestone group: Milestone 4

Primary goal: Add read-only projection shapes or mock-safe UI data boundaries for existing Rust authority outputs without serving HTTP or mutating state.

Scope:

* Read-only projection shapes.
* Mock-safe boundary adapters.
* Projection typing alignment.

Allowed:

* `core/src/api/` projection shape files
* `ui/src/api/` typed read-only adapters
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* HTTP serving
* state mutation
* policy execution
* provider integration

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Read-only projection boundaries exist for UI consumption without opening mutation surfaces.

Boundary note:

Projection shape exposure does not transfer authority.

## Phase 23: State and Run Overview UI

Milestone group: Milestone 4

Primary goal: Display lifecycle, execution decision, promotion decision, and run summary projections in the browser UI as read-only views.

Scope:

* State and run overview screens.
* Read-only lifecycle and decision cards.
* Run summary projection presentation.

Allowed:

* `ui/src/screens/`
* `ui/src/components/`
* `ui/src/api/` read-only consumers
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* intent submission controls
* mutation paths
* provider invocation
* policy decision logic in UI

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Operators can inspect state and run overview projections in read-only UI views.

Boundary note:

Displayed decisions remain Rust-owned projections.

## Phase 24: Context Packet and Memory Inspection UI

Milestone group: Milestone 4

Primary goal: Display context packet and memory snapshot projections in the browser UI as read-only inspection surfaces.

Scope:

* Context inspection views.
* Memory snapshot inspection views.
* Provenance display for inspected data.

Allowed:

* `ui/src/screens/`
* `ui/src/components/`
* `ui/src/api/` read-only consumers
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* memory edits
* context mutation
* policy mutation
* runtime execution

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Context and memory projections are inspectable as read-only surfaces.

Boundary note:

Inspection capability does not permit data mutation.

## Phase 25: Roadmap and Changelog Alignment Check

Milestone group: Milestone 4

Primary goal: Compare planned roadmap sequencing against CHANGELOG.md and correct future plan drift before continuing UI expansion.

Scope:

* Planned-vs-historical drift audit.
* Future phase sequencing corrections only.
* Checklist handoff update.

Allowed:

* `docs/roadmap/phase-map.md`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* implementation behavior changes
* governance changes
* architecture changes
* status tracking in roadmap

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Roadmap future sequencing is re-aligned to changelog history with no implementation behavior changes.

Boundary note:

Alignment phases update planned sequencing only.

## Phase 26: Policy, Validation, and Execution Decision UI

Milestone group: Milestone 4

Primary goal: Display policy, validation, and execution decision results as readable, non-authoritative UI projections.

Scope:

* Policy result views.
* Validation result views.
* Execution decision views.

Allowed:

* `ui/src/screens/`
* `ui/src/components/`
* `ui/src/api/` read-only consumers
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* UI decision authority
* direct policy evaluation
* state mutation
* provider calls

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Policy, validation, and execution decisions are visible as read-only projections.

Boundary note:

Decision authority remains in Rust.

## Phase 27: Ledger, Replay, Audit, and Clean Output UI

Milestone group: Milestone 4

Primary goal: Display ledger timeline, replay readiness/reconstruction results, audit projections, and clean output summaries without granting UI authority.

Scope:

* Ledger timeline views.
* Replay readiness and reconstruction views.
* Audit projection views.
* Clean output summary views.

Allowed:

* `ui/src/screens/`
* `ui/src/components/`
* `ui/src/api/` read-only consumers
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* ledger mutation
* replay repair
* audit mutation
* clean output authority changes

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Operators can inspect ledger/replay/audit/output projections with no authority leakage.

Boundary note:

UI remains projection-only for ledger, replay, audit, and output.

## Phase 28: Operator Intent Controls UI

Milestone group: Milestone 4

Primary goal: Add typed operator intent controls that emit intent-shaped data only; no direct mutation, policy bypass, or execution authority.

Scope:

* Intent control components.
* Typed intent payload emission.
* Rejection reason display.

Allowed:

* `ui/src/components/`
* `ui/src/api/` intent clients
* `tests/ui-contracts/`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* direct mutation
* policy bypass
* validation bypass
* provider invocation authority

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

UI emits typed intents only and cannot mutate authority surfaces directly.

Boundary note:

Intent submission is request-only.

## Phase 29: Responsive UI and Operator Usability Hardening

Milestone group: Milestone 4

Primary goal: Improve desktop, tablet, and mobile usability, including responsive layout, readable hierarchy, empty states, error states, and operator clarity.

Scope:

* Responsive layout refinements.
* Usability and hierarchy improvements.
* Empty/error state clarity.

Allowed:

* `ui/`
* UI tests
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* new authority paths
* policy logic in UI
* provider authority
* runtime mutation behavior

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

UI remains non-authoritative while usability improves across target form factors.

Boundary note:

Usability hardening must not add authority leakage.

## Phase 30: Roadmap and Changelog Alignment Check

Milestone group: Milestone 5

Primary goal: Compare planned roadmap sequencing against CHANGELOG.md and correct future plan drift before provider and integration work.

Scope:

* Planned-vs-historical drift audit.
* Future sequencing corrections only.
* Provider/integration readiness handoff update.

Allowed:

* `docs/roadmap/phase-map.md`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* provider implementation
* runtime implementation
* governance changes
* roadmap completion status tracking

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Roadmap future sequencing is corrected for drift before provider and integration expansion.

Boundary note:

Alignment phases remain documentation-only planning maintenance.

## Phase 31: Provider Adapter Boundary

Milestone group: Milestone 5

Primary goal: Define and implement the first provider adapter boundary without making provider output authoritative.

Scope:

* Provider adapter request/response boundary.
* Untrusted provider output intake path.
* Validation handoff hooks.

Allowed:

* `core/src/execution/`
* `core/src/api/`
* provider-boundary tests
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* provider-owned policy
* provider-owned state
* direct provider-to-ledger writes
* trusted provider output

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

A bounded provider adapter path exists with untrusted output treatment preserved.

Boundary note:

Provider output remains candidate input until Rust validation.

## Phase 32: End-to-End Controlled Model Run Loop

Milestone group: Milestone 5

Primary goal: Connect bounded context, provider output intake, validation, decisions, ledger recording, replay, audit, and clean output through a controlled local flow.

Scope:

* End-to-end local controlled flow wiring.
* Accepted/rejected branching.
* Ledger, replay, and audit linkages.

Allowed:

* `core/`
* bounded tests
* minimal script updates
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* direct acceptance of raw model output
* skipping validation
* skipping ledger events for accepted outcomes
* UI authority shortcuts

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Controlled model run flow completes deterministically with replay/audit-supporting outputs.

Boundary note:

Controlled flow requires validation before authority acceptance.

## Phase 33: Local LLM and IDE Integration Boundary

Milestone group: Milestone 5

Primary goal: Define local LLM and IDE integration surfaces without weakening Rust-owned authority boundaries.

Scope:

* Local LLM integration boundary.
* IDE integration boundary.
* Integration projection wiring.

Allowed:

* `core/src/api/`
* `core/src/execution/`
* `scripts/`
* integration tests
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* IDE/provider authority ownership
* direct integration writes to ledger or memory
* validation bypasses
* terminal-only operational dependency

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Local and IDE integration boundaries exist while retaining Rust-owned authority constraints.

Boundary note:

Integration expands entry points, not authority surfaces.

## Phase 34: Production Hardening

Milestone group: Milestone 5

Primary goal: Harden failure paths, validation coverage, packaging boundaries, and operational safety before any production-capable claim.

Scope:

* Failure-path expansion.
* Validation and negative-path coverage expansion.
* Packaging boundary and safety checks.

Allowed:

* `core/`
* `ui/`
* `tests/`
* `schemas/`
* `.github/workflows/`
* `scripts/`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* new unplanned feature surfaces
* governance drift
* architecture drift
* authority shortcuts for release speed

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

High-risk failure modes and authority-boundary regressions are covered by deterministic checks.

Boundary note:

Hardening reduces risk without expanding product scope.

## Phase 35: Roadmap and Changelog Alignment Check

Milestone group: Milestone 5

Primary goal: Compare planned roadmap sequencing against CHANGELOG.md and correct future plan drift before release-candidate planning.

Scope:

* Planned-vs-historical drift audit.
* Future sequencing corrections only.
* Release-candidate planning handoff update.

Allowed:

* `docs/roadmap/phase-map.md`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* release-candidate implementation
* runtime behavior changes
* roadmap completion history
* governance or architecture mutations

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Roadmap future sequencing is aligned with changelog history before release-candidate planning.

Boundary note:

Alignment phases preserve truth-dimension boundaries.

## Phase 36: Release Candidate Boundary

Milestone group: Milestone 5

Primary goal: Define the minimum release-candidate capability boundary and required evidence without claiming production readiness prematurely.

Scope:

* Release-candidate capability boundary definition.
* Required evidence checklist definition.
* Release-candidate validation framing.

Allowed:

* `checklists/release.md`
* `docs/roadmap/phase-map.md`
* `checklists/current-phase.md`
* `CHANGELOG.md`

Not allowed:

* production-ready claims
* authority model changes
* provider trust shortcuts
* roadmap completion status reporting

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Release-candidate boundary and evidence requirements are explicit without claiming production readiness.

Boundary note:

Release-candidate definition is planning and evidence framing, not final production assertion.

### Recurring roadmap/changelog alignment requirement

Every fifth phase after this reconciliation must include a roadmap/changelog alignment check. The check must compare `docs/roadmap/phase-map.md` against `CHANGELOG.md`, identify drift, and update only planned future sequencing. It must not move completed work out of `CHANGELOG.md` or record implementation status in the roadmap.


## Phase 37: Release-Candidate Evidence Collection Baseline

Milestone group: Milestone 5

Primary goal: Collect baseline release-candidate evidence without claiming release or production readiness.

## Phase 38: Static Boundary Lint Baseline (Maintenance Deviation)

Milestone group: Milestone 5

Primary goal: Apply a scoped maintenance deviation to establish AST-aware UI boundary lint baseline ahead of planned sequence continuation.

## Phase 39: UI Boundary Lint Diagnostic Hardening (Maintenance Deviation)

Milestone group: Milestone 5

Primary goal: Continue scoped maintenance deviation to harden AST lint diagnostics and deterministic self-test evidence.

## Phase 40: Roadmap/Changelog Reconciliation + AST Lint CI Alignment

Milestone group: Milestone 5

Primary goal: Reconcile planned future sequencing after Phase 38/39 maintenance deviation and verify CI runs the AST-aware UI boundary lint baseline.

## Phase 41: Functional Gap Audit and Roadmap Expansion

Milestone group: Milestone 5

Primary goal: Audit implemented-versus-planned functional gaps and expand planned forward phases toward a usable local harness without readiness claims.

Scope:

- Functional gap audit against historical and planned truth surfaces.
- Advisory operations report documenting implemented/partial/missing surfaces.
- Roadmap expansion from Phase 42 onward.

Allowed:

- `docs/roadmap/phase-map.md`
- `checklists/current-phase.md`
- `CHANGELOG.md`
- `docs/operations/functional-gap-audit-phase-41.md`

Not allowed:

- Runtime behavior implementation.
- UI behavior implementation.
- Provider execution implementation.
- Schema, workflow, governance, or architecture mutation.
- Release-candidate or production-readiness claims.

Validation gate:

Required commands:

```bash
./scripts/check.sh
cd ui && npm run typecheck && npm run lint && npm run build
```

Exit criteria:

Phase 41 is complete when the functional gap is documented and Phases 42+ are concretely expanded in planned truth format.

Boundary note:

This phase is documentation planning only and does not implement harness behavior.

## Phase 42: Local Runtime Configuration Boundary

Milestone group: Milestone 6

Primary goal: Add typed Rust configuration surfaces for local runs, provider mode selection, workspace metadata, and safety defaults.

Scope:

- Typed runtime config structs/enums and validation for local execution modes.

Allowed:

- Rust config/type modules and tests.

Not allowed:

- Provider calls, file watching, secrets persistence.

Validation gate:

- Rust checks and config-focused tests pass.

Exit criteria:

- Deterministic typed local runtime configuration boundary exists.

Boundary note:

Configuration remains Rust-owned authority input, not UI authority.

## Phase 43: Rust Read Projection Boundary

Milestone group: Milestone 6

Primary goal: Add Rust-owned read projections aggregating lifecycle/run/ledger/replay/audit/memory/context/provider/integration/output state.

Scope:

- Read projection structs, mapping logic, and projection tests.

Allowed:

- Rust projection modules and tests.

Not allowed:

- HTTP server, UI fetching, mutation entrypoints.

Validation gate:

- Rust checks and deterministic projection tests pass.

Exit criteria:

- Read projection boundary is typed and consumable by non-authoritative surfaces.

Boundary note:

Projection output is visibility data only.

## Phase 44: Local Application State Container

Milestone group: Milestone 6

Primary goal: Add in-memory app state container for run state, ledger, memory snapshot, replay report, audit projections, provider/integration state, and read projections.

Scope:

- In-memory state container and deterministic state transition tests.

Allowed:

- Rust state container modules and tests.

Not allowed:

- Persistence.

Validation gate:

- Rust checks and container state tests pass.

Exit criteria:

- Local in-memory application state boundary exists with deterministic behavior.

Boundary note:

Container centralizes authority state but remains non-durable.

## Phase 45: Roadmap and Changelog Alignment Check

Milestone group: Milestone 6

Primary goal: Verify planned/historical truth alignment after roadmap expansion and early runtime-boundary implementation.

Scope:

- Compare roadmap and changelog; reconcile only future planned sequence.

Allowed:

- Roadmap/checklist/changelog documentation updates.

Not allowed:

- Runtime/UI/provider behavior implementation.

Validation gate:

- Required docs scans and baseline validation commands pass.

Exit criteria:

- Alignment drift is resolved without collapsing truth dimensions.

Boundary note:

Roadmap remains planned truth; changelog remains historical truth.

## Phase 46: Local CLI Dry-Run Entry

Milestone group: Milestone 6

Primary goal: Add minimal Rust CLI/command entry for deterministic controlled flow from caller-supplied or fixture input with safe summary output.

Scope:

- CLI entrypoint and dry-run summary path.

Allowed:

- Rust CLI command parsing/routing and tests.

Not allowed:

- Real provider calls, persistence.

Validation gate:

- Rust checks plus CLI dry-run tests pass.

Exit criteria:

- Local dry-run entry executes deterministic controlled flow without external provider dependence.

Boundary note:

CLI remains request boundary; authority remains internal Rust paths.

## Phase 47: Local Persistence Boundary

Milestone group: Milestone 7

Primary goal: Add deterministic local persistence boundaries for ledger, memory snapshots, audit projections, and run records.

Scope:

- Persistence interfaces/adapters and deterministic write/read tests.

Allowed:

- Rust persistence boundary modules and tests.

Not allowed:

- Auto-repair, hidden writes, UI mutation.

Validation gate:

- Rust checks and persistence determinism tests pass.

Exit criteria:

- Durable local storage boundary exists with explicit authority-controlled writes.

Boundary note:

Persistence remains behind Rust-owned validation and policy boundaries.

## Phase 48: Provider Adapter Trait and Deterministic Stub

Milestone group: Milestone 7

Primary goal: Add provider adapter trait and deterministic stub provider returning untrusted output.

Scope:

- Provider trait, stub adapter, untrusted-output handling tests.

Allowed:

- Rust provider boundary modules and tests.

Not allowed:

- Real model invocation.

Validation gate:

- Rust checks and provider-stub boundary tests pass.

Exit criteria:

- Deterministic provider abstraction exists without trusted-output bypass.

Boundary note:

Provider output remains untrusted until Rust acceptance path completes.

## Phase 49: Real Local Provider Adapter Boundary

Milestone group: Milestone 7

Primary goal: Add first real local model adapter boundary behind untrusted provider interface.

Scope:

- Local model adapter boundary wiring and guarded tests.

Allowed:

- Rust provider adapter integration modules and tests.

Not allowed:

- Trusted output, direct ledger writes, UI authority.

Validation gate:

- Rust checks and local adapter boundary tests pass.

Exit criteria:

- First real provider adapter boundary exists with untrusted-output enforcement.

Boundary note:

Real adapter availability does not create authority outside Rust acceptance flow.

## Phase 50: Roadmap and Changelog Alignment Check

Milestone group: Milestone 7

Primary goal: Reconcile roadmap/changelog after persistence and provider-boundary expansion.

Scope:

- Planned-vs-historical alignment review and planned-sequence correction.

Allowed:

- Roadmap/checklist/changelog documentation updates.

Not allowed:

- Runtime/UI/provider behavior expansion outside alignment scope.

Validation gate:

- Required docs scans and baseline validation commands pass.

Exit criteria:

- Alignment remains clean with no truth-dimension drift.

Boundary note:

Alignment checks correct planning, not implementation status.

## Phase 51: Rust-Owned Operator Intent Submission

Milestone group: Milestone 8

Primary goal: Add Rust ingress for operator intents (approve/reject/retry/replay/context rebuild/memory requests).

Scope:

- Typed intent ingress interfaces, validation, and routing tests.

Allowed:

- Rust API/CLI ingress modules and tests.

Not allowed:

- UI direct mutation paths.

Validation gate:

- Rust checks and typed intent ingress tests pass.

Exit criteria:

- Operator intent requests can be submitted through Rust-owned typed boundaries.

Boundary note:

Intent submission is request-only; Rust remains decision authority.

## Phase 52: UI Live Read Projection Integration

Milestone group: Milestone 8

Primary goal: Replace fixture-backed UI data with read-only live projections from Rust-owned state or approved local read boundary.

Scope:

- UI read-projection integration and read-only rendering updates.

Allowed:

- UI data-access integration and rendering adjustments.

Not allowed:

- UI authority over runtime state.

Validation gate:

- UI typecheck/lint/build and boundary lint checks pass.

Exit criteria:

- UI displays live read projections without taking runtime authority.

Boundary note:

UI remains visibility surface only.

## Phase 53: UI Operator Intent Submission Boundary

Milestone group: Milestone 8

Primary goal: Convert request-preview controls into typed submission controls sending intent-shaped data to Rust ingress.

Scope:

- UI submit control wiring to Rust-owned intent boundary.

Allowed:

- UI intent form/control integration and boundary-safe tests.

Not allowed:

- Direct mutation or bypass of Rust intent validation.

Validation gate:

- UI checks and integration boundary checks pass.

Exit criteria:

- UI can submit typed intents through Rust ingress without authority bypass.

Boundary note:

Submission transport does not grant UI authority.

## Phase 54: End-to-End Local Harness Workflow

Milestone group: Milestone 8

Primary goal: Run full local controlled workflow across context assembly, provider intake, policy/validation, execution/promotion decision, ledger append, replay verification, audit projection, clean output, and UI inspection.

Scope:

- End-to-end local harness workflow integration and deterministic tests.

Allowed:

- Rust/UI integration and workflow test surfaces.

Not allowed:

- Production deployment claims.

Validation gate:

- Full local validation suite and workflow tests pass.

Exit criteria:

- Functional local harness workflow works end-to-end in local scope.

Boundary note:

Local harness completion is not production readiness.

## Phase 55: Roadmap and Changelog Alignment Check

Milestone group: Milestone 8

Primary goal: Verify planned/historical truth before release-candidate evidence expansion.

Scope:

- Compare planned sequence and completed history; reconcile forward plan only.

Allowed:

- Roadmap/checklist/changelog documentation updates.

Not allowed:

- Runtime/UI/provider behavior changes outside alignment scope.

Validation gate:

- Required docs scans and baseline validation commands pass.

Exit criteria:

- Truth-dimension alignment is preserved ahead of evidence expansion.

Boundary note:

Historical facts remain in changelog.

## Phase 56: Packaging and Startup Boundary

Milestone group: Milestone 9

Primary goal: Add reproducible local startup/install path for app execution.

Scope:

- Packaging/startup boundary docs/scripts/code needed for reproducible local harness boot.

Allowed:

- Minimal startup boundary implementation and validation steps.

Not allowed:

- Production-readiness claim.

Validation gate:

- Startup reproducibility checks and baseline validation commands pass.

Exit criteria:

- Local harness can be started reproducibly by operators.

Boundary note:

Packaging is local-operability boundary, not production deployment boundary.

## Phase 57: Release-Candidate Evidence Pass

Milestone group: Milestone 9

Primary goal: Populate release evidence using actual functional local workflow outputs.

Scope:

- Evidence capture/update against release checklist using real workflow runs.

Allowed:

- Checklist/operations evidence updates and minimal supporting validation outputs.

Not allowed:

- Production-readiness claim.

Validation gate:

- Required release-evidence commands and artifact checks pass.

Exit criteria:

- Release evidence reflects functional local workflow output traces.

Boundary note:

Evidence pass does not itself grant production readiness.

## Phase 58: Failure Injection and Recovery Hardening

Milestone group: Milestone 9

Primary goal: Add negative-path tests for corrupt ledger, provider failures, invalid config, UI submit errors, replay mismatch, persistence failures, and recovery boundaries.

Scope:

- Failure injection scenarios and recovery boundary tests.

Allowed:

- Rust/UI test expansions and boundary-safe hardening.

Not allowed:

- Hidden auto-repair bypassing explicit operator/Rust authority.

Validation gate:

- Expanded failure-path test suites pass deterministically.

Exit criteria:

- Core failure/recovery boundaries are tested and enforced.

Boundary note:

Recovery must remain explicit, auditable, and authority-safe.

## Phase 59: Production Readiness Gap Audit

Milestone group: Milestone 9

Primary goal: Identify remaining production blockers after a functional local harness exists.

Scope:

- Advisory production-gap audit and blocker classification.

Allowed:

- Operations/checklist/roadmap/changelog documentation updates.

Not allowed:

- Production-readiness claim.

Validation gate:

- Required audit scans and baseline validation commands pass.

Exit criteria:

- Production blockers are explicitly documented with follow-up planning direction.

Boundary note:

Gap audit identifies blockers; it does not assert blocker closure.

## Phase 60: Roadmap and Changelog Alignment Check

Milestone group: Milestone 9

Primary goal: Reconcile roadmap/changelog before any production-candidate work.

Scope:

- Planned/historical truth comparison and forward-plan reconciliation.

Allowed:

- Roadmap/checklist/changelog documentation updates.

Not allowed:

- Runtime/UI/provider behavior changes outside alignment scope.

Validation gate:

- Required docs scans and baseline validation commands pass.

Exit criteria:

- Roadmap/changelog truth alignment is clean before production-candidate planning.

Boundary note:

Alignment checkpoints preserve truth-dimension integrity and do not replace implementation evidence.
