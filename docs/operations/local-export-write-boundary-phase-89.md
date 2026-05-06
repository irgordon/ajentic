---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local Export Write Boundary - Phase 89

## Scope
Phase 89 adds a local, operator-readable write boundary for already encoded Phase 88 audit export bundles.

The phase title and scope were confirmed from `docs/roadmap/phase-map.md`, `docs/roadmap/phases.md`, and `docs/roadmap/sequencing.md`: Phase 89 is `Local Export Write Boundary`, with the goal of writing verified export bundles while preserving the boundary that export write is not ledger append, recovery, promotion, or live telemetry.

## Local export write model
The implementation accepts a `LocalExportWriteRequest` containing an export directory, a validated export file name, and an existing `AuditExportEnvelope`.

Phase 89 writes encoded export bundles only. It does not alter Phase 88 canonical encoding semantics, re-encode snapshot data, or reshape the envelope into a ledger payload.

The request/report authority helper stays in `core/src/api/observability.rs` while a minimal create-new verified file helper lives in `core/src/api/persistence.rs` because the Rust boundary lint centralizes filesystem calls there. The persistence helper is export-specific and does not classify exports as ledger append transactions.

## Export path safety
The export destination is untrusted input. The API does not accept an arbitrary full export path as its primary input. It accepts an export directory and a file name, validates the file name, canonicalizes the directory, and constructs the final path internally.

The file name must be a file name only, not a path. Safe names use only ASCII letters, ASCII digits, `.`, `_`, and `-`, and must end with `.ajentic-export`.

The export directory is canonicalized before writing. Repository-root-like dangerous targets are rejected when detectable, including the repository root itself and direct `docs`, `schema`, `schemas`, `ledger`, `audit`, and `memory` targets.

## Path traversal and overwrite rejection
Phase 89 rejects empty file names, absolute file names, path separators, `.` and `..`, parent traversal substrings, Windows-style drive prefixes or any `:`, unsafe characters, missing `.ajentic-export` suffixes, existing files, and symlink targets detectable through metadata.

Overwrite attempts are rejected before writing and again by create-new/no-overwrite file creation semantics.

## Atomic write and verification posture
Phase 89 uses an export-specific persistence helper for create-new/no-overwrite file creation at the final export path. The helper writes only the envelope encoded bytes, flushes and syncs the file, reads the file back, and compares the bytes exactly with the input envelope bytes.

A report may say `written=true` only after the write succeeds and readback verification matches the encoded envelope bytes. Write failures and verification failures return typed rejection reports.

## Relationship to Phase 88 encoding
Phase 88 remains the canonical encoding boundary. Phase 89 receives `AuditExportEnvelope` values and writes `encoded_bytes` only.

Phase 89 does not change canonical field order, line endings, size limits, status/reason code encoding, or data minimization semantics from Phase 88.

## Export is not ledger state
Phase 89 does not write ledger or audit append records. Export files are not ledger facts, are not durable append transactions, and are not authoritative state.

The local export report keeps `ledger_import_allowed=false` and `mutated_authority=false` for both success and rejection paths.

## Export is not recovery input
Phase 89 does not implement export import. Export bundles are not recovery candidates and are not accepted for in-memory use.

The local export report keeps `recovery_import_allowed=false` for both success and rejection paths.

## Export is not replay repair
Phase 89 does not read export bundles as replay evidence and does not repair replay state.

The local export report keeps `replay_repair_allowed=false` for both success and rejection paths.

## Retention and operator-managed files
Phase 89 does not delete, rotate, expire, retain, or otherwise manage existing operator export files.

Retention remains operator-managed. The write helper creates one new export file or rejects the request.

## Root integration-test coverage
The local export write API is publicly reachable through the existing `api::*` re-export surface.

Phase 89 adds root integration smoke tests covering a verified non-authoritative export write and path traversal name rejection without broad export reshaping.

## AST/boundary lint parity
`rg` scans are discovery and evidence only. They are not blocking enforcement.

Blocking enforcement remains `scripts/check.sh`, the Rust boundary lint, the UI AST lint, compiler/type checks, clippy, and tests.

No lint behavior changed in Phase 89. No new source pattern was identified that requires same-phase boundary lint maintenance.

## Test fidelity
New behavior is covered by same-phase module-local tests in `core/src/api/observability.rs`.

Cross-boundary behavior is covered by root integration smoke tests in `tests/integration_smoke.rs` because the API is publicly reachable without export reshaping.

Test names describe the invariant being protected. No tests were skipped after final edits.

## Validation evidence
Validation evidence for Phase 89 includes:

- `./scripts/check.sh`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- Rust boundary lint self-tests and scan
- UI AST lint self-tests and scan
- explicit UI typecheck, lint, and build
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- export-write, path-safety, no-authority, source-guard, readiness, and lint-wiring rg evidence scans

## Confirmed vs suspected
Confirmed: Phase 89 writes already encoded export bundle bytes only.

Confirmed: Phase 89 rejects path traversal and overwrite attempts.

Confirmed: Phase 89 does not implement export import, ledger/audit append, recovery acceptance, replay repair, promotion, live telemetry, provider/model execution, action execution, async/network/process/thread behavior, or authority mutation.

Confirmed: Phase 90 remains responsible for roadmap and Production Candidate gap audit.

Suspected: none.

## Deferred items
Roadmap and Production Candidate gap audit work remains deferred to Phase 90.

Any future lint changes, if needed, remain deferred to an out-of-band maintenance phase.

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.
