---
truth_dimension: orientation
authority_level: non_authoritative
mutation_path: readme_update
---

<div align="center">

# AJENTIC
### You, In Control

![Rust](https://img.shields.io/badge/Rust-B7410E?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Bash](https://img.shields.io/badge/Bash-4EAA25?style=for-the-badge&logo=gnu-bash&logoColor=white)
![Status](https://img.shields.io/badge/Status-Pre--Alpha-8B0000?style=for-the-badge)
![Open Source](https://img.shields.io/badge/Open_Source-Yes-28A745?style=for-the-badge&logo=opensourceinitiative&logoColor=white)

<br>

<a href="https://ajentic.dev/#getting-started">
  <img src="https://img.shields.io/badge/Get_Started-000000?style=for-the-badge&logo=rocket&logoColor=white" alt="Get Started" />
</a>&nbsp;&nbsp;
<a href="https://github.com/irgordon/ajentic">
  <img src="https://img.shields.io/badge/GitHub_Repo-24292e?style=for-the-badge&logo=github&logoColor=white" alt="GitHub Repo" />
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
See the latest updates in [CHANGELOG](CHANGELOG.md).

## Technology Stack

AJENTIC separates technology by responsibility:

| Layer            | Role                                                             |
|------------------|------------------------------------------------------------------|
| **Rust**         | authoritative core, validation, governance, replay, persistence  |
| **TypeScript**   | browser UI, non‑authoritative display surfaces                   |
| **Python**       | repository validation, support scripts                           |
| **Bash**         | local command orchestration                                      |
| **GitHub Actions** | CI validation gates, schema/policy enforcement                 |

## Architecture Overview

```mermaid
flowchart TB

    %% Interaction and operator surfaces (Top Layer)
    subgraph InteractionLayer ["Interaction Layer (Non-Authoritative)"]
        direction LR
        
        subgraph BashLayer ["Bash / Local Ops"]
            direction TB
            CLI1[Local Commands]:::bash
            CLI2[Validation Runner]:::bash
        end

        subgraph TSUILayer ["TypeScript UI"]
            direction TB
            UI1[Browser Review Surface]:::ts
            UI2[Context and Evidence Review]:::ts
            UI3[Output and Replay Display]:::ts
        end
    end

    %% Authoritative engine (Middle Layer)
    subgraph CoreLayer ["Authoritative Core"]
        direction TB
        
        subgraph RustCore ["Rust Core Engine"]
            direction LR
            %% Grouping the core into left-to-right columns to keep it compact
            R1[Typed Requests]:::rust
            R2[Bounded Context Assembly]:::rust
            R3[Provider Boundary]:::rust
            R4[Validation Gates]:::rust
            R5[Governance Rules]:::rust
            R6[Ledger and Replay]:::rust
            R7[Persistence Boundaries]:::rust
            R8[Audit Evidence]:::rust
        end
    end

    %% External model/provider surface & Repo Gates (Bottom Layer)
    subgraph ExternalAndGates ["External Integrations & Static Gates"]
        direction LR
        
        subgraph ProviderLayer ["Model Provider (Untrusted)"]
            direction TB
            M1[Local LLM]:::llm
            M2[Cloud LLM]:::llm
            M3[Candidate Model Output]:::llm
        end

        subgraph SupportLayer ["Repository Validation Gates"]
            direction LR
            
            subgraph PythonLayer ["Python Scripts"]
                direction TB
                P1[Repo Structure]:::python
                P2[Docs & Schema]:::python
            end

            subgraph CILayer ["GitHub Actions / CI"]
                direction TB
                G1[Boundary Lints]:::ci
                G2[Contract Checks]:::ci
                G3[Regression Tests]:::ci
            end
        end
    end

    %% === FLOW RELATIONSHIPS ===

    %% Operator and core flow
    BashLayer == "invokes bounded local workflow" ==> RustCore
    RustCore == "returns report, evidence, and clean output" ==> BashLayer

    %% Provider/model flow
    R2 == "bounded context" ==> R3
    
    %% Provider paths explicitly acting as the controlled boundary
    R3 == "provider request (controlled boundary)" ==> M1
    R3 == "provider request (controlled boundary)" ==> M2
    
    M1 --> M3
    M2 --> M3
    
    %% Output path clearly labeled as untrusted material routing straight to validation
    M3 == "returned model output (untrusted candidate material)" ==> R4

    %% Validation/governance internal flow (Guides the eye left-to-right inside the core)
    R4 --> R5 --> R6 --> R8

    %% UI projection flow
    RustCore == "read-only projections and evidence" ==> TSUILayer
    TSUILayer -. "reviews only; no authority" .-> RustCore

    %% Repository validation flow
    SupportLayer -. "checks repository changes" .-> RustCore
    SupportLayer -. "checks UI and docs boundaries" .-> InteractionLayer

    %% === STYLING ===
    classDef rust fill:#B7410E,stroke:#fff,stroke-width:2px,color:#fff;
    classDef ts fill:#3178C6,stroke:#fff,stroke-width:2px,color:#fff;
    classDef python fill:#FFD43B,stroke:#333,stroke-width:2px,color:#111;
    classDef bash fill:#4EAA25,stroke:#fff,stroke-width:2px,color:#fff;
    classDef ci fill:#24292E,stroke:#fff,stroke-width:2px,color:#fff;
    classDef llm fill:#6B46C1,stroke:#fff,stroke-width:2px,color:#fff;

    style InteractionLayer fill:none,stroke:#888,stroke-dasharray: 4 4,color:#ddd
    style CoreLayer fill:none,stroke:#B7410E,stroke-width:2px,color:#ddd
    style ExternalAndGates fill:none,stroke:none,color:#fff
    style ProviderLayer fill:none,stroke:#6B46C1,stroke-dasharray: 4 4,color:#ddd
    style SupportLayer fill:none,stroke:#888,stroke-dasharray: 4 4,color:#ddd
```

## Repository Model

The repository separates different kinds of truth:

| Artifact        | Definition                                |
|-----------------|--------------------------------------------|
| Governance      | what must always be true                   |
| Architecture    | how the system is organized                |
| Roadmap         | what may be attempted next                 |
| Changelog       | what has been completed                    |
| Checklists      | bounded execution steps                    |
| Tests & Code    | executable behavior                        |
| Schemas         | data contracts                             |
| Memory          | governed data                              |
| README          | human‑level orientation                    |

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
