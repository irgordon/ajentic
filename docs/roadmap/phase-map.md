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
| 60 | Roadmap and Changelog Alignment Check + Production-Path Expansion | Milestone 5 | Reconcile roadmap/changelog posture and expand post-59 plan. | Alignment/planning only; no runtime capability claims. |
| 61 | Data Durability and Atomic Persistence Implementation | Milestone 5 | Implement explicit typed local atomic persistence write boundary. | Persistence remains explicit/typed; no automatic or hidden writes. |
| 62 | Persistence Recovery and Corruption Detection | Milestone 5 | Add deterministic persisted-record verification and corruption detection reporting. | Verification is read-only and fail-closed; no auto-repair. |
| 63 | Error-Code Family and Reporting Standardization | Milestone 5 | Standardize family/context diagnostics while preserving stable code strings. | No execution behavior changes from diagnostic standardization alone. |
| 64 | Rust/TypeScript Contract Synchronization Boundary | Milestone 5 | Align TypeScript mirror contracts to Rust-owned semantics. | Mirrors are non-authoritative and non-transport runtime surfaces. |
| 65 | Roadmap and Changelog Alignment Check | Milestone 5 | Reconcile 61-64 planning/history boundaries and assess carry-forward risk. | Alignment/audit only; no runtime implementation. |
| 66 | Identity-Bound Operator Intent Authorization | Milestone 5 | Add deterministic Rust-owned intent authorization decisions. | Authorization is not execution and does not persist state. |
| 67 | Operator Intent Audit Record Boundary | Milestone 5 | Build typed audit-eligible proof-object record construction boundary. | Record construction is not append/persistence/execution. |
| 68 | Bounded Read Projection Slices | Milestone 5 | Define Rust-owned bounded projection slice request/result boundaries. | Read-only projection only; no side effects or authority transfer. |
| 69 | Async Provider Transport Boundary | Milestone 5 | Define typed async-origin provider transport ingress sequencing boundary. | Ingress validation only; provider payload remains untrusted. |
| 70 | Roadmap Documentation Realignment and Production Candidate Gap Audit | Milestone 5 | Realign roadmap planning surfaces and audit production-candidate gaps. | Documentation/governance hygiene only; no runtime behavior work. |
| 71 | Provider Execution Adapter Implementation | Milestone 5 | Implement real local/provider execution path behind Phase 69 transport boundary. | Provider output remains untrusted; no direct promotion/persistence/ledger authority. |
| 72 | Provider Failure, Timeout, and Retry Boundary | Milestone 5 | Add deterministic timeout/cancel/malformed-output/retry classification handling. | Retries do not mutate authoritative state unless Rust-owned sequencing records it. |
| 73 | Durable Ledger Persistence Lifecycle | Milestone 5 | Define and implement persisted ledger record lifecycle boundaries. | Persisted records are usable only after Phase-62-style verification. |
| 74 | Application State Recovery Boundary | Milestone 5 | Rehydrate in-memory state from verified persisted records. | Recovery is explicit, typed, and fail-closed with no silent repair. |
| 75 | Roadmap and Changelog Alignment Check | Milestone 5 | Reconcile provider execution and durable recovery planning/history posture. | Alignment checkpoint only; no readiness approval. |
| 76 | UI/Rust Transport Boundary | Milestone 5 | Add typed request/response transport boundary between UI and Rust surfaces. | UI remains non-authoritative; transport carries typed contracts only. |
| 77 | UI Operator Intent Submission Wiring | Milestone 5 | Wire UI intent submission to Rust ingress/authorization surfaces. | Submission does not execute action; Rust owns validation/authorization/audit eligibility. |
| 78 | Authorized Operator Action Execution Boundary | Milestone 5 | Define first narrow authorized operator action execution path. | Only explicitly authorized audit-eligible actions may execute. |
| 79 | End-to-End Local Harness Run | Milestone 5 | Connect execution/authorization/audit/persistence/recovery/projection into one local flow. | Local-first workflow only; no public production claim. |
| 80 | Roadmap and Changelog Alignment Check + Production Candidate Gap Audit | Milestone N | Determine remaining work before public usability or release-candidate consideration. | Gap audit only; not approval of RC/production/public usability. |

## Boundary reminder

Roadmap entries are planned truth and boundaries only.

Historical completion remains in `CHANGELOG.md`.
