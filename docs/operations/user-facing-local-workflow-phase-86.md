---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 86 - User-Facing Local Workflow Documentation

## Scope
Phase 86 is documentation only.

Phase 86 documents existing local workflows only.

Phase 86 does not add runtime capability.

Phase 86 does not add observability/export/startup/packaging behavior.

Phase 86 does not approve Production Candidate status.

Phase 87 remains responsible for read-only observability snapshot boundary.

## Documentation surfaces
- `docs/operations/local-workflow-guide.md`
- `docs/operations/user-facing-local-workflow-phase-86.md`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Operator vocabulary decisions
Phase 86 standardizes operator-facing wording only:
- Validation: checks repository integrity.
- Dry run: reports current non-executing posture and performs no provider/persistence/action behavior.
- Append: durable evidence transaction, not promotion or recovery.
- Recovery acceptance: explicit in-memory acceptance, not global state replacement.
- Replay: evidence verification, not live execution.
- Export: operator-readable copy, not authoritative ledger state.
- Production Candidate: not approved.

## Existing command coverage
Documented command surfaces are current and local:
- `./scripts/check.sh`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- boundary lint and lint self-tests
- UI type/lint/build checks

## Failure-mode documentation
The local workflow guide documents failure behavior for validation, dry-run, integration tests, append boundary checks, recovery acceptance checks, replay checks, and UI contract checks.

## Authority wording controls
Wording is constrained to non-authoritative posture:
- append is evidence
- recovery acceptance is in-memory only
- replay is evidence verification only
- dry-run is non-executing
- UI contracts are non-authoritative

## Future-surface wording controls
Documentation states future surfaces are not currently implemented:
- read-only observability snapshot is Phase 87 scope
- export encoding/write is later scope
- startup/packaging are later scope

## AST/boundary lint parity
Do not rely on rg scans as enforcement.

rg scans are discovery/evidence only.

Blocking enforcement must come from scripts/check.sh, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

If a phase adds a new source pattern that should be prohibited later, update the relevant boundary lint in an out-of-band maintenance phase before proceeding.

If lint behavior changes, add or update lint self-tests in the same maintenance phase.

## Test fidelity
New behavior requires tests in the same phase.

Cross-boundary behavior requires root integration coverage unless not publicly reachable; if deferred, document why.

Test names must describe the invariant being protected.

No phase is complete if tests were skipped after final edits.

Phase 86 is documentation-only, so no new Rust/TypeScript tests are required unless source behavior is changed, which this phase must avoid.

Even though no new tests are expected, the full existing test/lint/check suite must pass after final edits.

## Validation evidence
Validation includes `./scripts/check.sh`, explicit cargo tests, boundary lint commands, UI type/lint/build, dry-run CLI, and required wording/guard scans.

## Confirmed vs suspected
- Confirmed: documentation now reflects existing local workflow behavior and boundaries.
- Suspected: none.

## Deferred items
- Phase 87 read-only observability snapshot boundary.
- Phase 88 export encoding boundary.
- Phase 89 local export write boundary.

## Non-readiness statement
Phase 86 does not approve release-candidate readiness, production readiness, or public usability.
