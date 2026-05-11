---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 134 - Signing and Key-Custody Implementation Boundary

## Scope
Phase 134 defines the signing, key-custody, and verification-control boundary after Phase 133.

Phase 134 is the signing/key-custody boundary only. It defines requirements, classifies blocked dependencies, and preserves the existing release-readiness ladder. It does not create signatures, keys, attestations, release artifacts, public assets, package outputs, publication behavior, distribution behavior, deployment behavior, monitoring behavior, or readiness approval.

Phase 134 starts from the committed repository evidence that Phase 130 remains `rc_candidate_not_ready`, Phase 132 preserved `artifact_creation_deferred` and `artifact_contract_gap`, Phase 132.1 remains contract-only, and Phase 133 remains `checksum_provenance_blocked_by_missing_artifact`.

Status: `signing_controls_blocked_by_missing_artifact`.

## Evidence rule
Count only committed repository evidence.

Do not count roadmap plans, prompt text, requirements, clean validation, Phase 132.1 contract correction, or Phase 133 requirements as signing evidence. Do not treat signing/key-custody requirements as key creation or signing. Do not treat verification requirements as verification evidence. Do not infer signing readiness from missing prohibited files.

Committed evidence reviewed for Phase 134 includes the Phase 126 packaging contract, Phase 130 decision gate, Phase 131 remap, Phase 132 artifact boundary, Phase 132.1 artifact contract correction, Phase 133 checksum/provenance boundary, roadmap files, current checklist, changelog, validation script, and repository source/test/script/package surfaces.

## Phase 134 signing/key-custody boundary
The Phase 134 boundary is:

- define signing-control requirements for future governed artifacts;
- define key-custody policy requirements without creating key material;
- define verification-control requirements without creating verification evidence;
- classify signing controls as blocked when governed artifact, checksum, provenance, or custody decision evidence is missing;
- preserve Phase 130 `rc_candidate_not_ready`;
- preserve Phase 132 `artifact_creation_deferred` and `artifact_contract_gap`;
- preserve Phase 132.1 as contract-only correction;
- preserve Phase 133 `checksum_provenance_blocked_by_missing_artifact`;
- require Phase 132 artifact creation rerun, Phase 133 checksum/provenance evidence, Phase 139 reassembly, and Phase 140 decision before any later readiness claim can be considered;
- avoid implementing Phase 139 or Phase 140.

Current classification: `signing_controls_blocked_by_missing_artifact`, `signing_controls_blocked_by_missing_checksum`, `signing_controls_blocked_by_missing_provenance`, and `signing_controls_blocked_by_missing_key_custody_decision`.

## Phase 130 carry-forward
- Phase 130 is complete.
- Phase 130 decision status remains `rc_candidate_not_ready`.
- AJENTIC is not Release Candidate ready.
- AJENTIC is not Production Candidate ready.
- AJENTIC is not public/general-use ready.
- Phase 134 does not rerun Phase 130.
- Phase 134 does not approve Release Candidate status.
- Phase 134 records `requires_phase_140_decision` for the future decision gate dependency.

## Phase 132 relationship
- Phase 132 is complete.
- Phase 132 recorded `artifact_creation_deferred`.
- Phase 132 recorded `artifact_contract_gap`.
- Phase 132 created no governed binary/package artifact files.
- Phase 132 did not create checksum evidence.
- Phase 132 did not create provenance evidence.
- Phase 132 did not create signing evidence.
- Phase 134 does not infer signing evidence from Phase 132 documentation.
- Phase 134 records `requires_phase_132_artifact_creation_rerun` before signing controls can move beyond blocked classification.

## Phase 132.1 relationship
- Phase 132.1 is complete.
- Phase 132.1 corrected the future artifact contract only.
- Phase 132.1 did not create artifacts.
- Phase 132.1 did not create `artifacts/local/`.
- Phase 132.1 did not create an artifact manifest.
- Phase 132.1 did not execute artifact generation.
- Phase 132.1 did not create checksum evidence, provenance evidence, signing keys, signatures, attestations, release artifacts, public assets, or readiness evidence.
- Phase 134 does not infer signing evidence from Phase 132.1 documentation.

