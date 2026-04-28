# AJENTIC
> "Under Your Control"

AJENTIC is a governed harness for local and cloud-based large language models (LLMs). It treats generated output as untrusted, validates candidate solutions against explicit objectives and constraints, and promotes only governed outputs for downstream review.

**Current status:** v0.3.0 Phase 3 static CLI validation surface.

## Language ownership

| Language   | Role        | Authority |
|------------|-------------|-----------|
| Rust       | authority   | Authoritative harness, lifecycle, governance, ledger, replay, promotion |
| Python     | adaptation  | Non-authoritative adapters only |
| TypeScript | visibility  | Non-authoritative UI only |
| Bash       | glue        | Thin scripts only |
| Go         | optional    | Not part of v0.3.0 |

Generated output is **untrusted by default**. Every candidate solution must be validated against explicit objectives and constraints before promotion.

## First run

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
```

## Phase 3 CLI static checks

- This phase adds static run directory checks.
- The `validate` command checks required file presence and minimal static markers.
- The `inspect` command reports static file presence and byte lengths.
- YAML parsing and schema validation are reserved for later phases.
- Static validation does not prove governance validity or candidate correctness.

## Documentation

- [SPEC.md](SPEC.md) — System specification
- [GOVERNANCE.md](GOVERNANCE.md) — Governance rules
- [ROADMAP.md](ROADMAP.md) — Phased implementation plan
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contributor guide
- [AGENTS.md](AGENTS.md) — LLM coding agent guide
- [docs/](docs/) — Architecture and protocol documentation
