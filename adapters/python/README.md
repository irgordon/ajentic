# AJENTIC Python Adapters

Python adapters are **non-authoritative**.

## Phase 4 scope

- This phase establishes the adapter protocol boundary.
- The `mock_adapter.py` provider returns deterministic untrusted output.
- Rust validates adapter response shape and request linkage.
- A completed adapter response is not a passing candidate.
- Candidate creation and runtime checks are reserved for later phases.
- Timeout execution is reserved for a later phase.

## Included adapter

- `ajentic_adapter/providers/mock_adapter.py`
  - Reads a narrow `key=value` request from stdin.
  - Emits one deterministic `key=value` response to stdout.
  - Uses no external services, randomness, network calls, or file writes.

See [GOVERNANCE.md](../../GOVERNANCE.md) for the authority model.
