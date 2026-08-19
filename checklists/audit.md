---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Audit checklist

This checklist defines bounded repository and harness audit steps for AJENTIC.

This document does not define governance rules or architecture authority.

Audits collect review evidence only. They do not approve releases, promote model output, mutate runtime state, or replace Rust-owned validation.

## Audit setup

- [ ] Confirm working tree state with `git status --short`.
- [ ] Confirm current branch and base commit.
- [ ] Confirm the audit scope and excluded surfaces.
- [ ] Confirm active phase and roadmap boundary.

## Boundary inspection

- [ ] Inspect Rust authority boundaries for unintended delegation.
- [ ] Inspect TypeScript surfaces for projection-only behavior.
- [ ] Inspect Python scripts for adaptation-only behavior.
- [ ] Inspect Bash scripts for glue-only behavior.
- [ ] Inspect stale scaffold or generator surfaces.

## Contract and release inspection

- [ ] Inspect schema/Rust/TypeScript contract drift evidence.
- [ ] Run operator-intent contract map validation when operator intent contracts are in scope.
- [ ] Inspect workflow release permissions and release-mechanic changes.
- [ ] Inspect package versions and package-publication state.
- [ ] Inspect remote tag/release state when release boundaries are in scope.

## Validation and reporting

- [ ] Run phase-required validation commands.
- [ ] Record pass/fail status for each validation command.
- [ ] Report changed files, or explicitly report no file changes for audit-only work.
- [ ] Record findings through the appropriate mutation path.
