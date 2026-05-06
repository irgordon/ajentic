---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Production Use Roadmap Expansion - Phase 99.5

## Scope
Phase 99.5 is planning and alignment only.

Phase 99.5 adds no runtime behavior. Phase 99.5 adds no new capability. Phase 99.5 does not approve production human use. Phase 99.5 does not approve Production Candidate status. Phase 99.5 does not approve release-candidate readiness. Phase 99.5 does not approve public usability. Phase 99.5 does not approve startup/package readiness. Phase 99.5 does not start Phase 100. Phase 99.5 does not implement Phases 101-120.

The scope is limited to roadmap, operations, checklist, and changelog truth surfaces that describe planned post-100 staging toward controlled production human use.

## Why this expansion exists
The Phase 85-100 roadmap ended at the Production Candidate readiness decision gate. The repository had no post-100 roadmap coverage for staged production human use.

This expansion documents planned truth for Phases 101-120 so future work remains separated by trust boundary, evidence gate, and approval posture. It prevents Phase 100 from being interpreted as production approval.

## Evidence-only planning rule
Roadmap expansion is not implementation. Roadmap expansion is not approval. Roadmap expansion is not evidence of readiness.

Phase 99.5 evidence is documentation, validation output, and working-tree hygiene only. `rg` scans are discovery/evidence only and are not enforcement.

Blocking enforcement remains `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests. Phase 99.5 must not change lint behavior.

## Production human-use staging ladder
Production human use is staged in this exact ladder:

1. Local operator testing
2. Controlled human trial
3. Early human-use candidate
4. Release candidate
5. Production candidate
6. Public/general use

The ladder separates local evidence, controlled human evidence, release-candidate evidence, production-candidate posture, and public/general use.

## Phase 100 relationship
Phase 100 remains the immediate Production Candidate gap audit and readiness decision gate.

Phase 100 does not equal production. It does not approve production human use, release-candidate readiness, public usability, startup/package readiness, or public/general use. It decides only whether evidence supports a production-candidate branch/tag or whether more hardening and planning are required.

Phase 99.5 does not start Phase 100.

## Phases 101-120 planned expansion
Phases 101-120 are planned truth only. They do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.

| Phase | Title | Boundary | Goal |
| --- | --- | --- | --- |
| Phase 101 | Production Use Gap Decomposition | Audit/planning only. | Decompose remaining blockers for human use. |
| Phase 102 | Human Operator Workflow Contract | Documentation/contract only. | Define operator workflows, roles, and expected states. |
| Phase 103 | UI Runtime Review Surface Activation Boundary | UI usability only; no Rust authority and no live mutation. | Activate the browser review surface for human operator visibility. |
| Phase 104 | UI-to-Rust Local Transport Prototype Boundary | Local transport prototype only; no provider execution and no persistence authority. | Prototype local UI-to-Rust communication under non-authoritative constraints. |
| Phase 105 | Transport Abuse Hardening for Live Local Bridge | Hardening only; no broad capability. | Harden the live local bridge against malformed, spoofed, replayed, or hostile transport input. |
| Phase 106 | Provider Configuration Contract | Configuration contract only; no live provider execution. | Define provider configuration contracts without executing providers. |
| Phase 107 | Provider Execution Sandbox Boundary | Bounded provider execution only; provider output remains untrusted. | Introduce bounded provider execution under sandboxed constraints. |
| Phase 108 | Provider Timeout and Resource Limit Boundary | Provider hardening only; no promotion authority. | Add provider timeout and resource-limit enforcement. |
| Phase 109 | Durable Persistence Authority Decision Gate | Decision/audit only. | Determine whether local persistence can become authoritative. |
| Phase 110 | Authoritative Persistence Activation Boundary | Narrow persistence authority only if Phase 109 permits it. | Activate authoritative persistence only under Phase 109 evidence constraints. |
| Phase 111 | Recovery Lifecycle Hardening | Recovery lifecycle only; no silent recovery. | Harden recovery lifecycle behavior without silent repair or implicit promotion. |
| Phase 112 | Policy Versioning and Governance Evidence Boundary | Policy/governance versioning only. | Add policy versioning and governance evidence traceability. |
| Phase 113 | Deployment Configuration Contract | Deployment config only; no deployment automation. | Define deployment configuration contracts without deployment automation. |
| Phase 114 | Local Deployment Candidate Boundary | Local deployment candidate only; no public release. | Define a local deployment candidate boundary for controlled testing. |
| Phase 115 | Security Threat Model and Abuse-Case Audit | Security audit only. | Audit threat model, abuse cases, trust boundaries, and residual attack surfaces. |
| Phase 116 | Operator Documentation for Human Trial | Operator docs only; no readiness approval. | Prepare operator documentation for controlled human-trial use. |
| Phase 117 | Human Trial Dry Run | Dry run only; no public availability. | Rehearse controlled human-trial procedures without public availability. |
| Phase 118 | Release Candidate Evidence Assembly | Evidence assembly only; no automatic approval. | Assemble release-candidate evidence without approval. |
| Phase 119 | Production Candidate Reassessment | Decision gate only. | Reassess Production Candidate posture after controlled evidence assembly. |
| Phase 120 | Early Human-Use Candidate Gate | Final gate for controlled human use; not general public release. | Decide whether controlled early human use is permitted. |

## UI activation and local transport sequence
UI activation precedes live transport so the browser review surface can become useful for human operator visibility before any local communication path exists.

Live transport hardening follows live transport prototype because the hardening work must test the concrete local bridge boundary for malformed, spoofed, replayed, or hostile transport input.

## Provider configuration and sandboxing sequence
Provider configuration precedes provider execution so provider selection, configuration, and secret posture remain contractual before execution exists.

Provider sandboxing and resource limit enforcement precede provider authority decisions because provider output remains untrusted and execution must be bounded before any adjacent authority decision is considered.

## Persistence authority decision sequence
Persistence authority requires Phase 109 decision evidence before Phase 110 activation.

Phase 109 is a decision/audit gate. Phase 110 can activate only narrow authoritative persistence explicitly permitted by Phase 109 evidence constraints.

## Deployment and security sequence
Deployment configuration, local deployment candidacy, and security audit are separate gates.

Deployment configuration defines the contract without automation. Local deployment candidacy defines a controlled target without public release. Security audit reviews threat model, abuse cases, trust boundaries, and residual attack surfaces before human-trial documentation and rehearsal.

## Human trial and release-candidate evidence sequence
Human-trial documentation precedes human trial dry run. The dry run precedes release-candidate evidence assembly.

Phase 118 is Release Candidate Evidence Assembly, not Release Candidate approval. Release-candidate evidence remains evidence for review and does not approve release-candidate readiness.

## Production Candidate reassessment sequence
Phase 119 is Production Candidate Reassessment, not automatic approval.

Production Candidate reassessment follows release-candidate evidence assembly and remains a decision gate. It may preserve a blocked posture if evidence is incomplete.

## Early human-use candidate gate
Phase 120 is a controlled early human-use gate, not general public release.

The gate can decide only whether controlled early human use is permitted under future evidence. It does not approve public/general use.

## Roadmap/changelog truth posture
Roadmap remains planned truth.

CHANGELOG.md remains historical truth.

`docs/roadmap/phase-map.md` is the compact planned phase index. `docs/roadmap/phases.md` is the active expanded planning catalog. `docs/roadmap/sequencing.md` is ordering rationale. `CHANGELOG.md` records completed accepted work only after a phase is accepted.

## Non-approval guarantees
Phase 99.5 does not approve production human use.

Phase 99.5 does not approve Production Candidate status.

Phase 99.5 does not approve release-candidate readiness.

Phase 99.5 does not approve public usability.

Phase 99.5 does not approve startup/package readiness.

Phase 99.5 does not approve release, distribution, installer, signing, publishing, auto-update, package, or public/general use posture.

## Validation evidence
Phase 99.5 validation evidence is expected from the full existing suite after final documentation edits:

- `./scripts/check.sh`
- `cargo test --manifest-path core/Cargo.toml --all-targets`
- `cargo test --manifest-path core/Cargo.toml golden --all-targets`
- `cargo test --manifest-path core/Cargo.toml adversarial --all-targets`
- `cd ui && npm run test:api`
- `node scripts/test_rust_boundary_lint.mjs`
- `node scripts/rust_boundary_lint.mjs`
- `node scripts/test_lint_ui_boundaries.mjs`
- `node scripts/lint_ui_boundaries.mjs`
- `cd ui && npm run typecheck && npm run lint && npm run build`
- `cargo run --manifest-path core/Cargo.toml -- dry-run`
- `cargo build --manifest-path core/Cargo.toml`
- `cd ui && npm run build`

No validation command may print assertion failure, panic, traceback, failed assertion, partial pass count, or masked failure while returning 0. If such a validation-gate defect is found, it must be documented and deferred rather than repaired in Phase 99.5.

## AST/boundary lint parity
Phase 99.5 does not rely on `rg` scans as enforcement. `rg` scans are discovery/evidence only.

Blocking enforcement remains `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

