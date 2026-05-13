---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 170.5 Rust Maintainability Audit

## Scope

Phase 170.5 inspected committed Rust source and Rust test surfaces only:

- `core/src/**/*.rs`
- `tests/**/*.rs`
- `core/tests/**/*.rs` when present; no `core/tests` directory is present in this working tree.

This audit is advisory orientation for Phase 171+. It identifies maintainability risk and extraction candidates only.

## Evidence rule

Findings count only committed repository files and current working-tree inspection. The audit does not rely on chat summaries, future roadmap intent, speculative architecture preferences, uncommitted assumptions, or prior phase claims not supported by current files.

## Audit method

Commands and equivalent shell/Python inspection used:

- `git status --short`
- `find core/src tests core/tests -name "*.rs" -type f 2>/dev/null | sort`
- `wc -l` over discovered Rust files
- `rg -n "fn |pub fn |impl " core/src tests core/tests`
- Inline Python brace-count scan estimating file sizes, function spans, branch/match/if density, maximum indentation, phase test naming, definition density, and oversized candidates.

The Python function-span scan is approximate because it uses brace counting rather than a Rust parser. Findings below are therefore split into confirmed line-count findings and suspected maintainability risks where semantic review would be needed before refactor.

## Rust file size inventory

| Lines | File |
| ---: | --- |
| 22,187 | `core/src/api/local_operator_shell.rs` |
| 3,062 | `core/src/execution/mod.rs` |
| 2,649 | `tests/integration_smoke.rs` |
| 2,599 | `core/src/api/observability.rs` |
| 2,556 | `core/src/api/persistence.rs` |
| 2,088 | `tests/adversarial_corpus.rs` |
| 2,081 | `core/src/api/local_workflow.rs` |
| 1,293 | `core/src/api/local_deployment_candidate.rs` |
| 1,268 | `core/src/api/durable_persistence_authority_decision.rs` |
| 1,263 | `core/src/api/application_state.rs` |
| 1,087 | `core/src/api/deployment_configuration.rs` |
| 1,008 | `core/src/api/local_artifact_manifest.rs` |
| 926 | `core/src/api/operator_action.rs` |
| 848 | `core/src/api/provider_configuration.rs` |
| 836 | `core/src/api/provider_execution_sandbox.rs` |
| 769 | `core/src/api/governance_evidence_versioning.rs` |
| 633 | `core/src/api/read_projection.rs` |
| 588 | `core/src/api/local_transport.rs` |
| 586 | `core/src/replay/mod.rs` |
| 577 | `core/src/context/mod.rs` |
| 508 | `core/src/api/authorization.rs` |
| 436 | `core/src/ledger/mod.rs` |
| 433 | `core/src/api/intent_audit.rs` |
| 430 | `core/src/api/provider_transport.rs` |
| 412 | `core/src/execution/provider_execution.rs` |
| 408 | `core/src/execution/provider_failure.rs` |
| 389 | `core/src/audit/mod.rs` |
| 362 | `core/src/memory/mod.rs` |
| 315 | `core/src/api/operator_intent.rs` |
| 267 | `core/src/api/runtime_config.rs` |
| 236 | `core/src/api/diagnostics.rs` |
| 223 | `core/src/main.rs` |
| 222 | `core/src/validation/mod.rs` |
| 216 | `core/src/state/mod.rs` |
| 188 | `core/src/policy/mod.rs` |
| 127 | `core/src/api/integration.rs` |
| 50 | `core/src/lib.rs` |
| 45 | `core/src/api/mod.rs` |
| 32 | `core/src/errors/mod.rs` |

