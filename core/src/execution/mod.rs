#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Local,
    Cloud,
    Ide,
    Unknown,
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
    PolicyNotAllowed,
    ValidationNotPassed,
    ReplayNotReady,
}

impl ExecutionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForExecution => "ready_for_execution",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
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
    policy: &crate::policy::PolicyResult,
    validation: &crate::validation::ValidationResult,
    replay: &crate::replay::ReplayReport,
) -> ExecutionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed);
    }

    if policy.decision != crate::policy::PolicyDecision::Allowed {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed);
    }

    if validation.status != crate::validation::ValidationStatus::Pass {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed);
    }

    if replay.readiness != crate::replay::ReplayReadiness::Ready {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady);
    }

    ExecutionDecisionReport::allowed()
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
    pub fn allowed() -> Self {
        Self {
            decision: PromotionDecision::Allowed,
            reason: PromotionDecisionReason::ReadyForTier1Promotion,
        }
    }

    pub fn blocked(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Blocked,
            reason,
        }
    }

    pub fn rejected(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Rejected,
            reason,
        }
    }
}

pub fn decide_promotion(
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
    pub event: crate::ledger::LedgerEvent,
}

pub fn build_promotion_record(
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
    ReconstructionFailed,
    FinalStateNotPromotedTier1,
}

impl PromotionReplayVerificationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PromotionReplayVerified => "promotion_replay_verified",
            Self::LedgerNotReplayReady => "ledger_not_replay_ready",
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

pub fn verify_promotion_replay(
    ledger: &crate::ledger::Ledger,
) -> PromotionReplayVerificationReport {
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
pub struct ControlledRunRequest {
    pub run_id: String,
    pub context_packet_id: String,
    pub provider_output: ProviderOutput,
    pub lifecycle: crate::state::LifecycleState,
    pub policy: crate::policy::PolicyResult,
    pub validation: crate::validation::ValidationResult,
    pub replay: crate::replay::ReplayReport,
    pub ledger: crate::ledger::Ledger,
    pub actor: crate::ledger::LedgerActor,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledRunResult {
    pub status: ControlledRunStatus,
    pub reason: ControlledRunReason,
    pub execution_decision: ExecutionDecisionReport,
    pub promotion_decision: PromotionDecisionReport,
    pub ledger: crate::ledger::Ledger,
    pub promotion_replay: PromotionReplayVerificationReport,
    pub clean_output_summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledRunError {
    EmptyRunId,
    EmptyContextPacketId,
    MissingEvidenceRefs,
}

impl ControlledRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
        }
    }
}

impl ControlledRunRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run_id: impl Into<String>,
        context_packet_id: impl Into<String>,
        provider_output: ProviderOutput,
        lifecycle: crate::state::LifecycleState,
        policy: crate::policy::PolicyResult,
        validation: crate::validation::ValidationResult,
        replay: crate::replay::ReplayReport,
        ledger: crate::ledger::Ledger,
        actor: crate::ledger::LedgerActor,
        evidence_refs: Vec<String>,
    ) -> Result<Self, ControlledRunError> {
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(ControlledRunError::EmptyRunId);
        }
        let context_packet_id = context_packet_id.into();
        if context_packet_id.is_empty() {
            return Err(ControlledRunError::EmptyContextPacketId);
        }
        if evidence_refs.is_empty() {
            return Err(ControlledRunError::MissingEvidenceRefs);
        }

        Ok(Self {
            run_id,
            context_packet_id,
            provider_output,
            lifecycle,
            policy,
            validation,
            replay,
            ledger,
            actor,
            evidence_refs,
        })
    }
}

