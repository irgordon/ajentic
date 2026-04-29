# TASK_LIST

## Manual ROADMAP.md update tasks for Owner

ROADMAP.md is a governance file and should be updated manually by the Owner.

Current repository status: Phase 6 complete pending Owner acceptance and documentation/version alignment.

- [ ] Confirm Phase 0: Repository bootstrap remains complete.
- [ ] Confirm Phase 1: Contracts and schemas remains complete.
- [ ] Confirm Phase 2: Candidate lifecycle state machine remains complete.
- [ ] Confirm Phase 3: CLI static validation surface remains complete.
- [ ] Confirm Phase 4: Adapter protocol and mock adapter remains complete.
- [ ] Confirm Phase 5: Candidate creation and runtime adapter checks remains complete.
- [ ] Confirm Phase 6: Evaluation result ingestion is complete.
- [ ] Confirm Phase 7: Governance and promotion gates is not started unless the Owner explicitly approves.
- [ ] Confirm roadmap versioning follows the chosen milestone model: `0.x.0` for accepted milestones and `0.x.y` for iterative changes within a milestone.
- [ ] Confirm the current repository remains on the `0.0.x` alignment/pre-milestone line until the Owner accepts the core boundary milestone.
- [ ] Confirm `0.1.0` is reserved for the accepted core boundary milestone.
- [ ] Confirm no roadmap wording claims evaluator execution, governance approval, promotion, ledger persistence, replay, audit emission, real provider integration, API, or UI implementation.
- [ ] Confirm any evaluation wording distinguishes result ingestion and required evaluator satisfaction from governance approval or promotion eligibility.
- [ ] Confirm any lifecycle wording distinguishes `PromotedTier1` as a lifecycle state shape from governance authorization.
- [ ] Confirm any adapter wording distinguishes deterministic mock adapter output from real provider integration or trusted candidate output.

## Owner review tasks for Phase 9

- [ ] Confirm Phase 9 replay status.
- [ ] Confirm replay is Rust-owned.
- [ ] Confirm replay reconstructs from ledger facts only.
- [ ] Confirm replay does not call adapters or providers.
- [ ] Confirm replay does not regenerate candidate output.
- [ ] Confirm replay does not implement persistence.
- [ ] Confirm Phase 10 provider adapters are not started.
- [ ] Confirm no UI, API, or file IO behavior was added.
- [ ] Confirm version remains pre-milestone unless Owner accepts a milestone.
