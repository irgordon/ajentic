---
truth_dimension: orientation
authority_level: non_authoritative
mutation_path: readme_update
---

# AJENTIC

AJENTIC is a deterministic control layer for model-driven work.

It is designed for teams that want the speed of local or cloud large language models without surrendering review, validation, replay, or operator control.

The harness treats model output as candidate material. It assembles bounded context, validates outputs, records accepted events, supports replay, and presents the run through a human-readable browser UI.

## Why AJENTIC exists

Large language models can generate useful work, but raw model output is not a reliable execution boundary.

AJENTIC exists to put a controlled system around that output:
- clear context
- typed requests
- governed memory
- deterministic validation
- ledgered events
- replayable runs
- audit-friendly output
- human review

The goal is not to make the model autonomous.

The goal is to make model-driven work inspectable, repeatable, and controlled.

## What it will provide

The planned application includes:
- Rust authoritative core
- browser-based TypeScript UI
- bounded context inspection
- memory and provenance inspection
- policy and validation results
- run timeline
- replay view
- clean output view
- typed operator intent controls
- integration path for local and cloud model workflows

## Core idea

```text
User intent + model output
  → harness intake
  → context assembly
  → policy and validation
  → validated candidate output
  → controlled task execution
  → ledger
  → replay
  → audit
  → clean human-readable output

Raw model output is not clean output.

Clean output is output that has passed through the harness boundary.

### Project status

This repository is in pre-alpha development.

The initial work establishes the repository structure, governance boundaries, and validation gates needed before runtime behavior is implemented.

No production runtime capability should be assumed from placeholder files.

### Repository model

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

### Intended users

AJENTIC is intended for engineers and teams using model-assisted workflows who need:

* controlled model runs
* reviewable context
* traceable decisions
* replayable execution
* clear operator controls
* confidence that model output is not silently trusted

### Project boundary

This project is not an autonomous coding agent.

It is a harness for controlling and reviewing model-driven work.