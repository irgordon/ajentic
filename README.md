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

AI can produce convincing work, but convincing is not the same as correct.

AJENTIC adds a controlled review boundary around model output:

- clear inputs  
- bounded context  
- typed requests  
- validation checks  
- recorded events  
- replayable runs  
- audit‑friendly results  
- human review  

The goal is not autonomy.  
The goal is **inspectable, repeatable, controlled AI‑assisted work**.

## Uses

AJENTIC is designed to support:

- a Rust‑governed core  
- a browser‑based TypeScript UI  
- local and cloud model workflows  
- context review  
- memory and provenance inspection  
- policy and validation results  
- run history  
- replay visualization  
- clean output surfaces  
- operator intent controls  
- audit and export paths  

---

## Core Idea

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
Clean output is what has passed through the AJENTIC boundary.

## Project Status

Pre‑Alpha and under active development.
See the latest updates in [CHANGELOG.md].

## Technology Stack

AJENTIC separates technology by responsibility:

Layer	Role	
Rust	authoritative core, validation, governance, replay, persistence	
TypeScript	browser UI, non‑authoritative display surfaces	
Python	repository validation, support scripts	
Bash	local command orchestration	
GitHub Actions	CI validation gates, schema/policy enforcement	

## Architecture Overview

```mermaid
flowchart TB

    subgraph RustCore["Rust Core (Authoritative)"]
    direction TB
    A1[Validation]:::rust
    A2[Governance Rules]:::rust
    A3[Replay Engine]:::rust
    A4[Persistence Boundaries]:::rust
    end

    subgraph PythonLayer["Python (Support Scripts)"]
    direction TB
    B1[Repo Validation]:::python
    B2[Schema Checks]:::python
    end

    subgraph BashLayer["Bash (Local Ops)"]
    C1[CLI Orchestration]:::bash
    end

    subgraph TSUILayer["TypeScript UI (Non‑Authoritative)"]
    D1[Browser UI]:::ts
    D2[Context Review]:::ts
    D3[Output Display]:::ts
    end

    subgraph CI["GitHub Actions (CI Gates)"]
    E1[Schema Enforcement]:::ci
    E2[Policy Checks]:::ci
    E3[Determinism Tests]:::ci
    end

    TSUILayer --> RustCore
    PythonLayer --> RustCore
    BashLayer --> RustCore
    CI --> RustCore

    classDef rust fill:#dea584,stroke:#000,stroke-width:1px;
    classDef ts fill:#3178c6,stroke:#000,stroke-width:1px,color:#fff;
    classDef python fill:#ffd43b,stroke:#000,stroke-width:1px;
    classDef bash fill:#4e4e4e,stroke:#000,stroke-width:1px,color:#fff;
    classDef ci fill:#6cc644,stroke:#000,stroke-width:1px;
```

## Repository Model

The repository separates different kinds of truth:

• Governance — what must always be true
• Architecture — how the system is organized
• Roadmap — what may be attempted next
• Changelog — what has been completed
• Checklists — bounded execution steps
• Tests & Code — executable behavior
• Schemas — data contracts
• Memory — governed data
• README — human‑level orientation


This README is orientation only. It is not an authority source.

## Intended Users

AJENTIC is for engineers and teams who need:

• controlled model runs
• reviewable context
• traceable decisions
• replayable execution
• clear operator controls
• evidence that model output was not silently trusted

## Project Boundary

AJENTIC is not an autonomous coding agent.

It is a control interface for reviewing, validating, recording, and replaying AI‑assisted work.