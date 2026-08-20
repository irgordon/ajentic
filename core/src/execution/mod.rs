pub mod provider_failure;
pub use provider_failure::*;
pub mod provider_execution;
pub use provider_execution::*;
pub mod run_budget;
pub use run_budget::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Local,
    Cloud,
    Ide,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderEndpointKind {
    LocalProcess,
    LocalHttp,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalProviderCapability {
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_json_mode: bool,
    pub supports_system_prompt: bool,
    pub supports_temperature: bool,
}

impl LocalProviderCapability {
    pub fn none() -> Self {
        Self {
            supports_streaming: false,
            supports_tools: false,
            supports_json_mode: false,
            supports_system_prompt: false,
            supports_temperature: false,
        }
    }

    pub fn preview_local_model() -> Self {
        Self {
            supports_streaming: false,
            supports_tools: false,
            supports_json_mode: false,
            supports_system_prompt: true,
            supports_temperature: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderCapabilityAuthority {
    DescriptiveOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterConfigError {
    EmptyAdapterId,
    EmptyModelLabel,
    InvalidProviderKind,
    InvalidEndpointKind,
    InvalidMaxOutputTokens,
    InvalidTimeoutMillis,
    SecretMarkerDetected,
    CapabilityAuthorityInvalid,
}

impl LocalProviderAdapterConfigError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyAdapterId => "empty_adapter_id",
            Self::EmptyModelLabel => "empty_model_label",
            Self::InvalidProviderKind => "invalid_provider_kind",
            Self::InvalidEndpointKind => "invalid_endpoint_kind",
            Self::InvalidMaxOutputTokens => "invalid_max_output_tokens",
            Self::InvalidTimeoutMillis => "invalid_timeout_millis",
            Self::SecretMarkerDetected => "secret_marker_detected",
            Self::CapabilityAuthorityInvalid => "capability_authority_invalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterConfig {
    pub adapter_id: String,
    pub provider_kind: ProviderKind,
    pub endpoint_kind: LocalProviderEndpointKind,
    pub model_label: String,
    pub max_output_tokens: u32,
    pub timeout_millis: u64,
    pub capabilities: LocalProviderCapability,
    pub capability_authority: LocalProviderCapabilityAuthority,
}

impl LocalProviderAdapterConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adapter_id: impl Into<String>,
        provider_kind: ProviderKind,
        endpoint_kind: LocalProviderEndpointKind,
        model_label: impl Into<String>,
        max_output_tokens: u32,
        timeout_millis: u64,
        capabilities: LocalProviderCapability,
        capability_authority: LocalProviderCapabilityAuthority,
    ) -> Result<Self, LocalProviderAdapterConfigError> {
        let adapter_id = adapter_id.into();
        if adapter_id.is_empty() {
            return Err(LocalProviderAdapterConfigError::EmptyAdapterId);
        }
        let model_label = model_label.into();
        if model_label.is_empty() {
            return Err(LocalProviderAdapterConfigError::EmptyModelLabel);
        }
        if provider_kind == ProviderKind::Unknown {
            return Err(LocalProviderAdapterConfigError::InvalidProviderKind);
        }
        if endpoint_kind == LocalProviderEndpointKind::Unknown {
            return Err(LocalProviderAdapterConfigError::InvalidEndpointKind);
        }
        if max_output_tokens == 0 {
            return Err(LocalProviderAdapterConfigError::InvalidMaxOutputTokens);
        }
        if timeout_millis == 0 {
            return Err(LocalProviderAdapterConfigError::InvalidTimeoutMillis);
        }
        if has_secret_marker(&adapter_id) || has_secret_marker(&model_label) {
            return Err(LocalProviderAdapterConfigError::SecretMarkerDetected);
        }
        if capability_authority != LocalProviderCapabilityAuthority::DescriptiveOnly {
            return Err(LocalProviderAdapterConfigError::CapabilityAuthorityInvalid);
        }

        Ok(Self {
            adapter_id,
            provider_kind,
            endpoint_kind,
            model_label,
            max_output_tokens,
            timeout_millis,
            capabilities,
            capability_authority,
        })
    }
}

pub fn local_provider_config_allows_authority(_config: &LocalProviderAdapterConfig) -> bool {
    false
}

pub fn local_provider_config_can_invoke_real_provider(
    _config: &LocalProviderAdapterConfig,
) -> bool {
    false
}

pub fn local_provider_config_has_secret_marker(config: &LocalProviderAdapterConfig) -> bool {
    has_secret_marker(&config.adapter_id) || has_secret_marker(&config.model_label)
}

fn has_secret_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    normalized.contains("api_key")
        || normalized.contains("apikey")
        || normalized.contains("secret")
        || normalized.contains("token")
        || normalized.contains("bearer")
        || normalized.contains("password")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderOutputStatus {
    Received,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderOutputTrust {
    Untrusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRequest {
    pub id: String,
    pub provider_kind: ProviderKind,
    pub prompt_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderOutput {
    pub id: String,
    pub request_id: String,
    pub provider_kind: ProviderKind,
    pub content: String,
    pub status: ProviderOutputStatus,
    pub trust: ProviderOutputTrust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderBoundaryError {
    EmptyRequestId,
    EmptyPromptSummary,
    EmptyOutputId,
    EmptyOutputRequestId,
    EmptyOutputContent,
    OutputMarkedTrusted,
}

impl ProviderBoundaryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::EmptyOutputId => "empty_output_id",
            Self::EmptyOutputRequestId => "empty_output_request_id",
            Self::EmptyOutputContent => "empty_output_content",
            Self::OutputMarkedTrusted => "output_marked_trusted",
        }
    }
}

impl ProviderRequest {
    pub fn new(
        id: impl Into<String>,
        provider_kind: ProviderKind,
        prompt_summary: impl Into<String>,
    ) -> Result<Self, ProviderBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ProviderBoundaryError::EmptyRequestId);
        }

        let prompt_summary = prompt_summary.into();
        if prompt_summary.is_empty() {
            return Err(ProviderBoundaryError::EmptyPromptSummary);
        }

        Ok(Self {
            id,
            provider_kind,
            prompt_summary,
        })
    }
}

impl ProviderOutput {
    pub fn new_untrusted(
        id: impl Into<String>,
        request_id: impl Into<String>,
        provider_kind: ProviderKind,
        content: impl Into<String>,
        status: ProviderOutputStatus,
    ) -> Result<Self, ProviderBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ProviderBoundaryError::EmptyOutputId);
        }

        let request_id = request_id.into();
        if request_id.is_empty() {
            return Err(ProviderBoundaryError::EmptyOutputRequestId);
        }

        let content = content.into();
        if content.is_empty() {
            return Err(ProviderBoundaryError::EmptyOutputContent);
        }

        Ok(Self {
            id,
            request_id,
            provider_kind,
            content,
            status,
            trust: ProviderOutputTrust::Untrusted,
        })
    }
}

pub fn provider_output_is_authoritative(_output: &ProviderOutput) -> bool {
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderAdapterError {
    EmptyAdapterId,
    EmptyRequestId,
    EmptyPromptSummary,
    OutputConstructionFailed,
    RealProviderNotImplemented,
}

impl ProviderAdapterError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyAdapterId => "empty_adapter_id",
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::OutputConstructionFailed => "output_construction_failed",
            Self::RealProviderNotImplemented => "real_provider_not_implemented",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAdapterInvocation {
    pub adapter_id: String,
    pub request: ProviderRequest,
}

impl ProviderAdapterInvocation {
    pub fn new(
        adapter_id: impl Into<String>,
        request: ProviderRequest,
    ) -> Result<Self, ProviderAdapterError> {
        let adapter_id = adapter_id.into();
        if adapter_id.is_empty() {
            return Err(ProviderAdapterError::EmptyAdapterId);
        }
        if request.id.is_empty() {
            return Err(ProviderAdapterError::EmptyRequestId);
        }
        if request.prompt_summary.is_empty() {
            return Err(ProviderAdapterError::EmptyPromptSummary);
        }
        Ok(Self {
            adapter_id,
            request,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAdapterResult {
    pub adapter_id: String,
    pub output: ProviderOutput,
}

pub trait ProviderAdapter {
    fn adapter_id(&self) -> &str;
    fn invoke(
        &self,
        invocation: &ProviderAdapterInvocation,
    ) -> Result<ProviderAdapterResult, ProviderAdapterError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicStubProvider {
    adapter_id: String,
    response_prefix: String,
}

impl DeterministicStubProvider {
    pub fn new(
        adapter_id: impl Into<String>,
        response_prefix: impl Into<String>,
    ) -> Result<Self, ProviderAdapterError> {
        let adapter_id = adapter_id.into();
        if adapter_id.is_empty() {
            return Err(ProviderAdapterError::EmptyAdapterId);
        }
        let response_prefix = response_prefix.into();
        if response_prefix.is_empty() {
            return Err(ProviderAdapterError::EmptyPromptSummary);
        }
        Ok(Self {
            adapter_id,
            response_prefix,
        })
    }
}

impl ProviderAdapter for DeterministicStubProvider {
    fn adapter_id(&self) -> &str {
        &self.adapter_id
    }

    fn invoke(
        &self,
        invocation: &ProviderAdapterInvocation,
    ) -> Result<ProviderAdapterResult, ProviderAdapterError> {
        let output_id = format!("stub-output:{}:{}", self.adapter_id, invocation.request.id);
        let content = format!(
            "{} {}",
            self.response_prefix, invocation.request.prompt_summary
        );
        let output = ProviderOutput::new_untrusted(
            output_id,
            invocation.request.id.clone(),
            invocation.request.provider_kind,
            content,
            ProviderOutputStatus::Received,
        )
        .map_err(|_| ProviderAdapterError::OutputConstructionFailed)?;

        Ok(ProviderAdapterResult {
            adapter_id: self.adapter_id.clone(),
            output,
        })
    }
}

pub fn provider_adapter_result_is_authoritative(result: &ProviderAdapterResult) -> bool {
    provider_output_is_authoritative(&result.output)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub id: String,
    pub status: ExecutionStatus,
}

impl ExecutionPlan {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: ExecutionStatus::NotStarted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecision {
    Allowed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecisionReason {
    ReadyForExecution,
    LifecycleNotPassed,
    ReceiptBindingMismatch,
    PolicyNotAllowed,
    ValidationNotPassed,
    ReplayNotReady,
}

impl ExecutionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForExecution => "ready_for_execution",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::ReceiptBindingMismatch => "receipt_binding_mismatch",
            Self::PolicyNotAllowed => "policy_not_allowed",
            Self::ValidationNotPassed => "validation_not_passed",
            Self::ReplayNotReady => "replay_not_ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionDecisionReport {
    pub decision: ExecutionDecision,
    pub reason: ExecutionDecisionReason,
}

impl ExecutionDecisionReport {
    pub fn allowed() -> Self {
        Self {
            decision: ExecutionDecision::Allowed,
            reason: ExecutionDecisionReason::ReadyForExecution,
        }
    }

    pub fn blocked(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Blocked,
            reason,
        }
    }

    pub fn rejected(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Rejected,
            reason,
        }
    }
}

pub fn decide_execution(
    lifecycle: crate::state::LifecycleState,
    policy: &crate::policy::PolicyReceipt,
    validation: &crate::validation::ValidationReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> ExecutionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed);
    }

    if !execution_receipts_match(policy, validation, replay) {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReceiptBindingMismatch);
    }

    if validation.status() != crate::validation::ValidationStatus::Pass {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed);
    }

    if policy.decision() != crate::policy::PolicyDecision::Allowed {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed);
    }

    if !replay.ready() {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady);
    }

    ExecutionDecisionReport::allowed()
}