## Oversized Rust file findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| critical | oversized_file | split_module_candidate, requires_phase_171_attention | `core/src/api/local_operator_shell.rs` is 22,187 lines, far above the 3,000-line threshold. It combines local shell data types, provider configuration surfaces, provider output pipeline projections, candidate conversion/materialization, trial package/evidence/restore/error/review projections, transport handling, serialization/parsing, validation helpers, and many phase tests. |
| high | oversized_file | split_module_candidate, requires_later_hardening | `core/src/execution/mod.rs` is 3,062 lines, crossing the 3,000-line threshold and containing execution contracts plus many tests. |
| high | test_accumulation | test_reorganization_candidate | `tests/integration_smoke.rs` is 2,649 lines with 54 `phase_###` test functions. |
| high | oversized_file | split_module_candidate | `core/src/api/observability.rs` is 2,599 lines and combines snapshot/export encoding, replay append, bundle writing, and tests. |
| high | oversized_file | split_module_candidate | `core/src/api/persistence.rs` is 2,556 lines and combines persistence boundary types, append/decode helpers, recovery helpers, transaction decoding, and tests. |
| high | test_accumulation | test_reorganization_candidate | `tests/adversarial_corpus.rs` is 2,088 lines with 24 `phase_###` test functions. |
| high | oversized_file | split_module_candidate | `core/src/api/local_workflow.rs` is 2,081 lines and combines workflow harness construction, run orchestration, evidence replay verification, and tests. |
| medium | oversized_file | requires_later_hardening | Files between 750 and 1,500 lines include `application_state.rs`, `deployment_configuration.rs`, `durable_persistence_authority_decision.rs`, `governance_evidence_versioning.rs`, `local_artifact_manifest.rs`, `local_deployment_candidate.rs`, `operator_action.rs`, `provider_configuration.rs`, and `provider_execution_sandbox.rs`. |

## Largest function findings

All functions at or above 200 estimated lines:

| Lines | Branch hits | Function |
| ---: | ---: | --- |
| 371 | 16 | `core/src/api/local_operator_shell.rs:9677 classify_complete_local_operator_workflow_step` |
| 359 | 33 | `core/src/api/local_operator_shell.rs:13532 derive_trial_error_report_projection` |
| 343 | 21 | `core/src/api/local_operator_shell.rs:2815 derive_local_provider_output_pipeline_projection` |
| 333 | 23 | `core/src/api/local_operator_shell.rs:14501 derive_trial_evidence_review_findings` |
| 257 | 21 | `core/src/api/local_operator_shell.rs:10774 derive_trial_failure_drill_projection` |
| 242 | 6 | `core/src/api/local_workflow.rs:135 run_local_harness_workflow` |
| 237 | 21 | `core/src/api/local_operator_shell.rs:12413 derive_trial_replay_restore_verification_projection` |
| 203 | 8 | `core/src/execution/mod.rs:867 run_controlled_model_flow` |

Additional functions at or above 100 estimated lines include validation, serialization, projection, and phase-test functions in `local_operator_shell.rs`, `local_workflow.rs`, `observability.rs`, `persistence.rs`, `operator_action.rs`, `local_deployment_candidate.rs`, `deployment_configuration.rs`, `durable_persistence_authority_decision.rs`, `governance_evidence_versioning.rs`, and root integration/adversarial tests.

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| critical | oversized_function | helper_extraction_candidate, requires_phase_171_attention | `classify_complete_local_operator_workflow_step` is a 371-line classifier in the already critical shell file. Future workflow-step additions should not extend it directly without first creating a helper family. |
| critical | projection_accumulation | helper_extraction_candidate, requires_phase_171_attention | Trial projection derivation functions in `local_operator_shell.rs` are repeatedly above 200 lines and should be split by status derivation, blocker collection, evidence mapping, and summary construction. |
| high | validation_accumulation | helper_extraction_candidate | `validate_local_candidate_materialization_request` is 180 lines with high branch density and likely mixes validation categories that could become smaller field validators. |
| high | serialization_accumulation | helper_extraction_candidate | `parse_trial_session_evidence_record`, `parse_local_session_package`, `parse_controlled_internal_trial_package`, and `serialize_local_session_package` cluster serialization/parsing responsibility in the shell module. |
| high | oversized_function | helper_extraction_candidate | `run_local_harness_workflow` and `run_controlled_model_flow` are orchestration functions that should remain stable while helper extraction is planned around setup, execution, replay, and summary projection. |

