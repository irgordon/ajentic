---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 184.S - OOB Security Audit and Release-Authority Boundary Review

## Scope
Audit scope covered repository code and governance/operations surfaces requested for Phase 184.S with repository-truth priority over prior summaries.

## Evidence rule
Repository truth wins. Claims were accepted only when supported by current code/tests/scripts/docs in this repository state.

## Repository state reviewed
- Branch state reviewed from live working tree.
- Required scans executed across `core/src`, `ui/src`, `tests`, `core/tests`, docs, workflows, scripts, and package manifests.
- Validation commands executed after audit edits.

## Threat model
Primary threat: authority drift where model/provider output or stringly state could cross trust boundaries into approval/deployment/signing/publishing/action authorization/replay-repair/recovery-promotion/public-use authority.

## Authority-boundary findings
- Core authority-denial surfaces remain explicit and fail-closed for release/deployment/signing/publishing/public-use/action/replay/recovery/readiness claims via typed booleans and rejection reasons.
- Provider output remains classified as untrusted/non-authoritative in core and UI projections.
- No direct code path found where provider/model output sets approval/deployment/signing/publishing/authorization to true without denial.

## Execution-boundary findings
- No arbitrary shell execution path found in reviewed runtime surfaces; no `Command::new` usage found in `core/src/api/**`.
- Provider execution remains bounded to local deterministic/allowlisted contracts in reviewed surfaces.
- No WebSocket/fetch/XMLHttpRequest/browser remote execution path found in `ui/src/api/**` beyond local parsing/projection logic.

## Secret-boundary findings
- Secret-marker rejection logic is present in runtime/provider/operator shell boundaries.
- `std::env` reads exist primarily for CLI args/current_dir/temp_dir and test temp paths, not as credential loading in authority paths.
- No API key/token/private key/certificate/KMS loading behavior found in authority-sensitive runtime paths.

## Release-boundary findings
- Release/signing/publishing/deployment/public-distribution statuses are represented as denied/false by default in core structs and validated as prohibited claims in parsing/validation paths.
- No artifact publishing/tagging/release automation behavior found in runtime code paths.

## Persistence/replay findings
- Persistence and replay surfaces include deterministic validation and denial categories for replay repair/recovery promotion authority.
- No path found that repairs/re-writes history to force preferred authority outcomes.

## UI honesty findings
- UI copy repeatedly states non-authoritative, local-only, untrusted output boundaries and no approval/deployment/signing/publishing/public-use effects.
- No reviewed UI copy found that asserts real approval/release/deployment authority.

## String/status heuristic findings
- Multiple surfaces still use substring/status-string heuristics (`contains`, `starts_with`, lowercasing, split parsing) for classification/blocker detection.
- Risk remains that future wording changes could create brittle authority-adjacent behavior despite current deny-lists.

## Test-integrity findings
- Strong adversarial and integration coverage exists for forbidden claims and authority denials.
- Residual gap: some tests validate wording/summary text and string markers rather than purely typed behavior, increasing drift risk.

## Documentation-drift findings
- No critical contradiction found between code/tests and current non-authoritative boundary claims in reviewed docs.
- Checklist/changelog require update to record Phase 184.S audit state and pause recommendation.

## Supply-chain findings
- No active publish/deploy/signing automation surfaced in reviewed scripts/workflows/manifests during scans.
- Boundary-lint scripts include detection tokens for risky APIs; no evidence of enabled publish pipeline in requested surfaces.

## Critical findings
- None.

## High findings
1. **Stringly authority-adjacent classification remains brittle**: several core/UI areas rely on substring/status text interpretation (`contains("rejected")`, denial-marker scanning, lowercased token scans) that could regress under future copy/label drift.

## Medium findings
1. **Test strategy partially wording-coupled**: some checks assert summary text/marker presence and may miss semantic drift if wording remains plausible.
2. **Large boundary files increase review complexity**: very large API/UI files raise change-risk for subtle authority drift.

## Low findings
1. Historical/prohibition-heavy docs and changelog content create scan noise and manual review burden.

## False positives / accepted risk
- Scan hits in docs/tests for forbidden labels and release/signing terms are largely intentional fixtures/prohibitions.
- `std::env::temp_dir`/`args` usage is present but not credential authority by itself.

## Required repairs before release continuation
1. Replace authority-adjacent substring heuristics in critical classifiers with typed enums/structured parsers where feasible.
2. Add adversarial tests targeting status-string mutation/casing/token-boundary attacks for release/deployment/signing/approval claims.
3. Reduce wording-coupled assertions in core authority tests; prefer typed contract assertions.

## Rebuild trigger assessment
No rebuild trigger found.

## Recommendation
`release_path_should_pause_for_security_repairs`

## Validation output
- `git diff --check` passed.
- `CARGO_TARGET_DIR=/tmp/ajentic-security-audit-target ./scripts/check.sh` passed.
- Post-validation `git status --short` shows only Phase 184.S audit documentation/checklist/changelog edits.

## Zero-drift statement
This phase performed out-of-band audit documentation/checklist/changelog updates only. No Rust source, TypeScript source, runtime behavior, release artifact creation, signing, publishing, deployment, public distribution, or production/public-use approval behavior was modified.
