---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 197.1 Verifier-Receipt Derivation Evidence

## Baseline and branch

- Starting commit: `ac26996537258360e4380ab87b36e9ef71bb06a0`.
- Starting subject: `refactor(core): Isolate authority pipelines`.
- Branch: `mothra/phase-197-1-verifier-receipts`.

## Original abstraction mismatch

Phase 197 made `ValidationReceipt` and `PolicyReceipt` opaque, but their public evidence constructors still accepted caller-selected positive booleans. A caller could not construct `PASS` directly, yet could supply the moral equivalent of `schema_valid = true`, `deterministic_check_passed = true`, or `has_required_context = true`.

## Deterministic proof sources

| Prerequisite | Structured source | Bounded proof | Limitation |
| --- | --- | --- | --- |
| Evidence well formed | `EvidenceManifest` | Manifest construction plus `verify_binding` | Proves current manifest integrity, not external source truth. |
| Candidate shape | Typed `ProviderOutput` | Required fields, received status, untrusted posture | Proves the current Rust type shape only; no broader schema equivalence is claimed. |
| Deterministic candidate/evidence check | `ProviderOutput`, `EvidenceManifest`, `AuthorityBinding` | Candidate content and manifest SHA-256 digests match the binding | Does not prove factual correctness of candidate text. |
| Required context | Recorded `context_packet_id` | Identifier digest matches `context_packet_digest` | Does not redesign context assembly. |
| Required operator intent | Current bounded operator/intent source | Source digest matches `operator_intent_digest` | Preserves the Phase 197 bounded representation; no richer intent contract is invented. |

Missing structured input produces `UNKNOWN`. No passing verifier was fabricated for an absent proof source.

## Verifier surfaces

- `ValidationVerifierReceipt` in `core/src/verification/mod.rs` proves one of: manifest well-formedness, candidate shape, or candidate/evidence binding.
- `PolicyVerifierReceipt` in `core/src/verification/mod.rs` proves one of: required context binding or required operator-intent binding.
- Both receipt types have private fields and no public success constructor.
- Each receipt binds its check kind, complete `AuthorityBinding`, source digests, verifier identifier/version, deterministic status/reason, and receipt digest.
- `ValidationEvidence` and `PolicyEvidence` now accept only verifier receipts and reject kind, binding, source, or digest mismatches.
- `AuthorityEvaluationEvidence::new` now rejects validation and policy evidence from different bindings.

## Removed boolean paths and migrated callsites

- Removed `schema_valid`, `evidence_well_formed`, `deterministic_check_passed`, and `model_output_claims_valid` from `core/src/validation/mod.rs`.
- Removed `has_required_context`, `has_required_operator_intent`, and `model_output_claims_success` from `core/src/policy/mod.rs`.
- Migrated `core/src/api/local_workflow.rs` to verifier functions over the actual request, output, manifest, and binding.
- Migrated controlled-run re-derivation and replay evidence in `core/src/execution/mod.rs`.
- Migrated shared Rust test fixtures in `tests/common/mod.rs`.
- Added a narrow source-lint tripwire for obvious positive-boolean constructor regressions; Rust privacy and deterministic verifier functions remain the actual control.

## Focused regression evidence

- Matching verifier evidence produces validation `PASS` and policy `ALLOWED`.
- Failed candidate shape, candidate digest mismatch, manifest mismatch, context mismatch, and intent mismatch fail closed.
- Missing structured verifier input produces `UNKNOWN` and cannot authorize promotion.
- Cross-run, cross-candidate, and cross-revision verifier receipt reuse fails.
- Model text containing `valid`, `approved`, `schema valid`, `evidence complete`, `policy allowed`, or `operator authorized` does not change verifier status.
- Controlled-run construction rejects substituted verifier evidence.
- Identical fixed inputs produce identical receipt digests.

Focused and inherited Rust validation passes `1,240` tests; UI API behavior validation passes `141/141`; the full clean-tree repository wrapper passes with `All checks passed.`.

## Abstraction review

1. Production callers cannot create a positive validation prerequisite by supplying `true`.
2. Production callers cannot create a positive policy prerequisite by supplying `true`.
3. Production callers cannot directly construct a successful verifier receipt.
4. Receipts from another `AuthorityBinding` cannot satisfy the current run.
5. Model or adapter text cannot improve verifier status.
6. The local simulation cannot bypass verifier receipt creation.
7. No success boolean was moved behind another public helper.

## Boundary confirmation

- No real provider was activated.
- No external action execution was added.
- Promotion semantics and authority did not expand.
- Persistence authority did not expand.
- No replay repair or recovery promotion was added.
- No workflow behavior or package version changed.
- No package publication, installer, update channel, deployment, signing, notarization, tag, or GitHub Release was created.
- Phase 197.2 postcondition-to-success-criterion binding remains separate and unimplemented.
