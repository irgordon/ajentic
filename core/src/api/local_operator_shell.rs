#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRunStatus {
    Idle,
    StubCompleted,
    IntentRecorded,
}

impl LocalRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::StubCompleted => "stub_completed",
            Self::IntentRecorded => "intent_recorded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorIntentKind {
    Approve,
    Reject,
}

impl LocalOperatorIntentKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellError {
    RunNotStarted,
    EmptyOperatorId,
    EmptyReason,
    TargetMismatch,
    CandidateTargetMismatch,
    DuplicateDecisionRejected,
    AuthorityClaimRejected,
    ProviderExecutionRejected,
    ReadinessClaimRejected,
}

impl LocalOperatorShellError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RunNotStarted => "run_not_started",
            Self::EmptyOperatorId => "empty_operator_id",
            Self::EmptyReason => "empty_reason",
            Self::TargetMismatch => "target_mismatch",
            Self::CandidateTargetMismatch => "candidate_target_mismatch",
            Self::DuplicateDecisionRejected => "duplicate_decision_rejected",
            Self::AuthorityClaimRejected => "authority_claim_rejected",
            Self::ProviderExecutionRejected => "provider_execution_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionRecordKind {
    Approve,
    Reject,
}

impl LocalDecisionRecordKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
        }
    }
}

