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

<!-- Website Button -->
[![Website](https://img.shields.io/badge/Website-ajentic.dev-0A0A0A?logo=google-chrome&logoColor=white)](https://www.ajentic.dev)

</div>

---

AJENTIC helps you work with AI in a way that feels clear, predictable, and fully under your control.  
Instead of guessing what a model might do, AJENTIC gives you a simple, guided way to use powerful language models—locally or in the cloud—while keeping every step transparent and accountable.

It’s built for development teams that want the benefits of AI without the chaos: clean workflows, clear guardrails, and confidence that nothing moves forward unless you say so.

**Current status:** Alpha Pre-Release

This repository is under active development. Capabilities may expand, but authority boundaries and governance rules remain stable.

The README describes what the project is and how to begin using it.  
Implementation history, capability progression, and release decisions are recorded in the CHANGELOG.  
System rules and decision authority are defined in the governance and specification documents.

---

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

The CLI validation surface performs deterministic structural and protocol validation of run inputs and adapter responses.
It verifies required files, validates adapter responses, and enforces contract boundaries.
It does not perform governance approval or promotion decisions.

System behavior, approval authority, and promotion rules are defined in governance code and supporting documentation.
When behavior questions arise, refer to the governance documents and the CHANGELOG.

---

## Documentation

The documents below define the authoritative system rules and implementation history.

* [docs/SPEC.md](docs/SPEC.md) — System specification and architectural boundaries
* [docs/GOVERNANCE.md](docs/GOVERNANCE.md) — Governance and decision authority rules
* [docs/ROADMAP.md](docs/ROADMAP.md) — Planned capability progression
* [CHANGELOG.md](CHANGELOG.md) — Authoritative implementation and release history
* [CONTRIBUTING.md](CONTRIBUTING.md) — Contributor workflow and expectations
* [AGENTS.md](AGENTS.md) — LLM coding agent guidance
* [docs/](docs/) — Supporting architecture and protocol documentation

When documentation appears inconsistent, the CHANGELOG and governance documents take precedence.
