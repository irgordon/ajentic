---
truth_dimension: planned
authority_level: authoritative
mutation_path: roadmap_update
---
# Phase map

This document is the compact planned phase index.

It is planned truth only.

It does not record completed work.

Completed accepted work is historical truth in `CHANGELOG.md`.

Expanded planning detail for active phases is in `docs/roadmap/phases.md`.

Sequencing rationale is in `docs/roadmap/sequencing.md`.

## Phase index

| Phase | Title | Milestone group | One-sentence goal | Boundary note |
| --- | --- | --- | --- | --- |
| 80 | Roadmap and Changelog Alignment Check + Production Candidate Gap Audit | Milestone 5 | Assess production-candidate gaps after the Phase 71-79 block and align planning surfaces. | Alignment/audit/planning only; not release-candidate or production approval. |
| 81 | Local Harness Composition Hardening | Milestone 6 | Harden the bounded local harness composition with mismatch and negative-path coverage. | Hardening only; no new runtime authority. |
| 82 | Provider Evidence Replay and Failure Trace Boundary | Milestone 6 | Make provider execution and failure evidence replayable within local harness boundaries. | Replay is non-authoritative and non-mutating. |
| 83 | Durable Audit and Ledger Append Boundary | Milestone 6 | Define and constrain durable append eligibility for audit and ledger proof records. | Append eligibility is explicit and not state promotion or recovery. |
| 84 | Recovery Candidate Acceptance Boundary | Milestone 6 | Define explicit fail-closed acceptance of verified recovery candidates into in-memory state. | Acceptance is explicit under Rust-owned gates; no silent recovery. |
| 85 | Roadmap and Changelog Alignment Check | Milestone 6 | Reconcile 81-84 outcomes and planning/history posture before outward-facing surfaces. | Alignment only; no readiness approval. |
| 86 | User-Facing Local Workflow Documentation | Milestone 6 | Document supported local workflows, commands, failures, and operator expectations. | Documentation only; no runtime capability change. |
| 87 | Observability and Audit Export Boundary | Milestone 6 | Define read-only local export/report surfaces for diagnostics, audit, recovery, and execution decisions. | Export is read-only and non-authoritative. |
| 88 | Security and Abuse-Case Hardening Pass | Milestone 6 | Add negative-path hardening around transport abuse, spoofing, mismatch, corruption, drift, and injection seams. | Hardening only; no broad new capability. |
| 89 | Packaging and Local Startup Candidate Boundary | Milestone 6 | Define minimal local startup/package surfaces suitable for operator testing. | Usability boundary only; not production readiness. |
| 90 | Production Candidate Gap Audit and Readiness Decision Gate | Milestone 6 | Determine whether evidence supports a production-candidate path or further hardening. | Decision gate only; not automatic approval. |

## Boundary reminder

Roadmap entries are planned truth and boundaries only.

Historical completion remains in `CHANGELOG.md`.
