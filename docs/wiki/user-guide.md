---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# User Guide

[Documentation home](README.md) | [First review](getting-started.md#your-first-review) | [Glossary](glossary.md)

This guide is for people reviewing work, not changing code. Its short labels
match the interface. No programming is needed for the basic review once
someone has started the local application.

## How AJENTIC Helps You Review

Inspect the work and its record rather than accept an answer because it sounds
confident. Ask: **What do I have? What supports it? What still needs attention?**
A completed check is not automatically a correct answer, and recording approval
is not permission to act elsewhere.

The workspace uses pre-made local examples. It is not a chat box for new tasks,
a cloud AI connection, or a document-upload service. Read
[capabilities and limits](capabilities-and-limits.md) before planning real work.

## Find Your Way Around

| Label | What It Means | What You Can Do |
| --- | --- | --- |
| File Changes | The example file being compared | Select `sample-note.txt` to return to Compare. |
| Example Details | Information about the illustration | Read its purpose; Show Details also reveals raw run information. |
| Compare | Before/After sample text | Inspect the marked change and show or fold extra unchanged lines. |
| Result | The built-in draft and recorded checks | Run Sample, read Draft Output, and open Check Details. |
| Activity | Recorded local steps and choices | Use View All for the raw timeline; expand Decision History. |
| Tool Result | A summary of the sample and latest rejected request | Read what happened, not just a positive-looking icon. |
| Review Notes | Your temporary reminders | Text stays in this page; it is not saved or submitted as evidence. |
| Review Decision | Your local review choice | Record Approval or Reject Draft after loading the sample. |
| View / Checks | Local evidence and gaps | Inspect blocked, missing, rejected, and unfamiliar values. |
| View / Developer Details | Advanced local tests and raw projections | Not needed for the first review; these do not enable live tools. |
| Help | Bundled explanations | Opens another tab; full wiki links need internet access. |

On desktop, review controls stay at the bottom. On small screens the comparison
and controls stack vertically. Code scrolls inside its own area without making
the whole page wider.

## Understand the Comparison

Before and After are a pre-recorded illustration, not provider-generated output.
Red marks the removed sentence, green its replacement. `+1` and `-1` describe
one added and one removed line in this example.

The replacement says **This draft still needs review.** A draft's appearance is
not evidence that it is ready to use. **Show All Unchanged Lines** expands more
sample context; **Fold Unchanged Lines** returns to the compact view.

**Sample Comparison** and **No Files Written** remain visible. These controls
do not apply changes, save files, or repair a blocked output request.

## Read Statuses Without Guessing

| Message | Meaning | Next Step |
| --- | --- | --- |
| Local Demo | Predictable example data is in use. | Treat it as an exercise, not a live AI session. |
| Sample Loaded | The sample draft and display-check projection exist. | Read Result and Check Details. |
| Need Attention | Some recorded evidence is blocked. | Open Checks and inspect the reason. |
| Not Started | A check or its information is missing. | Decide whether your current path needs it. |
| Rejected / Request Rejected | A request or item was not accepted. | Stop that path and read Details. |
| Unknown status | A raw value has no friendly translation. | Inspect its raw details; unknown does not mean successful. |
| Pending | No review choice has been recorded. | Review the draft before deciding. |
| Recorded | A local choice is in the session record. | Confirm it matches your choice. |

Show Details and Check Details preserve raw values such as `intent_recorded`.
The workspace does not turn that value into a claim of task completion.
Checks labelled **Completed locally** or **Available** describe a particular
local item, not production readiness or factual truth.

## Review a Draft Carefully

1. Run Sample, then select Result and read Draft Output.
2. Read Check Details: which requirement did the check actually cover?
3. Look for errors, missing support, uncertainty, or side effects.
4. Add a temporary note if useful; it is only a local reminder.
5. Record a choice and confirm it in the footer and Decision History.

Both buttons become disabled after the first decision. The transport also
rejects duplicates. View / Reset Example starts fresh; it does not edit a saved record.

## Optional Provider Exercise

This developer-oriented test is not needed for the first review and does not
set up a real AI subscription. Start fresh, open **View / Developer Details**,
expand each named section, and use these controls:

| Order | Section | Button | Everyday Meaning |
| --- | --- | --- | --- |
| 1 | Sample Connector | Set Up Connector | Describe the permitted sample connector. |
| 2 | Connector Check | Check Connector | Check it without a live service call. |
| 3 | Sample Request | Request Sample | Request output from the local sample source. |
| 4 | Sample Settings | Set Up Sample | Record sample settings. |
| 5 | Sample Tool | Run Sample Tool | Produce predictable example output. |
| 6 | Draft Checks | Prepare Draft | Prepare a possible draft. |
| 7 | Draft Checks | Check Draft | Check its structure and recorded connections. |
| 8 | Draft Decision | Record Draft Decision | Record a choice about that checked sample. |
| 9 | Output Request | Create Draft Output | Request a local draft representation, not a file write. |

**Current limitation:** this sequence rejects the last request with
`provider_execution_result_id_mismatch`: the records disagree about which result
belongs to the draft. The rejection remains visible in Tool Result and Developer
Details. Returning to Compare does not turn its illustration into real output.
See [troubleshooting](troubleshooting.md#the-longer-provider-exercise-is-rejected).

Buttons beginning **Test Blocked** deliberately exercise a rejection. Do not
weaken checks because a negative test was rejected. Other Developer Details
sections retain release evidence, session summaries, restore previews, and
failure drills. A panel is not proof that an installer, backup, or external action exists.

## Notes, Refreshing, and Privacy

Review Notes accepts temporary text. **Attachments Not Connected** is disabled:
it does not upload images, import documents, or read files. Notes survive view
changes but not a refresh or Reset Example. They are not written to disk,
submitted as authority, or included in the recorded review intent.

The local transport also keeps its state in memory. Session-history and restore
summaries are not durable backups. Do not store real work, API keys, confidential
documents, or personal information here.

If an error is unexplained or records disagree, stop that path and keep the
exact message and steps. Use [Troubleshooting](troubleshooting.md). Approval
never means every fact was checked or a release or real-world action was authorized.