## Deep nesting / branch-density findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| critical | branch_density | helper_extraction_candidate | `derive_trial_error_report_projection` has an estimated 33 branch hits and 359 lines. Split error-source normalization, severity/status derivation, blocker summaries, and display projection. |
| high | branch_density | helper_extraction_candidate | `parse_trial_session_evidence_record` is 94 lines but has an estimated 35 branch hits, making it a dense parser even though it is below 100 lines. |
| high | branch_density | helper_extraction_candidate | `decode_durable_append_transaction` has an estimated 29 branch hits in 102 lines inside `persistence.rs`; decoding should be isolated from boundary-state responsibilities. |
| high | deep_nesting | helper_extraction_candidate | `derive_local_provider_output_pipeline_projection` reaches an estimated 32-space indentation and 343 lines; nested projection construction should be flattened by helper extraction. |
| high | deep_nesting | helper_extraction_candidate | `local_operator_shell_transport_step` reaches an estimated 28-space indentation with 10 match hits and is a transport accumulation point. |

## Transport and request handling findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| high | transport_accumulation | helper_extraction_candidate, requires_phase_171_attention | `local_operator_shell_transport_step` is a 155-line transport function with multiple match arms and shell-state routing. Phase 171 should not add release-candidate preparation routing directly into this body unless the integration is minimal and delegated. |
| medium | transport_accumulation | leave_as_is | `core/src/api/local_transport.rs` is 588 lines, below the large-file threshold. It appears bounded compared with the shell transport surface, but future transport variants should avoid migrating unrelated responsibilities into it. |
| low | transport_accumulation | leave_as_is | `core/src/api/mod.rs` is 45 lines and is not a monolithic transport or request surface. |

## Projection derivation findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| critical | projection_accumulation | split_module_candidate, requires_phase_171_attention | `local_operator_shell.rs` contains many large `derive_*_projection` functions. Release-candidate preparation projections should be placed in a new release-preparation module or tightly scoped helper family rather than appended to the monolith. |
| high | projection_accumulation | helper_extraction_candidate | Provider output pipeline, trial failure drill, trial replay restore verification, trial observability, trial error report, and trial evidence review derivations are current candidates for later extraction. |
| medium | repeated_pattern | helper_extraction_candidate | Repeated projection fields for status, blocker state, boundary markers, local-only/no-authority notes, and deterministic reason lists should become small helper families after behavior stabilizes. |

## Validation-function findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| high | validation_accumulation | helper_extraction_candidate | Candidate materialization validation and trial package/evidence validation functions are long enough to risk mixing field validation, boundary checks, and projection-ready reason construction. |
| medium | validation_accumulation | defer_until_behavior_stabilizes | Some validation helpers encode phase-specific safety wording. Refactor should preserve exact reason strings and stable ordering. |

## Serialization / parsing-function findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| high | serialization_accumulation | helper_extraction_candidate | Shell-local session package and trial evidence parsing/serialization should be candidates for `local_operator_shell_session_package` or `trial_evidence_codec` helpers. |
| high | serialization_accumulation | helper_extraction_candidate | Persistence transaction decoding should be a candidate for `persistence_transaction_codec` or `durable_append_decode` helpers. |
| medium | serialization_accumulation | helper_extraction_candidate | Observability export encoding functions should be candidates for `observability_export_encoding` helpers if future export fields expand. |

