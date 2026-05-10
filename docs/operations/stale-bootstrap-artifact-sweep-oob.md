---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Out-of-Band Stale Bootstrap Artifact Sweep

## Scope

This out-of-band maintenance pass inspected tracked repository files for stale, misleading, unused, or early-phase bootstrap artifacts that are no longer part of AJENTIC's active command, validation, governance, runtime, UI, release, deployment, or operator surfaces.

## Evidence rule

Deletion required tracked-file status, reference-scan evidence, no active CI/check/validator/package/README/AGENTS/current-roadmap/current-checklist/current-operations dependency, stale or misleading content, no replacement runtime behavior, and historical coverage in CHANGELOG.

## Maintenance boundary

This sweep is cleanup only. It does not implement Phase 127, Phase 128, runtime behavior, UI behavior, authority behavior, provider execution, persistence authority, replay repair, recovery promotion, action execution, release behavior, deployment behavior, or readiness approval.

## Historical truth belongs in CHANGELOG

Stale executable or placeholder surfaces were not retained as archaeological artifacts. Historical bootstrap facts remain in CHANGELOG and archived changelog files.

## Search commands

- `git status --short`
- `git ls-files`
- `rg -n "Phase 0|bootstrap|placeholder|todo|stub|temporary|early phase|ui-start|ui-build|start UI|dev server|npm run build|npm run dev|not implemented|future placeholder" .`
- `rg -n "scripts/dev-run.sh|dev-run.sh|scripts/ui-start.sh|ui-start.sh|scripts/replay.sh|replay.sh|scripts/memory-snapshot.sh|memory-snapshot.sh|scripts/memory-clear-ephemeral.sh|memory-clear-ephemeral.sh|docs/examples/prompts/placeholder.md|docs/examples/workflows/placeholder.md|operator-handbook" .`

## Candidate artifact inventory

| Candidate | Evidence | Initial concern |
| --- | --- | --- |
| `scripts/dev-run.sh` | Referenced only by `docs/operations/operator-handbook.md`, `scripts/bootstrap_repo.py`, and itself. | Placeholder convenience wrapper; output says placeholder while command runs the Rust CLI. |
| `scripts/ui-start.sh` | Referenced only by `docs/operations/operator-handbook.md`, `scripts/bootstrap_repo.py`, and itself. | Misleading browser UI development-server wrapper; actually runs `npm run build`. |
| `scripts/replay.sh` | Referenced only by `docs/operations/operator-handbook.md`, `scripts/bootstrap_repo.py`, and itself for exact script name. | Placeholder replay wrapper without current supported replay command surface. |
| `scripts/memory-snapshot.sh` | Referenced only by `docs/operations/operator-handbook.md`, `scripts/bootstrap_repo.py`, and itself. | Placeholder memory wrapper without current supported operator command surface. |
| `scripts/memory-clear-ephemeral.sh` | Referenced only by `docs/operations/operator-handbook.md`, `scripts/bootstrap_repo.py`, and itself. | Placeholder memory mutation wrapper without current supported operator command surface. |
| `docs/examples/prompts/placeholder.md` | Referenced only by `scripts/bootstrap_repo.py` and itself. | Phase 0 placeholder example file with no examples. |
| `docs/examples/workflows/placeholder.md` | Referenced only by `scripts/bootstrap_repo.py` and itself. | Phase 0 placeholder example file with no examples. |
| `docs/operations/operator-handbook.md` | Referenced only by `scripts/bootstrap_repo.py` and itself. | Phase 0 placeholder handbook listing stale wrappers as operator scripts. |
| `scripts/bootstrap_repo.py` | Used by `scripts/check.sh`. | Kept, but stale regeneration entries removed so validation does not recreate deleted artifacts. |
| `ui/package.json` placeholder `lint`/`build` scripts | Used by CI and `scripts/check.sh`. | Kept because current validation surfaces depend on them. |
| Archived changelog Phase 0/bootstrap entries | Historical archive references. | Kept as governed historical truth. |

## Reference scan results

Exact-name scans found the deleted wrapper scripts and placeholder example files were not referenced by active CI, `scripts/check.sh`, validators, package scripts, README, AGENTS, current roadmap, or current checklist. The only active-looking reference was the stale Phase 0 operator handbook, which was itself unreferenced and placeholder-only.

## Keep/delete/defer classification table

