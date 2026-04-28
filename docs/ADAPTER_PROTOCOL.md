# Adapter Protocol

This phase establishes the adapter protocol boundary.

## Phase 4 runtime request shape

Rust constructs a minimal runtime adapter request:

- `protocol_version`
- `run_id`
- `candidate_request_id`
- `provider`
- `model`
- `objective_ref`
- `constraints_ref`
- `domain_ref`
- `input_ref`
- `limits.timeout_ms`
- `limits.max_output_bytes`

## Phase 4 runtime response shape

Rust parses and validates a minimal runtime adapter response:

- `protocol_version`
- `adapter_name`
- `adapter_version`
- `run_id`
- `candidate_request_id`
- `status` (`COMPLETED`, `FAILED`, `BLOCKED`, `UNKNOWN`)
- `raw_output_ref`
- `structured_output_ref`
- `output_text`
- `error` (repeatable key for error lines)

## Trust boundary

- Rust validates adapter response shape and request linkage.
- The mock adapter returns deterministic untrusted output.
- A completed adapter response is not a passing candidate.
- Candidate creation and runtime checks are reserved for later phases.
- Timeout execution is reserved for a later phase.
