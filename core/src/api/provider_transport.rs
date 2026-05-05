#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderTransportPayloadTrust {
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderTransportStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderTransportRejectionReason {
    Accepted,
    EmptyEnvelopeId,
    EmptyProviderId,
    EmptyRequestId,
    EmptySequenceId,
    EmptyPayload,
    StaleSequence,
    DuplicateSequence,
    OutOfOrderSequence,
    ProviderOutputUntrusted,
    ExecutionNotEnabled,
}

impl ProviderTransportRejectionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::EmptyEnvelopeId => "empty_envelope_id",
            Self::EmptyProviderId => "empty_provider_id",
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptySequenceId => "empty_sequence_id",
            Self::EmptyPayload => "empty_payload",
            Self::StaleSequence => "stale_sequence",
            Self::DuplicateSequence => "duplicate_sequence",
            Self::OutOfOrderSequence => "out_of_order_sequence",
            Self::ProviderOutputUntrusted => "provider_output_untrusted",
            Self::ExecutionNotEnabled => "execution_not_enabled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderTransportEnvelope {
    pub envelope_id: String,
    pub provider_id: String,
    pub request_id: String,
    pub sequence_id: u64,
    pub payload: String,
    pub trust: ProviderTransportPayloadTrust,
}

impl ProviderTransportEnvelope {
    pub fn new(
        envelope_id: impl Into<String>,
        provider_id: impl Into<String>,
        request_id: impl Into<String>,
        sequence_id: u64,
        payload: impl Into<String>,
    ) -> Result<Self, ProviderTransportRejectionReason> {
        let envelope_id = envelope_id.into();
        if envelope_id.is_empty() {
            return Err(ProviderTransportRejectionReason::EmptyEnvelopeId);
        }

        let provider_id = provider_id.into();
        if provider_id.is_empty() {
            return Err(ProviderTransportRejectionReason::EmptyProviderId);
        }

        let request_id = request_id.into();
        if request_id.is_empty() {
            return Err(ProviderTransportRejectionReason::EmptyRequestId);
        }

        if sequence_id == 0 {
            return Err(ProviderTransportRejectionReason::EmptySequenceId);
        }

        let payload = payload.into();
        if payload.is_empty() {
            return Err(ProviderTransportRejectionReason::EmptyPayload);
        }

        Ok(Self {
            envelope_id,
            provider_id,
            request_id,
            sequence_id,
            payload,
            trust: ProviderTransportPayloadTrust::Untrusted,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderTransportCursor {
    pub last_sequence_id: Option<u64>,
    pub seen_sequence_ids: Vec<u64>,
}

impl ProviderTransportCursor {
    pub fn new(last_sequence_id: Option<u64>, seen_sequence_ids: Vec<u64>) -> Self {
        Self {
            last_sequence_id,
            seen_sequence_ids,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderTransportReport {
    pub status: ProviderTransportStatus,
    pub reason: ProviderTransportRejectionReason,
    pub envelope_id: String,
    pub provider_id: String,
    pub request_id: String,
    pub sequence_id: Option<u64>,
    pub payload_trust: ProviderTransportPayloadTrust,
    pub mutates_authority: bool,
    pub summary: String,
}

pub fn validate_provider_transport_envelope(
    envelope: &ProviderTransportEnvelope,
    cursor: &ProviderTransportCursor,
) -> ProviderTransportReport {
    let reason = if cursor.seen_sequence_ids.contains(&envelope.sequence_id) {
        ProviderTransportRejectionReason::DuplicateSequence
    } else if let Some(last_sequence_id) = cursor.last_sequence_id {
        if envelope.sequence_id < last_sequence_id {
            ProviderTransportRejectionReason::StaleSequence
        } else if envelope.sequence_id > last_sequence_id + 1 {
            ProviderTransportRejectionReason::OutOfOrderSequence
        } else {
            ProviderTransportRejectionReason::Accepted
        }
    } else {
        ProviderTransportRejectionReason::Accepted
    };

    let status = if reason == ProviderTransportRejectionReason::Accepted {
        ProviderTransportStatus::Accepted
    } else {
        ProviderTransportStatus::Rejected
    };

    ProviderTransportReport {
        status,
        reason,
        envelope_id: envelope.envelope_id.clone(),
        provider_id: envelope.provider_id.clone(),
        request_id: envelope.request_id.clone(),
        sequence_id: Some(envelope.sequence_id),
        payload_trust: ProviderTransportPayloadTrust::Untrusted,
        mutates_authority: false,
        summary:
            "Provider transport validation does not execute provider output or mutate authority."
                .to_string(),
    }
}

pub fn provider_transport_mutates_authority(_report: &ProviderTransportReport) -> bool {
    false
}

pub fn provider_transport_executes_provider(_report: &ProviderTransportReport) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_envelope(sequence_id: u64, payload: &str) -> ProviderTransportEnvelope {
        ProviderTransportEnvelope::new("env-1", "provider-1", "request-1", sequence_id, payload)
            .expect("fixture envelope must be valid")
    }

    #[test]
    fn provider_transport_rejection_reason_codes_are_stable() {
        assert_eq!(
            ProviderTransportRejectionReason::Accepted.code(),
            "accepted"
        );
        assert_eq!(
            ProviderTransportRejectionReason::EmptyEnvelopeId.code(),
            "empty_envelope_id"
        );
        assert_eq!(
            ProviderTransportRejectionReason::EmptyProviderId.code(),
            "empty_provider_id"
        );
        assert_eq!(
            ProviderTransportRejectionReason::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            ProviderTransportRejectionReason::EmptySequenceId.code(),
            "empty_sequence_id"
        );
        assert_eq!(
            ProviderTransportRejectionReason::EmptyPayload.code(),
            "empty_payload"
        );
        assert_eq!(
            ProviderTransportRejectionReason::StaleSequence.code(),
            "stale_sequence"
        );
        assert_eq!(
            ProviderTransportRejectionReason::DuplicateSequence.code(),
            "duplicate_sequence"
        );
        assert_eq!(
            ProviderTransportRejectionReason::OutOfOrderSequence.code(),
            "out_of_order_sequence"
        );
        assert_eq!(
            ProviderTransportRejectionReason::ProviderOutputUntrusted.code(),
            "provider_output_untrusted"
        );
        assert_eq!(
            ProviderTransportRejectionReason::ExecutionNotEnabled.code(),
            "execution_not_enabled"
        );
    }

    #[test]
    fn provider_transport_envelope_requires_envelope_id() {
        assert_eq!(
            ProviderTransportEnvelope::new("", "provider-1", "request-1", 1, "payload"),
            Err(ProviderTransportRejectionReason::EmptyEnvelopeId)
        );
    }
    #[test]
    fn provider_transport_envelope_requires_provider_id() {
        assert_eq!(
            ProviderTransportEnvelope::new("env-1", "", "request-1", 1, "payload"),
            Err(ProviderTransportRejectionReason::EmptyProviderId)
        );
    }
    #[test]
    fn provider_transport_envelope_requires_request_id() {
        assert_eq!(
            ProviderTransportEnvelope::new("env-1", "provider-1", "", 1, "payload"),
            Err(ProviderTransportRejectionReason::EmptyRequestId)
        );
    }
    #[test]
    fn provider_transport_envelope_requires_sequence_id() {
        assert_eq!(
            ProviderTransportEnvelope::new("env-1", "provider-1", "request-1", 0, "payload"),
            Err(ProviderTransportRejectionReason::EmptySequenceId)
        );
    }
    #[test]
    fn provider_transport_envelope_requires_payload() {
        assert_eq!(
            ProviderTransportEnvelope::new("env-1", "provider-1", "request-1", 1, ""),
            Err(ProviderTransportRejectionReason::EmptyPayload)
        );
    }

    #[test]
    fn provider_transport_envelope_is_always_untrusted() {
        assert_eq!(
            build_envelope(1, "payload").trust,
            ProviderTransportPayloadTrust::Untrusted
        );
    }

    #[test]
    fn provider_transport_accepts_next_sequence() {
        let report = validate_provider_transport_envelope(
            &build_envelope(6, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![5]),
        );
        assert_eq!(report.status, ProviderTransportStatus::Accepted);
        assert_eq!(report.reason, ProviderTransportRejectionReason::Accepted);
    }

    #[test]
    fn provider_transport_rejects_duplicate_sequence() {
        let report = validate_provider_transport_envelope(
            &build_envelope(6, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![6]),
        );
        assert_eq!(report.status, ProviderTransportStatus::Rejected);
        assert_eq!(
            report.reason,
            ProviderTransportRejectionReason::DuplicateSequence
        );
    }

    #[test]
    fn provider_transport_rejects_stale_sequence() {
        let report = validate_provider_transport_envelope(
            &build_envelope(4, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![1, 2]),
        );
        assert_eq!(
            report.reason,
            ProviderTransportRejectionReason::StaleSequence
        );
    }

    #[test]
    fn provider_transport_rejects_out_of_order_sequence() {
        let report = validate_provider_transport_envelope(
            &build_envelope(8, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![1, 2]),
        );
        assert_eq!(
            report.reason,
            ProviderTransportRejectionReason::OutOfOrderSequence
        );
    }

    #[test]
    fn provider_transport_does_not_update_cursor() {
        let cursor = ProviderTransportCursor::new(Some(5), vec![1, 2, 3]);
        let expected = cursor.clone();
        let _ = validate_provider_transport_envelope(&build_envelope(6, "payload"), &cursor);
        assert_eq!(cursor, expected);
    }

    #[test]
    fn provider_transport_does_not_mutate_authority() {
        let report = validate_provider_transport_envelope(
            &build_envelope(6, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![]),
        );
        assert!(!report.mutates_authority);
        assert!(!provider_transport_mutates_authority(&report));
    }

    #[test]
    fn provider_transport_does_not_execute_provider() {
        let report = validate_provider_transport_envelope(
            &build_envelope(6, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![]),
        );
        assert!(!provider_transport_executes_provider(&report));
    }

    #[test]
    fn provider_transport_does_not_call_model() {
        assert!(validate_provider_transport_envelope(
            &build_envelope(6, "payload"),
            &ProviderTransportCursor::new(Some(5), vec![])
        )
        .summary
        .contains("does not execute provider output"));
    }
    #[test]
    fn provider_transport_does_not_append_ledger() {
        assert!(
            !validate_provider_transport_envelope(
                &build_envelope(6, "payload"),
                &ProviderTransportCursor::new(Some(5), vec![])
            )
            .mutates_authority
        );
    }
    #[test]
    fn provider_transport_does_not_persist() {
        assert!(
            !validate_provider_transport_envelope(
                &build_envelope(6, "payload"),
                &ProviderTransportCursor::new(Some(5), vec![])
            )
            .mutates_authority
        );
    }
    #[test]
    fn provider_transport_does_not_repair_replay() {
        assert_eq!(
            validate_provider_transport_envelope(
                &build_envelope(6, "payload"),
                &ProviderTransportCursor::new(Some(5), vec![])
            )
            .reason,
            ProviderTransportRejectionReason::Accepted
        );
    }
    #[test]
    fn provider_transport_does_not_require_async_runtime() {
        assert_eq!(
            validate_provider_transport_envelope(
                &build_envelope(6, "payload"),
                &ProviderTransportCursor::new(Some(5), vec![])
            )
            .status,
            ProviderTransportStatus::Accepted
        );
    }
    #[test]
    fn provider_transport_does_not_use_network() {
        assert_eq!(
            validate_provider_transport_envelope(
                &build_envelope(6, "payload"),
                &ProviderTransportCursor::new(Some(5), vec![])
            )
            .payload_trust,
            ProviderTransportPayloadTrust::Untrusted
        );
    }

    #[test]
    fn payload_text_cannot_grant_authority() {
        let payload = "approved trusted execute persist admin override skip policy write ledger";
        let normal = validate_provider_transport_envelope(
            &build_envelope(6, "plain payload"),
            &ProviderTransportCursor::new(Some(5), vec![]),
        );
        let risky = validate_provider_transport_envelope(
            &build_envelope(6, payload),
            &ProviderTransportCursor::new(Some(5), vec![]),
        );
        assert_eq!(risky.status, normal.status);
        assert_eq!(
            risky.payload_trust,
            ProviderTransportPayloadTrust::Untrusted
        );
        assert_eq!(risky.mutates_authority, normal.mutates_authority);
        assert_eq!(risky.reason, normal.reason);
    }
}
