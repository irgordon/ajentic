---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# ADR 001: Independently Executable Validation Gates

Date: 2026-10-23

Status: Proposed for review

## Context

AJENTIC needs the same validation contract on developer machines and clean
GitHub Actions runners. JSON syntax checks cannot establish schema correctness;
typechecking cannot substitute for boundary linting. Local OS metadata and
stalled dependency retrieval must not obscure which validation actually ran.

This decision covers validation infrastructure. Outcome aggregation, claim
authority, independent observers, and browser-to-Rust integration are separate
runtime concerns and receive no correctness claim from these gates.

These choices form one executable validation baseline: dependency setup, schema
checks, language/workflow linting, process limits, and repository hygiene must
agree for the same check wrapper to work locally and on clean Actions runners.
That shared execution contract is why they are grouped here. Runtime authority
changes have a different acceptance contract and remain outside this ADR.

## Decision and Rationale

### Compile schemas with an established implementation

Use Ajv 8.20.0 with ajv-formats 3.0.1 as locked development dependencies.
The validator explicitly constructs `Ajv2020` from `ajv/dist/2020.js`, not Ajv's
default draft-07 export. This class supplies the Draft 2020-12 metaschema.
Ajv and ajv-formats are MIT-licensed. Shipped dependency licenses are reviewed
separately from package metadata; they are not uniformly MIT.
They reuse the existing Node/npm toolchain rather than introducing a second
dependency installation path for Python schema tooling.

The validator loads all repository schemas before compiling them, checks the
Draft 2020-12 metaschema, resolves references from that local catalog, and checks
formats. It has no remote schema loader. Missing references fail instead of
fetching mutable schemas from the network.

Ajv uses `strict: true`, including `strictTypes` and `strictRequired`. Initial
strict-type diagnostics identified only the `then.properties.evidence_refs` and
`then.properties.evidence_digests` clauses in `action-outcome.schema.json`, also
reachable through `run-trace.schema.json`. Each clause now repeats its already
inherited array type. No accepted payload shape changes and no strictness waiver
remains. Self-tests fail if either strictness setting is disabled.

Valid and invalid payload fixtures exercise the context-budget, memory-provenance,
operator-intent, task-contract, and action-outcome contracts. Every schema is
compiled, but these fixtures do not constitute full behavioral coverage of all
22 schemas. `tests/schema-coverage.json` lists every schema identity, marks the
5 payload-tested schemas and 17 compile-only schemas, and records each remaining
coverage gap. The gate rejects new schemas without an entry, stale entries,
unsupported statuses, and payload-tested entries without both positive and
negative cases. Compile-only status is explicit debt, not behavioral assurance.
Self-tests also prove rejection of invalid schemas, unknown references,
duplicate identities, malformed cases, and incorrect expected outcomes. The CLI
must return a nonzero status on failure.

### Separate linting from typechecking

Use the existing TypeScript AST boundary linter and its self-tests for
`npm run lint`. Keep `npm run typecheck` as a separate gate. This makes the lint
command meaningful while retaining established project boundary rules without
adding a competing general-purpose style configuration.
This command checks boundary rules only; it does not enforce formatting, general
style, or independent correctness of runtime authority.

### Validate workflow definitions independently

The Actions Scripts job runs actionlint 1.7.12, including its embedded shell
checks. The downloaded Linux binary is checked against a pinned SHA-256 before
execution. Workflow/job permissions remain read-only for validation.

RC and final publication conditions use YAML's stripped folded scalar (`>-`).
Trailing whitespace outside an expression must not turn an intended prerequisite
gate into an always-true string. The correction preserves the intended gates;
it does not dispatch either publication workflow or add publication permissions.
Actionlint's `if-cond` diagnostic is the regression guard. Python fixture tests
assert that `>` with a trailing newline is rejected and `>-` is accepted. The
Actions Scripts job installs actionlint before executing those fixtures; they
skip explicitly when the binary is unavailable in another environment.

