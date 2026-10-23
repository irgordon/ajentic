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

## Decision and Rationale

### Compile schemas with an established implementation

Use Ajv 8.20.0 with ajv-formats 3.0.1 as locked development dependencies.
Both are MIT-licensed; their locked transitive dependencies are MIT-licensed.
They reuse the existing Node/npm toolchain rather than introducing a second
dependency installation path for Python schema tooling.

The validator loads all repository schemas before compiling them, checks the
Draft 2020-12 metaschema, resolves references from that local catalog, and checks
formats. It has no remote schema loader. Missing references fail instead of
fetching mutable schemas from the network.

Strict keyword validation remains enabled. Strict type-context and required-key
linting are disabled because existing compound schemas inherit those constraints;
metaschema validation and payload constraint evaluation remain enabled.

Valid and invalid payload fixtures exercise the context-budget, memory-provenance,
operator-intent, task-contract, and action-outcome contracts. Every schema is
compiled, but these fixtures do not constitute full behavioral coverage of all
22 schemas. Self-tests also prove rejection of invalid schemas, unknown references,
duplicate identities, malformed cases, and incorrect expected outcomes. The CLI
must return a nonzero status on failure.

### Separate linting from typechecking

Use the existing TypeScript AST boundary linter and its self-tests for
`npm run lint`. Keep `npm run typecheck` as a separate gate. This makes the lint
command meaningful while retaining established project boundary rules without
adding a competing general-purpose style configuration.

### Validate workflow definitions independently

The Actions Scripts job runs actionlint 1.7.12, including its embedded shell
checks. The downloaded Linux binary is checked against a pinned SHA-256 before
execution. Workflow/job permissions remain read-only for validation.

RC and final publication conditions use YAML's stripped folded scalar (`>-`).
Trailing whitespace outside an expression must not turn an intended prerequisite
gate into an always-true string. The correction preserves the intended gates;
it does not dispatch either publication workflow or add publication permissions.

### Bound subprocesses and preserve useful diagnostics

Artifact dependency installation has a 90-second deadline. Other artifact build
commands have a 600-second deadline. Registry fetches use a 15-second timeout and
one retry; command logging is flushed before execution. Failures propagate.

Builds prefer existing npm cache entries but allow registry retrieval when entries
are missing. Cache availability is an optimization, never a prerequisite for a
clean Actions runner. Audit/funding requests are excluded from artifact builds
because they add unrelated network work; this is not a vulnerability-scan result.

Node.js 22 and Python 3.12 are selected in the relevant Actions jobs. Rust remains
on the existing stable toolchain. The two-build comparison establishes repeatability
within that environment, not across changing compiler versions or operating systems.

### Keep repository checks usable without broad exclusions

Ignore only Finder's `.DS_Store` metadata during structure checks. Other unexpected
root files still fail. A regression fixture exercises each behavior. README setup
installs locked development dependencies before invoking the check wrapper.

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

## References

- [Ajv Draft 2020-12 support](https://ajv.js.org/json-schema.html#draft-2020-12-breaking)
- [Ajv format validation](https://ajv.js.org/guide/formats.html)
- [actionlint v1.7.12 release](https://github.com/rhysd/actionlint/releases/tag/v1.7.12)
- `scripts/validate_schemas.mjs`
- `scripts/test_validate_schemas.mjs`
- `scripts/test_validation_infrastructure.py`
- `scripts/check.sh`
- `.github/workflows/ci.yml`
