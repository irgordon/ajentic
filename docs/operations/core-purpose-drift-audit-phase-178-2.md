---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 178.2 - OOB Core Purpose and Drift Audit

## Scope
Top-down audit of AJENTIC governance, roadmap/checklist/changelog state, Rust API and shell transport surfaces, provider configuration/invocation/validation flows, staged proposal and candidate materialization, replay/restore/session package handling, release-path dry-run surfaces, UI render/help copy, and validation gates before Phase 179.

## Evidence rule
This audit uses repository evidence only (committed files plus local command output from required scans/checks). No conclusions are based on prompt intent alone.

## Core purpose statement
AJENTIC remains scoped as a deterministic, bounded, auditable control interface between planner intent and local operator-visible review surfaces; it does not grant autonomous authority, provider trust by default, release/public/production approval, or live release execution rights.

## Audit method
- Executed required scan set (`rg`, file-size scans, changelog-size scan, check script, diff hygiene).
- Cross-checked governance/architecture/roadmap/changelog/checklist truth-dimension boundaries.
- Inspected high-risk runtime and UI files for determinism and authority drift markers.

## Determinism findings
- Deterministic IDs/digests and stable ordering are explicit across restore, installer/distribution contract, dry package, and evidence assembly surfaces (stable digest helpers, BTree map/set usage, explicit sorting). 
- Deterministic contract claims are also explicit in UI transport and local shell projection wiring.
- Risk note: monolithic files (`core/src/api/local_operator_shell.rs`, `ui/src/api/localOperatorShell.ts`) increase future drift probability, but this is maintainability risk, not current deterministic failure.

## Bounded-control findings
- Core surfaces remain typed request/response projections through Rust-owned API boundaries and local operator shell transport.
- Provider execution remains constrained to deterministic/local allowlisted pathways with explicit no-network/no-shell/no-secret markers.
- Dry-run and rehearsal release-path modules are separated and labeled as local-only/non-public/non-authoritative.

## Non-authority boundary findings
- Authority-denial language remains broad and repeated in Rust, UI, tests, and operations docs (no readiness/release/public-use/action/replay-repair/recovery-promotion approvals).
- Scan hits on approval terms are predominantly denial assertions and adversarial test payloads, not positive authority grants.

## Provider/execution guardrail findings
- Arbitrary provider execution remains blocked by allowlist/deterministic adapter constraints and validation rejection paths.
- No evidence found of enabling real shell/network/cloud/provider secret handling in authority surfaces.
- Provider output remains explicitly untrusted/descriptive unless later bounded phases change status.

## Release-path guardrail findings
- Release-candidate preparation, dry package, checksum/provenance, installer/distribution contract, signing key-custody dry run, evidence assembly, and gap review remain represented as non-authoritative evidence/projection surfaces.
- No evidence that these surfaces create release/public artifacts, signing outputs, publication, deployment, update channels, tags, or GitHub releases.

## UI honesty findings
- UI copy repeatedly states local-only, deterministic stub behavior, untrusted provider output, and non-approval boundaries.
- No UI control text found that grants authority absent Rust boundary support.

## Documentation truth-dimension findings
- Governance/architecture/roadmap/changelog/checklist partitioning remains intact by frontmatter and content intent.
- `docs/operations/*` continues to use orientation/advisory/readme_update conventions.

## Architecture/modularity findings
- Drift risk remains highest in large files and repeated boundary text patterns.
- File-size scan confirms multiple >1,000 LOC modules, including `core/src/api/local_operator_shell.rs` and `ui/src/api/localOperatorShell.ts`; this is a hardening/refactor signal, not immediate rebuild trigger.

## Test/validation integrity findings
- Tests continue to contain adversarial/fail-closed checks for trust/approval drift terms.
- `scripts/check.sh` was executed and failed on the initial cleanliness gate because the phase intentionally modifies allowed documentation files before re-running from a clean commit; `git diff --check` was clean before commit and final tree hygiene was verified.

## Drift findings
- **Finding A (modularity concentration):** oversized shell/UI orchestration files raise guardrail consistency drift risk over time. Severity: **medium**.
- **Finding B (boundary marker duplication):** repeated textual claim checks across modules may cause future inconsistency if one surface diverges. Severity: **low**.
- **Finding C (core-purpose authority drift):** no present evidence of active authority crossing deterministic/bounded/non-authoritative core purpose. Severity: **none**.

## Deviation severity table
| Finding | Severity | Surface | Disposition |
| --- | --- | --- | --- |
| Modularity concentration in large orchestrator files | medium | `core/src/api/local_operator_shell.rs`, `ui/src/api/localOperatorShell.ts` | refactor_required |
| Repeated boundary marker/check duplication | low | multi-module Rust/UI boundary text and checks | acceptable_drift |
| Core-purpose authority crossing | none | top-down runtime/UI/docs scan | no_drift |

## Required repair candidates
1. Continue Phase 179+ decomposition of monolithic shell orchestration into dedicated modules with unchanged runtime semantics.
2. Consolidate repeated boundary marker generation/validation helpers where repeated concrete usage is already stable.
3. Keep deterministic ordering/digest helpers centralized per surface to reduce divergence risk.

## Rebuild trigger assessment
No rebuild trigger found.

No evidence found of:
- unbounded execution,
- default provider trust,
- action authorization without typed operator decision,
- release/public/production approval authority,
- signing/publishing/deployment/public distribution behavior,
- replay/history mutation to repair outcomes,
- hidden validation failures,
- authoritative nondeterministic behavior.

## Phase 179 readiness assessment
Disposition: **repair_required_before_phase_179** is **not** required for safety guardrails; identified drift is maintainability/hardening level.

## Recommendation
**phase_179_can_proceed_with_caveats**.

Caveats:
- Treat modular decomposition and boundary-duplication reduction as explicit hardening goals.
- Preserve fail-closed authority denials and deterministic ordering guarantees while refactoring.

## Validation output
- Required scans executed (status, authority terms, execution/network/secret terms, release-path terms, truth-dimension metadata, file-size scan, changelog-size scan).
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-178-2-target ./scripts/check.sh`: failed at initial clean-tree gate after docs edits were present (expected gate behavior).
- `git diff --check`: passed.
- `git status --short`: verified.

## Zero-drift statement
Audit completed as out-of-band evidence review only.
No Rust source, TypeScript source, test, schema, roadmap, governance, architecture, package, lockfile, CI, or deployment infrastructure drift was introduced by Phase 178.2.
