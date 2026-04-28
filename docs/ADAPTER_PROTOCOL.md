# Adapter Protocol

Phase 1 defines adapter protocol contract fields only.

## AdapterRequestContract fields

- `protocol_version`
- `run_id`
- `candidate_request_id`
- `provider`
- `model`
- `objective_ref`
- `constraints_ref`
- `domain_ref`
- `input_ref`
- `limits`

## AdapterResponseContract fields

- `protocol_version`
- `adapter_name`
- `adapter_version`
- `status`
- `raw_output_ref`
- `structured_output_ref`
- `usage`
- `errors`

## Trust boundary

Adapter output remains untrusted. These contracts reserve boundary shape only;
validation and runtime handling will be added in later phases.
