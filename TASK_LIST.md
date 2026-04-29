# TASK_LIST

Last Updated: 2026-04-29 19:56 UTC

Purpose:  
This document is a governance verification checklist for the Owner.  
It is not an implementation backlog and must not be treated as a development task queue.

ROADMAP.md is a governance file and must be updated manually by the Owner.  
CHANGELOG.md is the authoritative record of implementation history, milestone acceptance, and version progression.

Current repository status: Phase 13 implementation complete pending Owner acceptance and documentation/version alignment.

---

## Manual ROADMAP.md update tasks for Owner

- [ ] Confirm Phase 0: Repository bootstrap remains complete.
- [ ] Confirm Phase 1: Contracts and schemas remain complete.
- [ ] Confirm Phase 2: Candidate lifecycle state machine remains complete.
- [ ] Confirm Phase 3: CLI validation surface remains complete.
- [ ] Confirm Phase 4: Adapter protocol and mock adapter remain complete.
- [ ] Confirm Phase 5: Candidate creation and runtime adapter checks remain complete.
- [ ] Confirm Phase 6: Evaluation result ingestion remains complete.
- [ ] Confirm Phase 7: Governance and promotion gates are not started unless the Owner explicitly approves.
- [ ] Confirm Phase 8: Ledger persistence and audit record infrastructure are not started unless explicitly approved.
- [ ] Confirm Phase 9: Replay capability is not started unless explicitly approved.
- [ ] Confirm Phase 10: Provider adapters are not started unless explicitly approved.
- [ ] Confirm Phase 11: Cloud adapter capability is not started unless explicitly approved.
- [ ] Confirm Phase 12: UI review surface is not started unless explicitly approved.
- [ ] Confirm Phase 13: Multi-domain capability implementation status matches CHANGELOG documentation.
- [ ] Confirm Phase 14: Reuse and bounded improvement capability is not started unless explicitly approved.

- [ ] Confirm roadmap and CHANGELOG versioning follow the chosen milestone model:
      0.x.0 for accepted milestones
      0.x.y for iterative changes within a milestone.

- [ ] Confirm the current repository remains on the 0.0.x alignment/pre-milestone line until the Owner accepts the core boundary milestone.
- [ ] Confirm 0.1.0 is reserved for the accepted core boundary milestone.

- [ ] Confirm no roadmap wording claims evaluator execution, governance approval, promotion, ledger persistence, replay, audit emission, real provider integration, API, or UI implementation unless explicitly accepted and recorded in CHANGELOG.md.

- [ ] Confirm any evaluation wording distinguishes result ingestion and required evaluator satisfaction from governance approval or promotion eligibility.
- [ ] Confirm any lifecycle wording distinguishes PromotedTier1 as a lifecycle state shape from governance authorization.
- [ ] Confirm any adapter wording distinguishes deterministic mock adapter output from real provider integration or trusted candidate output.

---

## Owner review tasks for Phase 9 — Replay capability boundary verification

- [ ] Confirm replay status aligns with CHANGELOG documentation.
- [ ] Confirm replay is Rust-owned.
- [ ] Confirm replay reconstructs state from ledger facts only.
- [ ] Confirm replay does not call adapters or providers.
- [ ] Confirm replay does not regenerate candidate output.
- [ ] Confirm replay does not implement persistence.
- [ ] Confirm no UI, API, or file I/O behavior was added.
- [ ] Confirm version remains pre-milestone unless the Owner accepts a milestone.

---

## Owner review tasks for Phase 11 — Cloud adapter boundary verification

- [ ] Confirm cloud adapter status aligns with CHANGELOG documentation.
- [ ] Confirm the cloud adapter remains Python-only adaptation.
- [ ] Confirm Rust remains the authority.
- [ ] Confirm cloud model output remains untrusted.
- [ ] Confirm adapter completion does not promote.
- [ ] Confirm replay does not call the cloud provider.
- [ ] Confirm no provider-specific governance was added.
- [ ] Confirm no provider-specific promotion was added.
- [ ] Confirm credentials remain environment-only and are never logged.
- [ ] Confirm no UI, API, persistence, or provider-specific replay behavior was added.
- [ ] Confirm version remains pre-milestone unless the Owner accepts a milestone.

---

## Owner review tasks for Phase 12 — UI boundary verification

- [ ] Confirm UI review surface status aligns with CHANGELOG documentation.
- [ ] Confirm UI remains non-authoritative.
- [ ] Confirm UI does not compute promotion eligibility.
- [ ] Confirm UI does not call providers or adapters.
- [ ] Confirm UI does not write ledger or audit records.
- [ ] Confirm UI does not perform replay.
- [ ] Confirm UI clearly displays failed, blocked, unknown, and missing states.
- [ ] Confirm no API/server/backend authority behavior was added.
- [ ] Confirm version remains pre-milestone unless the Owner accepts a milestone.

---

## Owner review tasks for Phase 13 — Multi-domain boundary verification

- [ ] Confirm multi-domain status aligns with CHANGELOG documentation.
- [ ] Confirm domains are Rust-owned compatibility profiles.
- [ ] Confirm at least two sample domains exist.
- [ ] Confirm unsupported objective types fail closed.
- [ ] Confirm missing required evaluators fail closed.
- [ ] Confirm domains do not alter lifecycle behavior.
- [ ] Confirm domains do not alter promotion behavior.
- [ ] Confirm domains do not alter ledger, audit, or replay behavior.
- [ ] Confirm domains do not affect adapter trust.
- [ ] Confirm Phase 14 reuse and bounded improvement capability is not started.
- [ ] Confirm no provider, API, or UI authority behavior was added.
- [ ] Confirm version remains pre-milestone unless the Owner accepts a milestone.

---

## Staleness detection rule

This task list should be reviewed and revalidated when:

- The CHANGELOG records a new accepted milestone
- A new phase implementation begins
- A governance boundary changes
- A major audit is completed
- The timestamp at the top of this file is older than the most recent audit date