### Bound subprocesses and preserve useful diagnostics

Artifact dependency installation has a 90-second deadline. Other artifact build
commands have a 600-second deadline. Registry fetches use a 15-second timeout and
one retry; command logging is flushed before execution. Failures propagate.
All three deadlines are provisional engineering defaults, not measured cold-runner
percentiles. They trade bounded failure during registry outages against tolerance
for a small dependency set. Local cached installs took roughly 0.17 seconds and
release builds roughly 4 seconds, but those are warm developer-machine observations,
not evidence that a cold Actions install fits 90 seconds. In Actions run
`40015237376`, the UI dependency-install step took about one second and the full
validation step took 219 seconds. Cache provenance was not established, so these
observations are not a cold-install percentile or a reason to treat the defaults
as calibrated. Cold-runner timing and
false-timeout frequency must be reviewed after remote execution becomes available;
increase the versioned limits if that evidence warrants it.

Builds prefer existing npm cache entries but allow registry retrieval when entries
are missing. Cache availability is an optimization, never a prerequisite for a
clean Actions runner. Audit/funding requests are excluded from artifact builds
because they add unrelated network work; this is not a vulnerability-scan result.
There is currently no dedicated dependency vulnerability-audit gate elsewhere in
the workflows. The license-inventory gate below is not its replacement and does
not assert that dependencies are vulnerability-free. A vulnerability-audit gate
remains a separate follow-up decision.

Node.js 22.23.4 and Python 3.12 are selected in the relevant Actions jobs. Rust remains
on the existing stable toolchain. The two-build comparison establishes repeatability
within that environment, not across changing compiler versions or operating systems.

| Input | Pinning level and remaining variability |
| --- | --- |
| npm dependencies | Exact resolved versions and integrity hashes in `ui/package-lock.json`. |
| Node.js | Exact 22.23.4 primary/build pin and exact 24.22.1 compatibility pin; updates require an explicit change. |
| Python | Minor version 3.12 where setup-python is used; patch can change. Other jobs use runner Python. |
| Rust | Moving `stable` channel; no exact compiler release pin. |
| Runner | Moving `ubuntu-latest` image. |
| Third-party Actions | Existing `uses:` major-version or `stable` tags; not commit-SHA pinned. |
| actionlint binary | Exact 1.7.12 archive and committed SHA-256. |

Toolchain/image updates can change artifacts or expose new nondeterminism. The
comparison does not promise identical output across workflow runs with different
toolchains. The binary checksum protects that download's content; it does not
make mutable `uses:` references or the entire supply chain immutable. Pinning
third-party Actions to reviewed commit SHAs is recorded follow-up work.

The Actions Rust 1.99 Clippy run identified `chunks_exact_to_as_chunks` diagnostics
in the SHA-256 infrastructure helpers, while local Rust 1.96 did not report them.
Use equivalent fixed-array chunk iteration rather than suppressing warnings or
downgrading CI. Both forms discard incomplete trailing chunks and preserve the
same word/block order; existing SHA-256 vectors and the Rust suites validate the
behavior-preserving update. This is toolchain compatibility, not an authority change.

### Enforce the supported Node security floors

The minimum supported LTS releases are 22.23.4 within Node 22 and 24.22.1 within
Node 24, based on the Node.js security releases dated 2026-10-21. Package engines
declare `^22.23.4 || ^24.22.1`; the runtime-policy self-tests require the lockfile
and every explicit workflow Node pin to agree with that policy.

Dependency installation has a preinstall hook; UI commands invoke the preflight
directly so disabling npm lifecycle hooks cannot bypass their check. The full
wrapper also invokes `scripts/check_node_version.mjs`. Older patches, prereleases, and
unlisted major lines are rejected. Rejecting an unlisted major denotes absence
of a supported project policy, not a claim that every such major is vulnerable.
This is a development prerequisite check, not runtime execution authority.

