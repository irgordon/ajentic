---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 190 - v1 Release Acceleration and Functional Freeze

## Decision

Phase 190 moves AJENTIC from release-platform preparation into direct v1 release execution.

The remaining pre-v1 roadmap is compressed to Phases 190-194. New governance-only, audit-only, and validation-only phases are no longer being added before v1.

Existing authority boundaries remain intact: Rust owns runtime authority, UI remains a review surface, evidence is not approval, and release execution must be explicitly scoped.

## Compressed v1 Track

| Phase | Scope |
| --- | --- |
| 190 | v1 release acceleration plan and functional freeze. |
| 191 | v1 artifact, signing, and release workflow activation. |
| 192 | UI/UX Release Candidate hardening. |
| 193 | v1 Release Candidate publication. |
| 194 | v1 final functional acceptance and release execution. |

## v1 Must-Have Scope

- App builds cleanly.
- Rust core tests pass.
- UI builds cleanly.
- UI typecheck, lint, and API tests pass.
- Reproducible artifact and internal integrity evidence checks pass.
- Release artifact workflow exists or is prepared for Phase 191.
- MIT license remains present.
- README documents the real install, run, and open path.
- UI has a coherent first-run and review experience.
- UI clearly displays status, evidence, errors, and release-relevant information.
- Primary screens do not have obvious blank states, broken navigation, or misleading status labels.
- Package versions align with the chosen v1 strategy before release execution.

## v1 Deferrals

- Advanced governance expansion.
- New evidence systems beyond existing Phase 189 evidence.
- New audit-only milestones.
- New replay-only milestones.
- Cross-domain expansion unless required by current product behavior.
- Provider expansion unless required for the v1 product path.
- Broad compliance or security programs beyond release-blocking basics.
- Additional roadmap theory.
- Additional speculative architecture documents.

## Functional Acceptance Checks

- `npm run build`
- `npm run typecheck`
- `npm run lint`
- `npm run test:api`
- `cargo fmt --manifest-path core/Cargo.toml -- --check`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- `python3 scripts/reproducible_artifacts.py --check`
- `python3 scripts/reproducible_artifacts.py --check-evidence`
- `scripts/validate_structure.py`
- `scripts/validate_docs.py`
- `scripts/check_help_pages.py`
- `git diff --check`
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-190-target ./scripts/check.sh`

## UI/UX Acceptance Checks

- Open the UI locally.
- Confirm primary navigation works.
- Confirm main screens render without blank or error states.
- Confirm release and status language is understandable.
- Confirm evidence, artifact, and release-status surfaces are understandable.
- Confirm obvious empty placeholders are removed, filled, or labeled as post-v1.
- Confirm the first-run path is clear from README.
- Confirm no UI screen claims authority that belongs to Rust.
- Confirm no UI label implies production safety beyond the actual release scope.

## Non-Release Statement

Phase 190 creates no release, tag, GitHub Release, signing, publishing, installer, update-channel, public artifact, deployment path, or package version change.