fn execution_receipts_match(
    policy: &crate::policy::PolicyReceipt,
    validation: &crate::validation::ValidationReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> bool {
    policy.binding() == validation.binding()
        && replay.binding() == validation.binding()
        && policy.validation_receipt_digest() == validation.digest()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionDecision {
    Allowed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionDecisionReason {
    ReadyForTier1Promotion,
    LifecycleNotPassed,
    ExecutionNotAllowed,
}

impl PromotionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForTier1Promotion => "ready_for_tier_1_promotion",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::ExecutionNotAllowed => "execution_not_allowed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionDecisionReport {
    pub decision: PromotionDecision,
    pub reason: PromotionDecisionReason,
}

impl PromotionDecisionReport {
    fn allowed() -> Self {
        Self {
            decision: PromotionDecision::Allowed,
            reason: PromotionDecisionReason::ReadyForTier1Promotion,
        }
    }

    fn blocked(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Blocked,
            reason,
        }
    }

    fn rejected(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Rejected,
            reason,
        }
    }
}

fn decide_promotion(
    lifecycle: crate::state::LifecycleState,
    execution: &ExecutionDecisionReport,
) -> PromotionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed);
    }

    if execution.decision != ExecutionDecision::Allowed {
        return PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed);
    }

    PromotionDecisionReport::allowed()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionRecordError {
    PromotionNotAllowed,
    LedgerEventInvalid,
}

impl PromotionRecordError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PromotionNotAllowed => "promotion_not_allowed",
            Self::LedgerEventInvalid => "ledger_event_invalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionRecord {
    event: crate::ledger::LedgerEvent,
}