Primary CI, artifact, RC, and final workflow build steps pin 22.23.4. The separate
Node 24 compatibility job uses 24.22.1 for UI and contract checks. The existing
required Full deterministic check depends on that job and uses `always()` plus
an explicit prerequisite-status check. Failed, skipped, cancelled, missing, and
unknown results fail the aggregate instead of allowing it to be skipped. Fixtures
exercise those status decisions. Python, Rust, runner images, and Action references
retain the variability recorded above.

### Keep repository checks usable without broad exclusions

Ignore only Finder's `.DS_Store` metadata during structure checks. Other unexpected
root files still fail. The exemption is recursive, including root, docs, and deeper
directories; it is not an exemption for all hidden files. A fixture asserts root,
docs, and docs/architecture metadata acceptance and another asserts unexpected
root-file rejection. README setup
installs locked development dependencies before invoking the check wrapper.

### Review dependency licenses against shipped notices

`node scripts/audit_dependency_licenses.mjs` checks every locked dependency's
installed version and declared license, then hashes its shipped license file
against `docs/contracts/development-license-review.json`. A new dependency,
version change, or license-text change requires an explicit review update.
This is a reviewed notice inventory, not an automatic legal interpretation or
an exhaustive audit of bundled third-party notices.

The audit command and `npm ls --prefix ui --all --long --json` give the following
inventory. The fast-uri discrepancy is retained explicitly rather than rewriting
upstream metadata in the lockfile.

| Locked package | Declared metadata | Reviewed main license file |
| --- | --- | --- |
| ajv 8.20.0 | MIT | MIT |
| ajv-formats 3.0.1 | MIT | MIT |
| fast-deep-equal 3.1.3 | MIT | MIT |
| fast-uri 3.0.1 | MIT | BSD-3-Clause |
| json-schema-traverse 1.0.0 | MIT | MIT |
| require-from-string 2.0.2 | MIT | MIT |
| typescript 5.9.3 | Apache-2.0 | Apache-2.0 |

The shipped `fast-uri/LICENSE` contains the three BSD redistribution conditions
despite MIT metadata. Retain that notice. Metadata-derived SBOM fields alone do
not resolve this discrepancy; the reviewed inventory records it separately.

## Alternatives Considered

| Alternative | Reason not selected |
| --- | --- |
| Continue parsing JSON only | Cannot detect invalid schema keywords, broken references, or payload-contract regressions. |
| Implement a custom schema validator | Duplicates a complex standard and creates an additional correctness boundary. |
| Add Python jsonschema dependencies | Valid alternative, but creates a second dependency installation path where npm already exists. |
| Add ESLint for this change | Adds a broader rule system without resolving the existing boundary-lint command ambiguity. |
| Force offline artifact builds | Would require pre-seeded caches and prevent independent clean-runner validation. |
| Ignore all untracked or hidden files | Could hide real structure violations along with OS metadata. |
| Combine validation and runtime authority fixes | Makes the validation baseline harder to review and would conflate contract-shape checks with independent evidence verification. |

## Consequences and Limits

- Validation has additional development dependencies, pinned through the lockfile.
- Clean runners require registry access; timeouts produce explicit failures.
- Workflow linting requires actionlint locally; Actions installs a checksum-pinned binary.
- Contract-shape validation does not establish real-world truth or repair the known Rust authority defects.
- Browser automation, full Rust/schema equivalence, and cross-toolchain reproducibility are outside this decision.
- Passing CI remains validation evidence, not release or production approval.

## Validation Contract

The schema gate runs `node scripts/test_validate_schemas.mjs` and
`node scripts/validate_schemas.mjs`. The Scripts gate runs the Python infrastructure
regressions, actionlint, and shell syntax checks. UI lint, typecheck, build, and API
tests run separately. The full clean-tree wrapper runs schema, artifact, integrity,
bundle, preflight, UI, and Rust checks. GitHub Actions repeats the applicable checks
from the committed branch rather than relying on developer-provided pass claims.

Schema and license-inventory self-tests run in the existing `Schemas` job. Workflow
lint and its condition fixtures run in `Scripts`. The full wrapper also checks the
coverage manifest and license inventory.

