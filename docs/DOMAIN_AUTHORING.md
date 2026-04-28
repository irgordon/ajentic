# Domain Authoring

Phase 1 defines domain contract fields only.

## Domain contract fields

- `id`
- `name`
- `objective_types`
- `constraint_types`
- `evaluators`
- `known_failure_modes`
- `promotion_thresholds`

## Authoring notes

- Domain files may configure evaluator lists and thresholds.
- Domain files cannot alter lifecycle authority or promotion authority.
- Runtime behavior is not implemented in Phase 1.
