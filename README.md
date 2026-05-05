---
truth_dimension: orientation
authority_level: non_authoritative
mutation_path: readme_update
---

<div align="center">

# **AJENTIC**  
### **You, In Control**

![Rust](https://img.shields.io/badge/Rust-Authority-orange?logo=rust)
![TypeScript](https://img.shields.io/badge/TypeScript-UI-blue?logo=typescript)
![Python](https://img.shields.io/badge/Python-Adapters-yellow?logo=python)
![Bash](https://img.shields.io/badge/Bash-Glue-lightgrey?logo=gnu-bash)
![Status](https://img.shields.io/badge/Status-Pre--Alpha-red)
![Production](https://img.shields.io/badge/Production-Not_Ready-lightgrey)
![Open Source](https://img.shields.io/badge/Open%20Source-Yes-brightgreen?logo=opensourceinitiative)

<br>

<a href="#getting-started">
  <img src="https://img.shields.io/badge/Get_Started-000000?style=for-the-badge&logo=rocket" />
</a>
&nbsp;&nbsp;
<a href="https://github.com/ajentic">
  <img src="https://img.shields.io/badge/GitHub_Repo-24292e?style=for-the-badge&logo=github&logoColor=white" />
</a>

</div>

---

AJENTIC helps teams use AI confidently **without giving the model control over the work**.

Instead of letting an AI system act on its own, AJENTIC creates a **safe, governed workspace** where every step is visible, reviewable, and repeatable. You see what the model produced, how it was handled, and what changed along the way — with a full record you can replay at any time.

The goal is simple:  
**Move faster with AI while keeping humans in charge of decisions, outcomes, and accountability.**

## Why Use AJENTIC?

AI tools can produce useful work quickly, but their output should not be trusted just because it looks right.

AJENTIC puts a controlled review system around that output:
- clear inputs
- bounded context
- typed requests
- validation checks
- recorded events
- replayable runs
- audit-friendly results
- human review

The goal is not to make the model autonomous.

The goal is to make AI-assisted work inspectable, repeatable, and controlled.

## Uses for 

The planned application includes:
- a Rust-controlled core
- a browser-based TypeScript UI
- local and cloud model workflow support
- context review
- memory and provenance review
- policy and validation results
- run history
- replay view
- clean output view
- operator intent controls
- audit and export surfaces

## Core idea

```text
User intent + model output
  → AJENTIC intake
  → context review
  → policy and validation
  → candidate output
  → controlled action boundary
  → recorded evidence
  → replay
  → audit
  → clean human-readable output
```

Raw model output is not clean output.

Clean output is output that has passed through the AJENTIC boundary.

Project status

This repository is in pre-alpha development.

The project currently focuses on repository structure, governance boundaries, validation gates, replay evidence, durable proof boundaries, and non-authoritative UI contracts.

Production readiness is not claimed.

Release-candidate readiness is not claimed.

Public usability is not claimed.

Technology stack

AJENTIC separates technology by responsibility:

Layer	Role
Rust	authoritative core, validation, governance, replay, persistence boundaries
TypeScript	browser UI and non-authoritative display surfaces
Python	repository validation and support scripts
Bash	local command orchestration
GitHub Actions	CI validation gates

Repository model

The repository separates different kinds of truth:

* Governance defines what must always be true.
* Architecture defines how the system is organized.
* Roadmap defines what may be attempted next.
* Changelog records completed work.
* Checklists guide bounded execution events.
* Tests and code define executable behavior.
* Schemas define data contracts.
* Memory stores governed data.
* This README explains the project for humans.

This README is orientation only. It is not an authority source.

Intended users

AJENTIC is intended for engineers and teams using AI-assisted workflows who need:

* controlled model runs
* reviewable context
* traceable decisions
* replayable execution
* clear operator controls
* evidence that model output was not silently trusted

Project boundary

AJENTIC is not an autonomous coding agent.

It is a control interface for reviewing, validating, recording, and replaying model-driven work.