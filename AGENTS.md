---
truth_dimension: navigation
authority_level: non_authoritative
mutation_path: agents_update
---

# AGENTS.md

This file is a short navigation contract for agents and tools working in this repository.

It is not the system of record.

It does not define governance rules, architecture authority, roadmap commitments, examples, or implementation details.

## Authoritative sources

| Source | Purpose |
| --- | --- |
| `docs/governance/GOVERNANCE.md` | Normative rules, authority model, prohibited patterns, and invariants. |
| `docs/governance/phase-execution-contract.md` | Normative phase execution loop contract and checklist/changelog boundaries. |
| `docs/architecture/ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/roadmap/phase-map.md` | Planned phase sequence and active phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts (JSON Schema). |

## Quick navigation

- **Rules and invariants**: `docs/governance/GOVERNANCE.md`, `docs/governance/phase-execution-contract.md`, `docs/governance/`
- **System structure**: `docs/architecture/ARCHITECTURE.md`, `docs/architecture/`
- **Roadmap and phases**: `docs/roadmap/phase-map.md`, `docs/roadmap/`
- **Active phase tasks**: `checklists/current-phase.md`
- **Data contracts**: `schemas/`
- **Runtime source**: `core/`
- **Browser UI source**: `ui/`
- **Operator scripts**: `scripts/`

## Release discipline reminders

These reminders point back to roadmap and governance authority; they are not standalone release authority.

- Do not create tags or GitHub Releases unless the active phase explicitly authorizes release execution.
- Do not add signing, publishing, deployment, installer, or update-channel behavior unless explicitly scoped.
- From Phase 190 onward, do not create new governance-only or audit-only phases before v1.
- Prioritize functional release blockers, artifact production, signing/release workflow, and UI/UX quality.
- Keep authority boundaries intact, but do not expand them into new documentation-only work.
- Validation workflows may produce evidence; they do not approve releases.
- Reproducible build workflows are validation evidence only.
- Do not convert internal candidate artifacts into public release artifacts.
- Do not treat artifact reproducibility as production readiness, public-release readiness, or v1.0 approval.
- Internal checksum, SBOM, and provenance evidence are review evidence only.
- Do not convert internal integrity evidence into release artifacts.
- Do not add GitHub artifact attestations before an explicitly scoped signing/attestation phase.
- Do not add `id-token: write` or `attestations: write` in Phase 189.
- Do not treat checksums, SBOMs, or provenance files as release approval.
- Do not add signing, publishing, deployment, installer, or update-channel behavior in Phase 189.
- Do not treat evidence generation as production readiness or v1.0 approval.
- Do not infer release readiness from passing tests, passing CI, or clean validation.
- Do not claim production readiness unless a roadmap decision gate approves it.
- Do not claim v1.0 readiness from package metadata.
- Do not infer licensing or add a license without an Owner-selected license decision.
- MIT is the selected project license.
- Do not change licensing or add dual licensing without explicit Owner instruction.
- Do not add third-party code unless its license is compatible and recorded.
- Do not infer release approval from license presence.
- Do not publish packages merely because package metadata says MIT.
- Do not normalize versions to `1.0.0` before the v1.0 decision gate.
- Do not introduce publishing, signing, deployment, installer, or update-channel mechanics in identity-alignment phases.
- Keep GitHub Actions release authority behind explicit release-platform phases.
- Keep workflow permissions least-privilege unless a later release-execution phase explicitly requires broader permissions.
- Keep release approval tied to explicit roadmap decision gates.
- Phase 193 RC publication uses the GitHub REST API, not the `gh` CLI.
- Phase 193 may create only the bounded `v1.0.0-rc.1` tag, prerelease, RC assets, and RC attestations after explicit confirmations.
- Do not treat RC publication as final v1.0 approval, production readiness, package publishing, installer approval, update-channel approval, or deployment approval.
- Phase 194 is the only final `v1.0.0` release execution phase.
- Do not create final tags outside the final-release workflow.
- Do not create final GitHub Releases outside the final-release workflow.
- Do not push all tags or force-push release tags.
- Verify the final tag before creating the final GitHub Release.
- Scope `contents: write` only to release publication jobs.
- Scope `id-token: write` and `attestations: write` only to attestation jobs.
- Do not carry RC-only exceptions into final release without explicit final-release Owner exception.
- Do not treat final release assets or attestations as governance authority.
- Do not call `gh release create`; use the GitHub API path when an explicitly scoped release phase authorizes publication.
- Do not create Git tags before the release phase explicitly authorizes tags.
- Do not add `contents: write` without explicit release-execution authorization.
- Attestation permissions are allowed only in explicitly scoped release-candidate attestation jobs.
- Do not add private signing keys without explicit Owner instruction.
- Do not treat attestation evidence as release approval.
- Do not treat a workflow artifact as a public distribution path.
- Do not add installer, update-channel, deployment, npm publish, Cargo publish, or registry publishing behavior.
- Keep browser UI primary copy in plain language for non-technical users.
- Keep raw technical statuses available in details or raw sections.
- Do not let TypeScript compute or claim release, promotion, or production authority.
- Do not hide blocked, failed, unknown, malformed, missing, or rejected states.
- Do not add external fonts, network font loading, or heavy UI dependencies without explicit scope.
- Do not convert read-only UI controls into release, publish, sign, deploy, or credential-changing actions.
- Post-v1, do not create new release tags without an explicit release phase.
- Post-v1, do not publish npm or Cargo packages without an explicit package-publication phase.
- Post-v1, do not create installers without an explicit installer phase.
- Post-v1, do not create update channels without an explicit update-channel phase.
- Post-v1, do not deploy without an explicit deployment phase.
- Post-v1, do not add OS signing or notarization without an explicit signing/notarization phase.
- Do not treat release assets, checksums, SBOM/provenance, attestations, UI, or scripts as governance authority.
- Preserve branch protection and ruleset settings unless the Owner explicitly changes them.
- Preserve Rust authority, TypeScript visibility, Python adaptation, and Bash glue boundaries.
- Do not bypass branch protection casually.
- Required checks must pass before any admin-bypass merge.
- Admin-bypass merges must be recorded when used.
- Solo-maintainer approval deadlocks must be documented rather than hidden.
- Do not use admin bypass to create releases, tags, package publications, installers, update channels, deployments, OS signing, notarization, or authority changes.

## Constraint reminder

Rust owns runtime authority.

TypeScript is a review and operator-intent surface only.

Bash scripts are operator wrappers only.

Model output is untrusted until validated through Rust-owned paths.

This file must remain short, stable, and non-authoritative.