## Phase 133 relationship
- Phase 133 is complete.
- Phase 133 status remains `checksum_provenance_blocked_by_missing_artifact`.
- Phase 133 did not create artifacts.
- Phase 133 did not generate checksums.
- Phase 133 did not create provenance attestations.
- Phase 133 did not create signing keys, signatures, verification evidence, release artifacts, public assets, or readiness evidence.
- Phase 134 does not infer signing evidence from Phase 133 requirements.
- Phase 134 records `requires_phase_133_checksum_provenance_evidence` before signing controls can move beyond blocked classification.

## Signing controls are governance, not signing
Signing controls are governance, not signing.

Phase 134 may define who must be able to authorize future signing, what future artifact evidence must be present, what future audit records must identify, and what future verification checks must document. Those controls are policy and governance requirements only. They are not key material, signatures, attestations, release artifacts, or public assets.

## Key custody is policy, not key creation
Key custody is policy, not key creation.

Phase 134 defines future custody requirements such as separation of duties, custody decision evidence, operator accountability, rotation expectations, revocation expectations, access logging expectations, and emergency handling expectations. It does not create private keys, certificates, keystores, passwords, secrets, hardware-token state, signing identities, public assets, or release infrastructure.

## Verification requirements are not signature verification evidence
Verification requirements are not signature verification evidence.

Phase 134 defines what future verification-control evidence must show after governed artifacts, checksums, provenance, and custody decisions exist. It does not execute verification, record verification results, create signature-verification proof, or satisfy any future evidence gate.

## Status model
Only the following status values are used for Phase 134 findings:

- `signing_controls_defined`
- `signing_controls_defined_with_findings`
- `signing_controls_blocked_by_missing_artifact`
- `signing_controls_blocked_by_missing_checksum`
- `signing_controls_blocked_by_missing_provenance`
- `signing_controls_blocked_by_missing_key_custody_decision`
- `signing_controls_deferred`
- `signing_controls_not_applicable`
- `requires_phase_132_artifact_creation_rerun`
- `requires_phase_133_checksum_provenance_evidence`
- `requires_phase_139_reassembly`
- `requires_phase_140_decision`

## Required enforcement lines
- Signing controls are governance, not signing.
- Key custody is policy, not key creation.
- Verification requirements are not signature verification evidence.
- Missing governed artifact evidence blocks signing.
- Missing checksum evidence blocks signing.
- Missing provenance evidence blocks signing.
- Phase 134 does not create signatures, keys, attestations, release artifacts, or public assets.
- Phase 134 does not activate signing, publishing, distribution, deployment, monitoring, or readiness.
- Phase 134 does not satisfy artifact, checksum, provenance, reassembly, or decision-gate evidence.
- Phase 134 does not approve Release Candidate status.

## Required non-readiness statement
Phase 134 is the signing/key-custody boundary only. It defines signing and verification requirements but does not create signatures, keys, attestations, release artifacts, or public assets. It does not activate signing, publishing, distribution, deployment, monitoring, or readiness. AJENTIC remains not Release Candidate ready, not Production Candidate ready, and not public/general-use ready. Phase 134 does not satisfy artifact, checksum, provenance, reassembly, or decision-gate evidence.

## Artifact dependency assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does a governed artifact exist? | No committed governed artifact exists. Phase 132 deferred artifact creation and Phase 132.1 corrected only the future artifact contract. | `signing_controls_blocked_by_missing_artifact` |
| Does Phase 134 create a governed artifact? | No. Phase 134 is governance boundary documentation only. | `requires_phase_132_artifact_creation_rerun` |
| Can signing controls proceed without artifact evidence? | No. Missing governed artifact evidence blocks signing. | `signing_controls_blocked_by_missing_artifact` |

