//! Phase 4 adapter protocol runtime boundary.
//!
//! Rust owns request construction and response validation at this trust boundary.

use crate::errors::AdapterProtocolError;

pub const PHASE4_PROTOCOL_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterLimits {
    pub timeout_ms: u64,
    pub max_output_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequest {
    pub protocol_version: String,
    pub run_id: String,
    pub candidate_request_id: String,
    pub provider: String,
    pub model: String,
    pub objective_ref: String,
    pub constraints_ref: String,
    pub domain_ref: String,
    pub input_ref: String,
    pub limits: AdapterLimits,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterStatus {
    Completed,
    Failed,
    Blocked,
    Unknown,
}

impl AdapterStatus {
    fn parse(value: &str) -> Result<Self, AdapterProtocolError> {
        match value {
            "COMPLETED" => Ok(Self::Completed),
            "FAILED" => Ok(Self::Failed),
            "BLOCKED" => Ok(Self::Blocked),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Err(AdapterProtocolError::InvalidStatus {
                value: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResponse {
    pub protocol_version: String,
    pub adapter_name: String,
    pub adapter_version: String,
    pub run_id: String,
    pub candidate_request_id: String,
    pub status: AdapterStatus,
    pub raw_output_ref: String,
    pub structured_output_ref: String,
    pub output_text: String,
    pub errors: Vec<String>,
}

pub fn build_adapter_request_text(request: &AdapterRequest) -> String {
    let mut text = String::new();
    push_line(&mut text, "protocol_version", &request.protocol_version);
    push_line(&mut text, "run_id", &request.run_id);
    push_line(
        &mut text,
        "candidate_request_id",
        &request.candidate_request_id,
    );
    push_line(&mut text, "provider", &request.provider);
    push_line(&mut text, "model", &request.model);
    push_line(&mut text, "objective_ref", &request.objective_ref);
    push_line(&mut text, "constraints_ref", &request.constraints_ref);
    push_line(&mut text, "domain_ref", &request.domain_ref);
    push_line(&mut text, "input_ref", &request.input_ref);
    push_line(
        &mut text,
        "timeout_ms",
        &request.limits.timeout_ms.to_string(),
    );
    push_line(
        &mut text,
        "max_output_bytes",
        &request.limits.max_output_bytes.to_string(),
    );
    text
}

pub fn parse_adapter_response_text(text: &str) -> Result<AdapterResponse, AdapterProtocolError> {
    let mut protocol_version = None;
    let mut adapter_name = None;
    let mut adapter_version = None;
    let mut run_id = None;
    let mut candidate_request_id = None;
    let mut status = None;
    let mut raw_output_ref = None;
    let mut structured_output_ref = None;
    let mut output_text = None;
    let mut errors = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(AdapterProtocolError::ParseFailed);
        };

        match key {
            "protocol_version" => protocol_version = Some(value.to_string()),
            "adapter_name" => adapter_name = Some(value.to_string()),
            "adapter_version" => adapter_version = Some(value.to_string()),
            "run_id" => run_id = Some(value.to_string()),
            "candidate_request_id" => candidate_request_id = Some(value.to_string()),
            "status" => status = Some(AdapterStatus::parse(value)?),
            "raw_output_ref" => raw_output_ref = Some(value.to_string()),
            "structured_output_ref" => structured_output_ref = Some(value.to_string()),
            "output_text" => output_text = Some(value.to_string()),
            "error" => errors.push(value.to_string()),
            _ => return Err(AdapterProtocolError::ParseFailed),
        }
    }

    Ok(AdapterResponse {
        protocol_version: protocol_version.ok_or(AdapterProtocolError::ParseFailed)?,
        adapter_name: adapter_name.ok_or(AdapterProtocolError::ParseFailed)?,
        adapter_version: adapter_version.ok_or(AdapterProtocolError::ParseFailed)?,
        run_id: run_id.ok_or(AdapterProtocolError::ParseFailed)?,
        candidate_request_id: candidate_request_id.ok_or(AdapterProtocolError::ParseFailed)?,
        status: status.ok_or(AdapterProtocolError::ParseFailed)?,
        raw_output_ref: raw_output_ref.ok_or(AdapterProtocolError::ParseFailed)?,
        structured_output_ref: structured_output_ref.ok_or(AdapterProtocolError::ParseFailed)?,
        output_text: output_text.ok_or(AdapterProtocolError::ParseFailed)?,
        errors,
    })
}

pub fn validate_adapter_response(
    request: &AdapterRequest,
    response: &AdapterResponse,
) -> Result<(), AdapterProtocolError> {
    if response.protocol_version != request.protocol_version {
        return Err(AdapterProtocolError::ProtocolVersionMismatch {
            expected: request.protocol_version.clone(),
            found: response.protocol_version.clone(),
        });
    }

    if response.run_id != request.run_id {
        return Err(AdapterProtocolError::RunIdMismatch {
            expected: request.run_id.clone(),
            found: response.run_id.clone(),
        });
    }

    if response.candidate_request_id != request.candidate_request_id {
        return Err(AdapterProtocolError::CandidateRequestIdMismatch {
            expected: request.candidate_request_id.clone(),
            found: response.candidate_request_id.clone(),
        });
    }

    if response.adapter_name.trim().is_empty() {
        return Err(AdapterProtocolError::EmptyAdapterName);
    }

    if response.adapter_version.trim().is_empty() {
        return Err(AdapterProtocolError::EmptyAdapterVersion);
    }

    let output_size = response.output_text.as_bytes().len();
    if output_size > request.limits.max_output_bytes {
        return Err(AdapterProtocolError::OutputTooLarge {
            max: request.limits.max_output_bytes,
            actual: output_size,
        });
    }

    Ok(())
}

fn push_line(buffer: &mut String, key: &str, value: &str) {
    buffer.push_str(key);
    buffer.push('=');
    buffer.push_str(value);
    buffer.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> AdapterRequest {
        AdapterRequest {
            protocol_version: PHASE4_PROTOCOL_VERSION.to_string(),
            run_id: "run_001".to_string(),
            candidate_request_id: "candidate_request_001".to_string(),
            provider: "mock".to_string(),
            model: "mock-v1".to_string(),
            objective_ref: "objective://mock".to_string(),
            constraints_ref: "constraints://mock".to_string(),
            domain_ref: "domain://mock".to_string(),
            input_ref: "input://mock".to_string(),
            limits: AdapterLimits {
                timeout_ms: 1000,
                max_output_bytes: 512,
            },
        }
    }

    fn sample_response() -> AdapterResponse {
        AdapterResponse {
            protocol_version: PHASE4_PROTOCOL_VERSION.to_string(),
            adapter_name: "mock".to_string(),
            adapter_version: "0.1.0".to_string(),
            run_id: "run_001".to_string(),
            candidate_request_id: "candidate_request_001".to_string(),
            status: AdapterStatus::Completed,
            raw_output_ref: "mock://raw/run_001".to_string(),
            structured_output_ref: "mock://structured/run_001".to_string(),
            output_text:
                "This is deterministic mock adapter output. It is untrusted and not evaluated."
                    .to_string(),
            errors: vec![],
        }
    }

    #[test]
    fn valid_completed_response_passes_validation() {
        let request = sample_request();
        let response = sample_response();

        assert!(validate_adapter_response(&request, &response).is_ok());
    }

    #[test]
    fn protocol_version_mismatch_fails() {
        let request = sample_request();
        let mut response = sample_response();
        response.protocol_version = "0.9.9".to_string();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert!(matches!(
            error,
            AdapterProtocolError::ProtocolVersionMismatch { .. }
        ));
    }

    #[test]
    fn run_id_mismatch_fails() {
        let request = sample_request();
        let mut response = sample_response();
        response.run_id = "run_other".to_string();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert!(matches!(error, AdapterProtocolError::RunIdMismatch { .. }));
    }

    #[test]
    fn candidate_request_id_mismatch_fails() {
        let request = sample_request();
        let mut response = sample_response();
        response.candidate_request_id = "candidate_request_other".to_string();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert!(matches!(
            error,
            AdapterProtocolError::CandidateRequestIdMismatch { .. }
        ));
    }

    #[test]
    fn empty_adapter_name_fails() {
        let request = sample_request();
        let mut response = sample_response();
        response.adapter_name = "   ".to_string();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert_eq!(error, AdapterProtocolError::EmptyAdapterName);
    }

    #[test]
    fn empty_adapter_version_fails() {
        let request = sample_request();
        let mut response = sample_response();
        response.adapter_version = " ".to_string();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert_eq!(error, AdapterProtocolError::EmptyAdapterVersion);
    }

    #[test]
    fn output_over_limit_fails() {
        let mut request = sample_request();
        request.limits.max_output_bytes = 10;
        let response = sample_response();

        let error = validate_adapter_response(&request, &response).unwrap_err();
        assert!(matches!(error, AdapterProtocolError::OutputTooLarge { .. }));
    }

    #[test]
    fn malformed_response_text_fails() {
        let response_text = "not-valid";

        let error = parse_adapter_response_text(response_text).unwrap_err();
        assert_eq!(error, AdapterProtocolError::ParseFailed);
    }

    #[test]
    fn invalid_status_fails() {
        let response_text = "protocol_version=0.1.0\n\
adapter_name=mock\n\
adapter_version=0.1.0\n\
run_id=run_001\n\
candidate_request_id=candidate_request_001\n\
status=PASS\n\
raw_output_ref=mock://raw/run_001\n\
structured_output_ref=mock://structured/run_001\n\
output_text=abc\n";

        let error = parse_adapter_response_text(response_text).unwrap_err();
        assert!(matches!(error, AdapterProtocolError::InvalidStatus { .. }));
    }
}
