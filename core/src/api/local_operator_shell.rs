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
            Self::AuthorityClaimRejected => "authority_claim_rejected",
            Self::ProviderExecutionRejected => "provider_execution_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
        }
    }
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellState {
    pub harness_status: String,
    pub non_production: bool,
    pub run: LocalRunProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorIntent {
    pub kind: LocalOperatorIntentKind,
    pub operator_id: String,
    pub target_run_id: String,
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
        },
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
    if intent.requests_authority_grant {
        return Err(LocalOperatorShellError::AuthorityClaimRejected);
    }
    if intent.requests_provider_execution {
        return Err(LocalOperatorShellError::ProviderExecutionRejected);
    }
    if intent.claims_readiness {
        return Err(LocalOperatorShellError::ReadinessClaimRejected);
    }

    let mut next = state.clone();
    next.run.status = LocalRunStatus::IntentRecorded;
    next.run.selected_intent = Some(intent.kind);
    next.run.timeline.push(format!(
        "operator intent recorded: {} by {}",
        intent.kind.code(),
        intent.operator_id
    ));
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::*;

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
