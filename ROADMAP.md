# AJENTIC Roadmap

## Release track

| Version | Scope |
|---------|-------|
| v0.0.0  | Skeleton and governance placeholders |
| v0.1.0  | Contracts and schemas |
| v0.2.0  | Lifecycle state machine |
| v0.3.0  | Adapter protocol and mock generation |
| v0.4.0  | Evaluation result ingestion |
| v0.5.0  | Governance and promotion gates |
| v0.6.0  | Ledger and audit trail |
| v0.7.0  | Replay |
| v0.8.0  | Local model adapter capability |
| v0.9.0  | Cloud model adapter capability |
| v0.10.0 | UI review surface |
| v0.11.0 | Multi-domain capability |
| v0.12.0 | Reuse and bounded improvement records |
| v0.13.0 | Hardening and failure testing |
| v0.14.0 | Early production capability |

## Milestone checkpoints

| Milestone | Description |
|-----------|-------------|
| A | Contract freeze |
| B | Adapter boundary freeze |
| C | Promotion authority freeze |
| D | Replay integrity freeze |
| E | Provider parity |
| F | Cross-domain stability |
| G | Failure hardening |

## Phase boundaries

**v0.0.0** — Repository skeleton only. No runtime behavior. Establishes language ownership, governance rules, schema placeholders, and documentation structure.

**v0.1.0** — Define the core contracts: objective, constraints, domain, candidate solution, evaluation result, and governance result schemas reach a stable draft.

**v0.2.0** — Implement the lifecycle state machine in Rust core. Candidate solutions move through defined states with auditable transitions.

**v0.3.0** — Define and implement the adapter protocol. Python adapters connect to the harness via a versioned protocol. Mock generation enables testing without real providers.

**v0.4.0** — Evaluation result ingestion. The harness can receive and normalize evaluation results from adapters.

**v0.5.0** — Governance gates. Promotion decisions are made by Rust core only. Milestone C: Promotion authority freeze.

**v0.6.0** — Ledger and audit trail. All lifecycle events are recorded. Milestone D: Replay integrity freeze.

**v0.7.0** — Replay. The harness can reconstruct state from ledger records.

**v0.8.0** — Local model adapter. At least one local model provider is supported.

**v0.9.0** — Cloud model adapter. At least one cloud model provider is supported. Milestone E: Provider parity.

**v0.10.0** — TypeScript UI review surface. Governed outputs are visible for human review.

**v0.11.0** — Multi-domain capability. More than one domain can be active simultaneously. Milestone F: Cross-domain stability.

**v0.12.0** — Reuse and bounded improvement records.

**v0.13.0** — Hardening and failure testing. All failure modes tested. Milestone G: Failure hardening.

**v0.14.0** — Early production capability.
