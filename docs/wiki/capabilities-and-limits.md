---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Capabilities and Limits

[Documentation home](README.md) | [User guide](user-guide.md) | [Maintainer guide](maintainer-guide.md)

This page distinguishes the project idea, the current local experience, and
release evidence. A visible label or a passing check is not a permission grant.

## What the Local Browser Example Can Do

| Capability | Current Meaning |
| --- | --- |
| Start a local simulation | Display the same built-in sample draft and recorded display checks. |
| Record Approve or Reject | Record one local review intent for the sample, with a timeline/replay summary. |
| Show check and evidence panels | Present local projections, missing information, rejected requests, and explanatory status. |
| Exercise provider-shaped requests | Use deterministic local test adapters and configurations, not a cloud subscription. |
| Show staged proposal and session summaries | Demonstrate review steps and source connections; not a guarantee of file output or durable storage. |
| Open local help | Read the bundled explanatory pages without signing in. External wiki links need internet access. |

The browser transport holds state in memory. A refresh or newly constructed
transport starts a fresh example. Do not rely on a session-history or restore
panel as a backup of real work.

## What Is Not a Supported User Path

- An arbitrary new task entered into a chat box or document-upload workflow.
- A live cloud AI connection configured by entering an API key in this shell.
- A live browser-to-Rust backend service merely because the browser is running.
- Browser approval that authorizes external actions, production use, or releases.
- A guaranteed successful end-to-end provider-to-file-output workflow.
- Durable browser-session storage, downloadable evidence, or restore capability
  inferred solely from the presence of a summary panel.
- An npm/Cargo registry installation, packaged app installer, automatic update
  channel, deployment service, OS-signed app, or notarized app.

The [user guide](user-guide.md#optional-provider-exercise) records a concrete
combined-flow limitation: candidate materialization rejects
`provider_execution_result_id_mismatch`. Do not document it as successful output.

## Where Real Decision-Making Belongs

Rust is AJENTIC's authoritative control layer. It owns runtime validation,
policy, state changes, execution boundaries, ledger/replay handling, and bound
receipts. A receipt is an internal record tied to particular inputs and evidence.

TypeScript displays information and collects local intent. Python supports
adaptation and evidence tooling; Bash connects operator commands. Documentation,
screens, scripts, checksums, and attestations do not acquire runtime authority.

The existence of Rust modules and tests is not proof that every planned product
path is integrated into the browser. The current CLI exposes a bounded dry-run
summary, not a complete end-user task submission interface.

## What the Published Release Means

[v1.0.0 is published on GitHub Releases](https://github.com/irgordon/ajentic/releases/tag/v1.0.0).
The final tag targets `6734a5b3cc223c41288b14575ad40f6fcf23fb6f`; its source RC is
`v1.0.0-rc.1`. The final review assets include the candidate bundle, asset manifest,
checksums, SBOM, provenance, release notes, and a final README asset.

The current `main` branch contains later maintenance changes. A simulated
release-preparation panel is not a live report of the GitHub release state.
See [release closure evidence](../operations/post-v1-release-closure-phase-195.md)
and [maintainer verification](maintainer-guide.md#preserve-release-evidence).

A release says that a particular snapshot and its assets were published. It does
not establish correctness of every AI claim, safety for every use, or permission
to publish packages, install services, deploy, sign, or notarize new artifacts.

## Known Assurance Gaps

Schema checks establish data shape, not factual truth. Payload fixtures cover
5 of 22 schemas; 17 are explicitly compile-only. Reproducibility checks compare
builds within their configured environment, not across every toolchain or OS.
There is no dedicated dependency vulnerability-audit gate.

Previously reviewed outcome aggregation, caller-supplied claim support,
reference/digest pairing, and independent-observation concerns remain follow-up
review areas. The validation infrastructure and these guides do not claim to
resolve them. Keep real factual verification and human judgment separate from
the successful completion of a local example.
