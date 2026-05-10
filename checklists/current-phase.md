---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 129.1 Out-of-Band UI TypeScript Command Drift Fix

## Task name
- [x] Phase 129.1 - Out-of-Band Maintenance - UI TypeScript Command Drift Fix.

## Out-of-band maintenance status
- [x] Phase 129.1 is out-of-band validation-compatibility maintenance only.
- [x] Validation compatibility is not runtime capability.
- [x] UI command correction is not UI behavior change.
- [x] Bootstrap idempotence must not restore stale TypeScript flags.
- [x] Unsupported TypeScript flags must not exist in active UI command surfaces.
- [x] Phase 129.1 does not implement Phase 130.
- [x] Phase 129.1 does not approve Release Candidate readiness.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Classify uncommitted files as clean before edits.
- [x] Remove or revert generated artifacts from prior runs before edits if present.
- [x] Record cleanup: no generated artifact drift was present before edits.

## Allowed surfaces
- [x] `scripts/check.sh` because it injected unsupported TypeScript flags and needed correction.
- [x] `docs/operations/ui-typescript-command-drift-fix-phase-129-1.md`.
- [x] `checklists/current-phase.md`.
- [x] `CHANGELOG.md`.
- [x] `ui/package.json` updated to use supported temporary TypeScript project-file command shape after validation proved direct file-argument compilation conflicts with local TypeScript when `tsconfig.json` is present.
- [x] `scripts/bootstrap_repo.py` updated to seed the same supported UI package command surfaces and not restore stale TypeScript flags.
- [x] `.github/workflows/ci.yml` inspected; no edit required because CI delegates to `npm run test:api`.

## Boundary rules
- [x] Phase 129.1 does not implement Phase 130.
- [x] Phase 129.1 does not change UI behavior.
- [x] Phase 129.1 does not change runtime behavior.
- [x] Phase 129.1 does not change authority behavior.
- [x] Phase 129.1 does not change tests, schemas, release behavior, deployment behavior, monitoring behavior, or readiness posture.
- [x] Phase 129.1 does not add provider execution, persistence authority, replay repair, recovery promotion, or action execution.

## Unsupported TypeScript flag scan checklist
- [x] Scan for `--ignoreConfig`.
- [x] Scan for `--ignoreDeprecations 6.0`.
- [x] Scan for broader `ignoreDeprecations` occurrences.
- [x] Classify matches as active command surface, bootstrap template, validation script, documentation/report context, changelog context, checklist context, or historical/prohibition context.
- [x] Confirm known bad flags are absent from active UI command surfaces after correction.

## Active command-surface checklist
- [x] `ui/package.json` `test:api` does not use `--ignoreConfig`.
- [x] `ui/package.json` `test:api` does not use `--ignoreDeprecations 6.0`.
- [x] `ui/package.json` `dev` does not use `--ignoreConfig`.
- [x] `ui/package.json` `dev` does not use `--ignoreDeprecations 6.0`.
- [x] `scripts/check.sh` delegates to `npm run test:api` without injecting unsupported flags.
- [x] `.github/workflows/ci.yml` delegates to `npm run test:api` without injecting unsupported flags.

## Bootstrap template checklist
- [x] Inspect `scripts/bootstrap_repo.py` for stale UI command generation.
- [x] Confirm bootstrap does not contain `--ignoreConfig`.
- [x] Confirm bootstrap does not contain `--ignoreDeprecations 6.0`.
- [x] Confirm bootstrap does not restore stale TypeScript flags.

## Bootstrap idempotence checklist
- [x] Run bootstrap idempotence command.
- [x] Re-scan for unsupported TypeScript flags after bootstrap.
- [x] Confirm bootstrap idempotence did not restore stale flags in `ui/package.json` or `scripts/check.sh`.

## UI package command correction checklist
- [x] Confirm `test:api` uses a supported temporary TypeScript project-file invocation that compiles `src/api/runBehaviorTests.ts` without stale flags.
- [x] Confirm `dev` uses a supported temporary TypeScript project-file invocation that compiles `src/app/localReviewRuntime.ts` without stale flags.
- [x] Confirm no `name`, `private`, `version`, `type`, or `devDependencies` changes were required.

## CI relationship checklist
- [x] Inspect `.github/workflows/ci.yml`.
- [x] Confirm CI runs `npm run test:api`.
- [x] Confirm CI does not inject unsupported TypeScript flags.

## scripts/check.sh relationship checklist
- [x] Inspect `scripts/check.sh`.
- [x] Correct stale direct TypeScript invocation with unsupported flags.
- [x] Confirm `scripts/check.sh` delegates to `npm run test:api`.

## Runtime non-change checklist
- [x] No Rust source changes.
- [x] No runtime behavior changes.
- [x] No runtime capability changes.

## UI non-change checklist
- [x] No TypeScript source changes.
- [x] No UI behavior changes.
- [x] UI command correction is not UI behavior change.

## Test assertion non-change checklist
- [x] No UI behavior test source changes.
- [x] No test assertion changes.
- [x] No test files changed.