## Checksum dependency assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does checksum evidence exist? | No committed checksum evidence exists for a governed artifact. | `signing_controls_blocked_by_missing_checksum` |
| Does Phase 134 generate checksum evidence? | No. Phase 134 does not create checksums or checksum files. | `requires_phase_133_checksum_provenance_evidence` |
| Can signing controls proceed without checksum evidence? | No. Missing checksum evidence blocks signing. | `signing_controls_blocked_by_missing_checksum` |

## Provenance dependency assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does provenance evidence exist? | No committed provenance evidence exists for a governed artifact. | `signing_controls_blocked_by_missing_provenance` |
| Does Phase 134 create provenance attestations? | No. Phase 134 does not create provenance attestations. | `requires_phase_133_checksum_provenance_evidence` |
| Can signing controls proceed without provenance evidence? | No. Missing provenance evidence blocks signing. | `signing_controls_blocked_by_missing_provenance` |

## Manifest dependency assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does manifest evidence exist? | No committed artifact manifest exists for a governed artifact. | `signing_controls_blocked_by_missing_artifact` |
| Does Phase 134 create a manifest? | No. Phase 134 does not create release artifacts or manifest evidence. | `requires_phase_132_artifact_creation_rerun` |
| Can manifest requirements substitute for manifest evidence? | No. Requirements are not evidence. | `signing_controls_deferred` |

## Key-custody decision assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does a key-custody decision exist? | No committed key-custody decision evidence exists. | `signing_controls_blocked_by_missing_key_custody_decision` |
| Does Phase 134 create key custody? | No. Key custody is policy, not key creation. | `signing_controls_defined_with_findings` |
| Can future signing proceed without custody decision evidence? | No. Missing key-custody decision evidence blocks signing-control progression. | `signing_controls_blocked_by_missing_key_custody_decision` |

## Existing key/signature/attestation scan assessment
| Review question | Assessment | Status |
| --- | --- | --- |
| Does any signing key, certificate, signature, attestation, or verification evidence already exist? | Repository scans are required for validation. Phase 134 creates none and treats any matching historical or fixture files as non-substitutive unless explicitly governed by committed evidence. | `signing_controls_defined_with_findings` |
| Does absence of prohibited files prove readiness? | No. Missing prohibited files cannot be used as signing readiness or release evidence. | `signing_controls_deferred` |
| Does Phase 134 create key material to satisfy its own requirements? | No. Phase 134 avoids creating keys or signatures to satisfy its own requirements. | `signing_controls_defined` |

## Signing/Key-Custody Requirement Table
| Requirement | Phase 134 definition | Current evidence classification | Status |
| --- | --- | --- | --- |
| Artifact input | Future signing requires a governed artifact produced through the corrected artifact contract. | Missing. | `requires_phase_132_artifact_creation_rerun` |
| Checksum input | Future signing requires checksum evidence for the governed artifact. | Missing. | `requires_phase_133_checksum_provenance_evidence` |
| Provenance input | Future signing requires provenance evidence for the governed artifact. | Missing. | `requires_phase_133_checksum_provenance_evidence` |
| Manifest input | Future signing requires a manifest that binds artifact, checksum, provenance, and custody references. | Missing. | `requires_phase_132_artifact_creation_rerun` |
| Custody decision | Future signing requires a committed custody decision before any key material is introduced. | Missing. | `signing_controls_blocked_by_missing_key_custody_decision` |
| Verification controls | Future verification must document artifact identity, checksum identity, provenance identity, signature identity, signer identity, custody decision reference, command surface, and result evidence. | Requirements only. | `signing_controls_defined_with_findings` |
| Separation from publication | Signing/key-custody requirements do not create publishing, distribution, release, deployment, or readiness authority. | Preserved. | `signing_controls_defined` |

