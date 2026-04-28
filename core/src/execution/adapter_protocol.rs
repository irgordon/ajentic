//! Phase 1 contract shapes for the Adapter Request and Adapter Response types.
//!
//! This file defines the contract shape only. Adapter output is untrusted.
//! Validation behavior will be added in later phases. No parsing,
//! serialization, or adapter subprocess logic is implemented here.

use std::collections::HashMap;

/// Contract shape for an adapter request sent from the Rust harness.
///
/// Field names align with `schemas/adapter_request.schema.json`.
pub struct AdapterRequestContract {
    /// Version of the adapter protocol this request conforms to.
    pub protocol_version: String,
    /// Run identifier associated with this adapter request.
    pub run_id: String,
    /// Unique identifier for this candidate generation request.
    pub candidate_request_id: String,
    /// Provider identifier the adapter should use.
    pub provider: String,
    /// Model identifier the adapter should use.
    pub model: String,
    /// Reference to the objective document for this request.
    pub objective_ref: String,
    /// Reference to the constraints document for this request.
    pub constraints_ref: String,
    /// Reference to the domain document for this request.
    pub domain_ref: String,
    /// Reference to the input artifact for this request.
    pub input_ref: String,
    /// Resource and output limits for this request.
    pub limits: HashMap<String, String>,
}

/// Contract shape for an adapter response returned to the Rust harness.
///
/// Adapter output is untrusted. The harness normalizes and validates all
/// adapter responses before any governance decision is made.
///
/// Field names align with `schemas/adapter_response.schema.json`.
pub struct AdapterResponseContract {
    /// Version of the adapter protocol this response conforms to.
    pub protocol_version: String,
    /// Name of the adapter that produced this response.
    pub adapter_name: String,
    /// Version of the adapter that produced this response.
    pub adapter_version: String,
    /// Status of the adapter execution.
    pub status: String,
    /// Reference to the raw output artifact. Adapter output is untrusted.
    pub raw_output_ref: String,
    /// Reference to the structured output artifact. Adapter output is untrusted.
    pub structured_output_ref: String,
    /// Resource usage reported by the adapter.
    pub usage: HashMap<String, String>,
    /// Error messages reported by the adapter, if any.
    pub errors: Vec<String>,
}
