---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# User Guide

[Documentation home](README.md) | [First review](getting-started.md#your-first-review) | [Glossary](glossary.md)

This guide is for people reviewing work, not changing code. You can follow the
first review without understanding programming. Technical labels below are
shown exactly as they appear so you can find them on the page.

## How AJENTIC Helps You Review

AJENTIC encourages you to look at the work and the record behind it, rather than
accepting an answer just because it sounds confident. Its local example shows
how a draft, check results, and your review choice can remain separate.

Use three questions as you look around: **What do I have? What supports it?
What still needs attention?** A completed test is not automatically a correct
answer, and a positive review is not permission for the software to act elsewhere.

The current interface is a local example with pre-made data. It has no ordinary
chat box for a new task, document-upload workflow, or live cloud AI connection.
See [capabilities and limits](capabilities-and-limits.md) before planning real work.

## Find Your Way Around

The current page uses stacked panels rather than a short, guided sequence.
Use your browser's Find command to jump to the exact section name.

| Screen Label | In Everyday Language | What to Look For |
| --- | --- | --- |
| Help and reference | Explanations of the page | **Open help** opens local reference pages in another tab. |
| Run history / timeline | The record of steps | What happened before and after your choice. |
| Bounded context | Information supplied to the example | The example's limited scope and simulated data source. |
| Candidate output | The draft being reviewed | Read the draft before choosing Approve or Reject. |
| Validation / policy result | Recorded checks and rules | Read the reason, not only a positive-looking label. |
| Operator controls | Your review buttons | Approve or Reject records a local choice, not an external action. |
| Local transport boundary | The last local request and its result | A rejected request includes a reason to investigate. |
| Replay / status projection | A summary of recorded choices | Decision count, latest choice, and consistency. |

For a first visit, these sections are enough. You can leave the release and
provider-testing panels alone until you want to explore their explanations.

## Read Statuses Without Guessing

| Label | What It Tells You | What to Do |
| --- | --- | --- |
| Ready (simulated) | The built-in example is available. | Start the local simulation; this is not a production-safety statement. |
| Completed locally | A particular example step finished. | Read which step completed and its explanation. |
| Available | Information exists for that particular check. | Inspect it; availability does not prove correctness. |
| Not started | Information or a step has not been supplied yet. | Decide whether it is needed for your current exercise. |
| Needs attention / Preparation blocked | A prerequisite or piece of information is missing or blocked. | Read the relevant reason; do not assume a later step succeeded. |
| Rejected | The request or evidence was not accepted. | Stop that path and inspect the explanation. |
| Unknown status | The interface has no friendly description for this raw value. | Open the raw details and ask for help if the meaning affects your decision. |

After recording a review choice, the top run badge can show **Unknown status**:
the raw value `intent_recorded` does not yet have a friendly display mapping.
Confirm the choice in **Selected operator intent**, the timeline, and the replay
summary instead. Unknown is not another word for successful.

## Review a Draft Carefully

1. Read the draft and the information supplied for it.
2. Read the check reason. Ask whether it checks the point you care about.
3. Look for errors, missing support, uncertainty, or side effects.
4. Record a choice only after understanding what that choice means.
5. Check that the recorded choice matches what you selected.

For a real AI-generated summary outside this demo, for example, readable wording
would not be enough: you would still compare it with the original material.
That example explains the review habit; it is not an available document-upload
feature in this interface.

## Understand the Other Panels

| Panel Group | What It Represents | What Not to Assume |
| --- | --- | --- |
| Release candidate preparation, Internal candidate bundle, Internal evidence, Release Candidate gap review | Review information about preparing and checking example release evidence | These panels do not publish a release or replace the actual GitHub release history. |
| Installer and distribution contract, Internal attestation evidence | Descriptions of packaging and evidence limits | Seeing a panel does not mean an installer or signed app exists. |
| Complete local operator workflow, Trial operator runbook | An ordered description of the local example | A description of a step is not proof that the step ran. |
| Trial failure drill, Stop-condition drill, Escalation guidance | Practice information about errors and when to ask for help | A warning shown here is not an automatic stop mechanism for an external system. |
| Local provider adapter contract, Controlled adapter dry run, Constrained local provider invocation | Tests of a permitted sample data source | These are not account settings for a real AI subscription. |
| Local provider configuration, Sandboxed provider execution, Provider output pipeline | Local sample requests and their review records | Do not enter credentials, cloud endpoints, or shell commands. |
| Staged proposal validation, Operator candidate decision, Local candidate materialization | Preparing, checking, and choosing a possible local draft | An enabled button is not a guarantee that the whole chain can finish. |
| Local session package, Session history, Local session restore, Restore preview | Summaries and previews of session evidence | Do not assume the browser saved files or can restore your work after refresh. |

## Optional Provider Exercise

This is a developer-oriented sample, not a requirement for your first review.
Its controls use pre-made local values; they do not configure a live AI service.
Start with a fresh page so an earlier experiment does not confuse the result.

| Order | Button Label | Plain Meaning |
| --- | --- | --- |
| 1 | Declare local test adapter contract | Describe the permitted sample connector. |
| 2 | Run controlled adapter dry run | Check that sample connector without a live service call. |
| 3 | Invoke allowlisted local provider | Request output from the permitted sample source. |
| 4 | Submit local test provider configuration | Record the sample source settings. |
| 5 | Run deterministic provider | Produce predictable example output. |
| 6 | Create staged conversion proposal | Prepare a possible draft for checking. |
| 7 | Validate staged proposal shape/linkage | Check the draft's structure and recorded connections. |
| 8 | Record validated staged proposal decision | Record the choice about this checked sample. |
| 9 | Materialize local candidate output | Request creation of the local draft representation. |

**Current limitation:** the combined sequence above reaches a rejection at the
last step with `provider_execution_result_id_mismatch`. This means the records
do not agree about which result belongs to the draft. The request is rejected;
do not treat it as completed output or try to bypass the check. The basic
[first review](getting-started.md#your-first-review) does not need this sequence.
See [troubleshooting](troubleshooting.md#the-longer-provider-exercise-is-rejected).

Buttons whose names start with **Submit rejected** or **Submit forbidden** are
deliberate negative tests. A rejection there is expected, not a request to weaken
the application's checks.

## Saving, Refreshing, and Privacy

The current local example keeps its state in memory. Refreshing or reopening the
page creates a fresh example. It is not a reliable place to store real work.
The session-package and export panels are summaries, not a promise that a file
was written or a durable backup exists.

The sample does not need your name, AI-service key, private documents, or real
customer data. Do not add those to examples or screenshots. If you report an
issue, share only the minimum information needed to explain it, with secrets
and personal information removed.

## When to Stop and Ask for Help

Stop the current path when a record is missing, two results disagree, an error
is unexplained, or you cannot tell what a button would do. Keep the exact message
and steps that led to it. Use [troubleshooting](troubleshooting.md) to distinguish
a normal example state from a problem worth reporting.

Approving a sample never means that AJENTIC has checked every fact, authorized
an external action, or made the work suitable for production use.
