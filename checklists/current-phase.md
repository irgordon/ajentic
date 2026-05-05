---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---
# Current Phase Checklist

## phase name
Phase 76.5 - Out-of-Band Validation Gate Non-Mutation and Coverage Alignment

## explicit out-of-band maintenance note
Phase 76.5 is an out-of-band maintenance fix before Phase 77.

## phase goal
Make `scripts/check.sh` accurately represent the whole-repo non-mutating validation bundle before Phase 77 UI intent submission wiring.

## working-tree hygiene gate
- [x] `git status --short` run before edits and classified.
- [x] Only allowed files modified for this phase.
- [x] Generated artifacts reviewed/reverted before staging.

## allowed surfaces
- [x] `scripts/check.sh`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/validation-gate-alignment-phase-76-5.md`
- [ ] Rust source
- [ ] TypeScript source
- [ ] roadmap/governance/architecture sources
- [ ] package/dependency files

## boundary rules
- [x] Validation tooling maintenance only.
- [x] No UI submission wiring or event handlers.
- [x] No runtime behavior changes.
- [x] No Rust behavior changes.
- [x] No TypeScript behavior changes.
- [x] Phase 77 scope remains unchanged.

## task checklist
- [x] Updated checklist to Phase 76.5 procedural truth.
- [x] Added operations documentation for validation-gate alignment.
- [x] Added `CHANGELOG.md` entry for `v0.0.76.5`.
- [x] Removed mutating Rust format command from `scripts/check.sh`.
- [x] Kept Rust formatting check command in `scripts/check.sh`.
- [x] Added UI typecheck/lint/build commands to `scripts/check.sh`.

## validation checklist
- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_rust_boundary_lint.mjs`
- [x] `node scripts/rust_boundary_lint.mjs`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`

## check.sh non-mutation checklist
- [x] No standalone `cargo fmt --manifest-path core/Cargo.toml` remains.
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check` remains.
- [x] `set -euo pipefail` fail-fast posture preserved.
- [x] Rust boundary lint self-tests run before Rust boundary lint.
- [x] UI AST lint self-tests run before UI AST lint.

## UI validation coverage checklist
- [x] `npm run typecheck` included.
- [x] `npm run lint` included.
- [x] `npm run build` included.
- [x] UI validation runs before Rust formatting/Rust check-test-clippy stage.

## validator split/frontmatter convention checklist
- [x] Documented structure/docs validator overlap and division.
- [x] Documented `docs/operations/*` uses `mutation_path: readme_update` convention.
- [x] Confirmed `scripts/validate_structure.py` unchanged.
- [x] Confirmed `scripts/validate_docs.py` unchanged.

## findings table
| Item | Finding |
| --- | --- |
| Validation mutation behavior | `scripts/check.sh` now performs non-mutating Rust format verification only. |
| UI coverage parity | Local check gate now includes UI typecheck, lint, and build. |
| Validator overlap | `validate_structure.py` enforces placement/required frontmatter presence and truth-dimension compatibility; `validate_docs.py` enforces exact frontmatter values and truth-surface contamination checks. |
| Operations frontmatter convention | `docs/operations/*` uses `mutation_path: readme_update` and remains enforced by docs validator expectations. |
| Phase relationship | Phase 76.5 is an out-of-band maintenance fix before Phase 77; Phase 77 scope remains unchanged. |

## deferred items table
| Deferred item | Owner phase |
| --- | --- |
| UI operator intent submission wiring | Phase 77 |
| Authorized action execution boundary | Phase 78 |

## validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | pass | Uncommitted files classified before edits. |
| `./scripts/check.sh` | pass | Whole-repo validation completed with non-mutating formatting check and UI coverage. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | pass | Explicit UI parity re-run completed. |
| `node scripts/test_rust_boundary_lint.mjs` | pass | Rust boundary lint self-tests passed. |
| `node scripts/rust_boundary_lint.mjs` | pass | Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | pass | UI AST lint passed. |
