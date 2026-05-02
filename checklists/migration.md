---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Migration checklist

This checklist defines migration steps for schema or data changes.

This document does not define governance rules or architecture authority.

This checklist is a placeholder for Phase 0. Migration steps will be defined when contracts change.

## Pre-migration

- [ ] Migration scope identified
- [ ] Schema version bump applied
- [ ] Migration tests written
- [ ] Replay compatibility verified

## Migration steps

- [ ] Apply schema change
- [ ] Run data migration if required
- [ ] Validate migrated data

## Post-migration

- [ ] CHANGELOG.md updated
- [ ] Old schema version archived if applicable