## Signing/Key-Custody Blocked/Deferred Table
| Blocker or deferral | Reason | Required later phase | Status |
| --- | --- | --- | --- |
| Governed artifact missing | No committed governed artifact exists. | Phase 132 artifact creation rerun. | `signing_controls_blocked_by_missing_artifact` |
| Checksum evidence missing | No committed checksum evidence exists. | Phase 133 checksum/provenance evidence after artifact evidence exists. | `signing_controls_blocked_by_missing_checksum` |
| Provenance evidence missing | No committed provenance evidence exists. | Phase 133 checksum/provenance evidence after artifact evidence exists. | `signing_controls_blocked_by_missing_provenance` |
| Custody decision missing | No committed custody decision evidence exists. | Future custody decision before key material. | `signing_controls_blocked_by_missing_key_custody_decision` |
| Reassembly not performed | No Phase 139 evidence reassembly has occurred. | Phase 139. | `requires_phase_139_reassembly` |
| Decision gate not performed | No Phase 140 decision has occurred. | Phase 140. | `requires_phase_140_decision` |

## Checksum/Provenance Dependency Table
| Dependency | Required before future signing? | Current committed evidence | Status |
| --- | --- | --- | --- |
| Governed checksum evidence | Yes. | Missing. | `signing_controls_blocked_by_missing_checksum` |
| Governed provenance evidence | Yes. | Missing. | `signing_controls_blocked_by_missing_provenance` |
| Phase 133 evidence rerun after artifact evidence | Yes. | Missing. | `requires_phase_133_checksum_provenance_evidence` |
| Checksum/provenance requirements alone | No, requirements alone are not evidence. | Present as requirements only. | `signing_controls_deferred` |

## Artifact-Existence Dependency Table
| Dependency | Current committed evidence | Signing-control effect | Status |
| --- | --- | --- | --- |
| Governed artifact | Missing. | Blocks signing. | `signing_controls_blocked_by_missing_artifact` |
| Artifact manifest | Missing. | Blocks signing evidence progression. | `signing_controls_blocked_by_missing_artifact` |
| Phase 132.1 contract correction | Present as contract only. | Does not satisfy artifact evidence. | `requires_phase_132_artifact_creation_rerun` |
| Phase 132 deferred artifact status | Present. | Preserved. | `requires_phase_132_artifact_creation_rerun` |

## Key-Material Absence Table
| Material category | Phase 134 action | Current classification | Status |
| --- | --- | --- | --- |
| Private key files | None created. | Must remain absent unless future governed custody decision exists. | `signing_controls_defined_with_findings` |
| Certificates | None created. | Must remain absent unless future governed custody decision exists. | `signing_controls_defined_with_findings` |
| Signature files | None created. | Must remain absent until governed artifact, checksum, provenance, and custody decision evidence exists. | `signing_controls_blocked_by_missing_artifact` |
| Attestations | None created. | Must remain absent until governed artifact and provenance evidence path exists. | `signing_controls_blocked_by_missing_provenance` |
| Verification evidence | None created. | Requirements only; evidence missing. | `signing_controls_deferred` |

## Phase 139 Reassembly Dependency Table
| Dependency | Phase 134 finding | Status |
| --- | --- | --- |
| Evidence reassembly | Phase 139 has not occurred. Phase 134 cannot reassemble missing artifact, checksum, provenance, signing, custody, or decision evidence. | `requires_phase_139_reassembly` |
| Ladder preservation | Phase 134 preserves the need for Phase 139 before Phase 140. | `requires_phase_139_reassembly` |
| No implementation | Phase 134 does not implement Phase 139. | `requires_phase_139_reassembly` |

## Phase 140 Decision-Gate Dependency Table
| Dependency | Phase 134 finding | Status |
| --- | --- | --- |
| Release Candidate decision | Phase 130 remains `rc_candidate_not_ready`; Phase 140 has not occurred. | `requires_phase_140_decision` |
| Decision-gate evidence | Missing artifact, checksum, provenance, reassembly, and signing-control evidence prevents any current approval. | `requires_phase_140_decision` |
| No implementation | Phase 134 does not implement Phase 140. | `requires_phase_140_decision` |

