# AJENTIC Python Adapters

Python adapters are **non-authoritative**.

## Phase 10 scope

- Phase 10 adds one local provider adapter behind the existing adapter boundary.
- Local model output remains untrusted adapter output.
- Provider source does not affect governance or promotion semantics.
- The local adapter cannot promote candidates.
- The local adapter cannot write ledger or audit records.
- Replay does not call local providers.
- Cloud providers remain reserved for a later phase.

## Included adapters

- `ajentic_adapter/providers/mock_adapter.py`
  - Reads a narrow `key=value` request from stdin.
  - Emits one deterministic `key=value` response to stdout.

- `ajentic_adapter/providers/ollama_adapter.py`
  - Reads the same `key=value` request format from stdin.
  - Calls a local Ollama endpoint using stdlib `urllib`.
  - Emits the same `key=value` adapter response format to stdout.
  - Maps unavailable provider, timeout, malformed provider JSON, and empty output to explicit non-success statuses.

## Local Ollama configuration

- Endpoint: `AJENTIC_OLLAMA_URL` (default: `http://127.0.0.1:11434/api/generate`)
- Model: `AJENTIC_OLLAMA_MODEL` (optional; falls back to request `model` value)

See [GOVERNANCE.md](../../GOVERNANCE.md) for the authority model.
