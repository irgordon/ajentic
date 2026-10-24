---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Troubleshooting

[Documentation home](README.md) | [First review](getting-started.md) | [Developer guide](developer-guide.md)

Start with what you can see. Keep the exact message and the steps before it;
you do not need to diagnose the code to report a useful problem.

## The Browser Page Will Not Open

The address `http://127.0.0.1:5173` only works while the local server is running
on the same computer as your browser. It is not a public website.

Ask the person who set it up to check the server terminal for the printed
address or an error. Do not enter account credentials or pay for access to
solve a connection failure. If the setup has not been done, use
[Installation](installation.md).

## The Review Buttons Are Disabled

On a fresh page, click **Run local simulation** first. Approve and Reject need
a sample draft to review. A disabled control in another panel may need an
earlier sample step; it is not a reason to change security settings.

## A Status Says Unknown or Needs Attention

**Unknown status** means the interface lacks a friendly translation for a raw
value. Open the nearby details and keep that value in your report. After a
review choice, `intent_recorded` is an expected example of this display gap:
check **Selected operator intent** and the timeline to confirm your choice.

**Needs attention**, **Rejected**, or **Preparation blocked** means you should
read the specific reason rather than continue as though everything passed.
Release-preparation panels can be incomplete even while the basic sample works.
See [status explanations](user-guide.md#read-statuses-without-guessing).

## I Cannot Change Approve to Reject

The sample accepts only one recorded decision for the same draft in a session.
A second attempt reports `duplicate_decision_rejected`. To practice the other
choice, refresh, run the simulation again, and select it. This clears the local
example; it is not an edit to a saved decision.

## My Session Disappeared After Refresh

The example stores its state in memory, not a durable browser database. A
refresh creates a new example. Session history and restore summaries do not
mean real work was saved. Do not rely on them for backups or important records.

## The Longer Provider Exercise Is Rejected

The optional [provider exercise](user-guide.md#optional-provider-exercise) uses
separate sample source/result records. In the current combined sequence, the
last request can report `provider_execution_result_id_mismatch` and
`materialization_rejected`.

Plain meaning: the records disagree about which result belongs to the proposed
draft, so the output request was refused. This is a known integration limitation,
not a completed output. Preserve the exact sequence and message for the maintainer.
Do not change IDs, edit state, or bypass checks to manufacture a positive result.
The shorter first-review exercise is separate and does not require this path.

## Installation and Startup

The following section is for the person running terminal commands.

| Symptom | Next Check |
| --- | --- |
| Node preflight fails | Check `node --version` in the same shell. Use the supported 22.x or 24.x floor in the installation guide, not an unsupported newer major. |
| Dependency download times out | Check registry connectivity and the reported timeout, then retry when connectivity is restored. Do not delete the lockfile to work around it. |
| TypeScript or Ajv cannot be found | Run the locked `npm ci --prefix ui` setup before build or schema commands. |
| Port 5173 is occupied | Identify the existing process and stop it only if it is yours and no longer needed. The current dev script has a fixed port. |
| Edits do not appear | Stop and restart the dev command to recompile, then refresh. There is no hot reload. |
| `cargo`, `rustfmt`, or `clippy` is missing | Install the Rust toolchain/components for the core path; they are not required merely to view an already running UI. |
| Full check rejects a dirty tree | Inspect `git status --short`; commit intended changes on your branch and preserve unrelated work. |
| Clippy fails only in CI | Compare toolchains. CI follows stable and may introduce new diagnostics; investigate the concrete diagnostic. |
| actionlint fixture tests skip locally | Install actionlint 1.7.12 for workflow verification. The Actions Scripts job installs its verified binary before running them. |

A timeout means the check did not finish. Report it next to passing checks, not
as a full pass or automatically as a product defect. For deeper existing
operator diagnostics, see the [technical troubleshooting reference](../operations/operator-troubleshooting-phase-98.md).

## Report a Problem Clearly

Include the page or command, expected result, actual result, exact error text,
and smallest repeatable sequence. For developers, include the branch/commit,
runtime versions, and failed Actions URL when applicable. Note whether the
problem is in the local simulation or a separate Rust test.

Remove credentials, private paths, personal data, and confidential content.
Do not post environment dumps or tokens. Use the repository's
[issue tracker](https://github.com/irgordon/ajentic/issues) for non-sensitive
reports; contact the maintainer before disclosing sensitive details publicly.
