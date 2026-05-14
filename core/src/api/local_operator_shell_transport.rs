//! Local operator shell transport request/response routing.

use super::*;

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
    SubmitProviderConfiguration(LocalProviderConfigurationCandidate),
    SubmitProviderAdapterDeclaration(LocalProviderAdapterConfigurationCandidate),
    ExecuteProvider(LocalProviderExecutionRequest),
    RunProviderAdapterDryRun(LocalProviderAdapterDryRunRequest),
    InvokeConstrainedLocalProvider(ConstrainedLocalProviderInvocationRequest),
    CreateStagedCandidateConversionProposal(StagedCandidateConversionProposalRequest),
    ValidateStagedCandidateConversionProposal(StagedCandidateConversionValidationRequest),
    SubmitOperatorCandidateDecision(OperatorCandidateDecisionRequest),
    MaterializeLocalCandidateOutput(LocalCandidateMaterializationRequest),
    StartControlledInternalTrialExecution(ControlledInternalTrialExecutionRequest),
    StepControlledInternalTrialExecution(ControlledInternalTrialExecutionRequest),
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
    pub local_session_evidence_export: LocalSessionEvidenceExport,
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

pub fn submit_local_provider_configuration(
    transport: &mut LocalOperatorShellTransport,
    candidate: LocalProviderConfigurationCandidate,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitProviderConfiguration(
        candidate,
    ))
}

pub fn submit_local_provider_adapter_declaration(
    transport: &mut LocalOperatorShellTransport,
    candidate: LocalProviderAdapterConfigurationCandidate,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitProviderAdapterDeclaration(
        candidate,
    ))
}

pub fn execute_local_provider(
    transport: &mut LocalOperatorShellTransport,
    request: LocalProviderExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::ExecuteProvider(request))
}

pub fn run_local_provider_adapter_dry_run(
    transport: &mut LocalOperatorShellTransport,
    request: LocalProviderAdapterDryRunRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::RunProviderAdapterDryRun(request))
}

pub fn invoke_constrained_local_provider(
    transport: &mut LocalOperatorShellTransport,
    request: ConstrainedLocalProviderInvocationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::InvokeConstrainedLocalProvider(
        request,
    ))
}

pub fn create_local_staged_candidate_conversion_proposal(
    transport: &mut LocalOperatorShellTransport,
    request: StagedCandidateConversionProposalRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::CreateStagedCandidateConversionProposal(request))
}

pub fn validate_local_staged_candidate_conversion_proposal(
    transport: &mut LocalOperatorShellTransport,
    request: StagedCandidateConversionValidationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::ValidateStagedCandidateConversionProposal(request))
}

pub fn submit_local_operator_candidate_decision(
    transport: &mut LocalOperatorShellTransport,
    request: OperatorCandidateDecisionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitOperatorCandidateDecision(
        request,
    ))
}

pub fn request_local_candidate_materialization(
    transport: &mut LocalOperatorShellTransport,
    request: LocalCandidateMaterializationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::MaterializeLocalCandidateOutput(
        request,
    ))
}

pub fn request_controlled_internal_trial_execution_start(
    transport: &mut LocalOperatorShellTransport,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StartControlledInternalTrialExecution(request))
}