## Non-Readiness Table
| Category | Phase 134 finding | Status |
| --- | --- | --- |
| Release Candidate | AJENTIC remains not Release Candidate ready; Phase 134 does not approve Release Candidate status. | `requires_phase_140_decision` |
| Production Candidate | AJENTIC remains not Production Candidate ready. | `signing_controls_deferred` |
| Public/general use | AJENTIC remains not public/general-use ready. | `signing_controls_deferred` |
| Production human use | Phase 134 creates no production-human-use approval. | `signing_controls_deferred` |

## Activation Prohibition Table
| Prohibited activation | Phase 134 finding | Status |
| --- | --- | --- |
| Signing | Not activated. | `signing_controls_blocked_by_missing_artifact` |
| Publishing | Not activated. | `signing_controls_not_applicable` |
| Distribution | Not activated. | `signing_controls_not_applicable` |
| Deployment | Not activated. | `signing_controls_not_applicable` |
| Monitoring | Not activated. | `signing_controls_not_applicable` |
| Installer/update-channel behavior | Not activated. | `signing_controls_not_applicable` |
| Readiness | Not activated. | `requires_phase_140_decision` |

## Meta-Boundary Table
| Boundary | Phase 134 classification | Status |
| --- | --- | --- |
| Governance vs. signing | Signing controls are governance, not signing. | `signing_controls_defined` |
| Policy vs. key creation | Key custody is policy, not key creation. | `signing_controls_defined` |
| Requirement vs. evidence | Verification requirements are not signature verification evidence. | `signing_controls_defined` |
| Missing evidence vs. approval | Missing evidence blocks approval inference. | `requires_phase_140_decision` |

## Ladder-Preservation Table
| Ladder rung | Preserved Phase 134 finding | Status |
| --- | --- | --- |
| Phase 130 decision | `rc_candidate_not_ready` preserved. | `requires_phase_140_decision` |
| Phase 132 artifact boundary | `artifact_creation_deferred` preserved. | `requires_phase_132_artifact_creation_rerun` |
| Phase 132.1 correction | Contract-only correction preserved. | `requires_phase_132_artifact_creation_rerun` |
| Phase 133 checksum/provenance boundary | `checksum_provenance_blocked_by_missing_artifact` preserved. | `requires_phase_133_checksum_provenance_evidence` |
| Phase 139 reassembly | Still required. | `requires_phase_139_reassembly` |
| Phase 140 decision | Still required. | `requires_phase_140_decision` |

## Cross-category inference rejection table
| Rejected inference | Reason | Status |
| --- | --- | --- |
| Requirements imply evidence | Requirements describe future controls only. | `signing_controls_deferred` |
| Clean validation implies signing evidence | Validation is not artifact, checksum, provenance, signature, custody, or verification evidence. | `signing_controls_deferred` |
| Missing prohibited files imply readiness | Absence of prohibited files is not approval evidence. | `requires_phase_140_decision` |
| Artifact contract implies artifact | Phase 132.1 is contract-only. | `requires_phase_132_artifact_creation_rerun` |
| Checksum/provenance requirements imply signing | Phase 133 requirements do not create signing evidence. | `requires_phase_133_checksum_provenance_evidence` |
| Signing/key-custody requirements imply publishing or release | Phase 134 creates no publishing, distribution, release, deployment, monitoring, or readiness behavior. | `signing_controls_not_applicable` |

## Phase 139 handoff
Phase 139 must not infer reassembly evidence from Phase 134 requirements. Any Phase 139 reassembly must start from committed governed artifact evidence, checksum evidence, provenance evidence, custody decision evidence, and verification-control evidence if those later exist.

Phase 134 handoff status: `requires_phase_139_reassembly`.

## Phase 140 handoff
Phase 140 must preserve that Phase 130 remains `rc_candidate_not_ready` unless a future decision gate evaluates complete committed evidence. Phase 134 does not approve Release Candidate status and does not supply artifact, checksum, provenance, reassembly, or decision-gate evidence.

