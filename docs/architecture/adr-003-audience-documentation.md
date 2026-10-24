---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# ADR 003: Audience-Oriented Product Documentation

Date: 2026-10-23

Status: Accepted by Owner instruction

## Context

The public README needs to explain the product without becoming a technical
manual. Setup commands alone do not teach a non-technical person how to review
a sample, and historical phase records do not form a developer or maintainer
handbook. The current browser is a simulated local shell, not a live provider
or backend integration; guidance must not turn product intent into a capability
claim.

## Decision and Why

Use `docs/wiki/README.md` as an audience navigation hub. Separate the user,
developer, and maintainer guides, with shared installation, troubleshooting,
glossary, and current-capability references.

The user path starts with a concrete sample and expected results, defines terms
at first use, and repeats exact screen labels so readers can find controls.
It explains the benefit of review before implementation details. Developer and
maintainer guides carry commands, module ownership, tests, CI, and release
constraints rather than burdening the user walkthrough with them.

Keep the bundled help's first-use instructions consistent with that path and
link to the full repository guides. Local help remains available without a
network connection; optional repository links are explicitly identified.

Ground walkthroughs in source and functional checks. Treat an actual rejection
as a limitation, not an undocumented success. The combined provider example's
`provider_execution_result_id_mismatch` remains visible without changing runtime
behavior in a documentation task. Do not promise durable storage, download
controls, arbitrary task entry, or live AI calls that the current shell lacks.

## Alternatives

| Alternative | Why Not Selected |
| --- | --- |
| One large README | Mixes audiences and hides the product message among setup commands. |
| One large wiki setup page | Provides installation but leaves usage, contribution, and maintenance journeys incomplete. |
| Link only to phase records | Makes readers reconstruct a user path from historical implementation evidence. |
| Write the intended future product manual | Misrepresents simulated controls and incomplete integrations as available features. |
| Require technical vocabulary before first use | Creates unnecessary barriers for people who only want to understand a review. |

## Consequences

Guides require coordinated updates when labels, controls, statuses, workflows,
or capabilities change. Link and walkthrough verification accompany substantive
updates. Advisory wiki frontmatter keeps instructions distinct from normative
governance, structural architecture, active checklists, and historical records.

Documentation does not fix authority defects or grant release approval. Routine
updates still use the existing CI-gated, zero-required-review PR path.
