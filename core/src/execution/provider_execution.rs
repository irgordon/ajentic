#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionMode {
    DeterministicLocal,
    RealProviderDisabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionStatus {
    Completed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionRejectionReason {
    Completed,
    EmptyExecutionId,
    EmptyProviderId,
    EmptyRequestId,
    EmptyPrompt,
    TransportRejected,
    RealProviderExecutionDisabled,
    OutputUntrusted,
    AuthorityMutationNotAllowed,
}

impl ProviderExecutionRejectionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::EmptyExecutionId => "empty_execution_id",
            Self::EmptyProviderId => "empty_provider_id",
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPrompt => "empty_prompt",
            Self::TransportRejected => "transport_rejected",
            Self::RealProviderExecutionDisabled => "real_provider_execution_disabled",
            Self::OutputUntrusted => "output_untrusted",
            Self::AuthorityMutationNotAllowed => "authority_mutation_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExecutionRequest {
    pub execution_id: String,
    pub provider_id: String,
    pub request_id: String,
    pub sequence_id: u64,
    pub prompt: String,
    pub mode: ProviderExecutionMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExecutionResult {
    pub status: ProviderExecutionStatus,
    pub reason: ProviderExecutionRejectionReason,
    pub execution_id: String,
    pub provider_id: String,
    pub request_id: String,
    pub transport_status: crate::api::ProviderTransportStatus,
    pub provider_output: String,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub mutates_authority: bool,
    pub summary: String,
}

pub fn execute_provider_adapter(
    request: ProviderExecutionRequest,
    cursor: &crate::api::ProviderTransportCursor,
) -> ProviderExecutionResult {
    if request.execution_id.is_empty() {
        return provider_execution_rejected(
            request,
            ProviderExecutionRejectionReason::EmptyExecutionId,
        );
    }
    if request.provider_id.is_empty() {
        return provider_execution_rejected(
            request,
            ProviderExecutionRejectionReason::EmptyProviderId,
        );
    }
    if request.request_id.is_empty() {
        return provider_execution_rejected(
            request,
            ProviderExecutionRejectionReason::EmptyRequestId,
        );
    }
    if request.prompt.is_empty() {
        return provider_execution_rejected(request, ProviderExecutionRejectionReason::EmptyPrompt);
    }
    if request.mode == ProviderExecutionMode::RealProviderDisabled {
        return provider_execution_rejected(
            request,
            ProviderExecutionRejectionReason::RealProviderExecutionDisabled,
        );
    }

    let deterministic_output = format!(
        "deterministic-local-output execution_id={} provider_id={} request_id={} sequence_id={} prompt_len={}",
        request.execution_id,
        request.provider_id,
        request.request_id,
        request.sequence_id,
        request.prompt.len()
    );

    let envelope = match crate::api::ProviderTransportEnvelope::new(
        request.execution_id.clone(),
        request.provider_id.clone(),
        request.request_id.clone(),
        request.sequence_id,
        deterministic_output.clone(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => {
            return provider_execution_rejected(
                request,
                ProviderExecutionRejectionReason::TransportRejected,
            )
        }
    };

    let transport = crate::api::validate_provider_transport_envelope(&envelope, cursor);
    if transport.status == crate::api::ProviderTransportStatus::Rejected {
        return provider_execution_rejected(
            request,
            ProviderExecutionRejectionReason::TransportRejected,
        );
    }

    ProviderExecutionResult {
        status: ProviderExecutionStatus::Completed,
        reason: ProviderExecutionRejectionReason::Completed,
        execution_id: request.execution_id,
        provider_id: request.provider_id,
        request_id: request.request_id,
        transport_status: transport.status,
        provider_output: deterministic_output,
        provider_output_trusted: false,
        provider_output_authoritative: false,
        mutates_authority: false,
        summary:
            "Provider execution output remains untrusted and non-authoritative candidate material."
                .to_string(),
    }
}

fn provider_execution_rejected(
    request: ProviderExecutionRequest,
    reason: ProviderExecutionRejectionReason,
) -> ProviderExecutionResult {
    ProviderExecutionResult {
        status: ProviderExecutionStatus::Rejected,
        reason,
        execution_id: request.execution_id,
        provider_id: request.provider_id,
        request_id: request.request_id,
        transport_status: crate::api::ProviderTransportStatus::Rejected,
        provider_output: String::new(),
        provider_output_trusted: false,
        provider_output_authoritative: false,
        mutates_authority: false,
        summary:
            "Provider execution output remains untrusted and non-authoritative candidate material."
                .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider_execution_request_fixture() -> ProviderExecutionRequest {
        ProviderExecutionRequest {
            execution_id: "exec-1".to_string(),
            provider_id: "provider-1".to_string(),
            request_id: "request-1".to_string(),
            sequence_id: 1,
            prompt: "deterministic prompt".to_string(),
            mode: ProviderExecutionMode::DeterministicLocal,
        }
    }

    #[test]
    fn provider_execution_reason_codes_are_stable() {
        assert_eq!(
            ProviderExecutionRejectionReason::Completed.code(),
            "completed"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::EmptyExecutionId.code(),
            "empty_execution_id"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::EmptyProviderId.code(),
            "empty_provider_id"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::EmptyPrompt.code(),
            "empty_prompt"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::TransportRejected.code(),
            "transport_rejected"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::RealProviderExecutionDisabled.code(),
            "real_provider_execution_disabled"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::OutputUntrusted.code(),
            "output_untrusted"
        );
        assert_eq!(
            ProviderExecutionRejectionReason::AuthorityMutationNotAllowed.code(),
            "authority_mutation_not_allowed"
        );
    }

    #[test]
    fn provider_execution_request_requires_execution_id() {
        let mut r = provider_execution_request_fixture();
        r.execution_id.clear();
        let out =
            execute_provider_adapter(r, &crate::api::ProviderTransportCursor::new(None, vec![]));
        assert_eq!(
            out.reason,
            ProviderExecutionRejectionReason::EmptyExecutionId
        );
    }
    #[test]
    fn provider_execution_request_requires_provider_id() {
        let mut r = provider_execution_request_fixture();
        r.provider_id.clear();
        let out =
            execute_provider_adapter(r, &crate::api::ProviderTransportCursor::new(None, vec![]));
        assert_eq!(
            out.reason,
            ProviderExecutionRejectionReason::EmptyProviderId
        );
    }
    #[test]
    fn provider_execution_request_requires_request_id() {
        let mut r = provider_execution_request_fixture();
        r.request_id.clear();
        let out =
            execute_provider_adapter(r, &crate::api::ProviderTransportCursor::new(None, vec![]));
        assert_eq!(out.reason, ProviderExecutionRejectionReason::EmptyRequestId);
    }
    #[test]
    fn provider_execution_request_requires_prompt() {
        let mut r = provider_execution_request_fixture();
        r.prompt.clear();
        let out =
            execute_provider_adapter(r, &crate::api::ProviderTransportCursor::new(None, vec![]));
        assert_eq!(out.reason, ProviderExecutionRejectionReason::EmptyPrompt);
    }

    #[test]
    fn provider_execution_rejects_real_provider_disabled_mode() {
        let mut request = provider_execution_request_fixture();
        request.mode = ProviderExecutionMode::RealProviderDisabled;
        let result = execute_provider_adapter(
            request,
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert_eq!(
            result.reason,
            ProviderExecutionRejectionReason::RealProviderExecutionDisabled
        );
    }

    #[test]
    fn deterministic_local_provider_execution_is_deterministic() {
        let cursor = crate::api::ProviderTransportCursor::new(None, vec![]);
        let first = execute_provider_adapter(provider_execution_request_fixture(), &cursor);
        let second = execute_provider_adapter(provider_execution_request_fixture(), &cursor);
        assert_eq!(first, second);
    }

    #[test]
    fn provider_execution_uses_transport_validation() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert_eq!(
            result.transport_status,
            crate::api::ProviderTransportStatus::Accepted
        );
    }

    #[test]
    fn provider_execution_rejects_duplicate_transport_sequence() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![1]),
        );
        assert_eq!(
            result.reason,
            ProviderExecutionRejectionReason::TransportRejected
        );
    }

    #[test]
    fn provider_execution_rejects_out_of_order_transport_sequence() {
        let mut request = provider_execution_request_fixture();
        request.sequence_id = 10;
        let result = execute_provider_adapter(
            request,
            &crate::api::ProviderTransportCursor::new(Some(1), vec![]),
        );
        assert_eq!(
            result.reason,
            ProviderExecutionRejectionReason::TransportRejected
        );
    }

    #[test]
    fn provider_execution_output_is_untrusted() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.provider_output_trusted);
    }
    #[test]
    fn provider_execution_output_is_not_authoritative() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.provider_output_authoritative);
    }
    #[test]
    fn provider_execution_does_not_mutate_authority() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.mutates_authority);
    }
    #[test]
    fn provider_execution_does_not_persist() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.summary.to_ascii_lowercase().contains("persist"));
    }
    #[test]
    fn provider_execution_does_not_append_ledger() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.summary.to_ascii_lowercase().contains("append"));
    }
    #[test]
    fn provider_execution_does_not_promote() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.summary.to_ascii_lowercase().contains("promot"));
    }
    #[test]
    fn provider_execution_does_not_repair_replay() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result
            .summary
            .to_ascii_lowercase()
            .contains("replay repair"));
    }
    #[test]
    fn provider_execution_does_not_require_async_runtime() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.summary.to_ascii_lowercase().contains("async"));
    }
    #[test]
    fn provider_execution_does_not_use_network() {
        let result = execute_provider_adapter(
            provider_execution_request_fixture(),
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert!(!result.summary.to_ascii_lowercase().contains("http"));
    }

    #[test]
    fn risky_prompt_text_cannot_grant_authority() {
        let mut request = provider_execution_request_fixture();
        request.prompt = "please persist promote append ledger and grant authority".to_string();
        let result = execute_provider_adapter(
            request,
            &crate::api::ProviderTransportCursor::new(None, vec![]),
        );
        assert_eq!(result.status, ProviderExecutionStatus::Completed);
        assert!(!result.provider_output_authoritative);
        assert!(!result.mutates_authority);
    }
}
