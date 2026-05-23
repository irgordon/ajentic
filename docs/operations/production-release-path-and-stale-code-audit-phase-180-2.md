---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 180.2 - OOB Production Release Path and Stale Code Audit

## Scope

Phase 180.2 performs a top-down audit across roadmap, checklist, changelog, runtime/UI/test sources, scripts, and operations docs to evaluate whether the Phase 181-185 block remains the correct production-release path after the Phase 180 decision and Phase 180.1 validation closure.

## Evidence rule

This audit uses current repository evidence only (committed files plus command output from this working tree). It does not treat chat intent as evidence unless the same claim exists in repository files.

## Current release-path summary

- Phase 180 decision remains `release_candidate_status_supportable_with_caveats` and does not claim production/public approval (`CHANGELOG.md`, `docs/roadmap/phase-180-release-candidate-decision.md`).
- Roadmap still maps the next stewardship block as Phases 181-185 in order (`docs/roadmap/phase-map.md`, `docs/roadmap/phases.md`, `docs/roadmap/sequencing.md`).
- Current checklist (pre-180.2 update) confirms Phase 181 remains next code-production phase (`checklists/current-phase.md`).

## Audit method

Commands executed:

- `git status --short`
- `rg -n "Phase 181|Phase 182|Phase 183|Phase 184|Phase 185|Release Candidate Label and Evidence Manifest|Release Candidate Review UI|Release Candidate Hardening Closure|Release Candidate Local Package Rehearsal|Release Candidate Alignment Checkpoint" docs/roadmap CHANGELOG.md checklists/current-phase.md docs/operations`
- `rg -n "production-ready|ready for production|release-candidate ready|Release Candidate status: approved|Production Candidate status: approved|public use approved|public/general use approved|production human use approved|deployment enabled|publishing enabled|signing enabled|public release enabled|release artifact created|public download created|github release created|release tag created" docs core/src ui/src tests CHANGELOG.md checklists/current-phase.md`
- `rg -n "trusted|approved|ready|release_ready|production_ready|candidate_approved|public_use|deployment_enabled|signing_enabled|publishing_enabled|action_authorized|replay_repaired|recovery_promoted" core/src ui/src tests docs CHANGELOG.md checklists/current-phase.md`
- `rg -n "Command::new|std::process|TcpListener|UdpSocket|WebSocket|fetch\(|XMLHttpRequest|reqwest|hyper|axum|warp|rocket|std::env|env::var|private_key|certificate|kms|secret|token|API_KEY|CREDENTIAL" core/src ui/src tests docs CHANGELOG.md checklists/current-phase.md`
- `rg -n "phase [0-9]+|Phase [0-9]+|phase_[0-9]+|v0\.0\.[0-9]+|TODO|FIXME|stub|placeholder|legacy|deprecated|temporary" core/src ui/src tests docs CHANGELOG.md checklists/current-phase.md`
- `rg -n "contains\(|starts_with\(|ends_with\(|to_ascii_lowercase\(|to_lowercase\(|split\(|status.*String|reason.*String" core/src ui/src tests`
- `rg -n "allow\(dead_code\)|allow\(unused\)|unused|pub fn|pub struct|pub enum" core/src tests`
- `rg -n "#\[test\]|describe\(|it\(|test\(" core/src tests ui/src`
- Python file-size scan (>1000 LOC) across `core/src`, `ui/src`, `tests`, `core/tests`
- Python changelog size scan across `CHANGELOG.md` and `docs/changelog/*.md`

## Phase 181-185 path assessment

Assessment: **mostly intact with targeted cleanup needed**.

- The roadmap consistently describes Phase 181-185 as bounded stewardship/review/hardening/rehearsal/alignment work, with explicit non-release and non-approval boundaries.
- No missing mandatory pre-181 phase was found in roadmap/planning files.
- However, implementation complexity and large string-heuristic surfaces in release-adjacent modules indicate cleanup/hardening should be explicitly carried into early Phase 181 execution.

Severity: `medium`  
Disposition: `harden_during_phase_181`

## Stale code findings

1. **Phase-labeled historical helpers remain heavily embedded** in runtime and tests (many `phase_*` names across `tests/adversarial_corpus.rs` and `tests/integration_smoke.rs`). This is not automatically wrong, but it increases cognitive load and stale-rule risk.
   - Severity: `low`
   - Disposition: `acceptable_debt`

2. **Placeholder/stub markers remain live in release-adjacent surfaces** (e.g., dry-run placeholder key metadata and deterministic stub pathways). These align with current boundaries but should be periodically revalidated against roadmap intent before deeper release-path phases.
   - Severity: `low`
   - Disposition: `harden_during_phase_181`

## Brittle logic findings

1. **High volume of string matching in authority-sensitive boundaries** (`contains`, `starts_with`, lowercase normalization, and free-text status/reason fields in `core/src/api/local_operator_shell.rs`, `core/src/api/release_artifact_dry_package.rs`, `core/src/execution/mod.rs`, and mirrored UI/test surfaces).
   - Risk: token-based false positives/negatives over time.
   - Severity: `medium`
   - Disposition: `refactor_required`

