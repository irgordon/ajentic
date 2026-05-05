---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist

## phase name
Phase 82.5 - Out-of-Band Root Integration Test Harness Baseline

## explicit out-of-band maintenance/testing note
Phase 82.5 is an out-of-band maintenance/testing fix before Phase 83.

## phase goal
Establish the first root integration-test baseline for existing cross-boundary local harness/replay behavior before Phase 83 durable append work.

## working-tree hygiene gate
- [x] `git status --short` reviewed before edits.
- [x] Working changes constrained to approved Phase 82.5 surfaces.

## allowed surfaces
- [x] `tests/`
- [x] `core/Cargo.toml` (minimal integration test target wiring only)
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/integration-test-baseline-phase-82-5.md`

## boundary rules
- [x] Testing scaffold/baseline only; no new runtime authority.
- [x] No durable append implementation.
- [x] No provider network execution, persistence writes, recovery acceptance, live UI transport, or action side effects.
- [x] No roadmap/governance/architecture scope mutation.

## task checklist
- [x] Updated checklist to Phase 82.5 procedural truth.
- [x] Added operations advisory doc for Phase 82.5.
- [x] Added root integration baseline test file under `tests/`.
- [x] Added `CHANGELOG.md` entry `v0.0.82.5`.

## validation checklist
- [x] `./scripts/check.sh`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `find tests -maxdepth 2 -type f -print`
- [x] Integration content scan command completed.
- [x] No-append/no-authority scan command completed.
- [x] Guarded-source diff command completed.
- [x] Readiness wording scan completed.
- [x] Out-of-band wording scan completed.
- [x] Lint wiring scan completed.

## root integration-test checklist
- [x] Root test asserts bounded completed local harness behavior.
- [x] Root test asserts non-authority/non-effect flags.
- [x] Root test asserts replay report mode is `Replay` and non-live execution.
- [x] Root test avoids durable append/persistence/network/live transport assertions.

## library/export surface checklist
- [x] Existing `ajentic_core` API exports were sufficient for integration assertions.
- [x] No `core/src/lib.rs` semantic/runtime behavior change required.
- [x] `core/Cargo.toml` update is minimal test target wiring only.

## Phase 83 deferral checklist
- [x] Phase 82.5 is an out-of-band maintenance/testing fix before Phase 83.
- [x] Phase 82.5 does not implement durable append.
- [x] Phase 83 remains responsible for durable audit/ledger append.

## zero-drift checklist
- [x] Roadmap docs unchanged.
- [x] Guarded runtime/UI/scripts/workflow surfaces unchanged.
- [x] No dependency additions.

## findings table
| Finding | Result |
| --- | --- |
| Root integration baseline presence | Added first compiled/runnable root integration test target and file. |
| Cross-boundary non-authority invariants | Verified harness/replay remain bounded and non-authoritative. |

## deferred items table
| Item | Phase |
| --- | --- |
| Durable audit/ledger append boundary | Phase 83 |
| Recovery candidate acceptance boundary | Phase 84 |

## validation log table
| Command | Result |
| --- | --- |
| `./scripts/check.sh` | pass |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | pass |
| Required scan commands | pass |