### Merge Enforcement

The historical settings below are superseded by the Owner-selected
solo-maintainer policy in `docs/governance/GOVERNANCE.md` and ADR 002:
required CI applies to administrators, with zero required approving reviews.

Workflow definitions alone do not make jobs mandatory. The public branch API for
`irgordon/ajentic` on 2026-10-23 reported protected `main` with these required check
names: Full deterministic check, TypeScript UI, Schemas, Scripts, Rust, Repository
validation, Internal candidate artifact reproducibility, and Memory Placement Gate.
The change preserves these names and places the new checks inside those required jobs.

The reported enforcement level was `non_admins`. This evidence does not establish
that administrators cannot bypass checks, nor verify every approval/ruleset setting.
No protection settings are changed by this decision. Remote execution is an
acceptance gate and must be verified against the relevant source head.

PR #300 records nine successful Actions checks for source head
`becc111f2aee2c8d854a79d6dc8a1992813d3f6b`: CI run `40015237376`, internal artifact
reproducibility run `40015237309`, and memory-placement run `40015237314`.
The preceding run exposed the floating-toolchain Clippy difference described
above; the failed aggregate remained blocking until that concrete lint defect
was fixed. Passing checks do not accept or merge the draft PR, resolve the
separate Rust authority findings, or approve a release.

## Tool Pin Verification

The following checks were made on 2026-10-23:

- Ajv 8.20.0: exact npm version metadata and tarball integrity match the lockfile;
  the upstream v8.20.0 GitHub release exists.
- ajv-formats 3.0.1: exact npm version metadata and tarball integrity match the
  lockfile. The version is verified through npm; a GitHub Release entry is not
  assumed (the v3.0.1 Release endpoint returned 404).
- actionlint 1.7.12: the release page/API identifies the Linux amd64 asset. Its
  downloaded SHA-256, upstream checksums file, API digest, and CI pin all equal
  `8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8`.

Reproduce version checks against `https://registry.npmjs.org/ajv/8.20.0` and
`https://registry.npmjs.org/ajv-formats/3.0.1`, comparing `dist.integrity` to the
corresponding lockfile entries. For actionlint, download the named release archive
and checksums file and compare `shasum -a 256` output with the committed CI value.

## References

- [Ajv Draft 2020-12 support](https://ajv.js.org/json-schema.html#draft-2020-12-breaking)
- [Ajv format validation](https://ajv.js.org/guide/formats.html)
- [actionlint v1.7.12 release](https://github.com/rhysd/actionlint/releases/tag/v1.7.12)
- [Ajv v8.20.0 release](https://github.com/ajv-validator/ajv/releases/tag/v8.20.0)
- [Ajv 8.20.0 npm metadata](https://registry.npmjs.org/ajv/8.20.0)
- [ajv-formats 3.0.1 npm metadata](https://registry.npmjs.org/ajv-formats/3.0.1)
- [actionlint upstream checksums](https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_checksums.txt)
- [main branch protection evidence endpoint](https://api.github.com/repos/irgordon/ajentic/branches/main)
- [PR #300](https://github.com/irgordon/ajentic/pull/300)
- [Independent CI evidence](https://github.com/irgordon/ajentic/actions/runs/40015237376)
- [Independent reproducibility evidence](https://github.com/irgordon/ajentic/actions/runs/40015237309)
- [Node.js 22.23.4 security release](https://nodejs.org/en/blog/release/v22.23.4)
- [Node.js 24.22.1 security release](https://nodejs.org/en/blog/release/v24.22.1)
- [GitHub skipped-job status behavior](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-jobs-with-conditions)
- `docs/contracts/development-license-review.json`
- `tests/schema-coverage.json`
- `scripts/validate_schemas.mjs`
- `scripts/test_validate_schemas.mjs`
- `scripts/test_validation_infrastructure.py`
- `scripts/check.sh`
- `.github/workflows/ci.yml`
