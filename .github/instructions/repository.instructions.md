---
applyTo: "**/*"
---

# Repository instructions

This file provides repository navigation and execution posture for agentic coding tools.

It is not governance.

It is not architecture.

It is not a roadmap.

Authoritative rules live in `docs/governance/` and, after Phase 0, in `GOVERNANCE.md`.

## Operating posture

Work in the smallest correct surface.

Do not widen scope without instruction.

Do not create new document categories.

Do not move rules into README, examples, scripts, workflows, memory, or UI.

Do not treat generated or model-produced output as trusted.

## Task discipline

For each change:

1. Identify the artifact type.
2. Identify the allowed directory.
3. Modify only the required surface.
4. Run the relevant validation.
5. Report files changed, commands run, validation results, and deviations.

## Boundary defaults

Rust owns runtime authority.

TypeScript UI is a review and operator-intent surface.

Bash scripts are wrappers.

GitHub workflows are enforcement wiring.

Schemas define contract truth.

Memory stores governed data only.

README is human orientation only.

Examples are non-authoritative.

## Prohibited defaults

Do not add policy to Bash.

Do not add authority to the UI.

Do not add schemas outside `schemas/`.

Do not add memory under `docs/`.

Do not put future plans in the changelog.

Do not record completed work in the roadmap.

Do not expand `AGENTS.md` into a monolithic instruction file.
