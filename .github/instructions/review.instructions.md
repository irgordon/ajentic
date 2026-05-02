---
applyTo: "**/*"
---

# Review instructions

This file defines review posture for repository changes.

It is not governance.

## Review checklist

A change should be rejected or revised when it:

- mixes artifact truth dimensions
- adds runtime authority outside Rust
- adds UI mutation authority
- adds Bash policy
- adds schema files outside `schemas/`
- adds Markdown documentation inside `memory/`
- records future plans in `CHANGELOG.md`
- records completed work in roadmap files
- creates a new top-level directory outside the canonical structure
- expands an instruction file into the system of record

## Required review output

When reviewing or completing a task, report:

1. Summary
2. Files created or changed
3. Commands run
4. Test or validation output
5. Deviations or notes

Do not claim validation passed unless the command was run or the reason it could not be run is stated.
