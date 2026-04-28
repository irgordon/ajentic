# Replay Rules

This file reserves the location for the AJENTIC replay rules documentation.

## Purpose

Replay reconstructs harness state from ledger records, enabling auditability, debugging, and bounded self-improvement.

## Rules

- Replay logic belongs exclusively to Rust core.
- Replay must produce the same governance results as the original run.
- Replay must not bypass governance or promotion rules.
- Python, TypeScript, and Bash components must not implement replay.

## v0.0.0 status

This file reserves the location for replay rules. Implementation will be added in later phases.
