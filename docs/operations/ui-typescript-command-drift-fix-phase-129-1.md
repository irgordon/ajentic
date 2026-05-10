---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# UI TypeScript Command Drift Fix - Phase 129.1

## Scope
Phase 129.1 is out-of-band validation-compatibility maintenance for stale UI TypeScript command drift in active validation command surfaces.

Allowed changed surfaces are `ui/package.json`, `scripts/bootstrap_repo.py`, `scripts/check.sh`, this operations report, `checklists/current-phase.md`, and `CHANGELOG.md`. `ui/package.json` required a safer supported TypeScript project-file invocation after validation proved direct file compilation conflicts with the local TypeScript version when `tsconfig.json` is present.

## Evidence rule
Count only committed repository evidence, validation command output, and direct scans of active command surfaces. Do not count prompt intent, speculative future work, or validation success as readiness approval.

## Phase 129.1 maintenance boundary
Phase 129.1 is validation-compatibility maintenance only. It does not implement Phase 130 and does not change UI behavior, runtime behavior, authority behavior, tests, schemas, release behavior, deployment behavior, monitoring behavior, or readiness posture.

## Validation compatibility is not runtime capability
Validation compatibility is not runtime capability.

UI command correction is not UI behavior change.

Bootstrap idempotence must not restore stale TypeScript flags.

Unsupported TypeScript flags must not exist in active UI command surfaces.

Phase 129.1 does not implement Phase 130.

Phase 129.1 does not approve Release Candidate readiness.

## Recurring failure summary
The recurring CI failure was `error TS5023: Unknown compiler option '--ignoreConfig'` during UI validation. Investigation confirmed `scripts/check.sh` injected unsupported TypeScript compiler flags directly instead of delegating to `npm run test:api`.

## Known bad flags
- `--ignoreConfig`
- `--ignoreDeprecations 6.0`

## Active command-surface scan
| Hit | Classification | Finding |
| --- | --- | --- |
| `ui/package.json` `test:api` | active command surface | Uses a supported temporary TypeScript project-file invocation and does not contain known bad flags. |
| `ui/package.json` `dev` | active command surface | Uses a supported temporary TypeScript project-file invocation and does not contain known bad flags. |
| `scripts/check.sh` UI validation block | validation script / active command surface | Previously injected known bad flags; corrected to delegate to `npm run test:api`. |
| `.github/workflows/ci.yml` UI API behavior step | CI command surface | Delegates to `npm run test:api` and does not inject known bad flags. |

## Bootstrap template scan
`scripts/bootstrap_repo.py` was scanned for `--ignoreConfig`, `--ignoreDeprecations`, `ignoreDeprecations`, `test:api`, `runBehaviorTests`, and `localReviewRuntime`. The bootstrap template did not contain the known bad flags and was updated to seed the same supported UI command surface as `ui/package.json`.

## Bootstrap idempotence check
`PYTHONDONTWRITEBYTECODE=1 python3 scripts/bootstrap_repo.py > /dev/null` completed successfully. The post-bootstrap scan found no unsupported TypeScript flags in active UI command surfaces.

## Corrected UI package command surface
`ui/package.json` now uses supported temporary TypeScript project-file invocations for `test:api` and `dev`, avoiding both unsupported stale flags and the TypeScript 6 file-argument/tsconfig conflict.

## Unsupported flag absence proof
After correction and bootstrap idempotence, repository scans for `--ignoreConfig|--ignoreDeprecations 6.0` only match Phase 129.1 report, checklist, and changelog prohibition/historical context. No active UI command surface contains those flags.

## CI relationship
`.github/workflows/ci.yml` runs UI API behavior tests through `npm run test:api`. It does not inject `--ignoreConfig` or `--ignoreDeprecations 6.0`.

## scripts/check.sh relationship
`scripts/check.sh` now delegates UI API behavior validation to `npm run test:api` after `npm run typecheck`, `npm run lint`, and `npm run build`. It no longer injects unsupported TypeScript compiler flags.

