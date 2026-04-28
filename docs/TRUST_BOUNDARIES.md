# Trust Boundaries

This file reserves the location for the AJENTIC trust boundary documentation.

## Generated output is untrusted

All machine-generated output is untrusted by default. No output advances to downstream review without passing governance thresholds.

## Trust boundary summary

| Component | Trust level |
|-----------|-------------|
| Rust core | Authoritative |
| Python adapters | Non-authoritative (advisory) |
| TypeScript UI | Non-authoritative (display only) |
| Bash scripts | Non-authoritative (glue only) |
| LLM output | Untrusted until governed |

## Prohibited trust escalations

- Adapter output must not be promoted without Rust governance.
- Missing or UNKNOWN results must not be treated as PASS.
- No bypass flags may override governance decisions.

## v0.0.0 status

This component is non-authoritative in v0.0.0. Implementation will be added in later phases.
