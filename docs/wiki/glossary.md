---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Plain-Language Glossary

[Documentation home](README.md) | [User guide](user-guide.md)

These definitions translate the words used by AJENTIC; they do not add new
permissions or change the meaning of a check.

| Term You May See | Everyday Meaning |
| --- | --- |
| Adapter | A connector description: the kind of source the example is allowed to use and the fields it accepts. |
| Allowlisted | Included in a specific list of permitted choices. It does not mean universally safe. |
| API | A defined way for one piece of software to ask another for something. |
| API key / token | A secret used to access a service. Treat it like a password; the sample does not need one. |
| Artifact / bundle | A file or collection of files produced by a build or evidence process. It is not necessarily an installable app. |
| Attestation / provenance | A record about where a file came from and how it was produced, not proof that its contents are correct. |
| Authority | Permission to make an actual system decision. Showing a result on screen does not grant this permission. |
| Boundary | A limit on what a part of the system may do. |
| Bounded context | The limited information supplied for one task or example. |
| Candidate | A proposed draft or result that still needs review. |
| Checksum / digest | A fingerprint of some data, used to notice changes. A matching fingerprint does not prove a factual claim is true. |
| CI / GitHub Actions | Automated checks run on GitHub when repository changes are submitted. Mainly relevant to developers and maintainers. |
| CLI | Command-line interface: a program used by typing commands in a terminal rather than clicking browser controls. |
| Contract / schema | A description of the shape and fields a piece of data is expected to have. |
| Deterministic | Designed to produce the same result from the same inputs. |
| Dry run | A limited rehearsal or check, not permission to perform a real-world action. |
| Evidence | Recorded information offered in support of a result. It can still be incomplete, wrong, or insufficient. |
| Fixture / stub / simulation | Pre-made example data or behavior used for testing, instead of a live service. |
| Harness | The surrounding system that constrains and records a workflow; in the sample, the local test environment. |
| Intent / operator decision | The choice a person asks the system to record. It is not automatically authorization to act. |
| Invocation | A request to run a particular permitted step. |
| Ledger / timeline | A record of events or decisions. A visible history is not necessarily saved to disk. |
| Linkage | The recorded connections between a task, its input, its result, and related checks. |
| Local / loopback / 127.0.0.1 | Running on or connecting back to your own computer, rather than a public website. |
| Materialize | Create the local representation of a draft from the checked records. It does not mean publish, deploy, or save a durable backup. |
| Operator | The person using the review interface. |
| Policy | The rules that determine whether a request is allowed. |
| Postcondition | Something that must be true after a step for the task's success criteria to be satisfied. |
| Projection | A view or summary of underlying information, not a new source of decision-making permission. |
| Provider | The source of output. In this browser example it is a simulated source, not a live AI subscription. |
| Raw details | The original technical values behind a friendly label; useful when a label is unclear. |
| Receipt | A bound internal record of a check or decision. Making a similar-looking object is not the same as obtaining an authorized receipt. |
| Release candidate / RC | A proposed release prepared for evaluation before a final release decision. |
| Replay | Checking or summarizing recorded events again, not repeating live external actions or repairing history. |
| Rust | The programming language used for AJENTIC's decision-making core. |
| SBOM | Software Bill of Materials: a list describing software components in an artifact. It is not a vulnerability-free guarantee. |
| Staged proposal | A possible draft prepared for checking before a local choice is recorded. |
| Terminal | An application where setup commands are typed. You do not need it for the first review once someone has started the page. |
| Untrusted | Not accepted as correct merely because it exists or a model produced it. |
| Validation | Checking specified requirements. Always ask which requirements were checked. |

For what particular labels mean on screen, see
[Reading statuses](user-guide.md#read-statuses-without-guessing).
