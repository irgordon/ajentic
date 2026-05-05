---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Audit Export Encoding Boundary - Phase 88

## Scope
Phase 88 implements deterministic audit export byte encoding for minimized Phase 87 observability snapshots.

Phase 88 implements encoding only. Encoding is not export write, persistence, ledger append, recovery, telemetry, or authority.

Phase 88 does not write export files. Phase 88 does not read or write persistence. Phase 88 does not append ledger/audit records. Phase 88 does not mutate authority.

## Canonical encoding model
The encoder emits a closed line-oriented byte contract with `key=value\n` records.

The contract uses:

- explicit `format_version`
- explicit `record_kind`
- fixed field order
- UTF-8 string bytes
- LF line endings only
- lowercase fixed enum, reason, and status codes
- decimal integers only
- booleans as `true` or `false`
- absent optional scalar fields as `none`
- diagnostics in stable input order

The encoder does not use generic serialization, JSON, YAML, TOML, map traversal, platform paths, timestamps, random identifiers, locale-dependent formatting, or OS-specific line endings.

## Determinism rules
The canonical field order is fixed in `core/src/api/observability.rs` and starts with `format_version`, `record_kind`, `export_id`, `snapshot_id`, `snapshot_mode`, `snapshot_status`, and `snapshot_reason`.

Optional sections always encode a `.present` line. When an optional section is absent, scalar fields in that section encode as `none` and boolean fields encode as `false`.

Repeated encoding of the same snapshot and export identifier returns identical byte arrays.

## Panic-safety and bounded input rules
The encoder traverses only the flat `ObservabilitySnapshot` contract produced by Phase 87 snapshot summaries.

Inputs are bounded by `AuditExportEncodingLimits`:

- `max_diagnostics`
- `max_field_len`
- `max_summary_len`
- `max_total_bytes`

Checked append helpers reject over-large fields or encoded buffers with typed `AuditExportEncodingReason` values rather than panicking on expected validation failures.

The encoder does not recursively traverse arbitrary structures.

## Format version and record kind
Phase 88 defines the format version from day one:

- `AUDIT_EXPORT_FORMAT_VERSION = "audit-export-v1"`
- `AUDIT_EXPORT_RECORD_KIND = "observability-snapshot"`

Both values are encoded in the first two canonical lines.

## Data minimization
Phase 88 encodes only minimized Phase 87 observability snapshot summaries.

Phase 88 does not expose raw provider payloads or secret material. It also does not encode raw ledger bytes, raw audit payload bytes, recovery candidate bytes, filesystem paths, environment data, or secrets.

Snapshots flagged as containing raw provider payloads or secret material are rejected before encoding.

## Relationship to Phase 87 observability snapshots
Phase 87 defines read-only, supplied-evidence observability snapshots. Phase 88 consumes those minimized snapshots and converts the closed snapshot contract into canonical bytes.

Phase 88 does not create new observations, read current runtime state, read persistence, or recompute authority.

## Relationship to Phase 89 local export write
Phase 89 remains responsible for local export write behavior.

Phase 88 returns an in-memory `AuditExportEnvelope` and sets `writes_files=false`, `reads_persistence=false`, `writes_persistence=false`, and `mutates_authority=false`.

## Non-authority guarantees
Phase 88 does not write files, read persistence, write persistence, append durable records, append ledger/audit records, accept recovery, replace global state, repair replay, execute providers, execute actions, or use live UI/Rust transport.

Authority-mutating snapshot flags reject encoding with `authority_mutation_not_allowed`.

## Golden-style test coverage
Module-local tests include exact inline byte coverage for a minimal snapshot through `audit_export_golden_minimal_snapshot_matches_expected_bytes`.

Additional tests cover reason codes, strict limits, empty export identifiers, unsupported modes, unbuilt snapshots, raw payload and secret flags, authority mutation, diagnostic count, field size, summary size, total byte limits, fixed field order, LF-only line endings, absent optional sections, boolean spelling, deterministic repeated encoding, and non-authority flags.

## Root integration-test coverage
The audit export encoding API is publicly reachable through the existing core API re-export surface.

Root integration smoke coverage includes:

- `root_integration_audit_export_encoding_is_deterministic`
- `root_integration_audit_export_encoding_is_non_authoritative`

No broad export reshaping was required.

## AST/boundary lint parity
rg scans are discovery and evidence only. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

Phase 88 does not change lint behavior. No lint self-tests were changed.

If a future phase adds a new source pattern that should be prohibited, the relevant boundary lint should be updated in an out-of-band maintenance phase before proceeding.

## Test fidelity
New encoding behavior is covered in the same phase by module-local tests and root integration smoke tests.

Test names describe the invariant being protected. No tests were skipped after final edits.

## Validation evidence
Validation evidence for Phase 88 includes:

- `./scripts/check.sh`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- Rust boundary lint self-tests and scan
- UI AST lint self-tests and scan
- explicit UI typecheck, lint, and build
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- encoding, determinism, no-generic-serialization, no-authority, data-minimization, source-guard, readiness, and lint-wiring rg evidence scans

## Confirmed vs suspected
Confirmed: Phase 88 implements canonical byte encoding only for minimized Phase 87 observability snapshots.

Confirmed: Phase 88 does not write export files, read or write persistence, append ledger/audit records, mutate authority, expose raw provider payloads, or expose secret material.

Suspected: none.

## Deferred items
Local export file write behavior remains deferred to Phase 89.

Future boundary lint changes, if needed, remain deferred to an out-of-band maintenance phase.

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.
