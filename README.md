---
truth_dimension: orientation
authority_level: non_authoritative
mutation_path: readme_update
---
# AJENTIC
**You, In Control**

![Rust](https://img.shields.io/badge/Rust-authority-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-UI-blue)
![Python](https://img.shields.io/badge/Python-adapters-yellow)
![Bash](https://img.shields.io/badge/Bash-glue-lightgrey)
![Status](https://img.shields.io/badge/status-pre--alpha-red)
![Production](https://img.shields.io/badge/production-not%20ready-lightgrey)

AJENTIC helps people use AI-generated work without giving the model control over the result.

It gives teams a place to review what the model produced, check how it was handled, record what happened, and replay the work later. The goal is simple: move faster with AI while keeping the human operator in charge.

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