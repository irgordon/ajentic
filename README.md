# AJENTIC
> "Under Your Control"

AJENTIC is a governed harness for local and cloud-based large language models (LLMs). It treats generated output as untrusted, validates candidate solutions against explicit objectives and constraints, and promotes only governed outputs for downstream review.

**Current status:** v0.4.0 Phase 4 adapter protocol and mock adapter boundary.

## Language ownership

| Language   | Role        | Authority |
|------------|-------------|-----------|
| Rust       | authority   | Authoritative harness, lifecycle, governance, ledger, replay, promotion |
| Python     | adaptation  | Non-authoritative adapters only |
| TypeScript | visibility  | Non-authoritative UI only |
| Bash       | glue        | Thin scripts only |
| Go         | optional    | Not part of v0.4.0 |

Generated output is **untrusted by default**. Every candidate solution must be validated against explicit objectives and constraints before promotion.

## First run

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
```

## Phase 4 adapter boundary

- This phase establishes the adapter protocol boundary.
- Rust can invoke a deterministic Python mock adapter through a subprocess.
- Rust validates response linkage, status vocabulary, and output-size limits.
- A completed adapter response is untrusted generated output only.
- Candidate creation, evaluation, governance, promotion, ledger, replay, and audit behavior are reserved for later phases.

## Documentation

- [SPEC.md](SPEC.md) — System specification
- [GOVERNANCE.md](GOVERNANCE.md) — Governance rules
- [ROADMAP.md](ROADMAP.md) — Phased implementation plan
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contributor guide
- [AGENTS.md](AGENTS.md) — LLM coding agent guide
- [docs/](docs/) — Architecture and protocol documentation
