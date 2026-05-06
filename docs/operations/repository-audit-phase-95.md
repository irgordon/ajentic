---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Repository Audit - Phase 95

## Scope
Phase 95 is a roadmap and hardening-depth alignment checkpoint only.

Phase 95 reconciles committed evidence from Phases 91, 92, 92.5, 93, 93.5, and 94 to decide whether startup/package work may proceed or whether the smallest necessary out-of-band hardening/testing phases must be inserted first.

Phase 95 does not implement runtime behavior.

Phase 95 does not add tests.

Phase 95 does not repair tooling.

Phase 95 does not begin Phase 96 or any startup/package work.

Phase 95 does not approve Production Candidate status.

## Evidence rule
Phase 95 counts committed evidence only:

- unit tests
- integration tests
- root integration tests
- Rust boundary lint
- UI AST lint
- `scripts/check.sh`
- operations docs
- checklists
- source diffs
- validation logs

Plans, intended hardening, architecture rationale alone, future roadmap items, unmerged/non-history agent runs, speculative safety claims, and prompt intent without committed files do not count as closure.

If evidence is missing or mixed, the gate is not approved.

## Decision status model
Each core question uses one of these statuses:

| Status | Meaning |
| --- | --- |
| Sufficient | Committed evidence closes the question for the current boundary. |
| Insufficient | Committed evidence shows a blocker. |
| Conditionally sufficient | Current boundary is closed, but a documented gap must be handled before a later surface. |

## Phase 91 evidence review
**Core question:** Is UI submission/transport spoofing hardened enough before any bridge exists?

**Status:** Conditionally sufficient.

Committed Phase 91 evidence shows a TypeScript submission boundary that rejects malformed submissions, risky authority-escalation text, and user-supplied capability/authority flags before transport eligibility. The evidence also records that live transport, Rust bridge, executable submission, persistence, ledger/audit append, provider execution, replay repair, authority mutation, and application-state mutation remain disabled.

The Phase 91 operations documentation and changelog explicitly state that no UI unit test harness exists and that typecheck/lint/build must not be claimed as behavioral coverage. The current evidence is enough for the pre-bridge TypeScript contract boundary, but it is not broad enough to let startup/package work proceed without a UI behavioral test harness baseline.

**Decision:** Phase 91 closes the current pre-bridge spoofing contract boundary, but Phase 95.1 is required before startup/package usability work.

## Phase 92 and 92.5 evidence review
**Core question:** Is the authorization/audit/action proof chain hardened against mismatch, absence, edge cases, and ordering errors?

**Status:** Sufficient.

Committed Phase 92 evidence covers authorization/audit/action mismatches, missing authorization, missing audit, combined mismatch deterministic reason selection, exact operator/action/request identity checks, non-execution, non-persistence, non-append, non-replay-repair, non-provider-trust, and no real-world effect on rejection.

Committed Phase 92.5 evidence expands proof-chain edge coverage for ordering and exactness seams without adding action authority. It preserves proof-only posture and keeps rejected requests fail-closed and diagnostic-only.

Stale proof lifecycle remains explicitly deferred because the current proof objects do not contain lifecycle/expiry fields. Phase 95 does not count that as solved. Within the current proof-chain boundary, the committed unit and root integration evidence is sufficient.

**Decision:** Phase 92/92.5 are sufficient for the current proof-chain scope. Stale proof lifecycle must remain deferred and must not be described as closed.

## Phase 93 and 93.5 evidence review
**Core question:** Is persistence corruption, append drift, export boundary, and recovery mismatch hardened with fail-closed behavior and no repair/authority leakage?

**Status:** Sufficient.

Committed Phase 93 evidence covers corrupted persisted records, checksum drift, missing audit/ledger payloads, malformed revisions, stale revision chains, transaction ID mismatch, tampered bytes, partial-write posture, failed writes, verification failures, audit-only/ledger-only transactions, and non-promotion/non-recovery/non-repair reports.

Committed Phase 93.5 evidence expands semantics-edge coverage for unsupported payload/envelope forms, unsupported transaction-kind input, duplicate fields, out-of-order revision drift, write-time-only append verification, corrupted reads, export bytes not verifying as append transactions, paired audit/ledger append requirements, export-not-ledger, export-not-recovery-input, export-not-replay-repair, expected recovery context matching, mismatch non-replacement, and non-repair posture.

Continuous integrity monitoring and concurrent writers remain unsupported and deferred. Within the current persistence/export/recovery boundary, the committed unit and root integration evidence is sufficient.

**Decision:** Phase 93/93.5 are sufficient for current persistence/export/recovery semantics. Continuous monitoring and concurrent-writer support are not solved and remain deferred.