Phase 99.5 does not change lint behavior. If a lint gap is discovered, it is deferred and not fixed in Phase 99.5.

## Test fidelity
Phase 99.5 is planning/documentation only, so no new Rust or TypeScript tests are expected.

The full existing test/lint/check suite must pass after final documentation edits. If tests are skipped after final edits, Phase 99.5 is not complete.

## Confirmed vs suspected
| Item | Classification | Evidence posture |
| --- | --- | --- |
| Phase 100 is the immediate gate. | Confirmed. | Existing roadmap surfaces identify Phase 100 as the Production Candidate readiness decision gate. |
| Post-100 roadmap coverage was absent before this phase. | Confirmed. | Pre-edit roadmap surfaces ended at Phase 100. |
| Phases 101-120 are planned truth only. | Confirmed. | Phase 99.5 roadmap edits define them as planned entries without implementation. |
| Production human use is ready. | Not confirmed. | Explicitly not approved by Phase 99.5. |

## Deferred items
| Item | Reason | Later owner |
| --- | --- | --- |
| Phase 100 Production Candidate gap audit and readiness decision. | Phase 99.5 does not start Phase 100. | Phase 100. |
| Implementation of Phases 101-120. | Phase 99.5 is planning and alignment only. | Future phases only if authorized. |
| Any lint behavior changes. | Phase 99.5 must not change lint behavior. | Separate maintenance phase if required. |
| Any runtime, transport, provider, persistence, deployment, release, or production approval work. | Explicitly outside Phase 99.5. | Later evidence-gated phases. |

## Non-readiness statement
Production human use is not approved.

Production Candidate status is not approved.

Release-candidate readiness is not approved.

Public usability is not approved.

Startup/package readiness is not approved.

Phases 101-120 are planned truth only and do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.
