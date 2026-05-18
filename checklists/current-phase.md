---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 173

## Phase name
Phase 173 - Checksum and Provenance Evidence for Dry Package.

## Phase goal
Produce local-only, non-public, non-authoritative checksum and provenance evidence for the Phase 172 dry package. Checksum and provenance evidence proves what the dry package contains; it does not sign, publish, release, deploy, or approve the dry package.

## Working-tree hygiene gate
- [x] Work only on allowed Phase 173 surfaces.
- [x] Do not modify roadmap, governance, architecture, release workflow, provider execution, action execution, installer, update-channel, signing, publishing, deployment, or approval behavior.

## Allowed surfaces
- [x] `core/src/api/release_dry_package_checksum_provenance.rs`.
- [x] Thin exports/integration in `core/src/api/mod.rs` and `core/src/api/local_operator_shell_state.rs`.
- [x] UI source under `ui/src/**`.
- [x] `CHANGELOG.md` and this checklist.

## Code-production deliverable checklist
- [x] Rust-owned checksum evidence.
- [x] Rust-owned provenance evidence.
- [x] Local shell checksum/provenance projection.
- [x] Visible UI checksum/provenance panel.

## Dedicated module checklist
- [x] Keep checksum/provenance derivation, validation, serialization, parsing, write/read, and read-back validation in `release_dry_package_checksum_provenance.rs`.
- [x] Keep local shell integration thin.

## Thin shell integration checklist
- [x] Add checksum/provenance projection field to local shell state.
- [x] Initialize as `not_generated` until valid dry package evidence is available.
- [x] Do not add default persistence, automatic save, background persistence, signing, publishing, deployment, installer, update-channel, provider execution, action execution, replay repair, or recovery promotion.

## Checksum evidence checklist
- [x] Deterministic checksum value derived from Phase 172 dry package content.
- [x] Checksum algorithm marker is `deterministic_fnv64_dry_package_payload`.
- [x] Checksum classification is `checksum_evidence_only`.

## Provenance evidence checklist
- [x] Deterministic provenance ID.
- [x] Deterministic provenance content digest.
- [x] Provenance classification is `provenance_evidence_only`.
- [x] Production classification is `non_production`.
- [x] Distribution classification is `local_only_non_public`.
- [x] Authority classification is `non_authoritative_evidence`.
- [x] Release classification is `release_not_approved`.

## Dry package linkage checklist
- [x] Link evidence to dry package ID.
- [x] Link evidence to dry package digest.
- [x] Link evidence to dry package included evidence summary/count.

## Phase 171 preparation linkage checklist
- [x] Link provenance to Phase 171 preparation ID.
- [x] Link provenance to Phase 171 preparation status.
- [x] Reject missing or inconsistent preparation linkage.

## Validation/fail-closed checklist
- [x] Reject missing dry package.
- [x] Reject not-projected, rejected, malformed, digest-mismatched, or ID-mismatched dry package input.
- [x] Reject readiness, release, deployment, public-use, production-use, signing, publishing, installer, update-channel, public-download, GitHub-release, release-tag, provider-trust, action-authorization, replay-repair, and recovery-promotion claims.
- [x] Reject malformed checksum/provenance evidence and deterministic digest mismatch.

## Explicit write/read helper checklist
- [x] Write helper requires caller-provided path.
- [x] Write helper validates evidence before writing.
- [x] Read helper requires caller-provided path.
- [x] Tests use temp directories only.

## Read-back validation checklist
- [x] Parse checksum/provenance evidence deterministically.
- [x] Validate checksum/provenance structure after read.
- [x] Report read-back validation as structure-only validation.

## UI checksum/provenance panel checklist
- [x] Render panel labeled `Checksum and provenance evidence`.
- [x] Render evidence status, dry package ID, checksum value, checksum algorithm, provenance ID, classifications, linkage summary, validation status, read-back validation status, rejection reason, and boundary markers.
- [x] Render required no-signing/no-release/no-public-distribution wording.
- [x] Expose no publish, sign, deploy, release, installer, update-channel, public-download, GitHub-release, release-tag, approval, or public-distribution controls.

## No-signing/no-release boundary checklist
- [x] State checksum/provenance evidence only.
- [x] State no signing.
- [x] State no publishing.
- [x] State no release artifact.
- [x] State no public distribution.
- [x] State no release readiness or Release Candidate approval.

## Rust test checklist
- [x] Deterministic checksum and serialization.
- [x] Changed dry package changes checksum.
- [x] Missing/invalid/malformed dry package rejection.
- [x] Phase 171 and Phase 172 provenance linkage.
- [x] Explicit write/read and read-back validation.
- [x] Malformed read-back and digest mismatch rejection.
- [x] No-authority boundaries.

## TypeScript test checklist
- [x] Visible checksum/provenance panel.
- [x] Dry package ID and checksum value.
- [x] Provenance linkage summary.
- [x] Read-back status.
- [x] Deterministic rendering.
- [x] Forbidden-label absence.

## Phase 174 handoff checklist
- [x] Phase 174 remains the next code-production phase for installer and distribution contract surface.

## Validation checklist
- [x] Run full check script.
- [x] Run direct Rust tests if needed.
- [x] Run direct UI typecheck, lint, build, test:api, and dev smoke if needed.
- [x] Run diff, status, checksum/provenance, dedicated-module, boundary, forbidden-label, filesystem, unsafe-execution, release/deployment, changed-file, and no-roadmap-drift scans.

## Deferred items
- [x] Signing, publishing, release artifacts, public artifacts, installers, update channels, public downloads, GitHub releases, release tags, deployment, readiness approval, Release Candidate approval, Production Candidate approval, public-use approval, production-use approval, provider trust, action authorization, replay repair, and recovery promotion remain deferred.

## Validation log
- [x] Record final command outcomes in the agent final response.

## Zero-drift checklist
- [x] Roadmap files not modified.
- [x] Governance and architecture files not modified.
- [x] No long governance report, readiness ladder, release matrix, or artifact sequencing table added.
