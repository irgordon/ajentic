---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Getting Started

[Documentation home](README.md) | [User guide](user-guide.md) | [Get help](troubleshooting.md)

## What AJENTIC Is For

A convincing AI answer is not the same as a checked answer. AJENTIC brings the
proposed work, recorded checks, and review history into view so you can ask:

- What was produced?
- What was actually checked?
- What is missing or uncertain?
- What decision did I record?

An AI-written summary might read well but leave out an important point. A review
process helps you look for support rather than relying on tone or confidence.
Today's local example lets you practice that process; it is not a document-upload
service or a live AI chat app.

## What You Need Today

If someone has started AJENTIC on your computer, open
[http://127.0.0.1:5173](http://127.0.0.1:5173). This local address refers to your
computer, not a public AJENTIC website. Otherwise, follow [Installation](installation.md),
or ask a developer to set it up with you. No packaged installer or hosted sign-in
is provided by this setup.

You do not need an AI-service account, payment details, or a secret API key to
try the sample. Do not paste credentials into the example or a support report.

## Your First Review

Allow about ten minutes once the page is running. No coding is needed.

### 1. Recognize the example

Look for **AJENTIC**, **Local Demo**, and **File Changes**. The screen starts on
**Compare**, with sample Before/After text. **Sample Comparison** and **No Files
Written** identify it as an illustration, not generated file output.

"Demo" means predictable, pre-made data is used instead of a real AI service.
You can inspect this comparison before running anything.

### 2. Load the sample draft

Click **Run Sample** in the top bar. On a small screen it uses a play icon with
the same accessible name. This loads a draft and recorded display checks; it
does not execute a live tool.

Select **Result**. Under **Draft Output**, you should see **Deterministic local
stub candidate**. That technical name means the same built-in example is used
each time, not a new AI answer. The file comparison is a separate illustration,
not a file created from this draft.

### 3. Read the checks

Open **Check Details** in Result. The reason says **deterministic fixture
satisfies local shell display checks only**. In everyday language: the example
passed checks for this local display exercise. That does not prove its facts
are correct or a real task is finished.

For the sample, **Output Trust** remains `false`: displaying or reviewing it
does not make it trusted. **View / Checks**, or the amber counts beside Tool
Result, opens other evidence, including blocked, missing, rejected, or unknown items.

### 4. Record a choice

Under **Review Decision**, select **Record Approval** or **Reject Draft**.
These buttons record a local choice only. They do not send messages, execute
tools, publish work, write files, or approve releases.

The footer confirms **Approval recorded for this local sample.** or **Rejection
recorded for this local sample.** Both decision buttons then become disabled.

Select **Activity**, then expand **Decision History**. **Decision Count** becomes
`1`; the record shows `approve` or `reject` and its identifier. This is a review
record, not proof that external work was completed.

### 5. Practice the other choice

Only one review decision is accepted for this sample in the session. Choose
**View / Reset Example** or refresh, run the sample again, and select the other
choice. Resetting clears the example and notes. **Do not rely on this page as
saved work.**

## What You Accomplished

You loaded a draft, inspected what its checks covered, recorded a review choice,
and found that choice in the history. This demonstrates why AJENTIC separates
a draft, a check result, and a person's review.

You did not connect a live AI service, independently verify a fact, or complete
a real-world task. Those different claims need their own support.

## Where to Go Next

- [User guide](user-guide.md): tabs, comparison, notes, statuses, and optional tests.
- [Glossary](glossary.md): technical words in everyday language.
- [Troubleshooting](troubleshooting.md): disabled controls, rejected requests, or setup problems.
- [Capabilities and limits](capabilities-and-limits.md): examples versus live capabilities.
