---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 170.5 - Out-of-Band Rust Surface Maintainability Audit

## Phase name
- [x] Phase 170.5 - Out-of-Band Rust Surface Maintainability Audit.

## Phase goal
- [x] Identify Rust maintainability risk before Phase 171.
- [x] Produce an advisory extraction map without implementing refactors.
- [x] Preserve Phase 171 as the next planned code-production phase.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Keep changes limited to allowed Phase 170.5 surfaces.
- [x] Confirm no Rust, TypeScript, test, schema, script, workflow, package, README, AGENTS, governance, architecture, roadmap, changelog archive, or UI/help drift.

## Allowed surfaces
- [x] `docs/operations/rust-maintainability-audit-phase-170-5.md`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Audit-only checklist
- [x] Identify large Rust source files.
- [x] Identify monolithic Rust modules.
- [x] Identify long functions.
- [x] Identify deeply nested and branch-dense functions.
- [x] Identify brittle projection, transport, validation, serialization, and parsing surfaces.
- [x] Identify stale early-phase surface candidates.
- [x] Identify repeated pattern candidates.
- [x] Produce a prioritized extraction map.

## Rust file inventory checklist
- [x] Inventory `core/src/**/*.rs`.
- [x] Inventory `tests/**/*.rs`.
- [x] Check `core/tests/**/*.rs` when present; no `core/tests` directory is present.
- [x] Record 39 Rust files in the audit inventory.

## Large file checklist
- [x] Record all Rust files over 750 lines.
- [x] Record all Rust files over 1,500 lines.
- [x] Record all Rust files over 3,000 lines.
- [x] Identify `core/src/api/local_operator_shell.rs` as a critical oversized_file finding.

## Large function checklist
- [x] Record all Rust functions over 100 estimated lines.
- [x] Record all Rust functions over 200 estimated lines.
- [x] Identify local shell projection/classification functions as oversized_function findings.

## Deep nesting / branch-density checklist
- [x] Identify deep_nesting candidates.
- [x] Identify branch_density candidates.
- [x] Record branch-dense parser/decoder/projection functions where present.

## Transport/projection/validation accumulation checklist
- [x] Identify transport_accumulation candidates.
- [x] Identify projection_accumulation candidates.
- [x] Identify validation_accumulation candidates.
- [x] Identify serialization_accumulation candidates.

## Test accumulation checklist
- [x] Identify test_accumulation in `core/src/api/local_operator_shell.rs`.
- [x] Identify test_accumulation in `tests/integration_smoke.rs`.
- [x] Identify test_accumulation in `tests/adversarial_corpus.rs`.

## Stale surface checklist
- [x] Identify stale_surface candidates from early phase transport/provider/persistence/deployment tests.
- [x] Mark stale surfaces as extraction candidates only, not removal candidates.

## Extraction candidate checklist
- [x] Recommend release-candidate preparation module isolation.
- [x] Recommend trial projection helper extraction candidates.
- [x] Recommend transport handler extraction candidates.
- [x] Recommend serialization/read-back helper extraction candidates.
- [x] Recommend validation helper extraction candidates.
- [x] Recommend phase-test reorganization candidates.

## Phase 171 handoff checklist
- [x] State decision status `phase_171_should_limit_changes_to_new_module`.
- [x] State that Phase 171 can proceed safely with caveats.
- [x] State that Phase 171 should avoid adding large release-candidate preparation logic directly to `local_operator_shell.rs`.
- [x] State that Phase 171 remains the next planned code-production phase.

## No-implementation checklist
- [x] No Rust source changes.
- [x] No TypeScript source changes.
- [x] No test changes.
- [x] No schema changes.
- [x] No runtime behavior.
- [x] No refactor implementation.
- [x] No Phase 171 implementation.
- [x] No release-candidate preparation behavior.
- [x] No release artifact creation.
- [x] No deployment behavior.
- [x] No provider execution expansion.
- [x] No persistence implementation.
- [x] No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-170-5-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] Rust inventory scan.
- [x] Large Rust file scan.
- [x] Maintainability report scan.
- [x] No-source-drift guard.
- [x] Readiness/release/provider scan.
- [x] Implementation-drift scan.

## Deferred items
- [x] Rust module splitting is deferred.
- [x] Transport handler extraction is deferred.
- [x] Trial projection helper extraction is deferred.
- [x] Candidate materialization validation helper extraction is deferred.
- [x] Persistence transaction codec extraction is deferred.
- [x] Observability export encoder extraction is deferred.
- [x] Root test reorganization is deferred.
- [x] Phase 171 release-candidate preparation implementation is deferred.

## Validation log
- [x] Full required validation passed after final edits.
- [x] No masked failures exist.
- [x] Generated artifacts were cleaned.

## Zero-drift checklist
- [x] Staged files match allowed Phase 170.5 surfaces.
- [x] Rust file inventory is recorded.
- [x] Large Rust files are identified.
- [x] Largest functions are identified.
- [x] Deep nesting or branch-density risks are identified where present.
- [x] Transport accumulation risks are identified where present.
- [x] Projection accumulation risks are identified where present.
- [x] Validation accumulation risks are identified where present.
- [x] Serialization/parsing accumulation risks are identified where present.
- [x] Test accumulation risks are identified where present.
- [x] Stale early-phase surface candidates are identified where present.
- [x] Repeated patterns are identified where present.
- [x] Recommended extraction map is present.
- [x] Phase 171 risk assessment is present.
- [x] Report states that Phase 171 can proceed with caveats and should limit changes to a new module.
- [x] No readiness, Release Candidate, Production Candidate, controlled-human-use, public-use, or production approval is introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] No runtime behavior or source drift is introduced.
