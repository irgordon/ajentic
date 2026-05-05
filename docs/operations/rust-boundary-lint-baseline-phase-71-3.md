---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 71.3 - Rust Boundary Lint Baseline

## Scope
Phase 71.3 is an out-of-band fix that adds enforcement tooling only.

Phase 71.3 out-of-band fix does not refactor Rust code.

Phase 71.3 out-of-band fix does not implement provider retry/failure logic.

## Why enforcement is needed now
Phase 71 expanded `core/src/execution/mod.rs`, and Phase 65 already recorded god-file risk. Enforcement must be stronger than advisory grep scans before Phase 72 adds provider failure/timeout/retry behavior.

## Boundary rules enforced
- `core/src/api/mod.rs` remains module/re-export/comment oriented.
- `execute_local_persistence_plan` and `verify_persisted_record_paths` are allowed only in `core/src/api/persistence.rs`.
- Filesystem tokens (`std::fs`, `File::`, `read_to_string`, `read_dir`, `write(`, `write!`, `writeln!`, `rename`, `sync_all`, `flush`) are limited to `core/src/api/persistence.rs`.
- Network/async/process/thread tokens are blocked in Rust source unless allowlisted.
- `core/src/execution/mod.rs` blocks newly prohibited external-boundary tokens.

## Warning-only grandfathered provider execution state
`ProviderExecution` in `core/src/execution/mod.rs` is warning-only until out-of-band fix Phase 71.5.

ProviderExecution in execution.rs is warning-only until out-of-band fix Phase 71.5.

Future enforcement can become stricter after extraction.

## Self-test coverage
`node scripts/test_rust_boundary_lint.mjs` covers pass/fail paths for API-mod restrictions, persistence-only symbols, forbidden token detection, warning-only ProviderExecution handling, IDE-friendly diagnostic format, and multi-violation reporting.

## CI/local validation wiring
- Local: `scripts/check.sh` now runs `node scripts/test_rust_boundary_lint.mjs` before `node scripts/rust_boundary_lint.mjs`.
- CI: Rust boundary lint is covered by local validation workflow usage; no separate CI command duplication was required this phase.

## Deferred AST/compiler-aware work
Current enforcement is deterministic token and path ownership checks without Rust AST parsing.

Compiler-aware semantic enforcement is deferred.

## Relationship to Phase 71.5
Phase 71.5 is the out-of-band structural extraction phase for provider execution decomposition from `core/src/execution/mod.rs`.

Phase 71.3 intentionally records this as deferred and warning-only.

## Validation evidence
Validation includes:
- `node scripts/test_rust_boundary_lint.mjs`
- `node scripts/rust_boundary_lint.mjs`
- `./scripts/check.sh`
- UI and dry-run checks plus wiring scans and source-diff guards.

## Confirmed vs suspected
Confirmed:
- Enforcement tooling is added and wired.
- Rust behavior is unchanged.
- ProviderExecution remains warning-only with explicit Phase 71.5 ownership.

Suspected:
- Additional allowlist refinements may be needed once Phase 71.5 extraction lands.

## Non-readiness statement
No readiness/public usability claim is made.
