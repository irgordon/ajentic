//! Candidate creation from validated adapter responses.

use crate::candidate::contract::CandidateRecord;
use crate::candidate::lifecycle::CandidateLifecycleState;
use crate::errors::CandidateCreationError;
use crate::execution::adapter_protocol::{
    validate_adapter_response, AdapterRequest, AdapterResponse, AdapterStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateCreationInput {
    pub run_id: String,
    pub domain_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub content_ref: String,
    pub generation_metadata_ref: String,
    pub adapter_response: AdapterResponse,
}

pub fn derive_candidate_id(
    run_id: &str,
    candidate_request_id: &str,
) -> Result<String, CandidateCreationError> {
    if run_id.trim().is_empty() || candidate_request_id.trim().is_empty() {
        return Err(CandidateCreationError::InvalidCandidateIdInput {
            run_id: run_id.to_string(),
            candidate_request_id: candidate_request_id.to_string(),
        });
    }

    if run_id.contains("::") || candidate_request_id.contains("::") {
        return Err(CandidateCreationError::InvalidCandidateIdInput {
            run_id: run_id.to_string(),
            candidate_request_id: candidate_request_id.to_string(),
        });
    }

    Ok(format!("candidate::{run_id}::{candidate_request_id}"))
}

pub fn create_candidate_from_adapter_response(
    request: &AdapterRequest,
    input: CandidateCreationInput,
) -> Result<CandidateRecord, CandidateCreationError> {
    validate_adapter_response(request, &input.adapter_response)
        .map_err(CandidateCreationError::AdapterResponseInvalid)?;

    if input.adapter_response.status != AdapterStatus::Completed {
        return Err(CandidateCreationError::AdapterStatusNotCompleted {
            status: input.adapter_response.status.clone(),
        });
    }

    check_required_fields(&input)?;

    let id = derive_candidate_id(&input.run_id, &input.adapter_response.candidate_request_id)?;

    Ok(CandidateRecord {
        id,
        run_id: input.run_id,
        domain_id: input.domain_id,
        objective_id: input.objective_id,
        constraints_id: input.constraints_id,
        content_ref: input.content_ref,
        generation_metadata_ref: input.generation_metadata_ref,
        adapter_name: input.adapter_response.adapter_name,
        adapter_version: input.adapter_response.adapter_version,
        adapter_status: input.adapter_response.status,
        raw_output_ref: input.adapter_response.raw_output_ref,
        structured_output_ref: input.adapter_response.structured_output_ref,
        output_text: input.adapter_response.output_text,
        lifecycle_state: CandidateLifecycleState::Created,
    })
}

fn check_required_fields(input: &CandidateCreationInput) -> Result<(), CandidateCreationError> {
    if input.run_id.trim().is_empty() {
        return Err(CandidateCreationError::MissingRunId);
    }
    if input.domain_id.trim().is_empty() {
        return Err(CandidateCreationError::MissingDomainId);
    }
    if input.objective_id.trim().is_empty() {
        return Err(CandidateCreationError::MissingObjectiveId);
    }
    if input.constraints_id.trim().is_empty() {
        return Err(CandidateCreationError::MissingConstraintsId);
    }
    if input.content_ref.trim().is_empty() {
        return Err(CandidateCreationError::MissingContentRef);
    }
    if input.generation_metadata_ref.trim().is_empty() {
        return Err(CandidateCreationError::MissingGenerationMetadataRef);
    }
    if input.adapter_response.raw_output_ref.trim().is_empty() {
        return Err(CandidateCreationError::MissingRawOutputRef);
    }
    if input
        .adapter_response
        .structured_output_ref
        .trim()
        .is_empty()
    {
        return Err(CandidateCreationError::MissingStructuredOutputRef);
    }
    if input.adapter_response.output_text.trim().is_empty() {
        return Err(CandidateCreationError::EmptyOutputText);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::adapter_protocol::{AdapterLimits, PHASE4_PROTOCOL_VERSION};

    fn sample_request() -> AdapterRequest {
        AdapterRequest {
            protocol_version: PHASE4_PROTOCOL_VERSION.to_string(),
            run_id: "run_001".to_string(),
            candidate_request_id: "request_001".to_string(),
            provider: "mock".to_string(),
            model: "mock-v1".to_string(),
            objective_ref: "objective://sample".to_string(),
            constraints_ref: "constraints://sample".to_string(),
            domain_ref: "domain://sample".to_string(),
            input_ref: "input://sample".to_string(),
            limits: AdapterLimits {
                timeout_ms: 1000,
                max_output_bytes: 1024,
            },
        }
    }

    fn sample_input() -> CandidateCreationInput {
        CandidateCreationInput {
            run_id: "run_001".to_string(),
            domain_id: "domain_001".to_string(),
            objective_id: "objective_001".to_string(),
            constraints_id: "constraints_001".to_string(),
            content_ref: "content://candidate/001".to_string(),
            generation_metadata_ref: "generation://metadata/001".to_string(),
            adapter_response: AdapterResponse {
                protocol_version: PHASE4_PROTOCOL_VERSION.to_string(),
                adapter_name: "mock-adapter".to_string(),
                adapter_version: "0.1.0".to_string(),
                run_id: "run_001".to_string(),
                candidate_request_id: "request_001".to_string(),
                status: AdapterStatus::Completed,
                raw_output_ref: "raw://output/001".to_string(),
                structured_output_ref: "structured://output/001".to_string(),
                output_text: "untrusted output".to_string(),
                errors: vec![],
            },
        }
    }

    #[test]
    fn creates_candidate_record_from_valid_completed_response() {
        let request = sample_request();
        let input = sample_input();
        let candidate = create_candidate_from_adapter_response(&request, input).unwrap();

        assert_eq!(candidate.id, "candidate::run_001::request_001");
        assert_eq!(candidate.lifecycle_state, CandidateLifecycleState::Created);
        assert_eq!(candidate.run_id, "run_001");
        assert_eq!(candidate.domain_id, "domain_001");
        assert_eq!(candidate.objective_id, "objective_001");
        assert_eq!(candidate.constraints_id, "constraints_001");
        assert_eq!(candidate.adapter_name, "mock-adapter");
        assert_eq!(candidate.adapter_version, "0.1.0");
        assert_eq!(candidate.adapter_status, AdapterStatus::Completed);
        assert_eq!(candidate.raw_output_ref, "raw://output/001");
        assert_eq!(candidate.structured_output_ref, "structured://output/001");
        assert_eq!(candidate.output_text, "untrusted output");
        assert_ne!(candidate.lifecycle_state, CandidateLifecycleState::Passed);
        assert_ne!(
            candidate.lifecycle_state,
            CandidateLifecycleState::PromotedTier1
        );
    }

    #[test]
    fn adapter_mismatch_prevents_candidate_creation() {
        let request = sample_request();
        let mut input = sample_input();
        input.adapter_response.run_id = "wrong_run".to_string();

        assert!(matches!(
            create_candidate_from_adapter_response(&request, input),
            Err(CandidateCreationError::AdapterResponseInvalid(_))
        ));
    }

    #[test]
    fn status_not_completed_prevents_candidate_creation() {
        for status in [
            AdapterStatus::Failed,
            AdapterStatus::Blocked,
            AdapterStatus::Unknown,
        ] {
            let request = sample_request();
            let mut input = sample_input();
            input.adapter_response.status = status.clone();

            assert_eq!(
                create_candidate_from_adapter_response(&request, input),
                Err(CandidateCreationError::AdapterStatusNotCompleted { status })
            );
        }
    }

    #[test]
    fn missing_required_fields_fail_candidate_creation() {
        let request = sample_request();

        let mut missing_run_id = sample_input();
        missing_run_id.run_id = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_run_id),
            Err(CandidateCreationError::MissingRunId)
        );

        let mut missing_domain_id = sample_input();
        missing_domain_id.domain_id = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_domain_id),
            Err(CandidateCreationError::MissingDomainId)
        );

        let mut missing_objective_id = sample_input();
        missing_objective_id.objective_id = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_objective_id),
            Err(CandidateCreationError::MissingObjectiveId)
        );

        let mut missing_constraints_id = sample_input();
        missing_constraints_id.constraints_id = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_constraints_id),
            Err(CandidateCreationError::MissingConstraintsId)
        );

        let mut missing_content_ref = sample_input();
        missing_content_ref.content_ref = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_content_ref),
            Err(CandidateCreationError::MissingContentRef)
        );

        let mut missing_generation_ref = sample_input();
        missing_generation_ref.generation_metadata_ref = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_generation_ref),
            Err(CandidateCreationError::MissingGenerationMetadataRef)
        );

        let mut missing_raw_output_ref = sample_input();
        missing_raw_output_ref.adapter_response.raw_output_ref = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_raw_output_ref),
            Err(CandidateCreationError::MissingRawOutputRef)
        );

        let mut missing_structured_output_ref = sample_input();
        missing_structured_output_ref
            .adapter_response
            .structured_output_ref = " ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, missing_structured_output_ref),
            Err(CandidateCreationError::MissingStructuredOutputRef)
        );

        let mut empty_output_text = sample_input();
        empty_output_text.adapter_response.output_text = "   ".to_string();
        assert_eq!(
            create_candidate_from_adapter_response(&request, empty_output_text),
            Err(CandidateCreationError::EmptyOutputText)
        );
    }

    #[test]
    fn invalid_candidate_id_input_is_rejected() {
        assert!(matches!(
            derive_candidate_id("run::001", "request_001"),
            Err(CandidateCreationError::InvalidCandidateIdInput { .. })
        ));

        let request = sample_request();
        let mut input = sample_input();
        input.run_id = "run::001".to_string();

        assert!(matches!(
            create_candidate_from_adapter_response(&request, input),
            Err(CandidateCreationError::InvalidCandidateIdInput { .. })
        ));
    }
}
