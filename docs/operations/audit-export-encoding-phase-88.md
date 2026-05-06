---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 88 - Audit Export Encoding Boundary

## Scope
Phase 88 implements deterministic audit export encoding only for Phase 87 observability snapshots.

Phase 88 does not write export files, read or write persistence, append ledger/audit records, accept recovery, repair replay, execute providers, execute actions, mutate authority, or expose live transport.

## Canonical encoding model
The encoder emits canonical UTF-8 bytes in a closed flat `key=value\n` contract.

The contract uses explicit `format_version`, explicit `record_kind`, fixed field order, LF line endings only, lowercase fixed status/reason codes, decimal integers, booleans as `true`/`false`, and explicit `none` for absent scalar fields in optional sections.

## Determinism rules
Phase 88 uses a small canonical encoder rather than generic serialization.

The encoder does not use JSON, YAML, TOML, arbitrary map traversal, timestamps, random identifiers, locale-dependent formatting, platform paths, or OS-specific line endings.

Vectors are encoded in stable input order. The export contract does not traverse maps.

## Panic-safety and bounded input rules
The encoder only accepts the closed, flat export contract derived from minimized Phase 87 `ObservabilitySnapshot` summaries.

The encoder does not recursively traverse arbitrary structures.

Explicit limits bound diagnostics count, individual field length, summary length, and total encoded byte length. Checked append helpers return typed rejection reasons for unsupported or over-large input.

## Format version and record kind
The initial format version is `audit-export-v1`.

The record kind is `observability-snapshot`.

Both fields are encoded first in the byte stream.

## Data minimization
Phase 88 encodes only minimized Phase 87 observability summary fields.

It does not encode raw provider payloads, raw ledger bytes, raw audit payload bytes, recovery candidate bytes, filesystem paths, environment data, or secret material.

Snapshots flagged as containing raw provider payloads or secret material are rejected before encoding.

## Relationship to Phase 87 observability snapshots
Phase 87 supplies read-only observability snapshots from caller-supplied evidence.

Phase 88 derives canonical bytes from those minimized snapshot summaries only and preserves their non-authoritative posture.

Unsupported current in-memory snapshot mode remains rejected for export encoding.

## Relationship to Phase 89 local export write
Phase 88 is not export write behavior.

Phase 88 does not write files and does not call persistence.

Phase 89 remains responsible for local export write behavior.

## Non-authority guarantees
Phase 88 does not mutate authority.

Phase 88 does not read or write persistence.

Phase 88 does not append ledger or audit records.

Phase 88 does not accept recovery candidates, repair replay, execute providers, execute actions, replace global state, or create live UI/Rust transport.

Successful envelopes explicitly report `writes_files=false`, `reads_persistence=false`, `writes_persistence=false`, and `mutates_authority=false`.

## Golden-style test coverage
Module-local tests include exact byte coverage for a minimal observability snapshot.

The golden-style test uses an inline byte string literal and verifies fixed field order, LF line endings, explicit absent optional fields, boolean spelling, deterministic repeated encoding, and version/record kind placement.

## Root integration-test coverage
Root integration smoke coverage exercises the public audit export encoding API through the core crate re-export.

The integration tests verify deterministic repeated encoding and non-authoritative envelope flags.

## AST/boundary lint parity
`rg` scans are discovery and evidence only.

Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

Phase 88 does not change lint behavior. No new lint self-test was required in this phase.

## Test fidelity
New Phase 88 behavior has module-local tests in the same phase.

Cross-boundary public API behavior has root integration smoke coverage.

No Phase 88 tests were skipped after final edits.

## Validation evidence
Validation evidence is recorded in `checklists/current-phase.md` and the final task report.

The required validation set includes repository checks, Rust tests, boundary lint commands, UI validation, CLI dry-run, encoding scans, determinism scans, data-minimization scans, source guards, readiness scan, and lint wiring scan.

## Confirmed vs suspected
### Confirmed
- Roadmap files identify Phase 88 as Audit Export Encoding Boundary.
- Phase 88 implements canonical byte encoding only.
- The encoder does not write export files.
- The encoder does not read or write persistence.
- The encoder does not append ledger/audit records.
- The encoder does not mutate authority.
- The encoder rejects raw provider payload and secret-material flags.

### Suspected
- Later phases may identify additional source patterns that should be promoted into a boundary lint in an out-of-band maintenance phase.

## Deferred items
| Item | Owner phase | Reason |
| --- | --- | --- |
| Local export write behavior | Phase 89 | Phase 88 is encoding only. |
| Boundary lint changes for future prohibited patterns | Out-of-band maintenance phase | Phase 88 does not change lint behavior. |

## Non-readiness statement
Phase 88 does not claim public usability, production readiness, Production Candidate approval, or release-candidate readiness.

Phase 88 implements encoding only and does not make the project externally usable or releasable.