#[cfg(test)]
fn build_promotion_record(
    event_id: impl Into<String>,
    revision: u64,
    actor: crate::ledger::LedgerActor,
    evidence_refs: Vec<String>,
    payload_summary: impl Into<String>,
    promotion: &PromotionDecisionReport,
) -> Result<PromotionRecord, PromotionRecordError> {
    if promotion.decision != PromotionDecision::Allowed {
        return Err(PromotionRecordError::PromotionNotAllowed);
    }

    let payload = crate::ledger::LedgerPayload::with_lifecycle_transition(
        payload_summary,
        crate::state::LifecycleState::PromotedTier1,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;

    let event = crate::ledger::LedgerEvent::new(
        event_id,
        revision,
        crate::ledger::LedgerEventType::StateTransition,
        actor,
        evidence_refs,
        payload,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;

    Ok(PromotionRecord { event })
}

pub fn build_authorized_promotion_record(
    authorization: &crate::state::PromotionAuthorization,
    manifest: &crate::authority::EvidenceManifest,
    actor: crate::ledger::LedgerActor,
    ledger: &crate::ledger::Ledger,
) -> Result<PromotionRecord, PromotionRecordError> {
    manifest
        .verify_binding(authorization.binding())
        .map_err(|_| PromotionRecordError::PromotionNotAllowed)?;
    let evidence_refs = authorized_evidence_refs(authorization, manifest);
    build_authorized_record(authorization, actor, ledger, evidence_refs)
}

fn authorized_evidence_refs(
    authorization: &crate::state::PromotionAuthorization,
    manifest: &crate::authority::EvidenceManifest,
) -> Vec<String> {
    let mut references = manifest.reference_ids();
    references.push(format!(
        "promotion_authorization:{}",
        authorization.digest().as_str()
    ));
    references
}

fn build_authorized_record(
    authorization: &crate::state::PromotionAuthorization,
    actor: crate::ledger::LedgerActor,
    ledger: &crate::ledger::Ledger,
    evidence_refs: Vec<String>,
) -> Result<PromotionRecord, PromotionRecordError> {
    let run_id = authorization.binding().run_id();
    let revision = ledger.last_revision().unwrap_or(0) + 1;
    let payload = crate::ledger::LedgerPayload::with_lifecycle_transition(
        format!("controlled_run:{run_id}:promotion"),
        crate::state::LifecycleState::PromotedTier1,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;
    let event = crate::ledger::LedgerEvent::new(
        format!("promotion:{run_id}"),
        revision,
        crate::ledger::LedgerEventType::StateTransition,
        actor,
        evidence_refs,
        payload,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;
    let previous_hash = ledger
        .last_event_hash()
        .cloned()
        .unwrap_or_else(|| crate::integrity::Digest::of_text("AJENTIC_LEDGER_GENESIS"));
    let parents = ledger
        .events()
        .last()
        .map(|event| vec![event.id.clone()])
        .unwrap_or_default();
    let seal = promotion_ledger_seal(authorization);
    let event = crate::ledger::LedgerEvent::new_bound(event, &seal, previous_hash, parents)
        .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;
    Ok(PromotionRecord { event })
}

fn promotion_ledger_seal(
    authorization: &crate::state::PromotionAuthorization,
) -> crate::ledger::LedgerSeal {
    crate::ledger::LedgerSeal {
        binding: authorization.binding().clone(),
        actor_authorization_ref: authorization.digest().as_str().into(),
        validation_receipt_ref: authorization.validation_receipt_digest().as_str().into(),
        policy_receipt_ref: authorization.policy_receipt_digest().as_str().into(),
        schema_version: "v1.0.0".into(),
        verifier_version: authorization.binding().verifier_version().into(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionAppendError {
    LedgerAppendFailed,
}

impl PromotionAppendError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LedgerAppendFailed => "ledger_append_failed",
        }
    }
}

pub fn append_promotion_record(
    ledger: &crate::ledger::Ledger,
    record: PromotionRecord,
) -> Result<crate::ledger::Ledger, PromotionAppendError> {
    ledger
        .append(record.event)
        .map_err(|_| PromotionAppendError::LedgerAppendFailed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionReplayVerificationStatus {
    Verified,
    NotVerified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionReplayVerificationReason {
    PromotionReplayVerified,
    LedgerNotReplayReady,
    LedgerIntegrityMismatch,
    PromotionAuthorizationMismatch,
    ReconstructionFailed,
    FinalStateNotPromotedTier1,
}

impl PromotionReplayVerificationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PromotionReplayVerified => "promotion_replay_verified",
            Self::LedgerNotReplayReady => "ledger_not_replay_ready",
            Self::LedgerIntegrityMismatch => "ledger_integrity_mismatch",
            Self::PromotionAuthorizationMismatch => "promotion_authorization_mismatch",
            Self::ReconstructionFailed => "reconstruction_failed",
            Self::FinalStateNotPromotedTier1 => "final_state_not_promoted_tier_1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionReplayVerificationReport {
    pub status: PromotionReplayVerificationStatus,
    pub reason: PromotionReplayVerificationReason,
    pub final_revision: u64,
    pub events_seen: u64,
    pub state_transitions_applied: u64,
}

impl PromotionReplayVerificationReport {
    pub fn verified(final_revision: u64, events_seen: u64, state_transitions_applied: u64) -> Self {
        Self {
            status: PromotionReplayVerificationStatus::Verified,
            reason: PromotionReplayVerificationReason::PromotionReplayVerified,
            final_revision,
            events_seen,
            state_transitions_applied,
        }
    }

    pub fn not_verified(reason: PromotionReplayVerificationReason) -> Self {
        Self {
            status: PromotionReplayVerificationStatus::NotVerified,
            reason,
            final_revision: 0,
            events_seen: 0,
            state_transitions_applied: 0,
        }
    }
}

pub fn verify_authorized_promotion_replay(
    ledger: &crate::ledger::Ledger,
    authorization: &crate::state::PromotionAuthorization,
    manifest: &crate::authority::EvidenceManifest,
    evidence: PromotionReplayEvidence<'_>,
) -> PromotionReplayVerificationReport {
    if ledger
        .verify_integrity(authorization.binding(), manifest)
        .is_err()
    {
        return PromotionReplayVerificationReport::not_verified(
            PromotionReplayVerificationReason::LedgerIntegrityMismatch,
        );
    }
    if !promotion_event_matches_authorization(ledger, authorization) {
        return PromotionReplayVerificationReport::not_verified(
            PromotionReplayVerificationReason::PromotionAuthorizationMismatch,
        );
    }
    if !promotion_decision_rederives(authorization, evidence) {
        return PromotionReplayVerificationReport::not_verified(
            PromotionReplayVerificationReason::PromotionAuthorizationMismatch,
        );
    }
    verify_promotion_replay(ledger)
}

fn promotion_decision_rederives(
    authorization: &crate::state::PromotionAuthorization,
    evidence: PromotionReplayEvidence<'_>,
) -> bool {
    let binding = authorization.binding().clone();
    let validation =
        crate::validation::evaluate_validation(binding.clone(), evidence.evaluations.validation());
    let policy =
        crate::policy::evaluate_policy(binding.clone(), evidence.evaluations.policy(), &validation);
    let rederived =
        crate::state::authorize_promotion(binding, &validation, &policy, evidence.replay);
    receipt_derivation_matches(evidence, &validation, &policy, rederived.as_ref().ok())
}

fn receipt_derivation_matches(
    evidence: PromotionReplayEvidence<'_>,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    authorization: Option<&crate::state::PromotionAuthorization>,
) -> bool {
    validation.digest() == evidence.validation.digest()
        && policy.digest() == evidence.policy.digest()
        && authorization.map(|item| item.digest()) == Some(evidence.authorization_digest)
}

fn promotion_event_matches_authorization(
    ledger: &crate::ledger::Ledger,
    authorization: &crate::state::PromotionAuthorization,
) -> bool {
    ledger
        .events()
        .last()
        .and_then(|event| event.integrity.as_ref())
        .map(|integrity| {
            integrity.actor_authorization_ref == authorization.digest().as_str()
                && integrity.validation_receipt_ref
                    == authorization.validation_receipt_digest().as_str()
                && integrity.policy_receipt_ref == authorization.policy_receipt_digest().as_str()
        })
        .unwrap_or(false)
}

fn verify_promotion_replay(ledger: &crate::ledger::Ledger) -> PromotionReplayVerificationReport {
    let events = ledger.events();

    if crate::replay::classify_replay_readiness(events).is_err() {
        return PromotionReplayVerificationReport::not_verified(
            PromotionReplayVerificationReason::LedgerNotReplayReady,
        );
    }

    let reconstruction = match crate::replay::reconstruct_harness_state(events) {
        Ok(reconstruction) => reconstruction,
        Err(_) => {
            return PromotionReplayVerificationReport::not_verified(
                PromotionReplayVerificationReason::ReconstructionFailed,
            );
        }
    };

    if reconstruction.final_state.lifecycle != crate::state::LifecycleState::PromotedTier1 {
        return PromotionReplayVerificationReport::not_verified(
            PromotionReplayVerificationReason::FinalStateNotPromotedTier1,
        );
    }

    PromotionReplayVerificationReport::verified(
        reconstruction.final_state.revision,
        reconstruction.events_seen,
        reconstruction.state_transitions_applied,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledRunStatus {
    Accepted,
    Rejected,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledRunReason {
    RunAccepted,
    ProviderOutputInvalid,
    LifecycleNotPassed,
    PolicyNotAllowed,
    ValidationNotPassed,
    ExecutionNotAllowed,
    PromotionNotAllowed,
    PromotionRecordInvalid,
    LedgerAppendFailed,
    PromotionReplayNotVerified,
}

impl ControlledRunReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RunAccepted => "run_accepted",
            Self::ProviderOutputInvalid => "provider_output_invalid",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::PolicyNotAllowed => "policy_not_allowed",
            Self::ValidationNotPassed => "validation_not_passed",
            Self::ExecutionNotAllowed => "execution_not_allowed",
            Self::PromotionNotAllowed => "promotion_not_allowed",
            Self::PromotionRecordInvalid => "promotion_record_invalid",
            Self::LedgerAppendFailed => "ledger_append_failed",
            Self::PromotionReplayNotVerified => "promotion_replay_not_verified",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityEvaluationEvidence {
    validation: crate::validation::ValidationEvidence,
    policy: crate::policy::PolicyEvidence,
}

#[derive(Debug, Clone, Copy)]
pub struct PromotionReplayEvidence<'a> {
    pub evaluations: &'a AuthorityEvaluationEvidence,
    pub validation: &'a crate::validation::ValidationReceipt,
    pub policy: &'a crate::policy::PolicyReceipt,
    pub replay: &'a crate::replay::ReplayReceipt,
    pub authorization_digest: &'a crate::integrity::Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledRunRequest {
    binding: crate::authority::AuthorityBinding,
    context_packet_id: String,
    provider_output: ProviderOutput,
    policy: crate::policy::PolicyReceipt,
    validation: crate::validation::ValidationReceipt,
    replay: crate::replay::ReplayReceipt,
    ledger: crate::ledger::Ledger,
    actor: crate::ledger::LedgerActor,
    evidence_manifest: crate::authority::EvidenceManifest,
    evaluation_evidence: AuthorityEvaluationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledRunResult {
    pub status: ControlledRunStatus,
    pub reason: ControlledRunReason,
    pub execution_decision: ExecutionDecisionReport,
    pub promotion_decision: PromotionDecisionReport,
    pub ledger: crate::ledger::Ledger,
    pub promotion_replay: PromotionReplayVerificationReport,
    pub reviewable_candidate_summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledRunError {
    EmptyRunId,
    EmptyContextPacketId,
    MissingEvidenceRefs,
    ReceiptBindingMismatch,
    CandidateDigestMismatch,
    EvidenceManifestMismatch,
    RevisionMismatch,
    ReceiptDerivationMismatch,
}

impl ControlledRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::ReceiptBindingMismatch => "receipt_binding_mismatch",
            Self::CandidateDigestMismatch => "candidate_digest_mismatch",
            Self::EvidenceManifestMismatch => "evidence_manifest_mismatch",
            Self::RevisionMismatch => "revision_mismatch",
            Self::ReceiptDerivationMismatch => "receipt_derivation_mismatch",
        }
    }
}

impl AuthorityEvaluationEvidence {
    pub fn new(
        validation: crate::validation::ValidationEvidence,
        policy: crate::policy::PolicyEvidence,
    ) -> Self {
        Self { validation, policy }
    }

    pub fn validation(&self) -> &crate::validation::ValidationEvidence {
        &self.validation
    }

    pub fn policy(&self) -> &crate::policy::PolicyEvidence {
        &self.policy
    }
}

impl ControlledRunRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        context_packet_id: impl Into<String>,
        provider_output: ProviderOutput,
        policy: crate::policy::PolicyReceipt,
        validation: crate::validation::ValidationReceipt,
        replay: crate::replay::ReplayReceipt,
        ledger: crate::ledger::Ledger,
        actor: crate::ledger::LedgerActor,
        evidence_manifest: crate::authority::EvidenceManifest,
        evaluation_evidence: AuthorityEvaluationEvidence,
    ) -> Result<Self, ControlledRunError> {
        let context_packet_id = context_packet_id.into();
        if context_packet_id.is_empty() {
            return Err(ControlledRunError::EmptyContextPacketId);
        }
        let binding = validation.binding().clone();
        validate_controlled_receipts(&binding, &policy, &validation, &replay)?;
        validate_controlled_evidence(&binding, &provider_output, &evidence_manifest)?;
        validate_receipt_derivation(&binding, &validation, &policy, &evaluation_evidence)?;
        validate_controlled_revision(&binding, &ledger)?;
        Ok(Self {
            binding,
            context_packet_id,
            provider_output,
            policy,
            validation,
            replay,
            ledger,
            actor,
            evidence_manifest,
            evaluation_evidence,
        })
    }
}

fn validate_receipt_derivation(
    binding: &crate::authority::AuthorityBinding,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    evidence: &AuthorityEvaluationEvidence,
) -> Result<(), ControlledRunError> {
    let rederived_validation =
        crate::validation::evaluate_validation(binding.clone(), evidence.validation());
    let rederived_policy =
        crate::policy::evaluate_policy(binding.clone(), evidence.policy(), &rederived_validation);
    if rederived_validation.digest() == validation.digest()
        && rederived_policy.digest() == policy.digest()
    {
        return Ok(());
    }
    Err(ControlledRunError::ReceiptDerivationMismatch)
}

fn validate_controlled_receipts(
    binding: &crate::authority::AuthorityBinding,
    policy: &crate::policy::PolicyReceipt,
    validation: &crate::validation::ValidationReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> Result<(), ControlledRunError> {
    if policy.binding() == binding
        && replay.binding() == binding
        && policy.validation_receipt_digest() == validation.digest()
    {
        return Ok(());
    }
    Err(ControlledRunError::ReceiptBindingMismatch)
}

fn validate_controlled_evidence(
    binding: &crate::authority::AuthorityBinding,
    output: &ProviderOutput,
    manifest: &crate::authority::EvidenceManifest,
) -> Result<(), ControlledRunError> {
    manifest
        .verify_binding(binding)
        .map_err(|_| ControlledRunError::EvidenceManifestMismatch)?;
    validate_candidate_digest(binding, output)
}

fn validate_candidate_digest(
    binding: &crate::authority::AuthorityBinding,
    output: &ProviderOutput,
) -> Result<(), ControlledRunError> {
    let observed = crate::integrity::Digest::of_text(&output.content);
    if &observed == binding.candidate_digest() {
        return Ok(());
    }
    Err(ControlledRunError::CandidateDigestMismatch)
}

fn validate_controlled_revision(
    binding: &crate::authority::AuthorityBinding,
    ledger: &crate::ledger::Ledger,
) -> Result<(), ControlledRunError> {
    if ledger.last_revision() == Some(binding.valid_through_revision()) {
        return Ok(());
    }
    Err(ControlledRunError::RevisionMismatch)
}

pub fn run_controlled_model_flow(request: ControlledRunRequest) -> ControlledRunResult {
    ControlledRunPipeline::new(request).execute()
}

struct ControlledRunPipeline {
    request: ControlledRunRequest,
}

type ControlledRunStep<T> = Result<T, ControlledRunResult>;

impl ControlledRunPipeline {
    fn new(request: ControlledRunRequest) -> Self {
        Self { request }
    }

    fn execute(self) -> ControlledRunResult {
        self.execute_policy().unwrap_or_else(|failure| failure)
    }

    fn execute_policy(&self) -> ControlledRunStep<ControlledRunResult> {
        self.validate_provider_output()?;
        let execution = self.authorize_execution()?;
        let (promotion, authorization) = self.authorize_promotion(&execution)?;
        let ledger = self.append_promotion(&authorization, execution, promotion)?;
        let replay = self.verify_promotion(&ledger, &authorization, execution, promotion)?;
        Ok(self.accepted_result(execution, promotion, ledger, replay))
    }

    fn validate_provider_output(&self) -> ControlledRunStep<()> {
        if !provider_output_is_authoritative(&self.request.provider_output) {
            return Ok(());
        }
        Err(self.failure(
            ControlledRunStatus::Rejected,
            ControlledRunReason::ProviderOutputInvalid,
            blocked_execution(),
            blocked_promotion(),
            unverified_promotion_replay(),
        ))
    }

    fn authorize_execution(&self) -> ControlledRunStep<ExecutionDecisionReport> {
        let execution = decide_execution(
            self.request.replay.final_state(),
            &self.request.policy,
            &self.request.validation,
            &self.request.replay,
        );
        if execution.decision == ExecutionDecision::Allowed {
            return Ok(execution);
        }
        let (status, reason) = controlled_execution_failure(execution);
        Err(self.failure(
            status,
            reason,
            execution,
            blocked_promotion(),
            unverified_promotion_replay(),
        ))
    }

    fn authorize_promotion(
        &self,
        execution: &ExecutionDecisionReport,
    ) -> ControlledRunStep<(
        PromotionDecisionReport,
        crate::state::PromotionAuthorization,
    )> {
        let promotion = decide_promotion(self.request.replay.final_state(), execution);
        if promotion.decision != PromotionDecision::Allowed {
            return Err(self.promotion_failure(*execution, promotion));
        }
        let authorization = crate::state::authorize_promotion(
            self.request.binding.clone(),
            &self.request.validation,
            &self.request.policy,
            &self.request.replay,
        )
        .map_err(|_| self.promotion_failure(*execution, promotion))?;
        Ok((promotion, authorization))
    }

    fn append_promotion(
        &self,
        authorization: &crate::state::PromotionAuthorization,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
    ) -> ControlledRunStep<crate::ledger::Ledger> {
        let record = build_authorized_promotion_record(
            authorization,
            &self.request.evidence_manifest,
            self.request.actor.clone(),
            &self.request.ledger,
        )
        .map_err(|error| self.promotion_record_failure(error, execution, promotion))?;
        append_promotion_record(&self.request.ledger, record)
            .map_err(|_| self.ledger_append_failure(execution, promotion))
    }

    fn verify_promotion(
        &self,
        ledger: &crate::ledger::Ledger,
        authorization: &crate::state::PromotionAuthorization,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
    ) -> ControlledRunStep<PromotionReplayVerificationReport> {
        let replay = verify_authorized_promotion_replay(
            ledger,
            authorization,
            &self.request.evidence_manifest,
            self.replay_evidence(authorization),
        );
        if replay.status == PromotionReplayVerificationStatus::Verified {
            return Ok(replay);
        }
        Err(self.failure(
            ControlledRunStatus::Blocked,
            ControlledRunReason::PromotionReplayNotVerified,
            execution,
            promotion,
            replay,
        ))
    }

    fn replay_evidence<'a>(
        &'a self,
        authorization: &'a crate::state::PromotionAuthorization,
    ) -> PromotionReplayEvidence<'a> {
        PromotionReplayEvidence {
            evaluations: &self.request.evaluation_evidence,
            validation: &self.request.validation,
            policy: &self.request.policy,
            replay: &self.request.replay,
            authorization_digest: authorization.digest(),
        }
    }

    fn accepted_result(
        &self,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
        ledger: crate::ledger::Ledger,
        replay: PromotionReplayVerificationReport,
    ) -> ControlledRunResult {
        let _audit_timeline = crate::audit::project_ledger_timeline(ledger.events());
        ControlledRunResult {
            status: ControlledRunStatus::Accepted,
            reason: ControlledRunReason::RunAccepted,
            execution_decision: execution,
            promotion_decision: promotion,
            ledger,
            promotion_replay: replay,
            reviewable_candidate_summary: Some(self.reviewable_candidate_summary()),
        }
    }

    fn reviewable_candidate_summary(&self) -> String {
        format!(
            "run_id={} context_packet_id={} provider_output_id={} raw provider output remains untrusted; a reviewable candidate was produced by the controlled flow; task completion is not proved",
            self.request.binding.run_id(),
            self.request.context_packet_id,
            self.request.provider_output.id
        )
    }

    fn promotion_failure(
        &self,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
    ) -> ControlledRunResult {
        self.failure(
            ControlledRunStatus::Blocked,
            ControlledRunReason::PromotionNotAllowed,
            execution,
            promotion,
            unverified_promotion_replay(),
        )
    }

    fn promotion_record_failure(
        &self,
        error: PromotionRecordError,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
    ) -> ControlledRunResult {
        let reason = match error {
            PromotionRecordError::PromotionNotAllowed => ControlledRunReason::PromotionNotAllowed,
            PromotionRecordError::LedgerEventInvalid => ControlledRunReason::PromotionRecordInvalid,
        };
        self.failure(
            ControlledRunStatus::Blocked,
            reason,
            execution,
            promotion,
            unverified_promotion_replay(),
        )
    }

    fn ledger_append_failure(
        &self,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
    ) -> ControlledRunResult {
        self.failure(
            ControlledRunStatus::Blocked,
            ControlledRunReason::LedgerAppendFailed,
            execution,
            promotion,
            unverified_promotion_replay(),
        )
    }

    fn failure(
        &self,
        status: ControlledRunStatus,
        reason: ControlledRunReason,
        execution: ExecutionDecisionReport,
        promotion: PromotionDecisionReport,
        replay: PromotionReplayVerificationReport,
    ) -> ControlledRunResult {
        ControlledRunResult {
            status,
            reason,
            execution_decision: execution,
            promotion_decision: promotion,
            ledger: self.request.ledger.clone(),
            promotion_replay: replay,
            reviewable_candidate_summary: None,
        }
    }
}

fn controlled_execution_failure(
    execution: ExecutionDecisionReport,
) -> (ControlledRunStatus, ControlledRunReason) {
    match (execution.decision, execution.reason) {
        (ExecutionDecision::Rejected, ExecutionDecisionReason::LifecycleNotPassed) => (
            ControlledRunStatus::Rejected,
            ControlledRunReason::LifecycleNotPassed,
        ),
        (ExecutionDecision::Blocked, ExecutionDecisionReason::PolicyNotAllowed) => (
            ControlledRunStatus::Blocked,
            ControlledRunReason::PolicyNotAllowed,
        ),
        (ExecutionDecision::Blocked, ExecutionDecisionReason::ValidationNotPassed) => (
            ControlledRunStatus::Blocked,
            ControlledRunReason::ValidationNotPassed,
        ),
        _ => (
            ControlledRunStatus::Blocked,
            ControlledRunReason::ExecutionNotAllowed,
        ),
    }
}

fn blocked_execution() -> ExecutionDecisionReport {
    ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady)
}

fn blocked_promotion() -> PromotionDecisionReport {
    PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed)
}

fn unverified_promotion_replay() -> PromotionReplayVerificationReport {
    PromotionReplayVerificationReport::not_verified(
        PromotionReplayVerificationReason::LedgerNotReplayReady,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{
        AuthorityBinding, AuthorityBindingInput, EvidenceManifest, EvidenceReference,
    };
    use crate::policy::PolicyReceipt;
    use crate::replay::ReplayReceipt;
    use crate::state::LifecycleState;
    use crate::validation::ValidationReceipt;

    fn ready_policy() -> PolicyReceipt {
        let validation = ready_validation();
        policy_for_validation(&validation)
    }

    fn policy_for_validation(validation: &ValidationReceipt) -> PolicyReceipt {
        let binding = validation.binding().clone();
        crate::policy::evaluate_policy(
            binding,
            &crate::policy::PolicyEvidence::new(true, true, false),
            validation,
        )
    }

    fn ready_validation() -> ValidationReceipt {
        let manifest = receipt_manifest();
        crate::validation::evaluate_validation(
            receipt_binding(&manifest),
            &crate::validation::ValidationEvidence::new(true, true, true, false, manifest),
        )
    }

    fn ready_replay() -> ReplayReceipt {
        let manifest = receipt_manifest();
        crate::replay::verify_replay_receipt(
            receipt_binding(&manifest),
            &ready_receipt_ledger(),
            &manifest,
        )
        .unwrap()
    }

    fn unknown_policy() -> PolicyReceipt {
        let manifest = receipt_manifest();
        let validation = ready_validation();
        crate::policy::record_unknown_policy(receipt_binding(&manifest), &validation)
    }

    fn unknown_validation() -> ValidationReceipt {
        let manifest = receipt_manifest();
        crate::validation::record_unknown_validation(receipt_binding(&manifest))
    }

    fn unknown_replay() -> ReplayReceipt {
        let manifest = receipt_manifest();
        crate::replay::record_unknown_replay(receipt_binding(&manifest))
    }

    fn ready_evaluation_evidence() -> AuthorityEvaluationEvidence {
        AuthorityEvaluationEvidence::new(
            crate::validation::ValidationEvidence::new(true, true, true, false, receipt_manifest()),
            crate::policy::PolicyEvidence::new(true, true, false),
        )
    }

    fn receipt_manifest() -> EvidenceManifest {
        EvidenceManifest::new(vec![
            EvidenceReference::new("evidence-1", crate::integrity::Digest::of_text("one")).unwrap(),
            EvidenceReference::new("evidence-2", crate::integrity::Digest::of_text("two")).unwrap(),
        ])
        .unwrap()
    }

    fn receipt_binding(manifest: &EvidenceManifest) -> AuthorityBinding {
        AuthorityBinding::new(AuthorityBindingInput {
            run_id: "run-1".into(),
            task_digest: crate::integrity::Digest::of_text("task"),
            operator_intent_digest: crate::integrity::Digest::of_text("intent"),
            context_packet_digest: crate::integrity::Digest::of_text("context-1"),
            candidate_digest: crate::integrity::Digest::of_text("candidate content"),
            policy_bundle_digest: crate::integrity::Digest::of_text("policy"),
            evidence_manifest_digest: manifest.digest().clone(),
            verifier_id: "test-verifier".into(),
            verifier_version: "1.0.0".into(),
            valid_through_revision: 2,
        })
        .unwrap()
    }

    fn ready_receipt_ledger() -> crate::ledger::Ledger {
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .unwrap()
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .unwrap();
        let validation = ready_validation();
        let policy = policy_for_validation(&validation);
        ledger
            .seal(&crate::ledger::LedgerSeal {
                binding: validation.binding().clone(),
                actor_authorization_ref: "test-actor-authorization".into(),
                validation_receipt_ref: validation.digest().as_str().into(),
                policy_receipt_ref: policy.digest().as_str().into(),
                schema_version: "v1.0.0".into(),
                verifier_version: "1.0.0".into(),
            })
            .unwrap()
    }

    #[test]
    fn provider_request_requires_id() {
        assert_eq!(
            ProviderRequest::new("", ProviderKind::Local, "summary"),
            Err(ProviderBoundaryError::EmptyRequestId)
        );
    }

    #[test]
    fn provider_request_requires_prompt_summary() {
        assert_eq!(
            ProviderRequest::new("req-1", ProviderKind::Local, ""),
            Err(ProviderBoundaryError::EmptyPromptSummary)
        );
    }

    #[test]
    fn provider_output_requires_id() {
        assert_eq!(
            ProviderOutput::new_untrusted(
                "",
                "request-1",
                ProviderKind::Local,
                "candidate",
                ProviderOutputStatus::Received
            ),
            Err(ProviderBoundaryError::EmptyOutputId)
        );
    }

    #[test]
    fn provider_output_requires_request_id() {
        assert_eq!(
            ProviderOutput::new_untrusted(
                "output-1",
                "",
                ProviderKind::Local,
                "candidate",
                ProviderOutputStatus::Received
            ),
            Err(ProviderBoundaryError::EmptyOutputRequestId)
        );
    }

    #[test]
    fn provider_output_requires_content() {
        assert_eq!(
            ProviderOutput::new_untrusted(
                "output-1",
                "request-1",
                ProviderKind::Local,
                "",
                ProviderOutputStatus::Received
            ),
            Err(ProviderBoundaryError::EmptyOutputContent)
        );
    }

    #[test]
    fn provider_output_is_always_untrusted() {
        let output = ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Local,
            "validated approved passed safe",
            ProviderOutputStatus::Received,
        )
        .expect("output should be valid");

        assert_eq!(output.trust, ProviderOutputTrust::Untrusted);
    }

    #[test]
    fn provider_output_is_not_authoritative() {
        for provider_kind in [
            ProviderKind::Local,
            ProviderKind::Cloud,
            ProviderKind::Ide,
            ProviderKind::Unknown,
        ] {
            let output = ProviderOutput::new_untrusted(
                "output-1",
                "request-1",
                provider_kind,
                "approved safe",
                ProviderOutputStatus::Received,
            )
            .expect("output should be valid");

            assert!(!provider_output_is_authoritative(&output));
        }
    }

    #[test]
    fn provider_output_does_not_infer_validation_status() {
        let output = ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Local,
            "validated passed",
            ProviderOutputStatus::Unknown,
        )
        .expect("output should be valid");

        assert_eq!(output.trust, ProviderOutputTrust::Untrusted);
        assert_eq!(output.status, ProviderOutputStatus::Unknown);
    }

    #[test]
    fn provider_output_does_not_infer_policy_status() {
        let output = ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Cloud,
            "approved safe",
            ProviderOutputStatus::Rejected,
        )
        .expect("output should be valid");

        assert_eq!(output.trust, ProviderOutputTrust::Untrusted);
        assert_eq!(output.status, ProviderOutputStatus::Rejected);
    }

    #[test]
    fn provider_boundary_error_codes_are_stable() {
        assert_eq!(
            ProviderBoundaryError::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            ProviderBoundaryError::EmptyPromptSummary.code(),
            "empty_prompt_summary"
        );
        assert_eq!(
            ProviderBoundaryError::EmptyOutputId.code(),
            "empty_output_id"
        );
        assert_eq!(
            ProviderBoundaryError::EmptyOutputRequestId.code(),
            "empty_output_request_id"
        );
        assert_eq!(
            ProviderBoundaryError::EmptyOutputContent.code(),
            "empty_output_content"
        );
        assert_eq!(
            ProviderBoundaryError::OutputMarkedTrusted.code(),
            "output_marked_trusted"
        );
    }

    #[test]
    fn provider_boundary_accepts_local_provider_kind() {
        let request = ProviderRequest::new("req-1", ProviderKind::Local, "prompt")
            .expect("request should be valid");
        assert_eq!(request.provider_kind, ProviderKind::Local);
    }

    #[test]
    fn provider_boundary_accepts_cloud_provider_kind() {
        let request = ProviderRequest::new("req-1", ProviderKind::Cloud, "prompt")
            .expect("request should be valid");
        assert_eq!(request.provider_kind, ProviderKind::Cloud);
    }

    #[test]
    fn provider_boundary_accepts_ide_provider_kind() {
        let request =
            ProviderRequest::new("req-1", ProviderKind::Ide, "prompt").expect("valid request");
        assert_eq!(request.provider_kind, ProviderKind::Ide);
    }

    #[test]
    fn provider_boundary_accepts_unknown_provider_kind() {
        let request = ProviderRequest::new("req-1", ProviderKind::Unknown, "prompt")
            .expect("request should be valid");
        assert_eq!(request.provider_kind, ProviderKind::Unknown);
    }

    #[test]
    fn provider_adapter_error_codes_are_stable() {
        assert_eq!(
            ProviderAdapterError::EmptyAdapterId.code(),
            "empty_adapter_id"
        );
        assert_eq!(
            ProviderAdapterError::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            ProviderAdapterError::EmptyPromptSummary.code(),
            "empty_prompt_summary"
        );
        assert_eq!(
            ProviderAdapterError::OutputConstructionFailed.code(),
            "output_construction_failed"
        );
        assert_eq!(
            ProviderAdapterError::RealProviderNotImplemented.code(),
            "real_provider_not_implemented"
        );
    }

    #[test]
    fn local_provider_config_error_codes_are_stable() {
        assert_eq!(
            LocalProviderAdapterConfigError::EmptyAdapterId.code(),
            "empty_adapter_id"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::EmptyModelLabel.code(),
            "empty_model_label"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::InvalidProviderKind.code(),
            "invalid_provider_kind"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::InvalidEndpointKind.code(),
            "invalid_endpoint_kind"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::InvalidMaxOutputTokens.code(),
            "invalid_max_output_tokens"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::InvalidTimeoutMillis.code(),
            "invalid_timeout_millis"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::SecretMarkerDetected.code(),
            "secret_marker_detected"
        );
        assert_eq!(
            LocalProviderAdapterConfigError::CapabilityAuthorityInvalid.code(),
            "capability_authority_invalid"
        );
    }

    #[test]
    fn local_provider_capability_none_is_closed() {
        let cap = LocalProviderCapability::none();
        assert!(!cap.supports_streaming);
        assert!(!cap.supports_tools);
        assert!(!cap.supports_json_mode);
        assert!(!cap.supports_system_prompt);
        assert!(!cap.supports_temperature);
    }

    #[test]
    fn local_provider_capability_preview_is_descriptive_only() {
        let cap = LocalProviderCapability::preview_local_model();
        assert!(!cap.supports_streaming);
        assert!(!cap.supports_tools);
        assert!(!cap.supports_json_mode);
        assert!(cap.supports_system_prompt);
        assert!(cap.supports_temperature);
    }

    fn valid_local_provider_config(
        endpoint_kind: LocalProviderEndpointKind,
    ) -> LocalProviderAdapterConfig {
        LocalProviderAdapterConfig::new(
            "local-preview-adapter",
            ProviderKind::Local,
            endpoint_kind,
            "preview-model-approved-validated-safe-execute-promote",
            1024,
            3000,
            LocalProviderCapability::preview_local_model(),
            LocalProviderCapabilityAuthority::DescriptiveOnly,
        )
        .expect("config should be valid")
    }

    #[test]
    fn local_provider_config_requires_adapter_id() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "model",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::EmptyAdapterId)
        );
    }

    #[test]
    fn local_provider_config_requires_model_label() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::EmptyModelLabel)
        );
    }

    #[test]
    fn local_provider_config_rejects_unknown_provider_kind() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Unknown,
                LocalProviderEndpointKind::Disabled,
                "model",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::InvalidProviderKind)
        );
    }

    #[test]
    fn local_provider_config_rejects_unknown_endpoint_kind() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Unknown,
                "model",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::InvalidEndpointKind)
        );
    }

    #[test]
    fn local_provider_config_rejects_zero_max_output_tokens() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "model",
                0,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::InvalidMaxOutputTokens)
        );
    }

    #[test]
    fn local_provider_config_rejects_zero_timeout() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "model",
                1,
                0,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::InvalidTimeoutMillis)
        );
    }

    #[test]
    fn local_provider_config_rejects_secret_markers_in_adapter_id() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "my_api_key_adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "model",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::SecretMarkerDetected)
        );
    }

    #[test]
    fn local_provider_config_rejects_secret_markers_in_model_label() {
        assert_eq!(
            LocalProviderAdapterConfig::new(
                "adapter",
                ProviderKind::Local,
                LocalProviderEndpointKind::Disabled,
                "preview_token_model",
                1,
                1,
                LocalProviderCapability::none(),
                LocalProviderCapabilityAuthority::DescriptiveOnly
            ),
            Err(LocalProviderAdapterConfigError::SecretMarkerDetected)
        );
    }

    #[test]
    fn local_provider_config_capabilities_do_not_grant_authority() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::Disabled);
        assert_eq!(
            config.capabilities,
            LocalProviderCapability::preview_local_model()
        );
        assert!(!local_provider_config_allows_authority(&config));
    }

    #[test]
    fn local_provider_config_allows_no_authority() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::Disabled);
        assert!(!local_provider_config_allows_authority(&config));
    }

    #[test]
    fn local_provider_config_cannot_invoke_real_provider() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::Disabled);
        assert!(!local_provider_config_can_invoke_real_provider(&config));
    }

    #[test]
    fn local_process_endpoint_is_metadata_only() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::LocalProcess);
        assert_eq!(
            config.endpoint_kind,
            LocalProviderEndpointKind::LocalProcess
        );
        assert!(!local_provider_config_can_invoke_real_provider(&config));
    }

    #[test]
    fn local_http_endpoint_is_metadata_only() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::LocalHttp);
        assert_eq!(config.endpoint_kind, LocalProviderEndpointKind::LocalHttp);
        assert!(!local_provider_config_can_invoke_real_provider(&config));
    }

    #[test]
    fn deterministic_stub_provider_remains_only_invoking_adapter() {
        let (provider, invocation) = stub_fixture();
        let result = provider
            .invoke(&invocation)
            .expect("stub invokes deterministically");
        assert_eq!(result.adapter_id, "stub-provider");
    }

    #[test]
    fn local_provider_config_has_secret_marker_is_deterministic() {
        let clean = valid_local_provider_config(LocalProviderEndpointKind::Disabled);
        assert!(!local_provider_config_has_secret_marker(&clean));
    }

    #[test]
    fn local_provider_config_does_not_read_env_or_files() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::Disabled);
        assert!(!local_provider_config_has_secret_marker(&config));
        assert!(!local_provider_config_allows_authority(&config));
    }

    #[test]
    fn local_provider_config_does_not_open_network_or_spawn_process() {
        let config = valid_local_provider_config(LocalProviderEndpointKind::LocalHttp);
        assert!(!local_provider_config_can_invoke_real_provider(&config));
    }

    #[test]
    fn provider_adapter_invocation_requires_adapter_id() {
        let request = ProviderRequest::new("req-1", ProviderKind::Local, "summary").unwrap();
        assert_eq!(
            ProviderAdapterInvocation::new("", request),
            Err(ProviderAdapterError::EmptyAdapterId)
        );
    }

    #[test]
    fn provider_adapter_invocation_requires_request_id() {
        let request = ProviderRequest {
            id: String::new(),
            provider_kind: ProviderKind::Local,
            prompt_summary: "summary".to_string(),
        };
        assert_eq!(
            ProviderAdapterInvocation::new("stub", request),
            Err(ProviderAdapterError::EmptyRequestId)
        );
    }

    #[test]
    fn provider_adapter_invocation_requires_prompt_summary() {
        let request = ProviderRequest {
            id: "req-1".to_string(),
            provider_kind: ProviderKind::Local,
            prompt_summary: String::new(),
        };
        assert_eq!(
            ProviderAdapterInvocation::new("stub", request),
            Err(ProviderAdapterError::EmptyPromptSummary)
        );
    }

    #[test]
    fn deterministic_stub_provider_requires_adapter_id() {
        assert_eq!(
            DeterministicStubProvider::new("", "prefix"),
            Err(ProviderAdapterError::EmptyAdapterId)
        );
    }

    fn stub_fixture() -> (DeterministicStubProvider, ProviderAdapterInvocation) {
        let provider = DeterministicStubProvider::new("stub-provider", "stub-response:")
            .expect("valid provider");
        let request = ProviderRequest::new("req-1", ProviderKind::Cloud, "prompt").unwrap();
        let invocation = ProviderAdapterInvocation::new("stub-provider", request).unwrap();
        (provider, invocation)
    }

    #[test]
    fn deterministic_stub_provider_invocation_returns_untrusted_output() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.trust, ProviderOutputTrust::Untrusted);
    }

    #[test]
    fn deterministic_stub_provider_output_is_not_authoritative() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert!(!provider_adapter_result_is_authoritative(&result));
    }

    #[test]
    fn deterministic_stub_provider_output_id_is_deterministic() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.id, "stub-output:stub-provider:req-1");
    }

    #[test]
    fn deterministic_stub_provider_output_request_id_matches_request() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.request_id, invocation.request.id);
    }

    #[test]
    fn deterministic_stub_provider_preserves_provider_kind() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(
            result.output.provider_kind,
            invocation.request.provider_kind
        );
    }

    #[test]
    fn deterministic_stub_provider_status_is_received() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.status, ProviderOutputStatus::Received);
    }

    #[test]
    fn deterministic_stub_provider_content_is_deterministic() {
        let (provider, invocation) = stub_fixture();
        let first = provider.invoke(&invocation).unwrap();
        let second = provider.invoke(&invocation).unwrap();
        assert_eq!(first.output.content, "stub-response: prompt");
        assert_eq!(first.output.content, second.output.content);
        assert_eq!(first.output.id, second.output.id);
    }

    #[test]
    fn deterministic_stub_provider_does_not_infer_policy_from_prompt() {
        let provider = DeterministicStubProvider::new("stub-provider", "stub-response:").unwrap();
        let request = ProviderRequest::new(
            "req-policy",
            ProviderKind::Local,
            "approved validated safe execute promote persist write",
        )
        .unwrap();
        let invocation = ProviderAdapterInvocation::new("stub-provider", request).unwrap();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.trust, ProviderOutputTrust::Untrusted);
        assert_eq!(result.output.status, ProviderOutputStatus::Received);
    }

    #[test]
    fn deterministic_stub_provider_does_not_infer_validation_from_prompt() {
        let provider = DeterministicStubProvider::new("stub-provider", "stub-response:").unwrap();
        let request =
            ProviderRequest::new("req-validation", ProviderKind::Ide, "validated").unwrap();
        let invocation = ProviderAdapterInvocation::new("stub-provider", request).unwrap();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.trust, ProviderOutputTrust::Untrusted);
        assert_eq!(result.output.status, ProviderOutputStatus::Received);
    }

    #[test]
    fn deterministic_stub_provider_does_not_infer_execution_from_prompt() {
        let provider = DeterministicStubProvider::new("stub-provider", "stub-response:").unwrap();
        let request =
            ProviderRequest::new("req-execution", ProviderKind::Unknown, "execute").unwrap();
        let invocation = ProviderAdapterInvocation::new("stub-provider", request).unwrap();
        let result = provider.invoke(&invocation).unwrap();
        assert_eq!(result.output.trust, ProviderOutputTrust::Untrusted);
        assert_eq!(result.output.status, ProviderOutputStatus::Received);
    }

    #[test]
    fn deterministic_stub_provider_does_not_append_ledger() {
        let (_, invocation) = stub_fixture();
        let initial = crate::ledger::Ledger::empty().events().len();
        assert_eq!(initial, 0);
        let provider = DeterministicStubProvider::new("stub-provider", "stub-response:").unwrap();
        let _ = provider.invoke(&invocation).unwrap();
        assert_eq!(crate::ledger::Ledger::empty().events().len(), 0);
    }

    #[test]
    fn deterministic_stub_provider_does_not_execute_controlled_flow() {
        let (provider, invocation) = stub_fixture();
        let result = provider.invoke(&invocation).unwrap();
        assert!(result.output.content.contains("stub-response:"));
        assert!(!result.output.content.contains("run_id="));
    }

    #[test]
    fn deterministic_stub_provider_does_not_persist() {
        let (provider, invocation) = stub_fixture();
        let first = provider.invoke(&invocation).unwrap();
        let second = provider.invoke(&invocation).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn execution_decision_reason_codes_are_stable() {
        assert_eq!(
            ExecutionDecisionReason::ReadyForExecution.code(),
            "ready_for_execution"
        );
        assert_eq!(
            ExecutionDecisionReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::PolicyNotAllowed.code(),
            "policy_not_allowed"
        );
        assert_eq!(
            ExecutionDecisionReason::ValidationNotPassed.code(),
            "validation_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::ReplayNotReady.code(),
            "replay_not_ready"
        );
    }

    #[test]
    fn execution_allows_when_all_inputs_are_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(report, ExecutionDecisionReport::allowed());
    }

    #[test]
    fn execution_rejects_when_lifecycle_not_passed() {
        let report = decide_execution(
            LifecycleState::Created,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_policy_not_allowed() {
        let report = decide_execution(
            LifecycleState::Passed,
            &unknown_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_blocks_when_validation_not_passed() {
        let validation = unknown_validation();
        let policy = policy_for_validation(&validation);
        let report = decide_execution(
            LifecycleState::Passed,
            &policy,
            &validation,
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_replay_not_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &unknown_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady)
        );
    }

    #[test]
    fn execution_priority_lifecycle_before_policy() {
        let report = decide_execution(
            LifecycleState::Created,
            &unknown_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_priority_validation_before_policy() {
        let validation = unknown_validation();
        let policy = policy_for_validation(&validation);
        let report = decide_execution(
            LifecycleState::Passed,
            &policy,
            &validation,
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_priority_validation_before_replay() {
        let validation = unknown_validation();
        let policy = policy_for_validation(&validation);
        let report = decide_execution(
            LifecycleState::Passed,
            &policy,
            &validation,
            &unknown_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_is_deterministic_for_equivalent_receipts() {
        let report_a = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );
        let report_b = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(report_a, ExecutionDecisionReport::allowed());
        assert_eq!(report_b, ExecutionDecisionReport::allowed());
        assert_eq!(report_a, report_b);
    }

    #[test]
    fn promotion_decision_reason_codes_are_stable() {
        assert_eq!(
            PromotionDecisionReason::ReadyForTier1Promotion.code(),
            "ready_for_tier_1_promotion"
        );
        assert_eq!(
            PromotionDecisionReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
        assert_eq!(
            PromotionDecisionReason::ExecutionNotAllowed.code(),
            "execution_not_allowed"
        );
    }

    #[test]
    fn promotion_allows_when_lifecycle_passed_and_execution_allowed() {
        let report = decide_promotion(LifecycleState::Passed, &ExecutionDecisionReport::allowed());

        assert_eq!(report, PromotionDecisionReport::allowed());
    }

    #[test]
    fn promotion_rejects_when_lifecycle_created() {
        let report = decide_promotion(LifecycleState::Created, &ExecutionDecisionReport::allowed());

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_rejects_when_lifecycle_promoted_tier_1() {
        let report = decide_promotion(
            LifecycleState::PromotedTier1,
            &ExecutionDecisionReport::allowed(),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_blocks_when_execution_blocked() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed)
        );
    }

    #[test]
    fn promotion_blocks_when_execution_rejected() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed)
        );
    }

    #[test]
    fn promotion_priority_lifecycle_before_execution() {
        let report = decide_promotion(
            LifecycleState::Created,
            &ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_decision_uses_execution_decision_not_reason() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport {
                decision: ExecutionDecision::Allowed,
                reason: ExecutionDecisionReason::PolicyNotAllowed,
            },
        );

        assert_eq!(report, PromotionDecisionReport::allowed());
    }

    #[test]
    fn promotion_does_not_mutate_or_transition_harness_state() {
        let state = crate::state::HarnessState {
            lifecycle: LifecycleState::Passed,
            revision: 7,
        };

        let report = decide_promotion(state.lifecycle, &ExecutionDecisionReport::allowed());

        assert_eq!(report, PromotionDecisionReport::allowed());
        assert_eq!(state.lifecycle, LifecycleState::Passed);
        assert_eq!(state.revision, 7);
    }

    fn ledger_actor() -> crate::ledger::LedgerActor {
        crate::ledger::LedgerActor::new(crate::ledger::LedgerActorType::Human, "operator-1")
            .expect("actor should be valid")
    }

    fn replay_event(
        id: &str,
        revision: u64,
        lifecycle: Option<LifecycleState>,
    ) -> crate::ledger::LedgerEvent {
        let payload = match lifecycle {
            Some(next) => crate::ledger::LedgerPayload::with_lifecycle_transition("summary", next)
                .expect("payload should be valid"),
            None => crate::ledger::LedgerPayload::new("summary").expect("payload should be valid"),
        };

        crate::ledger::LedgerEvent::new(
            id,
            revision,
            crate::ledger::LedgerEventType::StateTransition,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            payload,
        )
        .expect("event should be valid")
    }

    #[test]
    fn promotion_record_error_codes_are_stable() {
        assert_eq!(
            PromotionRecordError::PromotionNotAllowed.code(),
            "promotion_not_allowed"
        );
        assert_eq!(
            PromotionRecordError::LedgerEventInvalid.code(),
            "ledger_event_invalid"
        );
    }

    #[test]
    fn promotion_record_rejects_blocked_promotion() {
        let result = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
        );

        assert_eq!(result, Err(PromotionRecordError::PromotionNotAllowed));
    }

    #[test]
    fn promotion_record_rejects_rejected_promotion() {
        let result = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed),
        );

        assert_eq!(result, Err(PromotionRecordError::PromotionNotAllowed));
    }

    #[test]
    fn promotion_record_builds_state_transition_event_for_allowed_promotion() {
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(
            record.event.event_type,
            crate::ledger::LedgerEventType::StateTransition
        );
    }

    #[test]
    fn promotion_record_payload_targets_promoted_tier_1() {
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(
            record.event.payload.lifecycle_transition,
            Some(LifecycleState::PromotedTier1)
        );
    }

    #[test]
    fn promotion_record_preserves_caller_revision() {
        let record = build_promotion_record(
            "evt-1",
            42,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.revision, 42);
    }

    #[test]
    fn promotion_record_preserves_caller_actor() {
        let actor = ledger_actor();
        let record = build_promotion_record(
            "evt-1",
            1,
            actor.clone(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.actor.id, actor.id);
        assert_eq!(record.event.actor.actor_type, actor.actor_type);
    }

    #[test]
    fn promotion_record_preserves_evidence_refs() {
        let evidence_refs = vec![
            "evidence-1".to_string(),
            "evidence-2".to_string(),
            "evidence-3".to_string(),
        ];
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            evidence_refs.clone(),
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.evidence_refs, evidence_refs);
    }

    #[test]
    fn promotion_record_fails_on_invalid_ledger_event() {
        let blocked = build_promotion_record(
            "",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(blocked, Err(PromotionRecordError::LedgerEventInvalid));

        let zero_revision = build_promotion_record(
            "evt-1",
            0,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(zero_revision, Err(PromotionRecordError::LedgerEventInvalid));

        let no_evidence = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec![],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(no_evidence, Err(PromotionRecordError::LedgerEventInvalid));

        let empty_summary = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(empty_summary, Err(PromotionRecordError::LedgerEventInvalid));
    }

    #[test]
    fn promotion_record_uses_decision_not_reason() {
        let report = PromotionDecisionReport {
            decision: PromotionDecision::Allowed,
            reason: PromotionDecisionReason::LifecycleNotPassed,
        };
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &report,
        );

        assert!(record.is_ok());
    }

    #[test]
    fn promotion_record_does_not_append_to_ledger() {
        let ledger = crate::ledger::Ledger::empty();
        let _ = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert!(ledger.events().is_empty());
    }

    #[test]
    fn promotion_record_does_not_transition_harness_state() {
        let state = crate::state::HarnessState {
            revision: 10,
            lifecycle: LifecycleState::Passed,
        };

        let _ = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(state.revision, 10);
        assert_eq!(state.lifecycle, LifecycleState::Passed);
    }
    #[test]
    fn promotion_append_error_codes_are_stable() {
        assert_eq!(
            PromotionAppendError::LedgerAppendFailed.code(),
            "ledger_append_failed"
        );
    }

    #[test]
    fn promotion_append_succeeds_on_empty_ledger_with_revision_one() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(ledger.events().len(), 0);
        assert_eq!(next.events().len(), 1);
    }

    #[test]
    fn promotion_append_preserves_event_shape() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");
        let appended = &next.events()[0];

        assert_eq!(
            appended.event_type,
            crate::ledger::LedgerEventType::StateTransition
        );
        assert_eq!(
            appended.payload.lifecycle_transition,
            Some(LifecycleState::PromotedTier1)
        );
    }

    #[test]
    fn promotion_append_preserves_existing_ledger_order() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-2",
            2,
            actor,
            vec!["evidence-2".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(next.events().len(), 2);
        assert_eq!(next.events()[0].id, "evt-seed");
        assert_eq!(next.events()[1].id, "evt-2");
    }

    #[test]
    fn promotion_append_requires_valid_next_revision() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-3",
            3,
            actor,
            vec!["evidence-3".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let result = append_promotion_record(&ledger, record);

        assert_eq!(result, Err(PromotionAppendError::LedgerAppendFailed));
    }

    #[test]
    fn promotion_append_failure_does_not_mutate_ledger() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-3",
            3,
            actor,
            vec!["evidence-3".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let _ = append_promotion_record(&ledger, record);

        assert_eq!(ledger.events().len(), 1);
        assert_eq!(ledger.events()[0].id, "evt-seed");
        assert_eq!(ledger.events()[0].revision, 1);
    }

    #[test]
    fn promotion_append_does_not_transition_harness_state() {
        let state = crate::state::HarnessState {
            revision: 8,
            lifecycle: LifecycleState::Passed,
        };
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let _ = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(state.lifecycle, LifecycleState::Passed);
        assert_eq!(state.revision, 8);
    }

    #[test]
    fn promotion_append_does_not_require_replay_readiness() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let result = append_promotion_record(&ledger, record);

        assert!(result.is_ok());
    }

    #[test]
    fn promotion_replay_verification_reason_codes_are_stable() {
        assert_eq!(
            PromotionReplayVerificationReason::PromotionReplayVerified.code(),
            "promotion_replay_verified"
        );
        assert_eq!(
            PromotionReplayVerificationReason::LedgerNotReplayReady.code(),
            "ledger_not_replay_ready"
        );
        assert_eq!(
            PromotionReplayVerificationReason::ReconstructionFailed.code(),
            "reconstruction_failed"
        );
        assert_eq!(
            PromotionReplayVerificationReason::FinalStateNotPromotedTier1.code(),
            "final_state_not_promoted_tier_1"
        );
    }

    #[test]
    fn promotion_replay_verification_passes_for_valid_promoted_tier_1_replay() {
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed")
            .append(replay_event(
                "evt-3",
                3,
                Some(LifecycleState::PromotedTier1),
            ))
            .expect("append should succeed");

        let report = verify_promotion_replay(&ledger);

        assert_eq!(report.status, PromotionReplayVerificationStatus::Verified);
        assert_eq!(
            report.reason,
            PromotionReplayVerificationReason::PromotionReplayVerified
        );
    }

    #[test]
    fn promotion_replay_verification_fails_for_empty_ledger() {
        let report = verify_promotion_replay(&crate::ledger::Ledger::empty());
        assert_eq!(
            report.status,
            PromotionReplayVerificationStatus::NotVerified
        );
        assert_eq!(
            report.reason,
            PromotionReplayVerificationReason::LedgerNotReplayReady
        );
    }

    #[test]
    fn promotion_replay_verification_fails_for_non_replay_ready_ledger() {
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-1", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed");
        let report = verify_promotion_replay(&ledger);

        assert_eq!(
            report.status,
            PromotionReplayVerificationStatus::NotVerified
        );
        assert_eq!(
            report.reason,
            PromotionReplayVerificationReason::LedgerNotReplayReady
        );
    }

    #[test]
    fn promotion_replay_verification_fails_when_reconstruction_fails() {
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, None))
            .expect("append should succeed");

        let report = verify_promotion_replay(&ledger);
        assert_eq!(
            report.status,
            PromotionReplayVerificationStatus::NotVerified
        );
        assert_eq!(
            report.reason,
            PromotionReplayVerificationReason::ReconstructionFailed
        );
    }

    #[test]
    fn promotion_replay_verification_fails_when_final_state_not_promoted() {
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed");

        let report = verify_promotion_replay(&ledger);
        assert_eq!(
            report.status,
            PromotionReplayVerificationStatus::NotVerified
        );
        assert_eq!(
            report.reason,
            PromotionReplayVerificationReason::FinalStateNotPromotedTier1
        );
    }

    fn promoted_tier_1_ledger() -> crate::ledger::Ledger {
        crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed")
            .append(replay_event(
                "evt-3",
                3,
                Some(LifecycleState::PromotedTier1),
            ))
            .expect("append should succeed")
    }

    #[test]
    fn promotion_replay_verification_reports_final_revision() {
        let report = verify_promotion_replay(&promoted_tier_1_ledger());
        assert_eq!(report.final_revision, 3);
    }

    #[test]
    fn promotion_replay_verification_reports_events_seen() {
        let ledger = promoted_tier_1_ledger();
        let report = verify_promotion_replay(&ledger);
        assert_eq!(report.events_seen, ledger.events().len() as u64);
    }

    #[test]
    fn promotion_replay_verification_reports_state_transitions_applied() {
        let report = verify_promotion_replay(&promoted_tier_1_ledger());
        assert_eq!(report.state_transitions_applied, 3);
    }

    #[test]
    fn promotion_replay_verification_does_not_mutate_ledger() {
        let ledger = promoted_tier_1_ledger();
        let before = ledger.events().to_vec();
        let _ = verify_promotion_replay(&ledger);
        let after = ledger.events().to_vec();

        assert_eq!(before.len(), after.len());
        assert_eq!(before, after);
    }

    #[test]
    fn promotion_replay_verification_is_idempotent_for_valid_promotion_ledger() {
        let ledger = promoted_tier_1_ledger();

        let first = verify_promotion_replay(&ledger);
        let second = verify_promotion_replay(&ledger);

        assert_eq!(first, second);
    }

    #[test]
    fn promotion_replay_verification_does_not_mutate_valid_ledger() {
        let ledger = promoted_tier_1_ledger();
        let before_events = ledger.events().to_vec();
        let before_last_revision = ledger.last_revision();

        let _ = verify_promotion_replay(&ledger);
        let _ = verify_promotion_replay(&ledger);

        assert_eq!(ledger.events().len(), before_events.len());
        assert_eq!(ledger.events(), before_events.as_slice());
        assert_eq!(ledger.last_revision(), before_last_revision);
    }

    fn invalid_promotion_ledger() -> crate::ledger::Ledger {
        crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed")
    }

    #[test]
    fn promotion_replay_verification_is_idempotent_for_invalid_ledger() {
        let invalid_ledger = invalid_promotion_ledger();

        let first = verify_promotion_replay(&invalid_ledger);
        let second = verify_promotion_replay(&invalid_ledger);

        assert_eq!(first, second);
    }

    #[test]
    fn promotion_replay_verification_does_not_repair_invalid_ledger() {
        let invalid_ledger = invalid_promotion_ledger();
        let before_events = invalid_ledger.events().to_vec();
        let before_last_revision = invalid_ledger.last_revision();

        let first = verify_promotion_replay(&invalid_ledger);
        let second = verify_promotion_replay(&invalid_ledger);

        assert_eq!(first.status, PromotionReplayVerificationStatus::NotVerified);
        assert_eq!(
            second.status,
            PromotionReplayVerificationStatus::NotVerified
        );
        assert_eq!(first, second);
        assert_eq!(invalid_ledger.events().len(), before_events.len());
        assert_eq!(invalid_ledger.events(), before_events.as_slice());
        assert_eq!(invalid_ledger.last_revision(), before_last_revision);
    }

    fn untrusted_provider_output() -> ProviderOutput {
        ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Local,
            "candidate content",
            ProviderOutputStatus::Received,
        )
        .expect("provider output should be valid")
    }

    fn phase_32_request() -> ControlledRunRequest {
        ControlledRunRequest::new(
            "context-1",
            untrusted_provider_output(),
            ready_policy(),
            ready_validation(),
            ready_replay(),
            ready_receipt_ledger(),
            ledger_actor(),
            receipt_manifest(),
            ready_evaluation_evidence(),
        )
        .expect("request should be valid")
    }

    #[test]
    fn controlled_run_error_codes_are_stable() {
        assert_eq!(ControlledRunError::EmptyRunId.code(), "empty_run_id");
        assert_eq!(
            ControlledRunError::EmptyContextPacketId.code(),
            "empty_context_packet_id"
        );
        assert_eq!(
            ControlledRunError::MissingEvidenceRefs.code(),
            "missing_evidence_refs"
        );
    }

    #[test]
    fn controlled_run_reason_codes_are_stable() {
        assert_eq!(ControlledRunReason::RunAccepted.code(), "run_accepted");
        assert_eq!(
            ControlledRunReason::ProviderOutputInvalid.code(),
            "provider_output_invalid"
        );
        assert_eq!(
            ControlledRunReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
    }

    #[test]
    fn controlled_run_request_rejects_candidate_digest_mismatch() {
        let mut output = untrusted_provider_output();
        output.content = "different candidate".to_string();
        assert_eq!(
            ControlledRunRequest::new(
                "context-1",
                output,
                ready_policy(),
                ready_validation(),
                ready_replay(),
                ready_receipt_ledger(),
                ledger_actor(),
                receipt_manifest(),
                ready_evaluation_evidence(),
            ),
            Err(ControlledRunError::CandidateDigestMismatch)
        );
    }

    #[test]
    fn controlled_run_request_requires_context_packet_id() {
        assert_eq!(
            ControlledRunRequest::new(
                "",
                untrusted_provider_output(),
                ready_policy(),
                ready_validation(),
                ready_replay(),
                ready_receipt_ledger(),
                ledger_actor(),
                receipt_manifest(),
                ready_evaluation_evidence(),
            ),
            Err(ControlledRunError::EmptyContextPacketId)
        );
    }

    #[test]
    fn controlled_run_request_rejects_evidence_manifest_mismatch() {
        let mismatched = EvidenceManifest::new(vec![EvidenceReference::new(
            "other",
            crate::integrity::Digest::of_text("other"),
        )
        .unwrap()])
        .unwrap();
        assert_eq!(
            ControlledRunRequest::new(
                "context-1",
                untrusted_provider_output(),
                ready_policy(),
                ready_validation(),
                ready_replay(),
                ready_receipt_ledger(),
                ledger_actor(),
                mismatched,
                ready_evaluation_evidence(),
            ),
            Err(ControlledRunError::EvidenceManifestMismatch)
        );
    }

    #[test]
    fn controlled_flow_rejects_authoritative_provider_output_if_possible() {
        assert!(!provider_output_is_authoritative(
            &untrusted_provider_output()
        ));
        let result = run_controlled_model_flow(phase_32_request());
        assert_ne!(result.reason, ControlledRunReason::ProviderOutputInvalid);
    }

    #[test]
    fn controlled_flow_blocks_when_policy_not_allowed() {
        let mut request = phase_32_request();
        request.policy = unknown_policy();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::PolicyNotAllowed);
    }

    #[test]
    fn controlled_flow_blocks_when_validation_not_passed() {
        let mut request = phase_32_request();
        let validation = unknown_validation();
        request.policy = policy_for_validation(&validation);
        request.validation = validation;
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ValidationNotPassed);
    }

    #[test]
    fn replay_receipt_rejects_when_lifecycle_not_passed() {
        let manifest = receipt_manifest();
        let binding = AuthorityBinding::new(AuthorityBindingInput {
            run_id: "run-1".into(),
            task_digest: crate::integrity::Digest::of_text("task"),
            operator_intent_digest: crate::integrity::Digest::of_text("intent"),
            context_packet_digest: crate::integrity::Digest::of_text("context-1"),
            candidate_digest: crate::integrity::Digest::of_text("candidate content"),
            policy_bundle_digest: crate::integrity::Digest::of_text("policy"),
            evidence_manifest_digest: manifest.digest().clone(),
            verifier_id: "test-verifier".into(),
            verifier_version: "1.0.0".into(),
            valid_through_revision: 1,
        })
        .unwrap();
        let ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
            .unwrap()
            .seal(&crate::ledger::LedgerSeal {
                binding: binding.clone(),
                actor_authorization_ref: "test-actor-authorization".into(),
                validation_receipt_ref: "validation-receipt".into(),
                policy_receipt_ref: "policy-receipt".into(),
                schema_version: "v1.0.0".into(),
                verifier_version: "1.0.0".into(),
            })
            .unwrap();
        let result = crate::replay::verify_replay_receipt(binding, &ledger, &manifest);
        assert_eq!(
            result,
            Err(crate::replay::ReplayReceiptError::LifecycleNotPassed)
        );
    }

    #[test]
    fn controlled_flow_blocks_when_replay_not_ready() {
        let mut request = phase_32_request();
        request.replay = unknown_replay();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ExecutionNotAllowed);
    }

    #[test]
    fn generic_passed_lifecycle_cannot_promote_without_authorization() {
        assert_eq!(
            LifecycleState::Passed.transition_to(LifecycleState::PromotedTier1),
            Err(crate::state::LifecycleError::PromotionAuthorizationRequired)
        );
    }

    #[test]
    fn controlled_flow_appends_promotion_record_on_success() {
        let request = phase_32_request();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Accepted);
        assert_eq!(result.ledger.events().len(), 3);
        assert_eq!(result.ledger.events()[2].id, "promotion:run-1");
        assert_eq!(result.ledger.events()[2].revision, 3);
    }

    #[test]
    fn controlled_flow_verifies_promotion_replay_on_success() {
        let result = run_controlled_model_flow(phase_32_request());
        assert_eq!(
            result.promotion_replay.status,
            PromotionReplayVerificationStatus::Verified
        );
    }

    #[test]
    fn controlled_flow_returns_reviewable_candidate_summary_on_success() {
        let result = run_controlled_model_flow(phase_32_request());
        let summary = result
            .reviewable_candidate_summary
            .expect("accepted result should include a reviewable candidate summary");
        assert!(summary.contains("run_id=run-1"));
        assert!(summary.contains("context_packet_id=context-1"));
        assert!(summary.contains("provider_output_id=output-1"));
        assert!(summary.contains("remains untrusted"));
        assert!(summary.contains("task completion is not proved"));
    }

    #[test]
    fn controlled_flow_keeps_raw_provider_output_untrusted() {
        let request = phase_32_request();
        assert_eq!(
            request.provider_output.trust,
            ProviderOutputTrust::Untrusted
        );
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Accepted);
    }

    #[test]
    fn controlled_flow_returns_no_reviewable_candidate_on_failure() {
        let mut request = phase_32_request();
        request.policy = unknown_policy();
        let result = run_controlled_model_flow(request);
        assert!(result.reviewable_candidate_summary.is_none());
    }

    #[test]
    fn controlled_flow_failure_does_not_append_ledger() {
        let mut request = phase_32_request();
        let original = request.ledger.clone();
        request.policy = unknown_policy();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.ledger, original);
    }

    #[test]
    fn controlled_flow_does_not_infer_validation_from_provider_content() {
        let mut request = phase_32_request();
        request.provider_output = ProviderOutput::new_untrusted(
            "output-2",
            "request-1",
            ProviderKind::Cloud,
            "validated passed approved safe",
            ProviderOutputStatus::Received,
        )
        .expect("provider output should be valid");
        let validation = unknown_validation();
        request.policy = policy_for_validation(&validation);
        request.validation = validation;
        let result = run_controlled_model_flow(request);
        assert_eq!(result.reason, ControlledRunReason::ValidationNotPassed);
    }

    #[test]
    fn controlled_flow_does_not_infer_policy_from_provider_content() {
        let mut request = phase_32_request();
        request.provider_output = ProviderOutput::new_untrusted(
            "output-3",
            "request-1",
            ProviderKind::Cloud,
            "approved authorized safe",
            ProviderOutputStatus::Received,
        )
        .expect("provider output should be valid");
        request.policy = unknown_policy();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.reason, ControlledRunReason::PolicyNotAllowed);
    }

    #[test]
    fn provider_output_keywords_remain_untrusted() {
        let output = ProviderOutput::new_untrusted(
            "output-keywords",
            "request-1",
            ProviderKind::Cloud,
            "approved validated safe execute promote",
            ProviderOutputStatus::Rejected,
        )
        .expect("provider output should be valid");
        assert_eq!(output.trust, ProviderOutputTrust::Untrusted);
    }

    #[test]
    fn provider_output_is_not_authoritative_for_all_provider_kinds() {
        for provider_kind in [
            ProviderKind::Local,
            ProviderKind::Cloud,
            ProviderKind::Ide,
            ProviderKind::Unknown,
        ] {
            let output = ProviderOutput::new_untrusted(
                "output-kind",
                "request-1",
                provider_kind,
                "approved validated safe execute promote",
                ProviderOutputStatus::Unknown,
            )
            .expect("provider output should be valid");
            assert!(!provider_output_is_authoritative(&output));
        }
    }

    #[test]
    fn provider_output_status_uses_supplied_status_only() {
        for status in [
            ProviderOutputStatus::Received,
            ProviderOutputStatus::Rejected,
            ProviderOutputStatus::Unknown,
        ] {
            let output = ProviderOutput::new_untrusted(
                "output-status",
                "request-1",
                ProviderKind::Local,
                "approved validated safe execute promote",
                status,
            )
            .expect("provider output should be valid");
            assert_eq!(output.status, status);
        }
    }

    #[test]
    fn controlled_flow_keyword_content_cannot_bypass_policy() {
        let mut request = phase_32_request();
        request.provider_output = ProviderOutput::new_untrusted(
            "output-keyword-policy",
            "request-1",
            ProviderKind::Cloud,
            "approved validated safe execute promote",
            ProviderOutputStatus::Received,
        )
        .expect("provider output should be valid");
        request.policy = unknown_policy();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::PolicyNotAllowed);
        assert!(result.reviewable_candidate_summary.is_none());
    }

    #[test]
    fn controlled_flow_keyword_content_cannot_bypass_validation() {
        let mut request = phase_32_request();
        request.provider_output = ProviderOutput::new_untrusted(
            "output-keyword-validation",
            "request-1",
            ProviderKind::Cloud,
            "approved validated safe execute promote",
            ProviderOutputStatus::Received,
        )
        .expect("provider output should be valid");
        let validation = unknown_validation();
        request.policy = policy_for_validation(&validation);
        request.validation = validation;
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ValidationNotPassed);
        assert!(result.reviewable_candidate_summary.is_none());
    }

    #[test]
    fn promotion_replay_requires_full_transition_history() {
        let shortcut_ledger = crate::ledger::Ledger::empty()
            .append(replay_event("evt-1", 1, Some(LifecycleState::Created)))
            .expect("append should succeed")
            .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
            .expect("append should succeed");
        let report = verify_promotion_replay(&shortcut_ledger);
        assert_eq!(
            report.status,
            PromotionReplayVerificationStatus::NotVerified
        );
    }
}
