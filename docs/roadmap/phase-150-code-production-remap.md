---
truth_dimension: planned
authority_level: planned
mutation_path: roadmap_update
---

# Phase 150 Code-Production Remap

## Scope
Phase 150 remaps Phases 151-160 into a larger product-capability roadmap toward a usable local beta. It is an alignment checkpoint only and does not implement Phase 151.

## Phase 149 executable handoff basis
Phase 149 proved the local workflow reaches deterministic provider configuration and execution, provider output validation, staged proposal creation and validation, candidate review, operator approve/reject decision on validated staged proposals, and a Phase 150 code-production handoff generated from executable local shell state.

Phase 149 also preserved these gaps: no candidate output creation, no candidate materialization, no durable decision storage, no older local decision ledger append, no replay repair, no export promotion, no provider-output trust, and no readiness, release, deployment, or public-use approval.

## Why Phase 150 remaps aggressively
The next block must stop spending non-0/5 phases on tiny safety-only increments. Phases 151-160 now group work into visible, executable, local-beta capabilities while preserving the existing trust and release boundaries.

## Code-production rule after Phase 150
Every non-0/5 phase after Phase 150 must produce at least one visible UI capability, executable Rust capability, persisted local artifact, restore/replay/export capability, real adapter integration step, or end-to-end operator workflow improvement.

## Safety checks remain embedded in implementation phases
Safety checks remain mandatory, but they belong inside the implementation phase that introduces the executable capability they guard. A non-0/5 phase must not be safety-only unless it directly guards a newly introduced executable capability.

## Product-capability grouping rule
Phases 151-160 are grouped as larger product capability phases aimed at a usable local beta path, with 0/5 phases reserved for alignment checkpoints.

## Phase 151-160 remap table
| Phase | Title | Type | Product outcome |
| --- | --- | --- | --- |
| 151 | Persistent Local Session Package | Code-production | Persist and reload a complete local session package as a local artifact. |
| 152 | Session History and Restore UI | Code-production | Show local session history, restore status, replay, decisions, validation, and export state. |
| 153 | Real Local Provider Adapter Contract | Code-production | Add a real local provider adapter contract and non-executing UI configuration surface. |
| 154 | Controlled Adapter Dry-Run Harness | Code-production | Execute a deterministic fake adapter through the real adapter contract and existing pipeline. |
| 155 | Code-Production Alignment Checkpoint | Alignment | Reconcile 151-154 and decide whether constrained real local provider invocation may proceed. |
| 156 | Constrained Real Local Provider Invocation | Code-production | Enable exactly one allowlisted local provider invocation path. |
| 157 | Real Provider Output Pipeline Integration | Code-production | Route real local provider output through projection, validation, review, staging, staged validation, and decision. |
| 158 | Local Candidate Materialization | Code-production | Materialize validated staged proposals into local candidate output under Rust-owned boundaries. |
| 159 | Complete Local Operator Workflow | Code-production | Make configure-to-export local operator workflow usable end to end. |
| 160 | Production-Path Alignment Checkpoint | Alignment | Decide whether controlled internal trial packaging may proceed and remap the next block. |

## Phase 151 details
Goal: persist and reload a complete local session package as a real local artifact.

Must produce: Rust-owned local session package format, explicit write/read helpers using caller-provided paths, validation before write and after read, UI-visible export/save/restore status if feasible, and tests for deterministic package content, read-back validation, and no release/deployment/readiness claims.

Scope includes available provider configuration, provider execution result, provider output validation and review state, staged proposal and validation, candidate review state, operator decision state, decision ledger, replay/status projection, and local evidence export.

Boundary: local session persistence only; no production persistence claim, public release artifact, installer, update, signing, or publishing behavior.

## Phase 152 details
Goal: add usable UI for local session history, restore status, restored run state, replay, decisions, validation state, and export state.

Must produce UI-visible restored-session state, session history/list projection using deterministic fixtures or explicit paths, restore integrity/error display, and tests for restored UI state and failure display.

Boundary: no recovery promotion, production persistence claim, background daemon, or automatic remote sync.

## Phase 153 details
Goal: add a real provider adapter contract and UI configuration surface for local providers.

