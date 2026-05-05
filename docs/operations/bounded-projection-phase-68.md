---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Bounded Projection Boundary - Phase 68

## Scope
Phase 68 defines a Rust-owned bounded read projection slicing model with deterministic request validation and deterministic in-memory slice derivation.

## Rust-owned slicing model
Projection slicing is Rust-owned.

Typed Rust request/result surfaces define the bounded slicing contract for all current callers and future transport layers.

## Supported surfaces
The model supports general projection surfaces:
- Ledger
- Replay
- Audit
- Memory
- Context
- Output
- Intent

Authorization/audit is one use case, not the whole projection surface.

## Request bounds
Requests enforce deterministic bounds:
- Non-empty request id
- Known projection surface
- Non-zero limit
- Limit capped by `MAX_PROJECTION_SLICE_LIMIT`
- Offset must be within caller-supplied in-memory item range

## Summary/detail modes
The model exposes summary/detail mode selection as typed request metadata.

Phase 68 does not add UI-side behavior for mode-specific transport, caching, or pagination.

## Read-only invariant
A bounded projection slice is a read-only derived view.

Phase 68 does not mutate state, fetch data, read persistence files, call providers/models, repair replay, append ledgers, or execute intents/actions.

## Non-authority behavior
Item text content is treated as untrusted display data.

Slice status/reason/metadata are driven only by typed request bounds and deterministic item-count math, not by inferred authority from item contents.

## Relationship to future UI/Rust transport
Future UI/Rust transport must consume Rust-owned bounded projection semantics.

UI must not invent client-side authority, pagination, caching, filtering, or delta semantics before Rust-owned boundaries define them.

## Deferred TypeScript mirror work
Phase 68 does not add TypeScript mirror updates, transport wiring, or UI runtime pagination/caching behavior.

## Validation evidence
Validation includes repository checks, UI checks, dry-run checks, bounded projection scans, no-side-effect scans, and guard scans.

## Confirmed vs suspected
### Confirmed
- Projection slicing is Rust-owned and read-only.
- Deterministic limit/offset validation is enforced.
- Bounded slices are computed from caller-supplied in-memory items only.
- No provider/model/persistence/replay-repair/action-execution behavior was added.

### Suspected
- Future phases may add transport adapters that consume this Rust-owned slicing boundary.

## Non-readiness statement
Phase 68 does not add UI transport, UI caching, UI pagination, TypeScript mirror updates, persistence reads, provider execution, replay repair, ledger append, or action execution.

Phase 68 does not claim release-candidate readiness, production readiness, or public usability.