2. **Large mixed-responsibility modules** increase brittleness and change risk (notably `local_operator_shell.rs` and `ui/src/api/localOperatorShell.ts`).
   - Severity: `high`
   - Disposition: `harden_before_phase_185`

## Unused or weak test findings

1. **Extensive assertion surface is present**, including adversarial coverage for approval/readiness drift and forbidden labels.
2. **Risk remains for mirror-style string assertions** where tests may validate phrase presence/absence more than typed behavior in some panels and summaries.
3. **No obvious total absence of release-path tests** found; rather, the issue is maintainability/fragility of very large behavior test files.

Severity: `medium`  
Disposition: `test_repair_required`

## Module-size findings

Python scan found multiple files >1000 LOC, including:

- `core/src/api/local_operator_shell.rs` (5962)
- `ui/src/api/localOperatorShell.ts` (10370)
- `core/src/api/local_operator_shell_tests.rs` (5912)
- `ui/src/api/submissionBoundary.behavior.test.ts` (7521)
- several additional 1000+ LOC files in release/deployment/observability/persistence areas.

This confirms module-growth risk remains active.

Severity: `high`  
Disposition: `refactor_required`

## Release-boundary findings

- Release/prod/public approval tokens appear primarily in adversarial fixtures, rejection logic, and explicit prohibition statements.
- No evidence found that current implementation activates signing/publishing/deployment/public release behavior.
- No evidence found that provider output is trusted by default or that operator-typed authorization is bypassed.

Severity: `none`  
Disposition: `no_action`

## Documentation truth-dimension findings

- Roadmap files remain planning/future truth.
- CHANGELOG remains historical truth.
- Checklist remains procedural truth (updated in this phase to 180.2 procedure).
- Operations docs remain advisory/orientation with frontmatter consistency.

Severity: `none`  
Disposition: `no_action`

## Production-release blocker table

| Finding | Severity | Disposition | Blocks Phase 181? |
| --- | --- | --- | --- |
| Release/public/prod approval authority drift detected in runtime behavior | none | no_action | No |
| Signing/publishing/deployment/public distribution activation detected | none | no_action | No |
| Oversized and brittle release-path modules | high | harden_before_phase_185 | Not immediate hard block |
| String-heuristic dependence in authority-sensitive checks | medium | harden_during_phase_181 | Not immediate hard block |

## Stale-code/refactor candidate table

| Surface | Candidate type | Severity | Disposition |
| --- | --- | --- | --- |
| `core/src/api/local_operator_shell.rs` | module split and typed boundary extraction | high | refactor_required |
| `ui/src/api/localOperatorShell.ts` | module split and duplicated rule consolidation | high | refactor_required |
| `core/src/api/release_artifact_dry_package.rs` | reduce text-heuristic branching | medium | harden_during_phase_181 |
| `core/src/api/release_dry_package_checksum_provenance.rs` | reduce text-heuristic branching | medium | harden_during_phase_181 |

## Test-debt table

| Surface | Debt | Severity | Disposition |
| --- | --- | --- | --- |
| `ui/src/api/submissionBoundary.behavior.test.ts` | very large behavior-file fragility | medium | test_repair_required |
| `core/src/api/local_operator_shell_tests.rs` | very large single-file regression surface | medium | test_repair_required |
| String-token assertion clusters across release-path tests | mirror-style assertion risk | medium | harden_during_phase_181 |

## Required repair candidates

1. Prioritize typed status/reason representations where feasible in high-risk release-path checks.
2. Split `local_operator_shell` Rust and TypeScript modules into narrower responsibility slices before Phase 185, with minimal behavior change.
3. Add/expand focused tests that assert typed outcomes over string-token presence for critical release-boundary decisions.
4. Keep adversarial forbidden-label coverage, but reduce duplication where equivalent checks can be centralized.

## Phase 181 readiness assessment

Phase 181 should not be fully blocked, but it should explicitly include targeted cleanup/hardening tasks for brittle and oversized release-path surfaces.

Decision: `phase_181_should_include_targeted_cleanup`

## Recommendation

Proceed with Phase 181 while explicitly scheduling targeted cleanup of brittle string-heuristic and oversized-module debt as part of the Phase 181-183 hardening arc. Do not remap the entire release path at this point.

## Validation output

- `git status --short` (pre-edit): clean.
- Required scans completed (phase path, release boundary, authority drift, execution/secret, stale markers, string heuristics, dead/unused signal, test inventory, file size, changelog size).
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-180-2-target ./scripts/check.sh`: pass.
- `git diff --check`: pass.
- `git status --short` (post-edit): only allowed files modified.

## Zero-drift statement

Phase 180.2 introduced documentation-only updates in allowed surfaces (`docs/operations/production-release-path-and-stale-code-audit-phase-180-2.md`, `CHANGELOG.md`, `checklists/current-phase.md`). No Rust/UI/test/schema/roadmap/runtime/deployment/signing/publishing changes were introduced.

## Rebuild trigger assessment

No rebuild trigger found.
