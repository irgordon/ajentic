---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local Workflow Guide

## Scope
This guide documents supported local workflows and operator expectations that exist now.

## Operator vocabulary
- **Validation:** checks repository integrity.
- **Dry run:** reports current non-executing posture and performs no provider/persistence/action behavior.
- **Append:** durable evidence transaction, not promotion or recovery.
- **Recovery acceptance:** explicit in-memory acceptance, not global state replacement.
- **Replay:** evidence verification, not live execution.
- **Export:** operator-readable copy, not authoritative ledger state. Export is planned later and not currently implemented.
- **Production Candidate:** not approved.

## Supported local commands
- `./scripts/check.sh`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- `node scripts/test_rust_boundary_lint.mjs`
- `node scripts/rust_boundary_lint.mjs`
- `node scripts/test_lint_ui_boundaries.mjs`
- `node scripts/lint_ui_boundaries.mjs`
- `cd ui && npm run typecheck && npm run lint && npm run build`

## Validation workflow
### What this does
Runs repository validation gates for Rust, UI, and boundary lints.

### What happens when it fails
A failing command returns a non-zero exit code and blocks clean-phase closure.

### What this does not do
It does not execute providers, persist runtime mutations, append durable evidence, or approve Production Candidate status.

## Dry-run workflow
### What this does
Runs `cargo run --manifest-path core/Cargo.toml -- dry-run` to print the CLI non-executing summary.

### What happens when it fails
The command exits non-zero, indicating local CLI validation did not complete.

### What this does not do
It does not execute provider calls, persistence writes, action execution, replay verification, recovery acceptance, or live UI/Rust transport.

## Root integration-test workflow
### What this does
Runs `cargo test --manifest-path core/Cargo.toml --all-targets` to execute existing root integration coverage and module tests.

### What happens when it fails
Any failing test exits non-zero and indicates an invariant regression.

### What this does not do
It does not create new authority, production approval, or new runtime capabilities by itself.

## Durable append posture
### What this does
Documents append as a durable evidence transaction boundary.

### What happens when it fails
Append validation rejects invalid envelopes and reports non-commit outcomes.

### What this does not do
It does not promote state, recover state, or claim readiness approval.

## Recovery acceptance posture
### What this does
Documents explicit candidate acceptance for in-memory use when boundary checks pass.

### What happens when it fails
Mismatched identifiers, revisions, or candidate checks reject acceptance.

### What this does not do
It does not replace global state, persist state, or append new ledger/audit authority.

## Replay posture
### What this does
Documents replay as verification of captured evidence snapshots.

### What happens when it fails
Checksum and evidence mismatches reject replay verification.

### What this does not do
It does not perform live provider execution or create new authority artifacts.

## UI contract posture
### What this does
Documents UI as a non-authoritative operator-intent and review contract surface.

### What happens when it fails
UI lint/type/build or contract checks fail validation gates.

### What this does not do
It does not submit through live Rust ingress or execute live runtime mutations.

## Failure modes
- Validation command exits non-zero.
- Rust or UI lint/type/build checks fail.
- Dry-run command fails to produce the deterministic summary.
- Replay/append/recovery boundary checks reject mismatched evidence.

## What this does not do
- Does not add observability snapshots.
- Does not add export encoding or export writes.
- Does not add startup commands or packaging artifacts.
- Does not add provider live execution.
- Does not add live UI/Rust transport.
- Does not add replay repair behavior.
- Does not add recovery mutation authority.

## Troubleshooting
1. Re-run `./scripts/check.sh` first.
2. Re-run failing explicit command individually.
3. Confirm working-tree hygiene with `git status --short`.
4. Confirm scope wording stays non-authoritative and non-readiness.

## Non-readiness statement
This local workflow surface is operator-local evidence guidance only. Production Candidate status is not approved, and production readiness is not claimed.
