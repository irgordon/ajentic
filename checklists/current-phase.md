---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 169 - Local Beta Hardening Pass and User Help Surface

## Phase goal
- [x] Add visible local operator help and plain-English local HTML help pages for the local beta workflow.

## Working-tree hygiene gate
- [x] Keep changes within Phase 169 allowed surfaces.
- [x] Do not modify roadmap, governance, architecture, release, installer, deployment, or publishing files.

## Allowed surfaces
- [x] `ui/src/**`
- [x] `ui/help/**`
- [x] `ui/package.json`
- [x] `scripts/check_help_pages.py`
- [x] `scripts/check.sh`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] UI renders a visible Help entry point.
- [x] Local dev/build scripts serve local help pages from `ui/help`.
- [x] Deterministic help validation checks required files, links, phrases, and no forbidden labels.

## User help surface checklist
- [x] Help index exists.
- [x] Getting started help page exists.
- [x] Local workflow help page exists.
- [x] Provider setup help page exists.
- [x] Validation/review/candidates help page exists.
- [x] Trial package/evidence help page exists.
- [x] Restore/verification help page exists.
- [x] Errors/stop-conditions/escalation help page exists.
- [x] Glossary help page exists.
- [x] Safety boundaries help page exists.

## Help page content checklist
- [x] Help pages explain what AJENTIC is for.
- [x] Help pages explain the local workflow.
- [x] Help pages explain provider setup and constrained invocation.
- [x] Help pages explain validation, review, staged proposals, operator decisions, and local candidate materialization.
- [x] Help pages explain trial packages, runbooks, evidence, and trial execution harness status.
- [x] Help pages explain restore, read-back validation, replay/status, and verification.
- [x] Help pages explain blockers, errors, mismatch drilldowns, stop conditions, and escalation guidance.
- [x] Help pages explain boundaries and what the system does not approve or automate.

## Plain-English documentation checklist
- [x] Help pages use short direct wording for a non-engineering operator.
- [x] Help pages start from concrete operator tasks.
- [x] Help pages explain what each feature does, protects, prevents, or connects.
- [x] Help pages avoid phase-history exposition and release/readiness claims.

## UI help entry point checklist
- [x] UI includes a visible `Local help` panel.
- [x] UI links to `help/index.html`.
- [x] UI states help is explanatory only.
- [x] UI states help is local-only and non-production.
- [x] UI states the help entry point does not change runtime state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.

## Help validation checklist
- [x] Help index exists.
- [x] All required help pages exist.
- [x] Help index links to all required help pages.
- [x] Required user-facing phrases are present.
- [x] Forbidden labels are rejected from help pages.
- [x] Approval/readiness claim patterns are rejected from help pages.

## Local beta hardening checklist
- [x] Add a direct product-facing help link for operators.
- [x] Clarify no-authority help wording at the local shell entry point.
- [x] Keep hardening small and product-facing.

## No-authority checklist
- [x] No controlled-human-use approval.
- [x] No readiness approval.
- [x] No release approval.
- [x] No deployment approval.
- [x] No public/general-use approval.
- [x] No production-use approval.
- [x] No provider output trust.
- [x] No action authorization.
- [x] No trial execution expansion.
- [x] No provider execution expansion.
- [x] No network/cloud behavior.
- [x] No background service or daemon behavior.
- [x] No automated remediation, escalation, or stop-condition enforcement.
- [x] No replay repair.
- [x] No recovery promotion.
- [x] No release artifact creation, signing, publishing, deployment, installer, or update-channel behavior.

## TypeScript test checklist
- [x] Behavior test covers visible help entry point.
- [x] Behavior test covers help index link display.
- [x] Behavior test covers explanatory/local-only/non-production wording.
- [x] Behavior test covers no-mutation and no-authority help wording.
- [x] Repeated render remains deterministic through existing shell snapshot behavior.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-169-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `cd ui && timeout 10 npm run dev`
- [x] Help page scan.
- [x] Help entry point scan.
- [x] Plain-English boundary scan.
- [x] Forbidden label scan.
- [x] Unsafe execution scan.
- [x] Release/deployment scan.
- [x] Changed-file guard.
- [x] No-roadmap-drift guard.

## Phase 170 handoff checklist
- [x] Phase 170 remains the next production-path alignment checkpoint.
- [x] Phase 170 must verify user-facing help exists.

## Deferred items
- [x] No release, deployment, installer, signing, publishing, cloud, network, production persistence, automated stop-condition enforcement, automated escalation, replay repair, or recovery promotion work is included in Phase 169.

## Validation log
- [x] Full local validation passed after final edits.
- [x] Generated UI build artifacts were cleaned.
- [x] Local dev smoke started the local browser URL and was stopped by timeout.

## Zero-drift checklist
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches the Phase 169 diff.
- [x] Staged files match allowed Phase 169 surfaces.
- [x] Help pages remain explanatory, local-only, non-production, and non-authoritative.