pub fn request_controlled_internal_trial_execution_step(
    transport: &mut LocalOperatorShellTransport,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StepControlledInternalTrialExecution(request))
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
        LocalOperatorShellRequest::SubmitProviderConfiguration(candidate) => {
            match apply_local_provider_configuration_candidate(state, candidate) {
                Ok(next) => accepted("local_provider_configuration_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::SubmitProviderAdapterDeclaration(candidate) => {
            match apply_local_provider_adapter_declaration(state, candidate) {
                Ok(next) => accepted("local_provider_adapter_declaration_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::ExecuteProvider(request) => {
            match apply_local_provider_execution(state, request) {
                Ok(next) => accepted("local_provider_execution_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::RunProviderAdapterDryRun(request) => {
            match apply_local_provider_adapter_dry_run(state, request) {
                Ok(next) => accepted("local_provider_adapter_dry_run_executed", next),
                Err(projection) => {
                    let response_state = if state.local_provider_adapter_dry_run.status
                        == LocalProviderAdapterDryRunStatus::DryRunExecuted
                    {
                        state.clone()
                    } else {
                        let mut rejected_state = state.clone();
                        rejected_state.local_provider_adapter_dry_run = *projection;
                        rejected_state
                    };
                    rejected("local_provider_adapter_dry_run_rejected", response_state)
                }
            }
        }
        LocalOperatorShellRequest::InvokeConstrainedLocalProvider(request) => {
            match execute_constrained_local_provider_invocation(state, request) {
                Ok(next) => accepted("constrained_local_provider_invocation_executed", next),
                Err(projection) => {
                    let response_state = if state.constrained_local_provider_invocation.status
                        == ConstrainedLocalProviderInvocationStatus::InvocationExecuted
                    {
                        state.clone()
                    } else {
                        let mut rejected_state = state.clone();
                        rejected_state.constrained_local_provider_invocation = *projection;
                        rejected_state.local_provider_output_pipeline =
                            derive_local_provider_output_pipeline_projection(&rejected_state);
                        rejected_state
                    };
                    rejected(
                        "constrained_local_provider_invocation_rejected",
                        response_state,
                    )
                }
            }
        }
        LocalOperatorShellRequest::CreateStagedCandidateConversionProposal(request) => {
            match create_staged_candidate_conversion_proposal(state, request) {
                Ok(next) => accepted("staged_candidate_conversion_proposal_created", next),
                Err(error) => rejected(error.code(), state.clone()),
            }
        }
        LocalOperatorShellRequest::ValidateStagedCandidateConversionProposal(request) => {
            let mut next =
                validate_staged_candidate_conversion_proposal_for_phase_147(state, request);
            next.phase_150_code_production_handoff =
                derive_phase_150_code_production_handoff(&next);
            if next.staged_candidate_conversion_validation.status
                == StagedCandidateConversionValidationStatus::StagedProposalShapeValid
            {
                accepted("staged_candidate_conversion_validation_completed", next)
            } else {
                rejected("staged_candidate_conversion_validation_rejected", next)
            }
        }
        LocalOperatorShellRequest::SubmitOperatorCandidateDecision(request) => {
            match submit_operator_candidate_decision(state, request) {
                Ok(next) => accepted("operator_candidate_decision_recorded", next),
                Err(error) => {
                    let mut response_state = state.clone();
                    response_state.operator_candidate_decision =
                        rejected_operator_candidate_decision_projection(error);
                    response_state.phase_150_code_production_handoff =
                        derive_phase_150_code_production_handoff(&response_state);
                    rejected(error.code(), response_state)
                }
            }
        }
        LocalOperatorShellRequest::MaterializeLocalCandidateOutput(request) => {
            match materialize_local_candidate_output(state, request) {
                Ok(next) => accepted("local_candidate_materialized", next),
                Err(error) => {
                    let mut response_state = state.clone();
                    response_state.local_candidate_output = reject_local_candidate_materialization(
                        &state.local_candidate_output,
                        error,
                    );
                    rejected(error.code(), response_state)
                }
            }
        }
        LocalOperatorShellRequest::StartControlledInternalTrialExecution(request) => {
            let next = start_controlled_internal_trial_execution(state, request);
            if matches!(
                next.controlled_internal_trial_execution.status,
                ControlledInternalTrialRunStatus::TrialRunRejected
                    | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
                    | ControlledInternalTrialRunStatus::TrialRunBlocked
            ) {
                rejected("controlled_internal_trial_execution_rejected", next)
            } else {
                accepted("controlled_internal_trial_execution_started", next)
            }
        }
        LocalOperatorShellRequest::StepControlledInternalTrialExecution(request) => {
            let next = step_controlled_internal_trial_execution(state, request);
            if matches!(
                next.controlled_internal_trial_execution.status,
                ControlledInternalTrialRunStatus::TrialRunRejected
                    | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
                    | ControlledInternalTrialRunStatus::TrialRunBlocked
            ) {
                rejected("controlled_internal_trial_execution_step_rejected", next)
            } else {
                accepted("controlled_internal_trial_execution_stepped", next)
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
        local_session_evidence_export: state.local_session_evidence_export.clone(),
        state: attach_local_session_evidence_export(state),
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
        local_session_evidence_export: state.local_session_evidence_export.clone(),
        state: attach_local_session_evidence_export(state),
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}
