//! Deterministic provider parity validation for untrusted adapter outputs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderParityStatus {
    ParityMaintained,
    ParityViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderParityError {
    ProtocolMismatch,
    LifecycleMutationAttempt,
    GovernanceInfluenceAttempt,
    PromotionEligibilityChange,
    ReplayDependencyDetected,
    AuthorityBoundaryViolation,
    DeterminismFailure,
}

pub fn validate_provider_parity(
    local_adapter_response: &str,
    cloud_adapter_response: &str,
) -> Result<ProviderParityStatus, ProviderParityError> {
    let local = parse_adapter_response(local_adapter_response)?;
    let cloud = parse_adapter_response(cloud_adapter_response)?;

    if local.protocol != cloud.protocol {
        return Err(ProviderParityError::ProtocolMismatch);
    }

    if local.lifecycle != cloud.lifecycle {
        return Err(ProviderParityError::LifecycleMutationAttempt);
    }

    if local.governance != cloud.governance {
        return Err(ProviderParityError::GovernanceInfluenceAttempt);
    }

    if local.promotion != cloud.promotion {
        return Err(ProviderParityError::PromotionEligibilityChange);
    }

    if local.replay != cloud.replay {
        return Err(ProviderParityError::ReplayDependencyDetected);
    }

    if local.error != cloud.error {
        return Err(ProviderParityError::AuthorityBoundaryViolation);
    }

    Ok(ProviderParityStatus::ParityMaintained)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdapterClassification<'a> {
    protocol: &'a str,
    lifecycle: &'a str,
    governance: &'a str,
    promotion: &'a str,
    replay: &'a str,
    error: &'a str,
}

fn parse_adapter_response(
    response: &str,
) -> Result<AdapterClassification<'_>, ProviderParityError> {
    let mut protocol: Option<&str> = None;
    let mut lifecycle: Option<&str> = None;
    let mut governance: Option<&str> = None;
    let mut promotion: Option<&str> = None;
    let mut replay: Option<&str> = None;
    let mut error: Option<&str> = None;

    for segment in response.split(';') {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.splitn(2, ':');
        let key = parts.next().unwrap_or("").trim();
        let value = parts
            .next()
            .ok_or(ProviderParityError::ProtocolMismatch)?
            .trim();

        if value.is_empty() {
            return Err(ProviderParityError::ProtocolMismatch);
        }

        match key {
            "protocol" => protocol = Some(value),
            "lifecycle" => lifecycle = Some(value),
            "governance" => governance = Some(value),
            "promotion" => promotion = Some(value),
            "replay" => replay = Some(value),
            "error" => error = Some(value),
            _ => return Err(ProviderParityError::ProtocolMismatch),
        }
    }

    Ok(AdapterClassification {
        protocol: protocol.ok_or(ProviderParityError::ProtocolMismatch)?,
        lifecycle: lifecycle.ok_or(ProviderParityError::ProtocolMismatch)?,
        governance: governance.ok_or(ProviderParityError::ProtocolMismatch)?,
        promotion: promotion.ok_or(ProviderParityError::ProtocolMismatch)?,
        replay: replay.ok_or(ProviderParityError::ProtocolMismatch)?,
        error: error.ok_or(ProviderParityError::ProtocolMismatch)?,
    })
}

#[cfg(test)]
mod tests {
    use super::{validate_provider_parity, ProviderParityError, ProviderParityStatus};

    const PARITY_LOCAL: &str = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:none";
    const PARITY_CLOUD: &str = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:none";

    #[test]
    fn local_and_cloud_protocol_shapes_are_equivalent() {
        let result = validate_provider_parity(PARITY_LOCAL, PARITY_CLOUD);

        assert_eq!(result, Ok(ProviderParityStatus::ParityMaintained));
    }

    #[test]
    fn protocol_mismatch_is_rejected() {
        let local = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:none";
        let cloud = "protocol:v2;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:none";

        let result = validate_provider_parity(local, cloud);

        assert_eq!(result, Err(ProviderParityError::ProtocolMismatch));
    }

    #[test]
    fn provider_source_does_not_change_lifecycle_classification() {
        let cloud = "protocol:v1;lifecycle:authoritative;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:none";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::LifecycleMutationAttempt));
    }

    #[test]
    fn provider_source_does_not_change_governance_preconditions() {
        let cloud = "protocol:v1;lifecycle:advisory;governance:approved;promotion:pending_evaluation;replay:deterministic;error:none";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::GovernanceInfluenceAttempt));
    }

    #[test]
    fn provider_source_does_not_change_promotion_eligibility() {
        let cloud = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:eligible;replay:deterministic;error:none";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::PromotionEligibilityChange));
    }

    #[test]
    fn provider_source_does_not_change_replay_classification() {
        let cloud = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:provider_specific;error:none";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::ReplayDependencyDetected));
    }

    #[test]
    fn provider_error_mapping_is_consistent() {
        let cloud = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:timeout";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::AuthorityBoundaryViolation));
    }

    #[test]
    fn authority_boundary_violation_is_rejected() {
        let cloud = "protocol:v1;lifecycle:advisory;governance:preconditioned;promotion:pending_evaluation;replay:deterministic;error:authoritative_override";

        let result = validate_provider_parity(PARITY_LOCAL, cloud);

        assert_eq!(result, Err(ProviderParityError::AuthorityBoundaryViolation));
    }

    #[test]
    fn validation_is_deterministic() {
        let first = validate_provider_parity(PARITY_LOCAL, PARITY_CLOUD);
        let second = validate_provider_parity(PARITY_LOCAL, PARITY_CLOUD);

        assert_eq!(first, second);
        assert_eq!(first, Ok(ProviderParityStatus::ParityMaintained));
    }
}
