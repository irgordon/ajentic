---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local startup boundary - Phase 57

## Scope

This advisory document defines AJENTIC's current safe local startup and packaging boundary as command-surface documentation only.

It does not define governance, architecture, roadmap authority, or changelog history.

## Supported local commands

Current supported safe local command surface:

- `./scripts/check.sh`
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- `cd ui && npm run typecheck && npm run lint && npm run build`
- `node scripts/test_lint_ui_boundaries.mjs`
- `node scripts/lint_ui_boundaries.mjs`

## Current startup boundary

Startup is bounded to local validation and deterministic dry-run summary behavior.

No packaging or startup mechanism beyond direct local commands is introduced in this phase.

## Explicit non-capabilities

Current non-capabilities include:

- no real provider/model call
- no persistence
- no file/workspace scanning
- no API server
- no UI-to-Rust transport
- no operator action execution
- no release-candidate readiness
- no production readiness

## Dry-run posture

Phase 46 dry-run posture is preserved:

- deterministic and in-memory
- no provider/model call
- no persistence
- no service startup

## API decomposition carry-forward

Phase 56/56.5 API decomposition remains intact.

`core/src/api/mod.rs` remains a module/re-export compatibility surface only.

## Validation commands

Validation for this boundary is performed with:

- `./scripts/check.sh`
- `cd ui && npm run typecheck && npm run lint && npm run build`
- `node scripts/test_lint_ui_boundaries.mjs`
- `node scripts/lint_ui_boundaries.mjs`
- `cargo run --manifest-path core/Cargo.toml -- dry-run`

## Release-candidate impact

This phase does not claim release-candidate readiness.

This phase does not introduce release workflow or packaging workflow behavior.

## Deferred items

Deferred beyond this phase:

- packaging/installer implementation
- service/server startup behavior
- live provider execution
- persistence implementation
- UI/Rust live transport

## Non-readiness statement

AJENTIC remains non-release-candidate and non-production in this phase.
