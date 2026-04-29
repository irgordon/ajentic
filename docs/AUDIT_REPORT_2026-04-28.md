# AJENTIC repository audit report

Audit date: 2026-04-28  
Audited scope: actual repository state  
Reference scope: v0 architecture and Phase 0-6 implementation plan

## Executive summary
The repository is **not** a bootstrap-only skeleton. It contains substantial implemented behavior through candidate lifecycle, static CLI validation/inspect, adapter protocol parsing/validation, subprocess mock adapter invocation, candidate creation, and evaluation result ingestion in Rust. The current implemented surface aligns closely with the requested Phase 0-6 boundary, with no evidence of governance/promotion gate engines, ledger persistence, replay engine, or real provider integrations.

Main audit outcomes:
- Implemented behavior is materially beyond AGENTS.md “bootstrap only” wording.
- Phase 0-6 code exists with passing Rust tests/checks in this environment.
- No third-party runtime dependencies were found in Rust/Python/TypeScript manifests.
- No clear Phase 7+ runtime implementation was found.
- Documentation version/phase statements are inconsistent in places (README/SPEC/docs references to older phases).

## Validation command results
- `cargo fmt --all -- --check`: **PASS**
- `cargo test --workspace`: **PASS** (43 tests total: 38 core + 5 CLI)
- `cargo check --workspace`: **PASS**
- `./scripts/check.sh`: **PASS**
- `git diff -- ROADMAP.md`: **PASS** (no output; no local diff)
- `find schemas -name '*.schema.json' -type f -print | sort`: **PASS** (12 schema files listed)

## Repository structure findings
- Rust workspace present (`core`, `cli`) and builds.
- Python adapter scaffold includes deterministic mock adapter implementation.
- Schemas are concrete, non-empty, Draft 2020-12 shape files.
- Minimal run YAML examples exist and are non-empty.
- UI is scaffold-only (README/package placeholder/.gitkeep).
- Scripts are minimal glue checks/bootstrap/ci wrappers.

## Dependency audit
Dependency declarations inspected:
- `Cargo.toml`: workspace only.
- `core/Cargo.toml`: no dependencies.
- `cli/Cargo.toml`: path dependency only (`ajentic-core`).
- `adapters/python/pyproject.toml`: empty project dependencies.
- `ui/package.json`: no dependencies/devDependencies.

Result: no third-party dependencies declared in audited manifests.

## Rust authority audit
Implemented Rust behavior:
- Candidate lifecycle states + typed transitions/errors.
- Candidate creation from validated adapter response with deterministic ID (`candidate::{run_id}::{candidate_request_id}`).
- Adapter request/response parse + linkage/status/size validation.
- Subprocess invocation of Python mock adapter and failure handling.
- Evaluation ingestion and required evaluator checks (PASS-only satisfaction).
- Static run-directory validation (presence/non-empty/marker checks).
- CLI validate/inspect command surfaces.

Boundary findings:
- No governance approval engine found.
- No promotion decision computation engine found (only lifecycle state shape includes `PromotedTier1` transition path).
- No ledger append/persistence implementation found.
- No replay implementation found.
- No global mutable state patterns in production code.
- Some tests use system time/process ID for temp paths, but not runtime governance logic.

## Python adapter audit
- `mock_adapter.py` performs deterministic stdin->stdout key/value transformation.
- No provider SDK/model API calls.
- No network usage.
- No API key handling.
- No randomness/timestamps in adapter path.
- No file writes.
- No candidate promotion/lifecycle mutation/governance logic.

## TypeScript/UI audit
- UI remains visibility scaffold only.
- `package.json` has no dependencies.
- No implemented dashboard, decision logic, or hidden API behavior found.

## Bash/script audit
- `scripts/bootstrap.sh`, `scripts/check.sh`, `scripts/ci.sh` present.
- All use `#!/bin/sh` + `set -eu`.
- Scripts are glue-only: run checks and print messages.
- No policy/lifecycle/promotion decision logic.
- No dependency installation or generated artifacts.

## Schema audit
Schema files:
- adapter_request.schema.json
- adapter_response.schema.json
- audit_record.schema.json
- candidate_solution.schema.json
- constraints.schema.json
- domain.schema.json
- evaluation_result.schema.json
- governance_result.schema.json
- objective.schema.json
- policy.schema.json
- promotion_decision.schema.json
- reuse_event.schema.json

Per-file structure checks:
- `$schema`: present in all
- `title`: present in all
- `type: object`: present in all
- `required`: present in all
- `properties`: present in all
- `additionalProperties: false`: present in all

Additional schema findings:
- `policy.schema.json` exists.
- Schemas appear concrete (not empty placeholders).
- No remote `$ref` usage detected in audited set.
- No advanced conditional combinator logic detected as “clever bypass” patterns.
- Contract owner is not explicitly stated inside schema files; ownership is documented externally (Rust authority docs/governance).

## Example YAML audit
Files inspected:
- examples/minimal_run/objective.yaml
- examples/minimal_run/constraints.yaml
- examples/minimal_run/domain.yaml
- examples/minimal_run/policy.yaml

Findings:
- All are non-empty.
- Keys align with contract/schemas at shape level.
- Content reads as non-production examples.
- They are shape/config examples, not claims of runtime governance execution.