## Authority non-change checklist
- [x] No governance authority changes.
- [x] No runtime authority changes.
- [x] No provider authority changes.
- [x] No persistence authority changes.
- [x] No replay, recovery, or action-execution authority changes.

## Release/deployment non-change checklist
- [x] No package creation.
- [x] No release artifact creation.
- [x] No checksum generation.
- [x] No provenance attestation creation.
- [x] No installer/update-channel/signing/publishing changes.
- [x] No GitHub release, tag, public asset, public release, deployment automation, or production deployment behavior.

## Monitoring/logging/telemetry non-change checklist
- [x] No monitoring activation.
- [x] No logging activation.
- [x] No telemetry collection activation.
- [x] No collector, exporter, dashboard, alerting, production telemetry endpoint, telemetry token, ingestion URL, cron job, service file, scheduled collector, background service, or daemon behavior.

## Readiness prohibition checklist
- [x] No readiness approval.
- [x] No Release Candidate approval.
- [x] No Production Candidate approval.
- [x] No public/general-use approval.
- [x] No production-human-use approval.
- [x] Phase 129.1 does not approve Release Candidate readiness.

## Validation checklist
- [x] Required investigation scan completed.
- [x] Pre-bootstrap unsupported flag scan completed after correction.
- [x] Bootstrap idempotence completed.
- [x] Post-bootstrap unsupported flag scan completed.
- [x] UI validation completed.
- [x] Canonical validation completed.
- [x] Guarded diff completed.
- [x] Authority/readiness scan completed.

## Findings table
| Surface | Classification | Finding |
| --- | --- | --- |
| `ui/package.json` | active command surface | Updated to supported `test:api` and `dev` temporary project-file command shapes without stale flags. |
| `scripts/check.sh` | validation script / active command surface | Removed direct unsupported TypeScript flag invocation by delegating to `npm run test:api`. |
| `scripts/bootstrap_repo.py` | bootstrap template | Updated to seed supported UI package command surfaces without unsupported TypeScript flags. |
| `.github/workflows/ci.yml` | CI command surface | Delegates to `npm run test:api`; no unsupported flags. |
| Phase 129.1 report/checklist/changelog | documentation/report/checklist/changelog context | Contains known bad flags only as prohibition/historical context. |
| Archived and prior operations docs | historical/prohibition context | Existing references remain historical or operator context, not active command drift. |

## Changed file table
| File | Reason |
| --- | --- |
| `ui/package.json` | Use supported temporary TypeScript project-file command shapes without unsupported flags. |
| `scripts/bootstrap_repo.py` | Seed supported UI package command surfaces without restoring unsupported flags. |
| `scripts/check.sh` | Remove unsupported TypeScript flags from full-check UI validation by delegating to `npm run test:api`. |
| `docs/operations/ui-typescript-command-drift-fix-phase-129-1.md` | Add Phase 129.1 operations report. |
| `checklists/current-phase.md` | Update current procedural truth to Phase 129.1. |
| `CHANGELOG.md` | Add active Phase 129.1 changelog entry. |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | pass | Initial working tree clean. |
| `rg -n --hidden --glob '!core/target/**' --glob '!ui/node_modules/**' --glob '!.git/**' -- '--ignoreConfig|--ignoreDeprecations 6.0|ignoreDeprecations|test:api|runBehaviorTests|localReviewRuntime' .` | pass | Every hit classified. |
| `rg -n --hidden --glob '!core/target/**' --glob '!ui/node_modules/**' --glob '!.git/**' -- '--ignoreConfig|--ignoreDeprecations 6.0' .` | pass | After correction, active command surfaces clean; only prohibition context remains. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/bootstrap_repo.py > /dev/null` | pass | Bootstrap idempotence passed. |
| `cd ui && npm run test:api && npm run typecheck && npm run lint && npm run build` | pass | UI validation passed. |
| `git diff --check` | pass | No whitespace errors. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_structure.py` | pass | Structure validation passed. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_docs.py` | pass | Documentation validation passed. |
| `CARGO_TARGET_DIR=/tmp/ajentic-phase-129-1-target ./scripts/check.sh` | pass | Full deterministic check passed. |
| Guarded diff | pass | No guarded source, test, schema, governance, architecture, roadmap, lockfile, README, or AGENTS drift. |
| Authority/readiness scan | pass | No new approval or activation claim. |

## Zero-drift checklist
- [x] Unsupported TypeScript flags are absent from active UI command surfaces.
- [x] `scripts/bootstrap_repo.py` does not restore unsupported TypeScript flags.
- [x] Bootstrap idempotence passes.
- [x] `npm run test:api` passes.
- [x] Full `scripts/check.sh` passes.
- [x] No UI source changes are introduced.
- [x] No UI behavior test assertions are changed.
- [x] No runtime behavior is changed.
- [x] No authority behavior is changed.
- [x] No release/deployment/monitoring behavior is added.
- [x] No readiness claims are introduced.
- [x] CHANGELOG entry matches actual diff.