## Phase 129 relationship
Phase 129.1 follows Phase 129 as out-of-band maintenance. It does not alter Phase 129 dry-run conclusions, release dry-run posture, or evidence-map posture.

## Phase 130 non-implementation statement
Phase 129.1 does not implement Phase 130.

## Runtime non-change statement
No Rust source was changed. No runtime behavior was changed. No runtime capability was added.

## UI non-change statement
No TypeScript source was changed. UI command correction is not UI behavior change.

## Test assertion non-change statement
No UI behavior test source and no test assertion source were changed.

## Authority non-change statement
No governance authority, runtime authority, provider authority, persistence authority, replay authority, recovery authority, or action-execution authority was changed.

## Release/deployment non-change statement
No package, release artifact, checksum, provenance attestation, installer, update channel, signing, publishing, GitHub release, tag, public download, public release behavior, deployment automation, or production deployment behavior was created or activated.

## Monitoring/logging/telemetry non-change statement
No monitoring, logging, telemetry collection, collector, exporter, dashboard, alerting, production telemetry endpoint, telemetry token, ingestion URL, cron job, service file, scheduled collector, background service, or daemon behavior was activated.

## Readiness non-approval statement
Phase 129.1 does not approve Release Candidate readiness. It does not approve Production Candidate status, public/general use, production readiness, production deployment, or production human use.

## Validation log
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | pass | Initial working tree was clean. |
| `rg -n --hidden --glob '!core/target/**' --glob '!ui/node_modules/**' --glob '!.git/**' -- '--ignoreConfig|--ignoreDeprecations 6.0|ignoreDeprecations|test:api|runBehaviorTests|localReviewRuntime' .` | pass | Classified hits; only `scripts/check.sh` contained active unsupported flags before correction. |
| `rg -n --hidden --glob '!core/target/**' --glob '!ui/node_modules/**' --glob '!.git/**' -- '--ignoreConfig|--ignoreDeprecations 6.0' .` | pass | After correction, matches are limited to Phase 129.1 report/checklist/changelog prohibition context. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/bootstrap_repo.py > /dev/null` | pass | Bootstrap idempotence did not restore stale flags. |
| `cd ui && npm run test:api && npm run typecheck && npm run lint && npm run build` | pass | UI validation passed without unsupported flags. |
| `git diff --check` | pass | No whitespace errors. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_structure.py` | pass | Structure validation passed. |
| `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_docs.py` | pass | Documentation validation passed. |
| `CARGO_TARGET_DIR=/tmp/ajentic-phase-129-1-target ./scripts/check.sh` | pass | Full deterministic check passed. |
| Guarded diff scan | pass | No Rust, TypeScript source, tests, schemas, governance, architecture, roadmap, lockfile, README, or AGENTS drift. |
| Authority/readiness scan | pass | Matches are prohibition, historical, test, or planned-boundary context; no new approval claim. |

## Required follow-ups
No required follow-up remains for Phase 129.1.

## Deferred items
No Phase 130 implementation, release/deployment work, monitoring/logging/telemetry activation, provider execution, persistence expansion, replay repair, recovery promotion, or action execution was performed.

## Confirmed vs suspected
| Item | Status | Evidence |
| --- | --- | --- |
| `scripts/check.sh` stale flags | confirmed and corrected | Direct scan found known bad flags before correction. |
| `ui/package.json` stale flags | confirmed absent after correction | Direct scan found no known bad flags in package scripts; direct file-argument invocation was replaced by a temporary project-file invocation. |
| `scripts/bootstrap_repo.py` stale flags | confirmed absent after template alignment | Direct scan found no known bad flags in bootstrap template; template now seeds supported UI commands. |
| `.github/workflows/ci.yml` stale flags | confirmed absent | CI delegates to `npm run test:api`. |

## Non-readiness statement
This maintenance does not approve readiness, Release Candidate status, Production Candidate status, public/general use, production readiness, production deployment, or production human use.