## Phase-test accumulation findings

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| high | test_accumulation | test_reorganization_candidate | `core/src/api/local_operator_shell.rs` contains 128 `phase_###` test functions inside the largest runtime module. This makes the monolith harder to navigate and increases merge-conflict risk. |
| high | test_accumulation | test_reorganization_candidate | `tests/integration_smoke.rs` contains 54 `phase_###` test functions and has become a phase-accumulation surface. |
| medium | test_accumulation | test_reorganization_candidate | `tests/adversarial_corpus.rs` contains 24 `phase_###` test functions. It remains conceptually coherent as adversarial corpus coverage, but should eventually be partitioned by boundary family. |
| medium | test_accumulation | test_reorganization_candidate | Provider configuration, provider execution sandbox, governance evidence versioning, deployment configuration, and local deployment candidate modules each contain phase-specific test clusters. |

## Stale early-phase surface candidates

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| medium | stale_surface | requires_later_hardening | Phase 104-116 transport, persistence, provider, authority, and deployment-candidate test names remain concentrated in root tests and shell-adjacent modules. They are still evidence-bearing but candidates for boundary-family test files. |
| medium | stale_surface | helper_extraction_candidate | Early local transport rejection helper patterns and no-authority assertion patterns recur across integration and adversarial tests. They should be extracted only if exact assertions remain unchanged. |
| low | stale_surface | leave_as_is | `core/src/api/mod.rs` is small and should remain an index surface unless module count or public export rules change. |

## Repeated pattern candidates

| Severity | Category | Disposition | Finding |
| --- | --- | --- | --- |
| high | repeated_pattern | helper_extraction_candidate | Local-only boundary markers, no-authority claims, no-provider-execution claims, and reason-code arrays recur across shell projections and tests. |
| high | repeated_pattern | helper_extraction_candidate | Candidate conversion/materialization status, blocker, and validation-reason patterns recur in long shell functions. |
| medium | repeated_pattern | helper_extraction_candidate | Trial package, trial evidence, restore verification, error report, and evidence review surfaces repeat status/blocker/source-link construction. |
| medium | repeated_pattern | helper_extraction_candidate | Serialization read-back helpers repeat key-value parsing and deterministic field-order assumptions. |

## Recommended extraction map

| Priority | Severity | Category | Proposed target | Move | Remain | Timing | Phase 171+ protection |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | critical | release_candidate_risk | `core/src/api/release_candidate_preparation.rs` or `core/src/api/local_release_candidate_preparation.rs` | New Phase 171 release-candidate preparation types, validation, projection derivation, and serialization helpers. | Minimal shell integration and public exports. | Before or during Phase 171. | Prevents adding a new major release-preparation surface to `local_operator_shell.rs`. |
| 2 | critical | projection_accumulation | `core/src/api/local_operator_shell_trial_projection.rs` helper family | Trial error, observability, restore, evidence review, and failure drill projection helpers. | Shell state ownership and thin calls. | After Phase 171 unless Phase 171 touches trial projection logic. | Reduces risk of changing unrelated trial projections while adding release-preparation work. |
| 3 | high | transport_accumulation | `core/src/api/local_operator_shell_transport.rs` helper family | Request routing helpers and variant-specific handlers. | Existing local shell API and state mutation boundary. | During a future transport phase, not as part of this audit. | Keeps transport additions from becoming another branch-dense function. |
| 4 | high | serialization_accumulation | `core/src/api/local_session_package_codec.rs`, `core/src/api/trial_evidence_codec.rs`, or private helper modules | Parse/serialize/read-back helpers and deterministic field-order checks. | Public projection structs and validation contracts. | After behavior stabilizes or before adding release package serialization. | Avoids coupling release artifact/dry-package parsing with old session package parsing. |
| 5 | high | validation_accumulation | `core/src/api/local_candidate_materialization_validation.rs` | Field-level validation helpers and reason builders. | Public request/result types and top-level validation entry point. | Before major candidate-materialization expansion. | Preserves deterministic reasons while making additions safer. |
| 6 | high | oversized_file | `core/src/api/persistence_transaction_codec.rs` | Durable append transaction decode/encode helpers. | Persistence authority boundary types. | Before persistence behavior expands. | Keeps decode complexity isolated from future persistence authority surfaces. |
| 7 | high | test_accumulation | Boundary-family test files such as `tests/local_transport_boundary.rs`, `tests/provider_boundary.rs`, `tests/deployment_candidate_boundary.rs` | Phase-specific integration/adversarial tests grouped by boundary. | Existing assertions and deterministic payloads. | Later hardening phase. | Lowers root test merge conflicts and keeps Phase 171 tests discoverable. |
| 8 | medium | repeated_pattern | `core/src/api/boundary_marker_helpers.rs` or private local helpers | Repeated local-only/no-authority/provider-untrusted marker construction. | Rust-owned validation decisions. | Later, only after repeated concrete uses are stable. | Reduces copy drift in status/projection wording. |

