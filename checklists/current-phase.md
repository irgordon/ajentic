---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 197.1 - Verifier-Derived Validation and Policy Evidence

- Starting baseline:
  - [x] Verify Phase 197 head `ac26996537258360e4380ab87b36e9ef71bb06a0`.
  - [x] Branch `mothra/phase-197-1-verifier-receipts` from that exact head.
  - [x] Confirm the starting worktree was clean.
- Positive predicate inventory:
  - [x] Map manifest well-formedness to `EvidenceManifest` construction and binding verification.
  - [x] Map candidate shape to the actual typed `ProviderOutput` fields.
  - [x] Map deterministic candidate/evidence checks to candidate and manifest SHA-256 digests.
  - [x] Map required context to the recorded context packet identifier digest.
  - [x] Map required operator intent to the current bounded intent-source digest.
  - [x] Record that no richer cross-schema equivalence proof exists in the bounded simulation.
- Verifier receipts:
  - [x] Add opaque `ValidationVerifierReceipt` and `PolicyVerifierReceipt` types.
  - [x] Bind kind, authority binding, source digests, verifier identity/version, status, reason, and receipt digest.
  - [x] Keep positive receipt construction private to deterministic Rust verifier functions.
  - [x] Return `UNKNOWN` when required structured input is absent.
- Validation and policy migration:
  - [x] Remove public authority-bearing validation boolean parameters.
  - [x] Remove public authority-bearing policy boolean parameters.
  - [x] Remove advisory model claims from authority evidence bundles.
  - [x] Make validation evidence construction verify receipt kind, binding, source, and digest alignment.
  - [x] Make policy evidence construction verify receipt kind, binding, and digest alignment.
  - [x] Bind `ValidationReceipt` and `PolicyReceipt` digests to verifier evidence digests.
- Authority pipeline migration:
  - [x] Make `AuthorityEvaluationEvidence::new` fallible on mixed bindings.
  - [x] Preserve controlled validation and policy re-derivation.
  - [x] Verify underlying verifier-evidence digests before accepting substituted receipts.
  - [x] Route the local simulation through the same verifier functions.
  - [x] Preserve promotion and replay fail-closed behavior.
- Regression coverage:
  - [x] Add matching, failed, unknown, manifest-mismatch, model-text, cross-run, cross-candidate, and cross-revision tests.
  - [x] Add controlled receipt-substitution and missing-proof tests.
  - [x] Extend the Rust boundary lint with positive-boolean constructor tripwires.
  - [x] Confirm Rust boundary lint self-tests pass (`26/26`).
- Scope boundaries:
  - [x] No governance theory or architecture ownership change.
  - [x] No shared schema change.
  - [x] No Phase 197.2 postcondition-to-success-criterion implementation.
  - [x] No real provider, external action, persistence, replay-repair, release, or package authority expansion.
- Validation:
  - [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`.
  - [x] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`.
  - [x] `cargo test --manifest-path core/Cargo.toml --all-targets` (`1,240` tests).
  - [x] `python3 scripts/validate_structure.py`.
  - [x] `python3 scripts/validate_operator_intent_contract_map.py`.
  - [x] `python3 scripts/validate_docs.py`.
  - [x] `node scripts/rust_boundary_lint.mjs`.
  - [x] `node scripts/lint_ui_boundaries.mjs`.
  - [x] `cd ui && npm run typecheck && npm run test:api` (`141/141`).
  - [x] Deterministic artifact and release-preflight checks.
  - [x] `git diff --check`.
  - [x] Full clean-tree `./scripts/check.sh` after commit (`All checks passed.`).
