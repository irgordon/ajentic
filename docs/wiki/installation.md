---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Installation and Local Setup

[Documentation home](README.md) | [First review](getting-started.md) | [Setup problems](troubleshooting.md)

This page is for the person setting up the local example. Once it is running,
a non-technical user can follow [Getting started](getting-started.md) without
using a terminal. A terminal is an application where you type setup commands.

## Pick the Setup You Need

| Goal | Tools Needed |
| --- | --- |
| Open an example someone already started on this computer | A web browser |
| Install and start the browser example yourself | Git, a supported Node.js version, and npm |
| Run the Rust CLI or change the core | Rust and Cargo, in addition to the repository source |
| Run all contributor checks | The tools above, Python 3.11 or newer, Bash, `rustfmt`, and `clippy` |
| Change GitHub Actions workflows | actionlint 1.7.12 for local workflow linting; CI installs its own verified binary |

These instructions use a macOS/Linux-style shell. Native Windows end-to-end
setup has not been verified here; the Bash scripts and `/tmp` paths are not
PowerShell instructions. Do not assume the repository contains a Windows installer.

## Install the Supporting Tools

Use Git for source checkout. Install a supported LTS Node.js line, including npm,
from the [official Node.js downloads](https://nodejs.org/en/download).
The project supports:

- Node 22.23.4 or newer **within 22.x**.
- Node 24.22.1 or newer **within 24.x**.

Older patches, prereleases, and other major lines fail the project preflight.
For example, Node 26 is not supported just because its number is higher.
If you use a version manager, select the supported version in the same terminal
where you will run npm. Reopen the terminal after installing tools so commands
are available on `PATH`, the list of places your shell searches for programs.

Check the browser setup tools:

```sh
git --version
node --version
npm --version
```

The Rust CLI is a command-line interface: a program used by typing commands
rather than clicking browser controls. For core development and full validation, use the
[official Rust installation guide](https://rust-lang.org/tools/install/) and
[Python downloads](https://www.python.org/downloads/) if those tools are missing.
With a rustup-managed toolchain, install its check components:

```sh
rustup component add rustfmt clippy
cargo --version
rustc --version
python3 --version
bash --version
```

CI uses exact Node build pins and a floating stable Rust toolchain. See the
[developer guide](developer-guide.md#toolchain-and-validation-limits) for what
that does and does not guarantee.

## Download the Source

Run these commands in the folder where you keep projects:

```sh
git clone https://github.com/irgordon/ajentic.git
cd ajentic
```

`cd ajentic` makes the downloaded repository your current folder. Unless a guide
says otherwise, run its commands from this repository root.

The default branch is current `main`, including post-v1 maintenance. For the
published v1.0.0 snapshot, check out the existing `v1.0.0` tag in a clean clone
and follow that snapshot's instructions. Do not create or move a release tag to
select a version.

## Install the Locked Browser Dependencies

From the repository root:

```sh
npm ci --prefix ui --no-audit --no-fund --fetch-retries=1 --fetch-timeout=15000
```

This installs the exact dependency selection recorded in `ui/package-lock.json`
into `ui/node_modules`. Network access is needed for uncached packages.
Use `npm ci` rather than updating versions simply to get started.
Skipping the install-time audit request does not establish that dependencies
are free of vulnerabilities.

## Start and Open AJENTIC

```sh
npm run dev --prefix ui
```

Wait for compilation and the printed address, then open
[http://127.0.0.1:5173](http://127.0.0.1:5173). `127.0.0.1` means this computer.
You should see **AJENTIC Local Operator Shell** as the page title and the heading
**Local testing environment running simulated behaviors.**

Keep this terminal open. The server listens on the local loopback address, not a
public network address. Stop it with `Ctrl+C` in its terminal when finished.
Do not add public hosting or network exposure as a setup workaround.

The current script uses port 5173 and compiles into `/tmp/ajentic-ui-dev` when it
starts. It does not hot-reload: after source edits, stop it, start it again, and
refresh the browser. Compilation requires the dependencies installed above.

**Setup is complete when the page loads, not when a terminal merely starts a
command.** Follow [Your first review](getting-started.md#your-first-review) to
create and inspect the sample. No provider credentials are required.

## Optional Rust CLI Check

With Rust installed, from the repository root:

```sh
CARGO_TARGET_DIR=/tmp/ajentic-check-target cargo run --manifest-path core/Cargo.toml -- dry-run
```

Expect a deterministic CLI summary saying no provider/model was called and no
persistence occurred. This is not a command to submit a real task, start the
browser, or connect the browser to a backend. Unknown command names produce
safe usage text; do not infer a full command-line interface from that fallback.

For all contributor checks, continue to the [developer guide](developer-guide.md).
For installation failures, use [Troubleshooting](troubleshooting.md#installation-and-startup).