Phase 134 handoff status: `requires_phase_140_decision`.

## Required follow-ups
- `requires_phase_132_artifact_creation_rerun`: create governed artifact evidence only in a later authorized phase.
- `requires_phase_133_checksum_provenance_evidence`: create checksum and provenance evidence only after governed artifact evidence exists.
- `signing_controls_blocked_by_missing_key_custody_decision`: record a future key-custody decision before introducing key material.
- `requires_phase_139_reassembly`: reassemble evidence only in Phase 139 or another authorized later phase.
- `requires_phase_140_decision`: evaluate readiness only in Phase 140 or another authorized later decision gate.

## Deferred items
| Deferred item | Reason | Status |
| --- | --- | --- |
| Key creation | Requires future custody decision and governed evidence inputs. | `signing_controls_blocked_by_missing_key_custody_decision` |
| Signature creation | Requires governed artifact, checksum, provenance, and custody decision evidence. | `signing_controls_blocked_by_missing_artifact` |
| Attestation creation | Requires governed artifact and provenance path. | `signing_controls_blocked_by_missing_provenance` |
| Verification evidence | Requires future signature and verification run. | `signing_controls_deferred` |
| Publishing/distribution/deployment/readiness | Outside Phase 134. | `signing_controls_not_applicable` |

## Validation log
| Command | Expected Phase 134 result |
| --- | --- |
| `CARGO_TARGET_DIR=/tmp/ajentic-phase-134-target ./scripts/check.sh` | Full repository validation passes from a clean worktree. |
| `git diff --check` | No whitespace errors. |
| `git status --short` | Shows only intended Phase 134 changes before commit and clean state after commit. |
| `find . -type f \( -name "*.key" -o -name "*.pem" -o -name "*.p12" -o -name "*.pfx" -o -name "*.sig" -o -name "*.asc" -o -name "*.crt" -o -name "*.cer" -o -name "*.intoto" -o -name "*.attestation" -o -name "*.zip" -o -name "*.tar" -o -name "*.tar.gz" -o -name "*.tgz" -o -name "*.dmg" -o -name "*.msi" -o -name "*.pkg" -o -name "*.exe" -o -name "*.deb" -o -name "*.rpm" -o -name "*.appcast" -o -name "*.sha256" -o -name "*.sha512" \) -not -path "./.git/*" -not -path "./core/target/*" -not -path "./ui/node_modules/*" -print` | No Phase 134-created signing keys, certificates, signatures, attestations, checksums, provenance files, installer packages, release bundles, update-channel metadata, or public assets. |
| `rg -n "Phase 134|signing_controls|signing_controls_blocked_by_missing_artifact|signing_controls_blocked_by_missing_checksum|signing_controls_blocked_by_missing_provenance|Signing controls are governance, not signing|Key custody is policy, not key creation|Verification requirements are not signature verification evidence|Phase 139|Phase 140" docs/operations/signing-key-custody-boundary-phase-134.md checklists/current-phase.md CHANGELOG.md` | Phase 134 boundary language is present only in intended documentation surfaces. |
| `rg -n "gh release|git tag|published|public download|public asset|signing|signature|key custody|certificate|attestation|installer|update-channel|deployment|monitoring_enabled|telemetry_enabled|release_candidate_approved|production_candidate_approved|public_use_approved" .github scripts core ui tests docs CHANGELOG.md README.md AGENTS.md` | Matches are historical, planned, test, specification, or prohibition context; Phase 134 introduces no active release, signing, key creation, certificate creation, publishing, installer/update-channel, deployment, monitoring, readiness, Production Candidate, or public/general-use behavior. |
| `git diff -- '*.rs' '*.ts' '*.tsx' tests schemas .github README.md AGENTS.md package-lock.json pnpm-lock.yaml yarn.lock docs/changelog/*.md docs/governance/*.md docs/architecture/*.md` | No guarded drift. |