Must produce Rust adapter contract, adapter capability surface, non-executing adapter registry, UI provider adapter configuration panel, and tests for allowed and unsupported adapter declarations.

Boundary: no real model execution, arbitrary shell command field, network/cloud, secret execution path, or provider trust approval.

## Phase 154 details
Goal: execute a deterministic fake adapter through the real adapter contract and route output through the existing provider execution, result, validation, review, and staging pipeline.

Must produce executable deterministic fake adapter, Rust execution path through the adapter contract, UI-visible dry-run adapter result, and tests proving integration with the existing validation/review/staging flow.

Boundary: no arbitrary local model execution, no process spawning unless explicitly justified and safely bounded, no network/cloud, and no production claims.

## Phase 155 checkpoint
Goal: reconcile Phases 151-154 and decide whether constrained real local provider invocation may proceed.

Must produce a concise alignment note, roadmap/changelog/checklist update, and decision outcome for Phase 156.

Boundary: no implementation, runtime behavior, readiness, release, or deployment approval.

## Phase 156 details
Goal: enable exactly one allowlisted local provider invocation path.

Must produce explicit allowlisted provider invocation path, no arbitrary command string field, Rust validation for invocation target, UI-visible invocation result, and tests rejecting arbitrary commands, unsafe paths, network/cloud, secrets, and unsupported providers.

Boundary: no shell-general execution, cloud/network, public release, production readiness, or provider trust approval.

## Phase 157 details
Goal: route real local provider output through the existing projection, validation, review, staging, staged validation, and decision path.

Must produce integration from real provider output to existing result projection, reuse of existing provider output validation/review/staging path, UI-visible pipeline state, and tests proving no shortcut around validation, review, staging, or decision boundaries.

Boundary: no direct candidate materialization, provider-output trust, action execution, or production claims.

## Phase 158 details
Goal: materialize validated staged proposals into local candidate output.

Must produce Rust-owned candidate materialization boundary, local candidate output projection, UI candidate materialization panel, tests proving materialization requires validated staged proposal and prior decision conditions defined by roadmap, and tests proving no production approval or provider-output trust.

Boundary: local candidate output only; not production approval, release approval, or public-use approval.

## Phase 159 details
Goal: make a complete local operator workflow usable end to end.

Must produce a human-usable flow from provider configuration through execution, projection, validation, review, staged proposal, staged validation, staged review, operator decision, allowed materialization, replay, and session export/persistence.

Must include UI workflow improvements, error/rejection drilldowns, run history/status summary, and end-to-end local workflow tests.

Boundary: local beta workflow only; no production readiness, public release, installer, or deployment claim.

## Phase 160 checkpoint
Goal: decide whether controlled internal trial packaging may proceed.

Must produce a concise production-path alignment note, evidence-based decision on controlled internal trial packaging, roadmap/changelog/checklist update, and next block remap.

Boundary: no implementation, public release approval, production approval, or Release Candidate approval unless explicitly supported by committed evidence and in scope.

## Boundaries preserved
The remap preserves existing boundaries: model output remains untrusted until validated through Rust-owned paths; TypeScript remains a review and operator-intent surface; local artifacts are not public release artifacts; and 0/5 phases remain alignment checkpoints.

## What this remap does not approve
Phase 150 does not approve Phase 151 implementation, Rust source changes, TypeScript source changes, test changes, schema changes, runtime behavior, provider execution expansion, persistence implementation, candidate materialization implementation, release artifacts, packaging, deployment, installer/update-channel behavior, signing, publishing, readiness, Release Candidate status, Production Candidate status, public/general use, or production human use.

## Deferred items
Deferred until their mapped phases: persistence implementation in Phase 151, restore UI in Phase 152, real adapter contract in Phase 153, controlled dry run in Phase 154, constrained real invocation in Phase 156, real provider pipeline integration in Phase 157, local candidate materialization in Phase 158, and complete local beta workflow in Phase 159.

## Phase 151 handoff
Phase 151 is the next code-production phase. It should implement local session package persistence and restore using Phase 149 executable handoff evidence as input, without adding production persistence or release claims.