impl From<LocalOperatorIntentKind> for LocalDecisionRecordKind {
    fn from(kind: LocalOperatorIntentKind) -> Self {
        match kind {
            LocalOperatorIntentKind::Approve => Self::Approve,
            LocalOperatorIntentKind::Reject => Self::Reject,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionRecordStatus {
    Recorded,
}

impl LocalDecisionRecordStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionRecord {
    pub decision_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub operator_id: String,
    pub intent_kind: LocalDecisionRecordKind,
    pub decision_status: LocalDecisionRecordStatus,
    pub validation_status: String,
    pub recorded_sequence: u64,
    pub recorded_at_label: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionTimelineProjection {
    pub records: Vec<LocalDecisionRecord>,
    pub empty_message: String,
}

impl LocalDecisionTimelineProjection {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionLedger {
    pub records: Vec<LocalDecisionRecord>,
}

pub fn initial_local_decision_ledger() -> LocalDecisionLedger {
    LocalDecisionLedger {
        records: Vec::new(),
    }
}

pub fn project_local_decision_timeline(
    ledger: &LocalDecisionLedger,
) -> LocalDecisionTimelineProjection {
    LocalDecisionTimelineProjection {
        records: ledger.records.clone(),
        empty_message: "No recorded local operator decisions".to_string(),
    }
}

pub fn next_local_decision_sequence(ledger: &LocalDecisionLedger) -> u64 {
    ledger.records.len() as u64 + 1
}

fn append_local_decision(
    ledger: &LocalDecisionLedger,
    intent: &LocalOperatorIntent,
    candidate_id: &str,
) -> Result<LocalDecisionLedger, LocalOperatorShellError> {
    if ledger
        .records
        .iter()
        .any(|record| record.run_id == intent.target_run_id && record.candidate_id == candidate_id)
    {
        return Err(LocalOperatorShellError::DuplicateDecisionRejected);
    }

    let recorded_sequence = next_local_decision_sequence(ledger);
    let mut next = ledger.clone();
    next.records.push(LocalDecisionRecord {
        decision_id: format!("local-decision-{recorded_sequence:04}"),
        run_id: intent.target_run_id.clone(),
        candidate_id: candidate_id.to_string(),
        operator_id: intent.operator_id.clone(),
        intent_kind: intent.kind.into(),
        decision_status: LocalDecisionRecordStatus::Recorded,
        validation_status: "accepted_by_rust_local_validation".to_string(),
        recorded_sequence,
        recorded_at_label: format!("local-sequence-{recorded_sequence:04}"),
        reason: intent.reason.clone(),
    });
    Ok(next)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutput {
    pub candidate_id: String,
    pub title: String,
    pub body: String,
    pub provider_kind: String,
    pub provider_output_trusted: bool,
    pub provider_execution_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalValidationProjection {
    pub validation_id: String,
    pub policy_status: String,
    pub validation_status: String,
    pub reason: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRunProjection {
    pub run_id: String,
    pub status: LocalRunStatus,
    pub bounded_context: Vec<String>,
    pub candidate: Option<LocalCandidateOutput>,
    pub validation: Option<LocalValidationProjection>,
    pub selected_intent: Option<LocalOperatorIntentKind>,
    pub timeline: Vec<String>,
    pub replay_status: String,
    pub decision_timeline: LocalDecisionTimelineProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellState {
    pub harness_status: String,
    pub non_production: bool,
    pub run: LocalRunProjection,
    pub decision_ledger: LocalDecisionLedger,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorIntent {
    pub kind: LocalOperatorIntentKind,
    pub operator_id: String,
    pub target_run_id: String,
    pub target_candidate_id: String,
    pub reason: String,
    pub requests_authority_grant: bool,
    pub requests_provider_execution: bool,
    pub claims_readiness: bool,
}

impl LocalOperatorIntent {
    pub fn new(
        kind: LocalOperatorIntentKind,
        operator_id: impl Into<String>,
        target_run_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            operator_id: operator_id.into(),
            target_run_id: target_run_id.into(),
            target_candidate_id: "candidate-local-stub-133".to_string(),
            reason: reason.into(),
            requests_authority_grant: false,
            requests_provider_execution: false,
            claims_readiness: false,
        }
    }
}

pub fn initial_local_operator_shell_state() -> LocalOperatorShellState {
    LocalOperatorShellState {
        harness_status: "idle_local_harness".to_string(),
        non_production: true,
        run: LocalRunProjection {
            run_id: "local-stub-run-133".to_string(),
            status: LocalRunStatus::Idle,
            bounded_context: Vec::new(),
            candidate: None,
            validation: None,
            selected_intent: None,
            timeline: vec!["idle local harness initialized".to_string()],
            replay_status: "placeholder: replay/status projection is typed but not executing"
                .to_string(),
            decision_timeline: project_local_decision_timeline(&initial_local_decision_ledger()),
        },
        decision_ledger: initial_local_decision_ledger(),
    }
}

pub fn start_deterministic_stub_run(state: &LocalOperatorShellState) -> LocalOperatorShellState {
    let mut next = state.clone();
    next.harness_status = "deterministic_stub_completed".to_string();
    next.run.status = LocalRunStatus::StubCompleted;
    next.run.bounded_context = vec![
        "phase=133".to_string(),
        "scope=local operator UI shell".to_string(),
        "provider=deterministic stub only".to_string(),
        "network=disabled".to_string(),
    ];
    next.run.candidate = Some(LocalCandidateOutput {
        candidate_id: "candidate-local-stub-133".to_string(),
        title: "Deterministic local stub candidate".to_string(),
        body: "AJENTIC local shell rendered a deterministic candidate from a Rust-owned projection fixture.".to_string(),
        provider_kind: "deterministic_stub".to_string(),
        provider_output_trusted: false,
        provider_execution_enabled: false,
    });
    next.run.validation = Some(LocalValidationProjection {
        validation_id: "validation-local-stub-133".to_string(),
        policy_status: "pass_for_local_stub_review".to_string(),
        validation_status: "pass_for_local_stub_review".to_string(),
        reason: "deterministic fixture satisfies local shell display checks only".to_string(),
        authority: "rust".to_string(),
    });
    next.run.timeline = vec![
        "idle local harness initialized".to_string(),
        "deterministic stub run started".to_string(),
        "candidate output projected".to_string(),
        "validation and policy projection completed".to_string(),
    ];
    next.run.decision_timeline = project_local_decision_timeline(&next.decision_ledger);
    next
}

pub fn apply_local_operator_intent(
    state: &LocalOperatorShellState,
    intent: LocalOperatorIntent,
) -> Result<LocalOperatorShellState, LocalOperatorShellError> {
    if state.run.status == LocalRunStatus::Idle {
        return Err(LocalOperatorShellError::RunNotStarted);
    }
    if intent.operator_id.is_empty() {
        return Err(LocalOperatorShellError::EmptyOperatorId);
    }
    if intent.reason.is_empty() {
        return Err(LocalOperatorShellError::EmptyReason);
    }
    if intent.target_run_id != state.run.run_id {
        return Err(LocalOperatorShellError::TargetMismatch);
    }
    let Some(candidate) = state.run.candidate.as_ref() else {
        return Err(LocalOperatorShellError::RunNotStarted);
    };
    if intent.target_candidate_id != candidate.candidate_id {
        return Err(LocalOperatorShellError::CandidateTargetMismatch);
    }
    if intent.requests_authority_grant {
        return Err(LocalOperatorShellError::AuthorityClaimRejected);
    }
    if intent.requests_provider_execution {
        return Err(LocalOperatorShellError::ProviderExecutionRejected);
    }
    if intent.claims_readiness {
        return Err(LocalOperatorShellError::ReadinessClaimRejected);
    }

    let next_ledger =
        append_local_decision(&state.decision_ledger, &intent, &candidate.candidate_id)?;

    let mut next = state.clone();
    next.decision_ledger = next_ledger;
    next.run.status = LocalRunStatus::IntentRecorded;
    next.run.selected_intent = Some(intent.kind);
    next.run.decision_timeline = project_local_decision_timeline(&next.decision_ledger);
    next.run.timeline.push(format!(
        "operator intent recorded: {} by {} as decision {}",
        intent.kind.code(),
        intent.operator_id,
        next.run
            .decision_timeline
            .records
            .last()
            .map(|record| record.decision_id.as_str())
            .unwrap_or("local-decision-missing")
    ));
    Ok(next)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellTransportStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellForbiddenRequest {
    AuthorityGrant,
    ProviderExecution,
    ReadinessClaim,
    ProductionStateMutation,
    ReleaseArtifactCreation,
    DeploymentEnablement,
    SigningEnablement,
    PublishingEnablement,
}

impl LocalOperatorShellForbiddenRequest {
    pub fn rejection_code(&self) -> &'static str {
        match self {
            Self::AuthorityGrant => "authority_grant_rejected",
            Self::ProviderExecution => "provider_execution_rejected",
            Self::ReadinessClaim => "readiness_claim_rejected",
            Self::ProductionStateMutation => "production_state_mutation_rejected",
            Self::ReleaseArtifactCreation => "release_artifact_creation_rejected",
            Self::DeploymentEnablement => "deployment_enablement_rejected",
            Self::SigningEnablement => "signing_enablement_rejected",
            Self::PublishingEnablement => "publishing_enablement_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalOperatorShellRequest {
    GetInitialState,
    GetCurrentState,
    StartDeterministicStubRun,
    SubmitOperatorIntent(LocalOperatorIntent),
    Forbidden(LocalOperatorShellForbiddenRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellCapabilities {
    pub local_only: bool,
    pub non_production: bool,
    pub provider_execution_enabled: bool,
    pub cloud_model_calls_enabled: bool,
    pub broad_command_execution_enabled: bool,
    pub production_persistence_enabled: bool,
    pub release_artifact_creation_enabled: bool,
    pub deployment_enabled: bool,
    pub signing_authority_available: bool,
    pub publishing_authority_available: bool,
    pub readiness_approval_enabled: bool,
}

impl LocalOperatorShellCapabilities {
    pub fn local_stub_only() -> Self {
        Self {
            local_only: true,
            non_production: true,
            provider_execution_enabled: false,
            cloud_model_calls_enabled: false,
            broad_command_execution_enabled: false,
            production_persistence_enabled: false,
            release_artifact_creation_enabled: false,
            deployment_enabled: false,
            signing_authority_available: false,
            publishing_authority_available: false,
            readiness_approval_enabled: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellResponse {
    pub status: LocalOperatorShellTransportStatus,
    pub reason: String,
    pub state: LocalOperatorShellState,
    pub capabilities: LocalOperatorShellCapabilities,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellTransport {
    state: LocalOperatorShellState,
}

impl LocalOperatorShellTransport {
    pub fn new() -> Self {
        Self {
            state: initial_local_operator_shell_state(),
        }
    }

    pub fn current_state(&self) -> LocalOperatorShellState {
        self.state.clone()
    }

    pub fn step(&mut self, request: LocalOperatorShellRequest) -> LocalOperatorShellResponse {
        let response = local_operator_shell_transport_step(&self.state, request);
        if response.status == LocalOperatorShellTransportStatus::Accepted {
            self.state = response.state.clone();
        }
        response
    }
}

impl Default for LocalOperatorShellTransport {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_local_operator_shell_state(
    transport: &LocalOperatorShellTransport,
) -> LocalOperatorShellState {
    transport.current_state()
}

pub fn start_local_operator_shell_stub_run(
    transport: &mut LocalOperatorShellTransport,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StartDeterministicStubRun)
}

pub fn submit_local_operator_shell_intent(
    transport: &mut LocalOperatorShellTransport,
    intent: LocalOperatorIntent,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitOperatorIntent(intent))
}

pub fn local_operator_shell_transport_step(
    state: &LocalOperatorShellState,
    request: LocalOperatorShellRequest,
) -> LocalOperatorShellResponse {
    match request {
        LocalOperatorShellRequest::GetInitialState => accepted(
            "initial_shell_state_returned",
            initial_local_operator_shell_state(),
        ),
        LocalOperatorShellRequest::GetCurrentState => {
            accepted("current_shell_state_returned", state.clone())
        }
        LocalOperatorShellRequest::StartDeterministicStubRun => accepted(
            "deterministic_stub_run_completed",
            start_deterministic_stub_run(state),
        ),
        LocalOperatorShellRequest::SubmitOperatorIntent(intent) => {
            match apply_local_operator_intent(state, intent) {
                Ok(next) => accepted("local_operator_intent_recorded", next),
                Err(err) => rejected(err.code(), state.clone()),
            }
        }
        LocalOperatorShellRequest::Forbidden(forbidden) => {
            rejected(forbidden.rejection_code(), state.clone())
        }
    }
}

fn accepted(
    reason: impl Into<String>,
    state: LocalOperatorShellState,
) -> LocalOperatorShellResponse {
    LocalOperatorShellResponse {
        status: LocalOperatorShellTransportStatus::Accepted,
        reason: reason.into(),
        state,
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}

fn rejected(
    reason: impl Into<String>,
    state: LocalOperatorShellState,
) -> LocalOperatorShellResponse {
    LocalOperatorShellResponse {
        status: LocalOperatorShellTransportStatus::Rejected,
        reason: reason.into(),
        state,
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_initial_state_returns_idle_non_production_projection() {
        let mut transport = LocalOperatorShellTransport::new();
        let response = transport.step(LocalOperatorShellRequest::GetInitialState);

        assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
        assert_eq!(response.reason, "initial_shell_state_returned");
        assert!(response.state.non_production);
        assert_eq!(response.state.run.status, LocalRunStatus::Idle);
        assert!(response.capabilities.local_only);
        assert!(!response.capabilities.provider_execution_enabled);
        assert!(response.state.decision_ledger.records.is_empty());
        assert!(response.state.run.decision_timeline.is_empty());
    }

    #[test]
    fn transport_stub_run_returns_deterministic_output() {
        let state = initial_local_operator_shell_state();
        let first = local_operator_shell_transport_step(
            &state,
            LocalOperatorShellRequest::StartDeterministicStubRun,
        );
        let second = local_operator_shell_transport_step(
            &state,
            LocalOperatorShellRequest::StartDeterministicStubRun,
        );

        assert_eq!(first, second);
        assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
        assert_eq!(first.state.run.status, LocalRunStatus::StubCompleted);
        assert_eq!(
            first.state.run.candidate.as_ref().unwrap().candidate_id,
            "candidate-local-stub-133"
        );
        assert!(first.state.decision_ledger.records.is_empty());
        assert!(first.state.run.decision_timeline.records.is_empty());
    }

    #[test]
    fn transport_records_approve_and_reject_intents() {
        for kind in [
            LocalOperatorIntentKind::Approve,
            LocalOperatorIntentKind::Reject,
        ] {
            let mut transport = LocalOperatorShellTransport::new();
            let started = start_local_operator_shell_stub_run(&mut transport);
            let response = submit_local_operator_shell_intent(
                &mut transport,
                LocalOperatorIntent::new(
                    kind,
                    "operator-local",
                    &started.state.run.run_id,
                    "reviewed locally",
                ),
            );

            assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
            assert_eq!(response.state.run.selected_intent, Some(kind));
            assert_eq!(response.state.run.decision_timeline.records.len(), 1);
            let record = &response.state.run.decision_timeline.records[0];
            assert_eq!(record.intent_kind, kind.into());
            assert_eq!(record.decision_status, LocalDecisionRecordStatus::Recorded);
            assert_eq!(record.recorded_sequence, 1);
            assert_eq!(record.decision_id, "local-decision-0001");
            assert!(response.state.run.timeline.iter().any(|entry| entry
                == &format!(
                "operator intent recorded: {} by operator-local as decision local-decision-0001",
                kind.code()
            )));
        }
    }

    #[test]
    fn transport_rejects_invalid_target_run_and_candidate() {
        let mut transport = LocalOperatorShellTransport::new();
        let started = start_local_operator_shell_stub_run(&mut transport);

        let invalid_run = submit_local_operator_shell_intent(
            &mut transport,
            LocalOperatorIntent::new(
                LocalOperatorIntentKind::Approve,
                "operator-local",
                "wrong-run",
                "reviewed locally",
            ),
        );
        assert_eq!(
            invalid_run.status,
            LocalOperatorShellTransportStatus::Rejected
        );
        assert_eq!(invalid_run.reason, "target_mismatch");

        let mut invalid_candidate = LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &started.state.run.run_id,
            "reviewed locally",
        );
        invalid_candidate.target_candidate_id = "wrong-candidate".to_string();
        let response = submit_local_operator_shell_intent(&mut transport, invalid_candidate);
        assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
        assert_eq!(response.reason, "candidate_target_mismatch");
        assert!(response.state.decision_ledger.records.is_empty());
        assert!(transport.current_state().decision_ledger.records.is_empty());
    }

    #[test]
    fn duplicate_decision_for_same_run_candidate_fails_closed() {
        let mut transport = LocalOperatorShellTransport::new();
        let started = start_local_operator_shell_stub_run(&mut transport);
        let first = submit_local_operator_shell_intent(
            &mut transport,
            LocalOperatorIntent::new(
                LocalOperatorIntentKind::Approve,
                "operator-local",
                &started.state.run.run_id,
                "reviewed locally",
            ),
        );
        assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
        assert_eq!(first.state.decision_ledger.records.len(), 1);

        let duplicate = submit_local_operator_shell_intent(
            &mut transport,
            LocalOperatorIntent::new(
                LocalOperatorIntentKind::Reject,
                "operator-local",
                &started.state.run.run_id,
                "duplicate local decision",
            ),
        );
        assert_eq!(
            duplicate.status,
            LocalOperatorShellTransportStatus::Rejected
        );
        assert_eq!(duplicate.reason, "duplicate_decision_rejected");
        assert_eq!(duplicate.state.decision_ledger.records.len(), 1);
        assert_eq!(transport.current_state().decision_ledger.records.len(), 1);
    }

    #[test]
    fn forbidden_transport_requests_fail_closed() {
        let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
        for request in [
            LocalOperatorShellForbiddenRequest::AuthorityGrant,
            LocalOperatorShellForbiddenRequest::ProviderExecution,
            LocalOperatorShellForbiddenRequest::ReadinessClaim,
            LocalOperatorShellForbiddenRequest::ProductionStateMutation,
            LocalOperatorShellForbiddenRequest::ReleaseArtifactCreation,
            LocalOperatorShellForbiddenRequest::DeploymentEnablement,
            LocalOperatorShellForbiddenRequest::SigningEnablement,
            LocalOperatorShellForbiddenRequest::PublishingEnablement,
        ] {
            let response = local_operator_shell_transport_step(
                &state,
                LocalOperatorShellRequest::Forbidden(request),
            );
            assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
            assert_eq!(response.reason, request.rejection_code());
            assert_eq!(response.state, state);
        }
    }

    #[test]
    fn transport_exposes_no_production_or_provider_authority() {
        let response = local_operator_shell_transport_step(
            &initial_local_operator_shell_state(),
            LocalOperatorShellRequest::GetCurrentState,
        );
        let capabilities = response.capabilities;

        assert!(capabilities.local_only);
        assert!(capabilities.non_production);
        assert!(!capabilities.provider_execution_enabled);
        assert!(!capabilities.cloud_model_calls_enabled);
        assert!(!capabilities.broad_command_execution_enabled);
        assert!(!capabilities.production_persistence_enabled);
        assert!(!capabilities.release_artifact_creation_enabled);
        assert!(!capabilities.deployment_enabled);
        assert!(!capabilities.signing_authority_available);
        assert!(!capabilities.publishing_authority_available);
        assert!(!capabilities.readiness_approval_enabled);
    }

    #[test]
    fn deterministic_stub_run_produces_identical_projection() {
        let state = initial_local_operator_shell_state();
        let first = start_deterministic_stub_run(&state);
        let second = start_deterministic_stub_run(&state);
        assert_eq!(first, second);
        assert_eq!(first.run.status, LocalRunStatus::StubCompleted);
        assert!(
            !first
                .run
                .candidate
                .as_ref()
                .unwrap()
                .provider_execution_enabled
        );
    }

    #[test]
    fn approve_and_reject_intents_are_typed_and_recorded() {
        for kind in [
            LocalOperatorIntentKind::Approve,
            LocalOperatorIntentKind::Reject,
        ] {
            let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
            let intent = LocalOperatorIntent::new(
                kind,
                "operator-local",
                &state.run.run_id,
                "reviewed locally",
            );
            let updated =
                apply_local_operator_intent(&state, intent).expect("typed intent should record");
            assert_eq!(updated.run.status, LocalRunStatus::IntentRecorded);
            assert_eq!(updated.run.selected_intent, Some(kind));
            assert_eq!(updated.decision_ledger.records.len(), 1);
            assert_eq!(updated.run.decision_timeline.records.len(), 1);
        }
    }

    #[test]
    fn invalid_operator_intent_fails_closed() {
        let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
        let err = apply_local_operator_intent(
            &state,
            LocalOperatorIntent::new(
                LocalOperatorIntentKind::Approve,
                "",
                &state.run.run_id,
                "reviewed locally",
            ),
        )
        .expect_err("empty operator should fail");
        assert_eq!(err, LocalOperatorShellError::EmptyOperatorId);
    }

    #[test]
    fn ui_boundary_cannot_grant_authority_or_readiness_or_provider_execution() {
        let state = start_deterministic_stub_run(&initial_local_operator_shell_state());

        let mut authority_intent = LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &state.run.run_id,
            "reviewed locally",
        );
        authority_intent.requests_authority_grant = true;
        assert_eq!(
            apply_local_operator_intent(&state, authority_intent)
                .expect_err("authority grant fails"),
            LocalOperatorShellError::AuthorityClaimRejected
        );

        let mut provider_intent = LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &state.run.run_id,
            "reviewed locally",
        );
        provider_intent.requests_provider_execution = true;
        assert_eq!(
            apply_local_operator_intent(&state, provider_intent)
                .expect_err("provider execution fails"),
            LocalOperatorShellError::ProviderExecutionRejected
        );

        let mut readiness_intent = LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &state.run.run_id,
            "reviewed locally",
        );
        readiness_intent.claims_readiness = true;
        assert_eq!(
            apply_local_operator_intent(&state, readiness_intent)
                .expect_err("readiness claim fails"),
            LocalOperatorShellError::ReadinessClaimRejected
        );
    }
}