For every high or critical finding, tests that must remain unchanged are the existing Rust unit/integration/adversarial assertions and deterministic reason ordering. Refactor timing should avoid changing behavior during this out-of-band audit.

## Phase 171 risk assessment

Decision status: `phase_171_should_limit_changes_to_new_module`.

Phase 171 can proceed safely with caveats if it avoids adding large release-candidate preparation logic directly to `core/src/api/local_operator_shell.rs`. The safer path is to create a new release-candidate preparation module first, then wire only minimal projection or transport integration into existing shell surfaces. Phase 171 is not blocked by monolith risk, but extending the shell monolith would compound the critical oversized-file and projection-accumulation risks identified here.

If Phase 171 requires dry-package serialization/read-back behavior, refactor pressure rises toward `refactor_required_before_release_candidate_dry_package` for codec helpers, but this audit does not implement that refactor.

## Refactor deferral rules

- Do not refactor during Phase 170.5.
- Do not move tests during Phase 170.5.
- Do not change reason strings, ordering, field names, public enum variants, serialized formats, or boundary claims as part of this audit.
- Defer helper extraction until a behavior-producing phase has an explicit target and preserves existing tests unchanged.
- Prefer new narrowly scoped modules for Phase 171 release-candidate preparation over adding more logic to oversized shell surfaces.

## What Phase 170.5 does not change

- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No refactor implementation.
- No Phase 171 implementation.
- No release-candidate preparation behavior.
- No release artifact creation.
- No deployment behavior.
- No provider execution expansion.
- No persistence implementation.
- No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.

## Validation output

Required validation for this audit is recorded in `checklists/current-phase.md` after final edits. The audit-specific inventory commands identify 39 Rust files, 16 files at or above 750 lines, 8 functions at or above 200 estimated lines, and additional functions at or above 100 estimated lines.

## Deferred items

- Actual Rust module splitting.
- Transport handler extraction.
- Trial projection helper extraction.
- Candidate materialization validation helper extraction.
- Persistence transaction codec extraction.
- Observability export encoder extraction.
- Root test reorganization.
- Phase 171 release-candidate preparation implementation.

## Confirmed vs suspected

Confirmed:

- Rust file inventory and line counts.
- Oversized Rust files above 750, 1,500, and 3,000 lines.
- Approximate function spans above 100 and 200 lines by brace counting.
- Phase-test accumulation counts by `fn phase_###` naming.

Suspected pending semantic refactor review:

- Exact helper boundaries inside projection functions.
- Whether branch-dense parsing functions should split by field, section, or codec responsibility.
- Whether early-phase test surfaces should move into root tests or module-local tests.
- Whether repeated boundary marker helpers should be public, crate-private, or private to a new module.

## Phase 171 handoff

- Phase 171 remains the next planned code-production phase after this audit.
- Phase 171 should add release-candidate preparation work in a new Rust module or tightly scoped helper area.
- Phase 171 should keep existing shell integration thin.
- Phase 171 should avoid growing `local_operator_shell.rs` with another major projection/contract surface.
- Phase 171 should preserve all existing Rust tests and serialized boundary wording unless a later explicit behavior phase authorizes a bounded change.
