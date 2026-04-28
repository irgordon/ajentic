# UI Display Specification

This file reserves the location for the AJENTIC UI display specification.

## Purpose

The TypeScript UI is a visibility and review surface. It displays Rust-generated governance state for human review.

## Non-authoritative role

- The UI must not compute authoritative promotion eligibility.
- The UI must not modify governance state.
- The UI must display data sourced from the Rust harness only.

## Planned display surfaces

- Candidate solution list with governance status
- Tier-1 output review queue
- Audit record viewer
- Domain and objective browser

## v0.0.0 status

This component is non-authoritative in v0.0.0. No dashboard is implemented. Implementation will be added in later phases.
