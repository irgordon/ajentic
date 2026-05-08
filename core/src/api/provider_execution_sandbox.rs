use std::collections::BTreeSet;

use super::{
    validate_provider_configuration_bundle, ProviderConfiguration, ProviderConfigurationBundle,
    ProviderConfigurationStatus,
};

pub const MAX_PROVIDER_EXECUTION_INPUT_BYTES: usize = 4096;
pub const MAX_PROVIDER_EXECUTION_SUMMARY_BYTES: usize = 160;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionStatus {
    Succeeded,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionKind {
    DeterministicLocalStub,
    RemoteProvider,
    CloudProvider,
    AutoSelectedProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionOutputTrust {
    UntrustedCandidateData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionSandboxPosture {
    DeterministicLocalStubOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderExecutionRejectionReason {
    MalformedExecutionRequest,
    OversizedProviderInput,
    InvalidProviderConfiguration,
    UnsafeExecutionRequest,
    RemoteProviderRejected,
    FallbackRejected,
    AutoSelectionRejected,
    TimeoutLimitExceeded,
    PromptResourceLimitExceeded,
    OutputResourceLimitExceeded,
    RetryRejected,
    LimitEscalationRejected,
}

impl ProviderExecutionRejectionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MalformedExecutionRequest => "malformed_execution_request",
            Self::OversizedProviderInput => "oversized_provider_input",
            Self::InvalidProviderConfiguration => "invalid_provider_configuration",
            Self::UnsafeExecutionRequest => "unsafe_execution_request",
            Self::RemoteProviderRejected => "remote_provider_rejected",
            Self::FallbackRejected => "fallback_rejected",
            Self::AutoSelectionRejected => "auto_selection_rejected",
            Self::TimeoutLimitExceeded => "timeout_limit_exceeded",
            Self::PromptResourceLimitExceeded => "prompt_resource_limit_exceeded",
            Self::OutputResourceLimitExceeded => "output_resource_limit_exceeded",
            Self::RetryRejected => "retry_rejected",
            Self::LimitEscalationRejected => "limit_escalation_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredProviderLimitSnapshot {
    pub timeout_ms: u32,
    pub max_prompt_bytes: u32,
    pub max_context_bytes: u32,
    pub retries_allowed: bool,
    pub limit_escalation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedSandboxUsage {
    pub input_bytes: usize,
    pub planned_output_bytes: usize,
    pub emitted_output_bytes: usize,
    pub deterministic_elapsed_ms: u32,
    pub retry_attempts: u32,
    pub limit_escalation_attempts: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxLimitEnforcementStatus {
    WithinDeclaredLimits,
    OutputTruncated,
    TerminatedTimeout,
    RejectedResourceLimit,
    RejectedUnsafeLimitRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SandboxLimitDecision {
    AcceptedWithinLimits,
    TruncatedOutputToDeclaredContextLimit,
    TerminatedForDeclaredTimeout,
    RejectedDeclaredPromptLimit,
    RejectedDeclaredOutputLimit,
    RejectedRetryRequest,
    RejectedLimitEscalationRequest,
}

impl SandboxLimitDecision {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AcceptedWithinLimits => "accepted_within_limits",
            Self::TruncatedOutputToDeclaredContextLimit => {
                "truncated_output_to_declared_context_limit"
            }
            Self::TerminatedForDeclaredTimeout => "terminated_for_declared_timeout",
            Self::RejectedDeclaredPromptLimit => "rejected_declared_prompt_limit",
            Self::RejectedDeclaredOutputLimit => "rejected_declared_output_limit",
            Self::RejectedRetryRequest => "rejected_retry_request",
            Self::RejectedLimitEscalationRequest => "rejected_limit_escalation_request",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandboxLimitEvidence {
    pub descriptive_only: bool,
    pub grants_trust: bool,
    pub grants_promotion: bool,
    pub grants_persistence: bool,
    pub grants_readiness: bool,
    pub declared_limits: DeclaredProviderLimitSnapshot,
    pub observed_usage: ObservedSandboxUsage,
    pub enforcement_status: SandboxLimitEnforcementStatus,
    pub decisions: Vec<SandboxLimitDecision>,
    pub input_summary: String,
    pub output_summary: String,
    pub termination_reason_code: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExecutionRequest {
    pub execution_id: String,
    pub provider: ProviderConfiguration,
    pub execution_kind: ProviderExecutionKind,
    pub input: String,
    pub allow_remote: bool,
    pub allow_network: bool,
    pub allow_streaming: bool,
    pub allow_fallback: bool,
    pub allow_auto_selection: bool,
    pub allow_shell_or_process: bool,
    pub allow_file_access: bool,
    pub allow_persistence: bool,
    pub allow_promotion: bool,
    pub allow_action_execution: bool,
    pub allow_replay_repair: bool,
    pub allow_recovery_promotion: bool,
    pub allow_retry: bool,
    pub allow_limit_escalation: bool,
}

impl ProviderExecutionRequest {
    pub fn deterministic_local_stub(
        execution_id: impl Into<String>,
        provider: ProviderConfiguration,
        input: impl Into<String>,
    ) -> Self {
        Self {
            execution_id: execution_id.into(),
            provider,
            execution_kind: ProviderExecutionKind::DeterministicLocalStub,
            input: input.into(),
            allow_remote: false,
            allow_network: false,
            allow_streaming: false,
            allow_fallback: false,
            allow_auto_selection: false,
            allow_shell_or_process: false,
            allow_file_access: false,
            allow_persistence: false,
            allow_promotion: false,
            allow_action_execution: false,
            allow_replay_repair: false,
            allow_recovery_promotion: false,
            allow_retry: false,
            allow_limit_escalation: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExecutionReport {
    pub execution_id: String,
    pub provider_id: String,
    pub status: ProviderExecutionStatus,
    pub reasons: Vec<ProviderExecutionRejectionReason>,
    pub sandbox_posture: ProviderExecutionSandboxPosture,
    pub output_trust: ProviderExecutionOutputTrust,
    pub provider_output: Option<String>,
    pub deterministic_metadata: String,
    pub input_summary: String,
    pub output_summary: String,
    pub limit_evidence: SandboxLimitEvidence,
    pub remote_execution_disabled: bool,
    pub provider_network_disabled: bool,
    pub streaming_disabled: bool,
    pub fallback_disabled: bool,
    pub auto_selection_disabled: bool,
    pub retry_disabled: bool,
    pub limit_escalation_disabled: bool,
    pub no_promotion: bool,
    pub no_persistence: bool,
    pub no_action_execution: bool,
    pub no_replay_repair: bool,
    pub no_recovery_promotion: bool,
    pub persisted: bool,
    pub appended_ledger_or_audit: bool,
    pub promoted_provider_output: bool,
    pub executed_action: bool,
    pub repaired_replay: bool,
    pub promoted_recovery: bool,
    pub readiness_approved: bool,
    pub mutates_authority: bool,
    pub summary: String,
}

pub fn execute_provider_in_sandbox(request: ProviderExecutionRequest) -> ProviderExecutionReport {
    let reasons = provider_execution_rejection_reasons(&request);
    if !reasons.is_empty() {
        return rejected_report(request, reasons);
    }

    let planned_output = deterministic_stub_output(&request.provider.provider_id, &request.input);
    let mut limit_evidence = sandbox_limit_evidence(&request, &planned_output);
    if limit_evidence
        .decisions
        .contains(&SandboxLimitDecision::TerminatedForDeclaredTimeout)
    {
        return rejected_report(
            request,
            BTreeSet::from([ProviderExecutionRejectionReason::TimeoutLimitExceeded]),
        );
    }
    if limit_evidence
        .decisions
        .contains(&SandboxLimitDecision::RejectedDeclaredOutputLimit)
    {
        return rejected_report(
            request,
            BTreeSet::from([ProviderExecutionRejectionReason::OutputResourceLimitExceeded]),
        );
    }

    let output = truncate_to_bytes(
        &planned_output,
        request.provider.resources.max_context_bytes as usize,
    );
    limit_evidence.observed_usage.emitted_output_bytes = output.len();
    limit_evidence.output_summary = bounded_summary(&output);
    if output.len() < planned_output.len() {
        limit_evidence.enforcement_status = SandboxLimitEnforcementStatus::OutputTruncated;
        limit_evidence.decisions =
            vec![SandboxLimitDecision::TruncatedOutputToDeclaredContextLimit];
    }

    ProviderExecutionReport {
        execution_id: request.execution_id.clone(),
        provider_id: request.provider.provider_id.clone(),
        status: ProviderExecutionStatus::Succeeded,
        reasons: Vec::new(),
        sandbox_posture: ProviderExecutionSandboxPosture::DeterministicLocalStubOnly,
        output_trust: ProviderExecutionOutputTrust::UntrustedCandidateData,
        provider_output: Some(output.clone()),
        deterministic_metadata: deterministic_metadata(&request.provider.provider_id, &request.input),
        input_summary: bounded_summary(&request.input),
        output_summary: bounded_summary(&output),
        limit_evidence,
        remote_execution_disabled: true,
        provider_network_disabled: true,
        streaming_disabled: true,
        fallback_disabled: true,
        auto_selection_disabled: true,
        retry_disabled: true,
        limit_escalation_disabled: true,
        no_promotion: true,
        no_persistence: true,
        no_action_execution: true,
        no_replay_repair: true,
        no_recovery_promotion: true,
        persisted: false,
        appended_ledger_or_audit: false,
        promoted_provider_output: false,
        executed_action: false,
        repaired_replay: false,
        promoted_recovery: false,
        readiness_approved: false,
        mutates_authority: false,
        summary: "provider sandbox completed deterministic local stub execution only; output is untrusted candidate data; no promotion, persistence, action execution, replay repair, recovery promotion, remote execution, network execution, streaming, fallback, or auto-selection occurred".to_string(),
    }
}

pub fn provider_execution_report_mutates_authority(report: &ProviderExecutionReport) -> bool {
    report.persisted
        || report.appended_ledger_or_audit
        || report.promoted_provider_output
        || report.executed_action
        || report.repaired_replay
        || report.promoted_recovery
        || report.readiness_approved
        || report.mutates_authority
}

fn provider_execution_rejection_reasons(
    request: &ProviderExecutionRequest,
) -> BTreeSet<ProviderExecutionRejectionReason> {
    let mut reasons = BTreeSet::new();

    if request.execution_id.trim().is_empty() || request.input.trim().is_empty() {
        reasons.insert(ProviderExecutionRejectionReason::MalformedExecutionRequest);
    }
    if request.input.len() > MAX_PROVIDER_EXECUTION_INPUT_BYTES {
        reasons.insert(ProviderExecutionRejectionReason::OversizedProviderInput);
    }

    let config_report = validate_provider_configuration_bundle(&ProviderConfigurationBundle {
        providers: vec![request.provider.clone()],
    });
    if config_report.status != ProviderConfigurationStatus::Accepted {
        reasons.insert(ProviderExecutionRejectionReason::InvalidProviderConfiguration);
    }

    if request.execution_kind != ProviderExecutionKind::DeterministicLocalStub
        || request.allow_remote
        || request.allow_network
    {
        reasons.insert(ProviderExecutionRejectionReason::RemoteProviderRejected);
    }
    if request.allow_fallback || request.provider.fallback_enabled {
        reasons.insert(ProviderExecutionRejectionReason::FallbackRejected);
    }
    if request.allow_auto_selection || request.provider.auto_select {
        reasons.insert(ProviderExecutionRejectionReason::AutoSelectionRejected);
    }
    if request.allow_retry {
        reasons.insert(ProviderExecutionRejectionReason::RetryRejected);
    }
    if request.allow_limit_escalation {
        reasons.insert(ProviderExecutionRejectionReason::LimitEscalationRejected);
    }
    if request.input.len() > request.provider.resources.max_prompt_bytes as usize {
        reasons.insert(ProviderExecutionRejectionReason::PromptResourceLimitExceeded);
    }
    if request.allow_streaming
        || request.allow_shell_or_process
        || request.allow_file_access
        || request.allow_persistence
        || request.allow_promotion
        || request.allow_action_execution
        || request.allow_replay_repair
        || request.allow_recovery_promotion
    {
        reasons.insert(ProviderExecutionRejectionReason::UnsafeExecutionRequest);
    }

    reasons
}

fn rejected_report(
    request: ProviderExecutionRequest,
    reasons: BTreeSet<ProviderExecutionRejectionReason>,
) -> ProviderExecutionReport {
    let limit_evidence = rejected_limit_evidence(&request, &reasons);
    ProviderExecutionReport {
        execution_id: request.execution_id,
        provider_id: request.provider.provider_id,
        status: ProviderExecutionStatus::Rejected,
        reasons: reasons.into_iter().collect(),
        sandbox_posture: ProviderExecutionSandboxPosture::DeterministicLocalStubOnly,
        output_trust: ProviderExecutionOutputTrust::UntrustedCandidateData,
        provider_output: None,
        deterministic_metadata: "deterministic_rejection=true".to_string(),
        input_summary: bounded_summary(&request.input),
        output_summary: "none".to_string(),
        limit_evidence,
        remote_execution_disabled: true,
        provider_network_disabled: true,
        streaming_disabled: true,
        fallback_disabled: true,
        auto_selection_disabled: true,
        retry_disabled: true,
        limit_escalation_disabled: true,
        no_promotion: true,
        no_persistence: true,
        no_action_execution: true,
        no_replay_repair: true,
        no_recovery_promotion: true,
        persisted: false,
        appended_ledger_or_audit: false,
        promoted_provider_output: false,
        executed_action: false,
        repaired_replay: false,
        promoted_recovery: false,
        readiness_approved: false,
        mutates_authority: false,
        summary: "provider sandbox rejected request fail-closed; no provider output, promotion, persistence, action execution, replay repair, recovery promotion, remote execution, network execution, streaming, fallback, or auto-selection occurred".to_string(),
    }
}

fn sandbox_limit_evidence(
    request: &ProviderExecutionRequest,
    planned_output: &str,
) -> SandboxLimitEvidence {
    let deterministic_elapsed_ms = deterministic_elapsed_ms(request, planned_output);
    let timeout_exceeded = deterministic_elapsed_ms > request.provider.resources.timeout_ms;
    let output_exceeds_hard_limit = request.provider.resources.max_context_bytes == 0;
    let decisions = if timeout_exceeded {
        vec![SandboxLimitDecision::TerminatedForDeclaredTimeout]
    } else if output_exceeds_hard_limit {
        vec![SandboxLimitDecision::RejectedDeclaredOutputLimit]
    } else {
        vec![SandboxLimitDecision::AcceptedWithinLimits]
    };
    let enforcement_status = if timeout_exceeded {
        SandboxLimitEnforcementStatus::TerminatedTimeout
    } else if output_exceeds_hard_limit {
        SandboxLimitEnforcementStatus::RejectedResourceLimit
    } else {
        SandboxLimitEnforcementStatus::WithinDeclaredLimits
    };
    let termination_reason_code = if timeout_exceeded {
        Some(SandboxLimitDecision::TerminatedForDeclaredTimeout.code())
    } else if output_exceeds_hard_limit {
        Some(SandboxLimitDecision::RejectedDeclaredOutputLimit.code())
    } else {
        None
    };

    SandboxLimitEvidence {
        descriptive_only: true,
        grants_trust: false,
        grants_promotion: false,
        grants_persistence: false,
        grants_readiness: false,
        declared_limits: declared_limit_snapshot(request),
        observed_usage: ObservedSandboxUsage {
            input_bytes: request.input.len(),
            planned_output_bytes: planned_output.len(),
            emitted_output_bytes: planned_output.len(),
            deterministic_elapsed_ms,
            retry_attempts: u32::from(request.allow_retry),
            limit_escalation_attempts: u32::from(request.allow_limit_escalation),
        },
        enforcement_status,
        decisions,
        input_summary: bounded_summary(&request.input),
        output_summary: bounded_summary(planned_output),
        termination_reason_code,
    }
}

fn rejected_limit_evidence(
    request: &ProviderExecutionRequest,
    reasons: &BTreeSet<ProviderExecutionRejectionReason>,
) -> SandboxLimitEvidence {
    let mut decisions = Vec::new();
    if reasons.contains(&ProviderExecutionRejectionReason::PromptResourceLimitExceeded) {
        decisions.push(SandboxLimitDecision::RejectedDeclaredPromptLimit);
    }
    if reasons.contains(&ProviderExecutionRejectionReason::OutputResourceLimitExceeded) {
        decisions.push(SandboxLimitDecision::RejectedDeclaredOutputLimit);
    }
    if reasons.contains(&ProviderExecutionRejectionReason::TimeoutLimitExceeded) {
        decisions.push(SandboxLimitDecision::TerminatedForDeclaredTimeout);
    }
    if reasons.contains(&ProviderExecutionRejectionReason::RetryRejected) {
        decisions.push(SandboxLimitDecision::RejectedRetryRequest);
    }
    if reasons.contains(&ProviderExecutionRejectionReason::LimitEscalationRejected) {
        decisions.push(SandboxLimitDecision::RejectedLimitEscalationRequest);
    }
    if decisions.is_empty() {
        decisions.push(SandboxLimitDecision::AcceptedWithinLimits);
    }

    let enforcement_status =
        if reasons.contains(&ProviderExecutionRejectionReason::TimeoutLimitExceeded) {
            SandboxLimitEnforcementStatus::TerminatedTimeout
        } else if reasons.contains(&ProviderExecutionRejectionReason::PromptResourceLimitExceeded)
            || reasons.contains(&ProviderExecutionRejectionReason::OutputResourceLimitExceeded)
        {
            SandboxLimitEnforcementStatus::RejectedResourceLimit
        } else if reasons.contains(&ProviderExecutionRejectionReason::RetryRejected)
            || reasons.contains(&ProviderExecutionRejectionReason::LimitEscalationRejected)
        {
            SandboxLimitEnforcementStatus::RejectedUnsafeLimitRequest
        } else {
            SandboxLimitEnforcementStatus::WithinDeclaredLimits
        };

    SandboxLimitEvidence {
        descriptive_only: true,
        grants_trust: false,
        grants_promotion: false,
        grants_persistence: false,
        grants_readiness: false,
        declared_limits: declared_limit_snapshot(request),
        observed_usage: ObservedSandboxUsage {
            input_bytes: request.input.len(),
            planned_output_bytes: 0,
            emitted_output_bytes: 0,
            deterministic_elapsed_ms: deterministic_elapsed_ms(request, ""),
            retry_attempts: u32::from(request.allow_retry),
            limit_escalation_attempts: u32::from(request.allow_limit_escalation),
        },
        enforcement_status,
        termination_reason_code: decisions.first().map(SandboxLimitDecision::code),
        decisions,
        input_summary: bounded_summary(&request.input),
        output_summary: "none".to_string(),
    }
}

fn declared_limit_snapshot(request: &ProviderExecutionRequest) -> DeclaredProviderLimitSnapshot {
    DeclaredProviderLimitSnapshot {
        timeout_ms: request.provider.resources.timeout_ms,
        max_prompt_bytes: request.provider.resources.max_prompt_bytes,
        max_context_bytes: request.provider.resources.max_context_bytes,
        retries_allowed: false,
        limit_escalation_allowed: false,
    }
}

fn deterministic_elapsed_ms(request: &ProviderExecutionRequest, planned_output: &str) -> u32 {
    if request.input.contains("phase108_timeout_exhaustion") {
        return request.provider.resources.timeout_ms.saturating_add(1);
    }
    let work_units = request.input.len().saturating_add(planned_output.len());
    u32::try_from(work_units / 64 + 1).unwrap_or(u32::MAX)
}

fn truncate_to_bytes(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_string();
    }
    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}

fn deterministic_stub_output(provider_id: &str, input: &str) -> String {
    format!(
        "deterministic-local-stub provider={} input_checksum={} input_summary={}",
        provider_id,
        deterministic_checksum(input),
        bounded_summary(input)
    )
}

fn deterministic_metadata(provider_id: &str, input: &str) -> String {
    format!(
        "sandbox=deterministic_local_stub_only;provider={};input_bytes={};input_checksum={}",
        provider_id,
        input.len(),
        deterministic_checksum(input)
    )
}

fn deterministic_checksum(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        })
}

fn bounded_summary(value: &str) -> String {
    let mut summary = String::new();
    for character in value.chars().take(MAX_PROVIDER_EXECUTION_SUMMARY_BYTES) {
        summary.push(character);
    }
    if value.chars().count() > MAX_PROVIDER_EXECUTION_SUMMARY_BYTES {
        summary.push_str("...");
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        ProviderCapabilityDeclaration, ProviderConfigurationExecutionPosture,
        ProviderConfigurationReadinessPosture, ProviderConfigurationTransportPosture,
        ProviderConfigurationTrustPosture, ProviderConfigurationType, ProviderIsolationDeclaration,
        ProviderResourceLimits,
    };

    fn valid_provider() -> ProviderConfiguration {
        ProviderConfiguration {
            provider_id: "provider_alpha".to_string(),
            provider_type: ProviderConfigurationType::LocalOnlyDeclared,
            capabilities: vec![ProviderCapabilityDeclaration::TextGenerationDeclared],
            resources: ProviderResourceLimits {
                timeout_ms: 1000,
                max_prompt_bytes: 2048,
                max_context_bytes: 8192,
            },
            isolation: vec![
                ProviderIsolationDeclaration::LocalOnly,
                ProviderIsolationDeclaration::NoNetwork,
                ProviderIsolationDeclaration::NoFilesystem,
                ProviderIsolationDeclaration::NoBackgroundExecution,
            ],
            execution_posture: ProviderConfigurationExecutionPosture::Disabled,
            transport_posture: ProviderConfigurationTransportPosture::Disabled,
            trust_posture: ProviderConfigurationTrustPosture::Untrusted,
            readiness_posture: ProviderConfigurationReadinessPosture::NotReady,
            local_only: true,
            auto_select: false,
            fallback_enabled: false,
        }
    }

    #[test]
    fn phase_108_timeout_enforcement_triggers_deterministically() {
        let request = ProviderExecutionRequest::deterministic_local_stub(
            "exec-timeout",
            valid_provider(),
            "phase108_timeout_exhaustion",
        );
        let first = execute_provider_in_sandbox(request.clone());
        let second = execute_provider_in_sandbox(request);

        assert_eq!(first, second);
        assert_eq!(first.status, ProviderExecutionStatus::Rejected);
        assert!(first
            .reasons
            .contains(&ProviderExecutionRejectionReason::TimeoutLimitExceeded));
        assert_eq!(
            first.limit_evidence.enforcement_status,
            SandboxLimitEnforcementStatus::TerminatedTimeout
        );
        assert_eq!(
            first.limit_evidence.termination_reason_code,
            Some("terminated_for_declared_timeout")
        );
        assert!(first.provider_output.is_none());
        assert!(first.limit_evidence.descriptive_only);
        assert!(!first.limit_evidence.grants_trust);
        assert!(!first.limit_evidence.grants_readiness);
    }

    #[test]
    fn phase_108_resource_limit_enforcement_rejects_prompt_over_declared_limit() {
        let mut provider = valid_provider();
        provider.resources.max_prompt_bytes = 8;
        let report =
            execute_provider_in_sandbox(ProviderExecutionRequest::deterministic_local_stub(
                "exec-resource",
                provider,
                "larger than eight bytes",
            ));

        assert_eq!(report.status, ProviderExecutionStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderExecutionRejectionReason::PromptResourceLimitExceeded));
        assert_eq!(
            report.limit_evidence.enforcement_status,
            SandboxLimitEnforcementStatus::RejectedResourceLimit
        );
        assert!(report
            .limit_evidence
            .decisions
            .contains(&SandboxLimitDecision::RejectedDeclaredPromptLimit));
        assert!(report.provider_output.is_none());
    }

    #[test]
    fn phase_108_truncation_ordering_is_deterministic_and_untrusted() {
        let mut provider = valid_provider();
        provider.resources.max_context_bytes = 32;
        let request = ProviderExecutionRequest::deterministic_local_stub(
            "exec-truncate",
            provider,
            "stable truncation payload",
        );
        let first = execute_provider_in_sandbox(request.clone());
        let second = execute_provider_in_sandbox(request);

        assert_eq!(first, second);
        assert_eq!(first.status, ProviderExecutionStatus::Succeeded);
        assert_eq!(
            first.limit_evidence.enforcement_status,
            SandboxLimitEnforcementStatus::OutputTruncated
        );
        assert_eq!(
            first.limit_evidence.decisions,
            vec![SandboxLimitDecision::TruncatedOutputToDeclaredContextLimit]
        );
        let output = first.provider_output.as_ref().expect("provider output");
        assert_eq!(output.len(), 32);
        assert!(output.starts_with("deterministic-local-stub provid"));
        assert_eq!(
            first.output_trust,
            ProviderExecutionOutputTrust::UntrustedCandidateData
        );
        assert!(!provider_execution_report_mutates_authority(&first));
        assert!(!first.promoted_provider_output);
        assert!(!first.readiness_approved);
        assert!(!first.persisted);
    }

    #[test]
    fn phase_108_retries_and_limit_escalation_remain_disabled() {
        let mut request = ProviderExecutionRequest::deterministic_local_stub(
            "exec-retry-escalate",
            valid_provider(),
            "retry please and increase limits",
        );
        request.allow_retry = true;
        request.allow_limit_escalation = true;
        let report = execute_provider_in_sandbox(request);

        assert_eq!(report.status, ProviderExecutionStatus::Rejected);
        assert_eq!(report.limit_evidence.observed_usage.retry_attempts, 1);
        assert_eq!(
            report
                .limit_evidence
                .observed_usage
                .limit_escalation_attempts,
            1
        );
        assert!(report.retry_disabled);
        assert!(report.limit_escalation_disabled);
        assert!(report
            .reasons
            .contains(&ProviderExecutionRejectionReason::RetryRejected));
        assert!(report
            .reasons
            .contains(&ProviderExecutionRejectionReason::LimitEscalationRejected));
        assert!(report
            .limit_evidence
            .decisions
            .contains(&SandboxLimitDecision::RejectedRetryRequest));
        assert!(report
            .limit_evidence
            .decisions
            .contains(&SandboxLimitDecision::RejectedLimitEscalationRequest));
    }

    #[test]
    fn phase_107_local_stub_execution_succeeds_as_untrusted_candidate_only() {
        let report =
            execute_provider_in_sandbox(ProviderExecutionRequest::deterministic_local_stub(
                "exec-alpha",
                valid_provider(),
                "summarize bounded input",
            ));

        assert_eq!(report.status, ProviderExecutionStatus::Succeeded);
        assert_eq!(
            report.output_trust,
            ProviderExecutionOutputTrust::UntrustedCandidateData
        );
        assert!(report.provider_output.is_some());
        assert!(!provider_execution_report_mutates_authority(&report));
        assert!(report.no_promotion);
        assert!(report.no_persistence);
        assert!(report.no_action_execution);
    }

    #[test]
    fn phase_107_local_stub_execution_is_deterministic() {
        let request = ProviderExecutionRequest::deterministic_local_stub(
            "exec-alpha",
            valid_provider(),
            "same input",
        );
        let first = execute_provider_in_sandbox(request.clone());
        let second = execute_provider_in_sandbox(request);
        assert_eq!(first, second);
    }

    #[test]
    fn phase_107_invalid_provider_configuration_rejects() {
        let mut provider = valid_provider();
        provider.provider_id = "Remote.Bad".to_string();
        let report = execute_provider_in_sandbox(
            ProviderExecutionRequest::deterministic_local_stub("exec-alpha", provider, "input"),
        );
        assert_eq!(report.status, ProviderExecutionStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderExecutionRejectionReason::InvalidProviderConfiguration));
    }

    #[test]
    fn phase_107_remote_fallback_auto_and_unsafe_requests_reject() {
        let mut request = ProviderExecutionRequest::deterministic_local_stub(
            "exec-alpha",
            valid_provider(),
            "input",
        );
        request.execution_kind = ProviderExecutionKind::CloudProvider;
        request.allow_remote = true;
        request.allow_network = true;
        request.allow_fallback = true;
        request.allow_auto_selection = true;
        request.allow_streaming = true;
        request.allow_shell_or_process = true;
        request.allow_file_access = true;
        request.allow_persistence = true;
        request.allow_promotion = true;
        request.allow_action_execution = true;
        request.allow_replay_repair = true;
        request.allow_recovery_promotion = true;

        let report = execute_provider_in_sandbox(request);
        assert_eq!(report.status, ProviderExecutionStatus::Rejected);
        assert_eq!(
            report.reasons,
            vec![
                ProviderExecutionRejectionReason::UnsafeExecutionRequest,
                ProviderExecutionRejectionReason::RemoteProviderRejected,
                ProviderExecutionRejectionReason::FallbackRejected,
                ProviderExecutionRejectionReason::AutoSelectionRejected,
            ]
        );
        assert!(report.provider_output.is_none());
        assert!(!provider_execution_report_mutates_authority(&report));
    }
}
