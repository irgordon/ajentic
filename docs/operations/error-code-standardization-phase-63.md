---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Phase 63 - Error-Code Family and Reporting Standardization

## Scope
Phase 63 adds family/context diagnostics around existing stable error codes for reporting surfaces.

## Error-code ambiguity problem
Multiple enums can legitimately emit identical `code()` strings, which is ambiguous if treated as globally unique.

## Diagnostic family model
`DiagnosticFamily` classifies source context and `ErrorDiagnostic` stores `{family, code, summary}`.

## Existing code string preservation
All existing `code()` values remain unchanged in this phase.

## Duplicate-code handling
Duplicate code strings are allowed when scoped by family. `diagnostic_key(...)` combines family label + code for display disambiguation.

## Summary text constraints
Summaries are short, stable, and non-sensitive. They do not include paths, payload bytes, secrets, operator text, or provider output.

## API-side coverage
Phase 63 includes diagnostics for operator intent, integration, runtime config, read projection, application state, persistence validation/execution/recovery, and local workflow.

## Execution-side coverage
Execution-owned families are not wired in this phase.

## Deferred work
Execution module mappings for provider/promotion/controlled-run/decision families are deferred to a follow-up phase.

## Validation evidence
Phase checks include repository checks, UI type/lint/build checks, dry-run, and static scans for diagnostics/code-preservation/readiness language.

## Confirmed vs suspected
Confirmed: family-key disambiguation is deterministic and preserves source `code()` values. Suspected: no consumer behavior changes because mappings are additive and unused by runtime flow.

## Non-readiness statement
Phase 63 does not implement UI/CLI reporting. Phase 63 does not claim release-candidate readiness, production readiness, or public usability.