## Phase 94 evidence review
**Core question:** Is provider-output injection, replay tampering, failure-trace spoofing, and retry escalation hardened so LLM output cannot be misinterpreted as authority?

**Status:** Conditionally sufficient.

Committed Phase 94 evidence covers provider-output injection text, replay evidence tampering, failure-trace spoofing, retry escalation attempts, reason-code-over-text behavior, provider-untrusted invariants, replay verification-only invariants, retry non-scheduling invariants, and non-authority flags. Root integration evidence covers provider output remaining non-authoritative, replay tampering rejecting without side effects, and failure trace spoofing not scheduling retry.

The current Phase 94 boundary is closed for the committed local harness and replay surfaces. However, adversarial fixture depth is still concentrated in current public harness/replay APIs. Before startup/package work broadens operator paths, a dedicated adversarial corpus pass should cover provider output, replay evidence, failure trace, export summary, and operator intent surfaces more systematically.

**Decision:** Phase 94 is conditionally sufficient for current provider/replay/failure spoofing scope. Phase 95.3 is required before startup/package usability work.

## Residual authority-seam review
**Core question:** Are there remaining paths where LLM output, replay evidence, export files, recovery bytes, or UI submission data could be misinterpreted as authority?

**Status:** Insufficient.

Committed evidence strongly protects the current seams independently: UI submissions are preview-only, proof-chain requests reject mismatches, persistence/export/recovery inputs fail closed, and provider/replay/failure text remains non-authoritative. The remaining gap is not a known live authority leak in committed runtime behavior; it is the absence of one deterministic cross-boundary golden invariant harness proving that the same representative input stays non-authoritative across UI projection/read-model surfaces, Rust local workflow, replay evidence, observability snapshot, export bytes, and recovery rejection boundaries.

Because Phase 95 places the burden of proof on proceeding, this missing end-to-end invariant evidence blocks Phase 96.

**Decision:** Phase 95.2 is required before startup/package usability work.

## AST/boundary lint parity review
**Core question:** Do AST/boundary lints still match the live code shape, or are there uncovered patterns?

**Status:** Conditionally sufficient.

Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy where wired, and tests. `rg` scans are discovery/evidence only and are not enforcement.

The lint scripts and self-tests remain committed and wired. Phase 95 did not find a concrete new live source pattern requiring an immediate lint rule change. However, the hardening block added more typed boundary surfaces, export semantics, replay evidence, and UI submission shapes. A later lint expansion phase should be inserted only if Phase 95.1, Phase 95.2, or Phase 95.3 exposes a concrete uncovered source pattern.

**Decision:** Lint parity is conditionally sufficient for current live code shape. Phase 95.4 is recommended as conditional follow-up only if a concrete uncovered pattern is found.

## UI behavioral test harness review
**Core question:** Does the absence of a UI behavioral test harness constitute a blocker that requires an out-of-band testing phase before startup/package work?

**Status:** Insufficient.

Yes. Phase 91 intentionally did not add package dependencies or a UI unit/behavioral test harness, and it explicitly avoided claiming typecheck/lint/build as behavioral coverage. That was acceptable for the Phase 91 contract-layer hardening boundary, but it is not enough proof before startup/package usability work.

**Decision:** Phase 95.1 - Out-of-Band UI Behavioral Test Harness Baseline is required before Phase 96.

## Cross-boundary golden invariant review
**Core question:** Are cross-boundary deterministic invariants tested end-to-end before startup/package work?

**Status:** Insufficient.

No single committed deterministic golden invariant test currently follows the same input through the same harness, same replay evidence, same observability posture, same export bytes, and same recovery/export rejection posture. Existing root integration tests cover important public API seams, but the burden of proof for startup/package work requires a cross-boundary golden invariant baseline.

**Decision:** Phase 95.2 - Out-of-Band Cross-Boundary Golden Invariant Tests is required before Phase 96.

## Adversarial LLM-output corpus review
**Core question:** Is adversarial LLM-output fixture depth sufficient or does it need a dedicated out-of-band pass?

**Status:** Insufficient.

Phase 94 has useful adversarial text coverage for provider output, replay tampering, failure trace spoofing, retry escalation, and reason-code-over-text behavior. The corpus is still shallow across all surfaces that startup/package work would expose to operators: provider output, replay evidence, failure trace, export summary, operator intent, risky text, and UI submission text.

**Decision:** Phase 95.3 - Out-of-Band LLM Output Adversarial Corpus Hardening is required before Phase 96.

## Phase 96 gate decision
**Core question:** Is Phase 96 allowed to start, or should 95.1/95.2/95.3/95.4 intermediate hardening phases be inserted?

**Status:** Insufficient.

