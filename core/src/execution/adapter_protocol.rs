//! Phase 1 adapter protocol contract shapes.
//! These definitions reserve the boundary contract only.

use crate::evaluation::contract::ContractStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequestContract {
    pub protocol_version: String,
    pub run_id: String,
    pub candidate_request_id: String,
    pub provider: String,
    pub model: String,
    pub objective_ref: String,
    pub constraints_ref: String,
    pub domain_ref: String,
    pub input_ref: String,
    pub limits: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResponseContract {
    pub protocol_version: String,
    pub adapter_name: String,
    pub adapter_version: String,
    pub status: ContractStatus,
    pub raw_output_ref: String,
    pub structured_output_ref: String,
    pub usage: Vec<String>,
    pub errors: Vec<String>,
}
