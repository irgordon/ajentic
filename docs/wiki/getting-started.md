---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Getting Started

[Documentation home](README.md) | [User guide](user-guide.md) | [Get help](troubleshooting.md)

## What AJENTIC Is For

A convincing AI answer is not the same thing as a checked answer. AJENTIC brings
the proposed work, recorded checks, and review history into view so you can ask:

- What was produced?
- What was actually checked?
- What is missing or uncertain?
- What decision did I make, and why should I trust the result?

For example, an AI-written summary might read well but leave out an important
point. A review process helps you look for support rather than relying on tone
or confidence. AJENTIC's current sample lets you practice that review process;
it does not yet let you upload a meeting transcript and generate a live summary.

## What You Need Today

The browser example runs on your own computer. There is no app-store installer
or hosted sign-in page in this repository's supported setup.

If someone has already started AJENTIC for you on the same computer, open
[http://127.0.0.1:5173](http://127.0.0.1:5173). This is a local address: it refers
to your computer, not an AJENTIC website on the internet.

Otherwise, follow [Installation](installation.md), or ask a developer to complete
it with you. You do not need an AI-service account, payment details, or an API
key to try the sample. An API key is a secret used to access a service; do not
paste one into this demonstration or a support report.

## Your First Review

Allow about ten minutes once the page is running. No coding is needed for these
steps. The page is long, so your browser's Find command can help locate a label:
`Ctrl+F` on Windows/Linux or `Command+F` on macOS.

### 1. Recognize the example environment

At the top, look for **Local testing environment running simulated behaviors.**
and **Ready (simulated)**. "Simulated" means the example uses predictable,
pre-written data instead of contacting an AI service.

Some panels show **Not started**, **Needs attention**, or **Unknown status**.
That does not mean your installation failed. Many panels are demonstrations of
later review steps and do not start with complete information.

### 2. Create the sample

Find **Run local simulation**, near **Bounded context**, and click it once.
"Bounded context" means the limited information supplied for this example.

You should now see:

- **Candidate output**, containing **Deterministic local stub candidate**.
- **Run history / timeline**, with the sample's recorded steps.
- **Validation / policy result**, with the sample's check results.
- Enabled **Approve** and **Reject** buttons under **Operator controls**.

"Candidate" simply means a draft to look at. "Deterministic local stub" means
the same built-in example is returned each time; it is not a fresh AI answer.

### 3. Look at the work and its checks

Read the sample and its check explanation before selecting a button. The check
reason says **deterministic fixture satisfies local shell display checks only**.
In everyday language: the example is suitable for this display exercise. It
does not prove that AI-generated facts are correct or that a real task is done.

The **Provider output trust flag** remains `false`. This means the sample has
not become trusted just because it is displayed or you reviewed it.

### 4. Record one decision

Click **Approve** to practice recording a positive review, or **Reject** to
practice declining the draft. Here, these buttons only record your local choice.
They do not send a message, publish work, run a command, or approve a release.

Check **Selected operator intent**. It should show `approve` or `reject`.
"Intent" means the choice you asked the example to record. The timeline adds
an entry for that choice. Under **Replay / status projection**, **Decision count**
becomes `1`; the latest decision kind matches your choice.

### 5. Try the other choice in a fresh example

Only one decision is accepted for that sample in the current session. If you
click the other button afterward, **Local transport boundary** reports
`duplicate_decision_rejected`. That means the example refused to add another
decision for the same sample; it is not an approval-change workflow.

To practice again, refresh the browser, click **Run local simulation**, and
choose the other button. Refreshing starts a fresh local example and clears
its in-memory choices. **Do not rely on this page as saved work.**

## What You Accomplished

You created a sample, inspected its explanation, recorded one review decision,
and found the record of that choice. This demonstrates why AJENTIC separates
"there is a draft," "a check ran," and "someone reviewed it."

It does not show a real AI connection, an independently verified fact, or a
finished real-world task. Those are different claims that need their own support.

## Where to Go Next

- [User guide](user-guide.md): understand the rest of the screen and its labels.
- [Glossary](glossary.md): translate unfamiliar terms.
- [Troubleshooting](troubleshooting.md): disabled buttons, unclear status, or a page that will not open.
- [Capabilities and limits](capabilities-and-limits.md): see which features are examples rather than live services.