Phase 96 is not approved to start.

Startup/package work is not approved because UI behavioral coverage is missing, cross-boundary golden invariant tests are missing, and adversarial LLM-output corpus depth is not broad enough for provider, replay, failure trace, export, operator intent, and UI submission surfaces.

Phase 95.4 is not mandatory from this audit alone because no concrete uncovered lint pattern was found, but it remains the correct smallest follow-up if the inserted testing phases expose lint parity drift.

## Recommended intermediate phases
Preferred order:

1. **Phase 95.1 - Out-of-Band UI Behavioral Test Harness Baseline**  
   Boundary: UI contract testing only; no live transport, no UI authority.
2. **Phase 95.2 - Out-of-Band Cross-Boundary Golden Invariant Tests**  
   Boundary: deterministic tests only; no new capability.
3. **Phase 95.3 - Out-of-Band LLM Output Adversarial Corpus Hardening**  
   Boundary: adversarial fixtures/tests only; no provider execution or authority change.
4. **Phase 95.4 - Out-of-Band Boundary Lint Coverage Expansion**  
   Boundary: validation tooling only; no runtime behavior. Insert only if concrete uncovered patterns are found.

These are planning recommendations only. Phase 95 does not implement any follow-up phase.

## Roadmap/changelog alignment
Roadmap remains planned truth.

`CHANGELOG.md` remains historical truth.

Roadmap files still identify Phase 95 as the alignment checkpoint after Phases 91-94 and Phase 96 as the planned local startup command boundary. Phase 95 did not modify roadmap files because the audit found no small correction required.

Plans do not count as closure. Architecture rationale alone does not count as closure.

## Production Candidate status
Production Candidate status: not approved.

Phase 95 does not approve Production Candidate status, public usability, production readiness, release-candidate readiness, or startup/package work.

## Required follow-ups
| Follow-up | Required before Phase 96? | Reason |
| --- | --- | --- |
| Phase 95.1 UI Behavioral Test Harness Baseline | Yes | UI submission behavior lacks actual behavioral test coverage. |
| Phase 95.2 Cross-Boundary Golden Invariant Tests | Yes | Same-input deterministic invariants are not tested end-to-end across current boundaries. |
| Phase 95.3 LLM Output Adversarial Corpus Hardening | Yes | Adversarial text fixture depth is shallow across provider, replay, failure trace, export, operator intent, and UI surfaces. |
| Phase 95.4 Boundary Lint Coverage Expansion | Conditional | Needed only if concrete uncovered patterns are found by the inserted testing phases or later lint review. |

## Deferred items
| Item | Reason |
| --- | --- |
| Stale proof lifecycle and expiry semantics | Current proof objects do not contain lifecycle/expiry fields; Phase 92/92.5 must not be treated as solving this. |
| Continuous persistence integrity monitoring | Phase 93/93.5 explicitly preserve write-time verification and do not add continuous monitoring. |
| Concurrent writers | Phase 93/93.5 preserve single-writer assumptions. |
| Live transport or Rust bridge | Phase 91 and Phase 95 do not add live transport. |
| Provider execution | Phase 94 and Phase 95 do not execute providers. |
| Startup/package implementation | Blocked until required out-of-band evidence phases close. |
| Production Candidate approval | Reserved for a later decision gate with complete evidence. |

## Confirmed vs suspected
### Confirmed
- Phase 95 is audit and planning only.
- Phase 95 counts committed evidence only.
- Phase 95 did not implement runtime behavior, tests, tooling repairs, startup, packaging, or follow-up phases.
- Phase 91 is conditionally sufficient for the current pre-bridge UI contract boundary.
- Phase 92/92.5 are sufficient for the current proof-chain boundary.
- Phase 93/93.5 are sufficient for the current persistence/export/recovery boundary.
- Phase 94 is conditionally sufficient for the current provider/replay/failure spoofing boundary.
- UI behavioral harness coverage is missing and blocks Phase 96.
- Cross-boundary golden invariant evidence is missing and blocks Phase 96.
- Adversarial LLM-output corpus breadth is insufficient before startup/package work.
- No concrete new uncovered lint source pattern was found during Phase 95.
- Phase 96 is not approved to start.
- Production Candidate status is not approved.

### Suspected
- Phase 95.1, Phase 95.2, or Phase 95.3 may reveal concrete lint parity gaps. If they do, Phase 95.4 should be inserted before implementation continues.

## Non-readiness statement
AJENTIC is not approved for Production Candidate status.

AJENTIC is not approved for startup/package usability work.

Phase 96 is not approved to start from Phase 95 evidence.

The smallest necessary next work is out-of-band evidence hardening/testing, beginning with Phase 95.1.