## Documentation accuracy audit
Observed mismatch/overclaim issues:
1) README claims current status is “v0.4.0 Phase 4 adapter protocol,” but repository includes Phase 5/6 behavior (candidate creation + evaluation ingestion).
2) SPEC and multiple docs still describe older phase states (Phase 1/3/4 notes) despite implemented later-phase code.
3) CONTRIBUTING/AGENTS/docs include bootstrap/v0.0.0 language that conflicts with implemented runtime modules.

These are documentation consistency issues more than code boundary violations.

## Test coverage audit
Current tests prove:
- Lifecycle transition legality/illegality and typed errors.
- Adapter protocol parse/validation failures and success cases.
- Subprocess adapter call success/failure paths.
- Candidate creation gating on validated completed adapter response.
- Evaluation ingestion linkage/duplication/required evaluator checks.
- CLI argument and validate/inspect command behavior.

Tests do **not** prove:
- Governance approvals/promotion eligibility (not implemented).
- Ledger append-only invariants (not implemented).
- Replay determinism from persisted ledger (not implemented).
- Real provider integration behavior (not implemented).

## Phase completion checklist
- [x] Phase 0: Repository bootstrap
- [x] Phase 1: Contracts and schemas
- [x] Phase 2: Candidate lifecycle state machine
- [x] Phase 3: CLI static validation surface
- [x] Phase 4: Adapter protocol and mock adapter
- [x] Phase 5: Candidate creation and runtime adapter checks
- [x] Phase 6: Evaluation result ingestion
- [ ] Phase 7: Governance and promotion gates
- [ ] Phase 8: Append-only ledger and audit records
- [ ] Phase 9: Replay
- [ ] Phase 10: Local model adapter capability
- [ ] Phase 11: Cloud model adapter capability
- [ ] Phase 12: TypeScript UI review surface
- [ ] Phase 13: Multi-domain capability
- [ ] Phase 14: Reuse event and bounded improvement records
- [ ] Phase 15: Hardening and negative testing
- [ ] Phase N: Early production capability

## Architecture invariant checklist
- [x] Rust remains authority
- [x] Python remains adaptation
- [x] TypeScript remains visibility
- [x] Bash remains glue
- [x] Generated output remains untrusted
- [x] Adapter output remains untrusted
- [x] UNKNOWN is not PASS
- [x] No unsafe bypass flags exist
- [x] No third-party dependencies were added without approval
- [x] ROADMAP.md remained Owner-controlled
- [x] No Phase 7+ behavior exists before approval

## Boundary violations
Severity: MAJOR  
File(s): README.md, SPEC.md, CONTRIBUTING.md, docs/FIRST_RUN.md, docs/LANGUAGE_ARCHITECTURE.md, docs/ADAPTER_PROTOCOL.md  
Finding: Documentation phase/status claims lag implemented code surface and may mislead phase-gate decisions.  
Evidence: README current status says v0.4.0 Phase 4 while code includes candidate creation/evaluation ingestion modules and tests.  
Recommended correction: Synchronize docs to actual implemented phase boundaries before Phase 7 acceptance.

## Overimplementation findings
Severity: NOTE  
File(s): core/src/candidate/lifecycle.rs  
Finding: Lifecycle includes `PromotedTier1` transition shape in Phase 2+ state machine; this is allowed as type shape but can be interpreted as promotion readiness if docs are unclear.  
Evidence: Transition allows `Passed -> PromotedTier1` while no governance gate implementation exists.  
Recommended correction: Keep but explicitly reiterate “shape only, not authorization” in docs adjacent to lifecycle references.

## Underimplementation findings
Severity: NOTE  
File(s): core/src/ledger/mod.rs, core/src/replay/mod.rs, ui/src/.gitkeep  
Finding: Phase 8+ capabilities not implemented (expected by current phase plan).  
Evidence: Placeholder modules/scaffold only.  
Recommended correction: None before Phase 7; this is expected.

## Documentation overclaim findings
Severity: MAJOR  
File(s): README.md, SPEC.md, CONTRIBUTING.md  
Finding: Mixed phase claims (bootstrap/Phase3/Phase4) conflict with actual code (Phase6 features).  
Evidence: README says current status Phase 4; SPEC says Phase 3 status section; CONTRIBUTING says bootstrap-only.  
Recommended correction: Align all top-level docs to one authoritative current phase statement and explicit non-implemented boundaries.

## Required corrections before Phase 7
1. Update top-level docs to consistently represent actual implemented state through Phase 6 and explicit Phase 7 not started boundary.
2. Add one authoritative “current phase/version” source of truth and reference it across README/SPEC/docs.
3. Ensure roadmap/version labels are encoding-clean and internally consistent with changelog/version tags.

## Recommended Owner decisions
1. Confirm whether implemented Phase 6 behavior is approved as official current state.
2. Approve a documentation alignment pass as a pre-Phase-7 gate.
3. Freeze Phase 7 code until documentation gate reflects true Phase 0-6 status and invariants.

## Final assessment
Code-level boundary compliance is largely aligned with intended Phase 0-6 implementation scope, with no detected Phase 7+ runtime engine behavior. The primary risk before Phase 7 is **documentation drift/inconsistency**, which can cause governance and release-gate confusion.
