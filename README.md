---
truth_dimension: orientation
authority_level: non_authoritative
mutation_path: readme_update
---

<div align="center">

# AJENTIC
### AI work, under your control

![Rust](https://img.shields.io/badge/Rust-B7410E?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Bash](https://img.shields.io/badge/Bash-4EAA25?style=for-the-badge&logo=gnu-bash&logoColor=white)
![Status](https://img.shields.io/badge/Status-v1.0.0_Published-28A745?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-28A745?style=for-the-badge)

<br>

<a href="https://ajentic.dev/#getting-started">
  <img src="https://img.shields.io/badge/Get_Started-000000?style=for-the-badge&logo=rocket&logoColor=white" alt="Get Started" />
</a> &nbsp;&nbsp;
<a href="https://github.com/irgordon/ajentic/releases/tag/v1.0.0">
  <img src="https://img.shields.io/badge/v1.0.0_Release-28A745?style=for-the-badge&logo=github&logoColor=white" alt="v1.0.0 Release" />
</a>

</div>

---

AJENTIC helps people use AI without handing control over to the model.

It gives you a clear place to review AI-generated work, see what happened, and keep a record of each step. Instead of letting an AI system act on its own, AJENTIC keeps the work visible, checkable, and repeatable.

The goal is simple:

**Use AI faster, while keeping people in charge.**

## Why AJENTIC Exists

AI can sound confident even when it is wrong.

AJENTIC helps reduce that risk by putting a review layer around AI work. It lets you see what the AI produced, what checks were run, what was blocked, and what still needs attention.

AJENTIC is not about replacing human judgment. It is about making AI-assisted work easier to inspect, easier to explain, and easier to trust.

## What You Can Do

AJENTIC is designed to help you:

- review AI-generated drafts before anyone relies on them
- see what checks passed, failed, or still need attention
- keep a record of what happened
- inspect the history of a run
- compare AI output with the task and rules it was supposed to follow
- avoid hidden or automatic decisions
- keep people responsible for final outcomes

In plain language:

**The AI can suggest. AJENTIC helps you review. People decide.**

## How It Works

AJENTIC follows a simple control model:

1. You define the task.
2. You define the rules.
3. The AI produces a draft.
4. AJENTIC records and checks the work.
5. A person reviews the result.
6. The run can be reviewed again later.

The AI-generated draft is treated as untrusted until it is reviewed. The browser interface helps you see status, evidence, errors, and history, but it does not make final decisions.

## What Makes AJENTIC Different

Many AI tools focus on speed.

AJENTIC focuses on controlled speed:

- clear inputs
- clear rules
- visible checks
- reviewable results
- repeatable runs
- human approval controls
- records that are easy to review later

AJENTIC is built for situations where "the AI said so" is not good enough.

## Who It Is For

AJENTIC is useful for:

- students learning how to use AI responsibly
- researchers reviewing AI-assisted work
- builders testing AI workflows
- teams that need records and review steps
- developers creating safer AI-assisted systems
- organizations that want AI help without hidden automation

You do not need to think of AJENTIC as a complex developer tool. Think of it as a workspace for checking AI work before trusting it.

## Current Release Status

**Current status: v1.0.0 GitHub Release published.**

AJENTIC v1.0.0 is available through GitHub Releases:
[https://github.com/irgordon/ajentic/releases/tag/v1.0.0](https://github.com/irgordon/ajentic/releases/tag/v1.0.0).

The release includes review assets such as the final candidate bundle, checksums, SBOM evidence, provenance evidence, asset manifest, release notes, and final README asset. The source release candidate `v1.0.0-rc.1` remains intact.

Important limits:

- no npm package publication
- no Cargo package publication
- no package registry publication
- no app-store installer
- no automatic update channel
- no deployment service
- no OS signing or notarization
- no automatic production approval

The browser interface is read-only and used for review. It does not make final decisions.

Developer validation includes contract and boundary drift checks for duplicated schema/Rust/TypeScript surfaces. Release assets, evidence, scripts, and UI projections remain review surfaces; they do not become runtime authority.

## What AJENTIC Does Not Do

AJENTIC does not:

- make the AI automatically correct
- replace human review
- approve work on its own
- deploy anything automatically
- publish packages automatically
- turn AI output into truth
- give the browser interface final decision power

AJENTIC helps you review AI work more clearly. It does not remove your responsibility to decide whether the work is good enough.

## Running Locally

Clone the repository:

```sh
git clone https://github.com/irgordon/ajentic.git
cd ajentic
```

Run the project checks:

```sh
CARGO_TARGET_DIR=/tmp/ajentic-check-target ./scripts/check.sh
```

Run the local browser interface:

```sh
cd ui
npm ci
npm run dev
```

Then open the local address shown in your terminal, usually `http://127.0.0.1:5173`.

The browser interface is local and read-only. It is for reviewing status, evidence, and help text. It does not publish releases, deploy systems, or approve AI output.

## For Developers

AJENTIC keeps its internal responsibilities separated:

```text
Rust       = protected decision engine
Python     = AI/model adapters and support scripts
TypeScript = browser interface
Bash       = local setup and check scripts
```

The main rule:

**The model does not decide. The browser does not decide. Scripts do not decide. The protected core decides what is valid.**

This keeps the system easier to inspect, test, and explain.

## Release Evidence

AJENTIC v1.0.0 includes GitHub Release assets for review:

- final release bundle
- asset manifest
- checksums
- SBOM evidence
- provenance evidence
- release notes

These files help reviewers understand what was released. They are evidence, not decision authority.

GitHub Actions remains the release evidence platform for the release candidate and final GitHub Release paths. Release assets, checksums, SBOM/provenance files, attestations, workflows, scripts, and UI surfaces are evidence or visibility surfaces only. They do not approve package publication, installers, update channels, deployments, OS signing, notarization, backend authority changes, or UI authority changes.

Post-v1 release closure evidence is recorded in `docs/operations/`.

## License

MIT

## Short Version

AJENTIC helps you use AI with a review process.

It keeps AI work visible, checkable, and under human control.

This README is orientation only. It is not an authority source.