pub fn run_controlled_model_flow(request: ControlledRunRequest) -> ControlledRunResult {
    let failed = |status, reason, execution_decision, promotion_decision, promotion_replay| {
        ControlledRunResult {
            status,
            reason,
            execution_decision,
            promotion_decision,
            ledger: request.ledger.clone(),
            promotion_replay,
            clean_output_summary: None,
        }
    };

    if provider_output_is_authoritative(&request.provider_output) {
        return failed(
            ControlledRunStatus::Rejected,
            ControlledRunReason::ProviderOutputInvalid,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady),
            PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
            PromotionReplayVerificationReport::not_verified(
                PromotionReplayVerificationReason::LedgerNotReplayReady,
            ),
        );
    }

    let execution = decide_execution(
        request.lifecycle,
        &request.policy,
        &request.validation,
        &request.replay,
    );
    match (execution.decision, execution.reason) {
        (ExecutionDecision::Rejected, ExecutionDecisionReason::LifecycleNotPassed) => {
            return failed(
                ControlledRunStatus::Rejected,
                ControlledRunReason::LifecycleNotPassed,
                execution,
                PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        (ExecutionDecision::Blocked, ExecutionDecisionReason::PolicyNotAllowed) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::PolicyNotAllowed,
                execution,
                PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        (ExecutionDecision::Blocked, ExecutionDecisionReason::ValidationNotPassed) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::ValidationNotPassed,
                execution,
                PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        (ExecutionDecision::Blocked, ExecutionDecisionReason::ReplayNotReady) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::ExecutionNotAllowed,
                execution,
                PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        _ if execution.decision != ExecutionDecision::Allowed => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::ExecutionNotAllowed,
                execution,
                PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        _ => {}
    }

    let promotion = decide_promotion(request.lifecycle, &execution);
    match (promotion.decision, promotion.reason) {
        (PromotionDecision::Rejected, PromotionDecisionReason::LifecycleNotPassed) => {
            return failed(
                ControlledRunStatus::Rejected,
                ControlledRunReason::LifecycleNotPassed,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        (PromotionDecision::Blocked, PromotionDecisionReason::ExecutionNotAllowed) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::PromotionNotAllowed,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        _ if promotion.decision != PromotionDecision::Allowed => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::PromotionNotAllowed,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        _ => {}
    }

    let record = match build_promotion_record(
        format!("promotion:{}", request.run_id),
        request.ledger.last_revision().unwrap_or(0) + 1,
        request.actor,
        request.evidence_refs,
        format!("controlled_run:{}:promotion", request.run_id),
        &promotion,
    ) {
        Ok(record) => record,
        Err(PromotionRecordError::PromotionNotAllowed) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::PromotionNotAllowed,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
        Err(PromotionRecordError::LedgerEventInvalid) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::PromotionRecordInvalid,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
    };

    let next_ledger = match append_promotion_record(&request.ledger, record) {
        Ok(next) => next,
        Err(_) => {
            return failed(
                ControlledRunStatus::Blocked,
                ControlledRunReason::LedgerAppendFailed,
                execution,
                promotion,
                PromotionReplayVerificationReport::not_verified(
                    PromotionReplayVerificationReason::LedgerNotReplayReady,
                ),
            );
        }
    };

    let replay_verification = verify_promotion_replay(&next_ledger);
    if replay_verification.status != PromotionReplayVerificationStatus::Verified {
        return failed(
            ControlledRunStatus::Blocked,
            ControlledRunReason::PromotionReplayNotVerified,
            execution,
            promotion,
            replay_verification,
        );
    }

    let _audit_timeline = crate::audit::project_ledger_timeline(next_ledger.events());
    let clean_output_summary = Some(format!(
        "run_id={} context_packet_id={} provider_output_id={} raw provider output remains untrusted; clean output is based on accepted controlled flow",
        request.run_id, request.context_packet_id, request.provider_output.id
    ));

    ControlledRunResult {
        status: ControlledRunStatus::Accepted,
        reason: ControlledRunReason::RunAccepted,
        execution_decision: execution,
        promotion_decision: promotion,
        ledger: next_ledger,
        promotion_replay: replay_verification,
        clean_output_summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{PolicyDecision, PolicyResult};
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::LifecycleState;
    use crate::validation::{ValidationResult, ValidationStatus};

    fn ready_policy() -> PolicyResult {
        PolicyResult::allowed("required_policy_evidence_present")
    }

    fn ready_validation() -> ValidationResult {
        ValidationResult::pass("validation_evidence_passed")
    }

    fn ready_replay() -> ReplayReport {
        ReplayReport::replayable(2)
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
            &PolicyResult::unknown(),
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
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
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
            &ReplayReport::unknown(),
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
            &PolicyResult::unknown(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_priority_policy_before_validation() {
        let report = decide_execution(
            LifecycleState::Passed,
            &PolicyResult::unknown(),
            &ValidationResult::unknown(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_priority_validation_before_replay() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
            &ReplayReport::unknown(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_does_not_require_reason_strings_for_decision() {
        let policy_a = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_a",
        };
        let policy_b = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_b",
        };

        let validation_a = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_a",
        };
        let validation_b = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_b",
        };

        let replay_a = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 1,
            reason: "replay_reason_a",
        };
        let replay_b = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 99,
            reason: "replay_reason_b",
        };

        let report_a =
            decide_execution(LifecycleState::Passed, &policy_a, &validation_a, &replay_a);
        let report_b =
            decide_execution(LifecycleState::Passed, &policy_b, &validation_b, &replay_b);

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
            "run-1",
            "context-1",
            untrusted_provider_output(),
            LifecycleState::Passed,
            ready_policy(),
            ready_validation(),
            ready_replay(),
            crate::ledger::Ledger::empty()
                .append(replay_event("evt-1", 1, Some(LifecycleState::Evaluating)))
                .expect("append should succeed")
                .append(replay_event("evt-2", 2, Some(LifecycleState::Passed)))
                .expect("append should succeed"),
            ledger_actor(),
            vec!["evidence-1".to_string()],
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
    fn controlled_run_request_requires_run_id() {
        assert_eq!(
            ControlledRunRequest::new(
                "",
                "context-1",
                untrusted_provider_output(),
                LifecycleState::Passed,
                ready_policy(),
                ready_validation(),
                ready_replay(),
                crate::ledger::Ledger::empty(),
                ledger_actor(),
                vec!["evidence-1".to_string()],
            ),
            Err(ControlledRunError::EmptyRunId)
        );
    }

    #[test]
    fn controlled_run_request_requires_context_packet_id() {
        assert_eq!(
            ControlledRunRequest::new(
                "run-1",
                "",
                untrusted_provider_output(),
                LifecycleState::Passed,
                ready_policy(),
                ready_validation(),
                ready_replay(),
                crate::ledger::Ledger::empty(),
                ledger_actor(),
                vec!["evidence-1".to_string()],
            ),
            Err(ControlledRunError::EmptyContextPacketId)
        );
    }

    #[test]
    fn controlled_run_request_requires_evidence_refs() {
        assert_eq!(
            ControlledRunRequest::new(
                "run-1",
                "context-1",
                untrusted_provider_output(),
                LifecycleState::Passed,
                ready_policy(),
                ready_validation(),
                ready_replay(),
                crate::ledger::Ledger::empty(),
                ledger_actor(),
                vec![],
            ),
            Err(ControlledRunError::MissingEvidenceRefs)
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
        request.policy = PolicyResult::unknown();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::PolicyNotAllowed);
    }

    #[test]
    fn controlled_flow_blocks_when_validation_not_passed() {
        let mut request = phase_32_request();
        request.validation = ValidationResult::unknown();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ValidationNotPassed);
    }

    #[test]
    fn controlled_flow_rejects_when_lifecycle_not_passed() {
        let mut request = phase_32_request();
        request.lifecycle = LifecycleState::Created;
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Rejected);
        assert_eq!(result.reason, ControlledRunReason::LifecycleNotPassed);
    }

    #[test]
    fn controlled_flow_blocks_when_replay_not_ready() {
        let mut request = phase_32_request();
        request.replay = ReplayReport::unknown();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ExecutionNotAllowed);
    }

    #[test]
    fn controlled_flow_blocks_when_promotion_not_allowed() {
        let mut request = phase_32_request();
        request.lifecycle = LifecycleState::PromotedTier1;
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Rejected);
        assert_eq!(result.reason, ControlledRunReason::LifecycleNotPassed);
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
    fn controlled_flow_returns_clean_output_summary_on_success() {
        let result = run_controlled_model_flow(phase_32_request());
        let summary = result
            .clean_output_summary
            .expect("accepted result should include clean output summary");
        assert!(summary.contains("run_id=run-1"));
        assert!(summary.contains("context_packet_id=context-1"));
        assert!(summary.contains("provider_output_id=output-1"));
        assert!(summary.contains("remains untrusted"));
        assert!(summary.contains("accepted controlled flow"));
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
    fn controlled_flow_returns_no_clean_output_on_failure() {
        let mut request = phase_32_request();
        request.policy = PolicyResult::unknown();
        let result = run_controlled_model_flow(request);
        assert!(result.clean_output_summary.is_none());
    }

    #[test]
    fn controlled_flow_failure_does_not_append_ledger() {
        let mut request = phase_32_request();
        let original = request.ledger.clone();
        request.policy = PolicyResult::unknown();
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
        request.validation = ValidationResult::unknown();
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
        request.policy = PolicyResult::unknown();
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
        request.policy = PolicyResult::unknown();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::PolicyNotAllowed);
        assert!(result.clean_output_summary.is_none());
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
        request.validation = ValidationResult::unknown();
        let result = run_controlled_model_flow(request);
        assert_eq!(result.status, ControlledRunStatus::Blocked);
        assert_eq!(result.reason, ControlledRunReason::ValidationNotPassed);
        assert!(result.clean_output_summary.is_none());
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