| Artifact or reference | Classification | Reason |
| --- | --- | --- |
| `scripts/dev-run.sh` | delete | Unreferenced outside stale handbook/bootstrap regeneration; placeholder convenience wrapper. |
| `scripts/ui-start.sh` | delete | Unreferenced outside stale handbook/bootstrap regeneration; name/comment imply dev server but behavior runs build. |
| `scripts/replay.sh` | delete | Unreferenced outside stale handbook/bootstrap regeneration; no current supported replay operator wrapper. |
| `scripts/memory-snapshot.sh` | delete | Unreferenced outside stale handbook/bootstrap regeneration; no current supported memory snapshot operator wrapper. |
| `scripts/memory-clear-ephemeral.sh` | delete | Unreferenced outside stale handbook/bootstrap regeneration; no current supported memory-clear operator wrapper. |
| `docs/examples/prompts/placeholder.md` | delete | Phase 0 placeholder-only example document; no active examples. |
| `docs/examples/workflows/placeholder.md` | delete | Phase 0 placeholder-only example document; no active examples. |
| `docs/operations/operator-handbook.md` | delete | Placeholder-only handbook that advertised deleted stale wrappers. |
| `scripts/bootstrap_repo.py` | keep with reference cleanup | Used by `scripts/check.sh`; stale regeneration entries removed. |
| `ui/package.json` placeholder commands | keep | Used by CI and `scripts/check.sh`; current validation surface. |
| `docs/changelog/*` bootstrap mentions | keep | Historical truth. |
| `docs/roadmap/*` stub-provider mentions | keep | Current/historical planned-truth reconciliation, not stale bootstrap artifact. |

## Deleted artifacts

- `scripts/dev-run.sh`
- `scripts/ui-start.sh`
- `scripts/replay.sh`
- `scripts/memory-snapshot.sh`
- `scripts/memory-clear-ephemeral.sh`
- `docs/examples/prompts/placeholder.md`
- `docs/examples/workflows/placeholder.md`
- `docs/operations/operator-handbook.md`

## Updated references

- Removed deleted script and placeholder document regeneration entries from `scripts/bootstrap_repo.py`.
- No README or AGENTS references required updates.
- No CI workflow references required updates.
- No package script references to deleted artifacts required updates. The existing UI API behavior test command was kept compatible with the installed TypeScript command-line configuration boundary so validation could run without changing UI source or test assertions.

## Deferred artifacts

| Artifact | Reason deferred |
| --- | --- |
| `scripts/bootstrap_repo.py` | Used by `scripts/check.sh` for bootstrap idempotence validation. |
| `ui/package.json` `lint` and `build` placeholder commands | Used by CI and `scripts/check.sh`; deleting or replacing them would change current validation behavior. |
| Archived changelog bootstrap references | Historical truth, not active command surfaces. |
| Current stub-provider references in Rust/tests/docs | Active governed runtime/test terminology, not stale bootstrap artifact. |

## Rationale for each deletion

Each deleted file was tracked, stale or misleading, unreferenced by active validation/CI/package/README/AGENTS/roadmap/checklist surfaces after stale handbook/bootstrap references were accounted for, and removable without replacement behavior.

## Runtime non-change statement

No Rust source was changed. No runtime command behavior was replaced or added.

## UI non-change statement

No TypeScript source was changed. The UI API behavior test command remains the existing validation surface and was adjusted only for TypeScript command-line configuration compatibility; UI behavior and test assertions were not changed.

## Authority non-change statement

No governance authority, runtime authority, provider authority, persistence authority, replay authority, recovery authority, or action-execution authority was changed.

## Release/deployment non-change statement

No package, release artifact, installer, update channel, signing, publishing, GitHub release, tag, public download, public release, deployment automation, or production deployment behavior was created.

## Validator impact

`validate_structure.py`, `validate_docs.py`, and `scripts/check.sh` continue to pass after removing bootstrap regeneration entries for deleted artifacts.

## CI impact

No workflow files changed. The deleted files were not referenced by active CI workflows.

## README/AGENTS impact

README and AGENTS did not reference the deleted artifacts directly and were not changed.

## Roadmap/changelog truth posture

Roadmap truth was not changed. Historical truth for Phase 0/bootstrap remains in CHANGELOG and archived changelog files, not stale executable or placeholder surfaces.

## Required follow-ups

No follow-up is required for this sweep. Deferred artifacts remain documented above because they are used by active validation or are historical truth.

## Confirmed vs suspected

Confirmed deletions are listed in the Deleted artifacts section. Suspected-but-kept items are listed in the Deferred artifacts section.

## Non-readiness statement

This sweep does not approve readiness, Release Candidate status, Production Candidate status, public/general use, production readiness, production deployment, or production human use.
