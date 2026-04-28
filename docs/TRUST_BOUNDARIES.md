# Trust Boundaries

## Generated output is untrusted

All machine-generated output is untrusted by default.

## Trust boundary summary

| Component | Trust level |
|-----------|-------------|
| Rust core | Authoritative |
| Python adapters | Non-authoritative (advisory) |
| TypeScript UI | Non-authoritative (display only) |
| Bash scripts | Non-authoritative (glue only) |
| LLM output | Untrusted until governed |

## Phase 4 adapter boundary

- Rust owns adapter request construction, subprocess invocation, and adapter response validation.
- Python owns deterministic mock adapter behavior only.
- A completed adapter response is not a passing candidate.
- Candidate creation and runtime governance checks are reserved for later phases.

## Prohibited trust escalations

- Adapter output must not be treated as candidate promotion approval.
- Missing or unknown adapter statuses must not be treated as governance pass.
- No bypass flags may override governance decisions.
