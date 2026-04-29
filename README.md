<div align="center">

# **AJENTIC**

### **"Under Your Control"**

<!-- Language Badges -->
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)
![Bash](https://img.shields.io/badge/Bash-4EAA25?logo=gnubash&logoColor=white)

<!-- Project Badges -->
![Open Source](https://img.shields.io/badge/Open%20Source-🌐-blue)
![Agentic Coding](https://img.shields.io/badge/Agentic%20Coding-⚡-purple)

</div>

---

AJENTIC helps you work with AI in a way that feels clear, predictable, and fully under your control.  
Instead of guessing what a model might do, AJENTIC gives you a simple, guided way to use powerful language models—locally or in the cloud—while keeping every step transparent and accountable.

It’s built for development teams that want the benefits of AI without the chaos: clean workflows, clear guardrails, and confidence that nothing moves forward unless you say so.



**Current status:** Pre-Release Active Development

## Language ownership

| Language   | Role        | Authority |
|------------|-------------|-----------|
| Rust       | authority   | Authoritative harness control surfaces |
| Python     | adaptation  | Non-authoritative adapters and experimental helpers |
| TypeScript | visibility  | Non-authoritative UI and review surfaces |
| Bash       | glue        | Thin local and CI command wrappers |
| Go         | optional    | Not part of the current implementation path |

Generated output is **untrusted by default**.

Adapter output is not approval. Candidate creation is not validation. Evaluation result ingestion is not governance approval. Required evaluator satisfaction is not promotion eligibility.

Only Rust may own lifecycle, governance, ledger, replay, audit, and promotion decisions. Python, TypeScript, Bash, and future Go wrappers must not define candidate truth or promotion authority.

---

<div align="center">
  <img src="ajentic_arch_flow.png" width="500" />
</div>

---


## First run

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
````

The current CLI validation surface performs static run-directory checks only. It checks required file presence, non-empty content, and expected plain-text markers. It does not parse YAML, validate schemas, evaluate candidates, apply governance, or approve promotion.

Before any phases begin, refer to the documentation and roadmap status should remain aligned with the actual repository state. Promotion logic must have one authoritative home in Rust governance code. It must not be duplicated in candidate lifecycle, evaluation ingestion, Python adapters, TypeScript UI, Bash scripts, or CLI convenience paths.

## Documentation

* [docs/SPEC.md](docs/SPEC.md) — System specification
* [docs/GOVERNANCE.md](docs/GOVERNANCE.md) — Governance rules
* [docs/ROADMAP.md](docs/ROADMAP.md) — Phased implementation plan
* [CHANGELOG.md](CHANGELOG.md) — Version and phase history
* [CONTRIBUTING.md](CONTRIBUTING.md) — Contributor guide
* [AGENTS.md](AGENTS.md) — LLM coding agent guide
* [docs/](docs/) — Architecture and protocol documentation
