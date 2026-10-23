use std::collections::HashSet;

use crate::integrity::Digest;
use crate::task::{SuccessCriterion, TaskContract};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStatus {
    Succeeded,
    Partial,
    Failed,
    Blocked,
    Unknown,
    NotAttempted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolReturnStatus {
    Succeeded,
    Failed,
    TimedOut,
    Blocked,
    Unknown,
    NotAttempted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostconditionObservationState {
    Observed,
    Failed,
    Unknown,
    NotChecked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostconditionResultStatus {
    Satisfied,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriterionStatus {
    Satisfied,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostconditionObservation {
    pub id: String,
    pub state: PostconditionObservationState,
    pub observed_value: Option<String>,
    pub evidence_refs: Vec<String>,
    pub evidence_digests: Vec<Digest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutcomeInput {
    pub action_id: String,
    pub task_id: String,
    pub run_id: String,
    pub action: String,
    pub tool: String,
    pub argument_digest: Digest,
    pub target: String,
    pub recipient: Option<String>,
    pub tool_return_status: ToolReturnStatus,
    pub observed_effect: Option<String>,
    pub postcondition_observations: Vec<PostconditionObservation>,
    pub exact_errors: Vec<String>,
    pub partial_side_effects: Vec<String>,
    pub retries: u32,
    pub compensation: Vec<String>,
    pub remaining_uncertainty: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub evidence_digests: Vec<Digest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutcome {
    input: ActionOutcomeInput,
    status: ActionStatus,
    postcondition_results: Vec<PostconditionResult>,
    unresolved_required_postcondition_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostconditionResult {
    postcondition_id: String,
    status: PostconditionResultStatus,
    evidence_refs: Vec<String>,
    evidence_digests: Vec<Digest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriterionOutcome {
    criterion_id: String,
    status: CriterionStatus,
    supporting_postcondition_ids: Vec<String>,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimSupportStatus {
    Supported,
    Contradicted,
    Unverified,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimEvidence {
    pub supported: bool,
    pub contradicted: bool,
    pub not_applicable: bool,
    pub evidence_refs: Vec<String>,
    pub source_identity: String,
    pub source_version: String,
    pub verifier_id: String,
    pub verifier_version: String,
    pub assumptions: Vec<String>,
    pub contradictions: Vec<String>,
    pub uncertainty_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimReport {
    claim_id: String,
    material_claim: String,
    claim_type: String,
    material: bool,
    support_status: ClaimSupportStatus,
    evidence: ClaimEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeRunResult {
    status: ActionStatus,
    actions: Vec<ActionOutcome>,
    claims: Vec<ClaimReport>,
    criterion_outcomes: Vec<CriterionOutcome>,
    material_errors: Vec<String>,
    partial_side_effects: Vec<String>,
    unmet_success_criteria: Vec<String>,
    unresolved_uncertainty: Vec<String>,
    summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeError {
    EmptyActionId,
    TaskMismatch,
    EmptyRunId,
    ActionNotPermitted,
    ToolNotPermitted,
    EmptyTarget,
    MissingPostconditions,
    EmptyPostconditionObservationId,
    UnknownPostconditionObservation,
    DuplicatePostconditionObservation,
    EmptyEvidenceReference,
    MissingEvidenceDigest,
    EvidenceBindingMismatch,
    MissingObservedValue,
    EmptyClaimId,
    EmptyClaimText,
    EmptyClaimType,
    MissingSourceIdentity,
    MissingVerifierIdentity,
}

impl OutcomeError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EmptyActionId => "empty_action_id",
            Self::TaskMismatch => "task_mismatch",
            Self::EmptyRunId => "empty_run_id",
            Self::ActionNotPermitted => "action_not_permitted",
            Self::ToolNotPermitted => "tool_not_permitted",
            Self::EmptyTarget => "empty_target",
            Self::MissingPostconditions => "missing_postconditions",
            Self::EmptyPostconditionObservationId => "empty_postcondition_observation_id",
            Self::UnknownPostconditionObservation => "unknown_postcondition_observation",
            Self::DuplicatePostconditionObservation => "duplicate_postcondition_observation",
            Self::EmptyEvidenceReference => "empty_evidence_reference",
            Self::MissingEvidenceDigest => "missing_evidence_digest",
            Self::EvidenceBindingMismatch => "evidence_binding_mismatch",
            Self::MissingObservedValue => "missing_observed_value",
            Self::EmptyClaimId => "empty_claim_id",
            Self::EmptyClaimText => "empty_claim_text",
            Self::EmptyClaimType => "empty_claim_type",
            Self::MissingSourceIdentity => "missing_source_identity",
            Self::MissingVerifierIdentity => "missing_verifier_identity",
        }
    }
}

impl ActionOutcome {
    pub fn status(&self) -> ActionStatus {
        self.status
    }

    pub fn input(&self) -> &ActionOutcomeInput {
        &self.input
    }

    pub fn postcondition_results(&self) -> &[PostconditionResult] {
        &self.postcondition_results
    }

    pub fn unresolved_required_postcondition_ids(&self) -> &[String] {
        &self.unresolved_required_postcondition_ids
    }
}

impl PostconditionResult {
    pub fn postcondition_id(&self) -> &str {
        &self.postcondition_id
    }

    pub fn status(&self) -> PostconditionResultStatus {
        self.status
    }

    pub fn evidence_refs(&self) -> &[String] {
        &self.evidence_refs
    }

    pub fn evidence_digests(&self) -> &[Digest] {
        &self.evidence_digests
    }
}

impl CriterionOutcome {
    pub fn criterion_id(&self) -> &str {
        &self.criterion_id
    }

    pub fn status(&self) -> CriterionStatus {
        self.status
    }

    pub fn supporting_postcondition_ids(&self) -> &[String] {
        &self.supporting_postcondition_ids
    }

    pub fn evidence_refs(&self) -> &[String] {
        &self.evidence_refs
    }
}

impl ClaimReport {
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn support_status(&self) -> ClaimSupportStatus {
        self.support_status
    }

    pub fn material(&self) -> bool {
        self.material
    }

    pub fn material_claim(&self) -> &str {
        &self.material_claim
    }

    pub fn claim_type(&self) -> &str {
        &self.claim_type
    }

    pub fn evidence(&self) -> &ClaimEvidence {
        &self.evidence
    }
}

impl AuthoritativeRunResult {
    pub fn status(&self) -> ActionStatus {
        self.status
    }

    pub fn actions(&self) -> &[ActionOutcome] {
        &self.actions
    }

    pub fn claims(&self) -> &[ClaimReport] {
        &self.claims
    }

    pub fn criterion_outcomes(&self) -> &[CriterionOutcome] {
        &self.criterion_outcomes
    }

    pub fn criterion_outcome(&self, id: &str) -> Option<&CriterionOutcome> {
        self.criterion_outcomes
            .iter()
            .find(|outcome| outcome.criterion_id == id)
    }

    pub fn material_errors(&self) -> &[String] {
        &self.material_errors
    }

    pub fn partial_side_effects(&self) -> &[String] {
        &self.partial_side_effects
    }

    pub fn unmet_success_criteria(&self) -> &[String] {
        &self.unmet_success_criteria
    }

    pub fn unresolved_uncertainty(&self) -> &[String] {
        &self.unresolved_uncertainty
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }
}

pub fn evaluate_action_outcome(
    task: &TaskContract,
    input: ActionOutcomeInput,
) -> Result<ActionOutcome, OutcomeError> {
    validate_action_input(task, &input)?;
    let results = derive_postcondition_results(&input.postcondition_observations);
    let unresolved = unresolved_required_postconditions(task, &results);
    let status = determine_action_status(task, &input, &results);
    Ok(ActionOutcome {
        input,
        status,
        postcondition_results: results,
        unresolved_required_postcondition_ids: unresolved,
    })
}

pub fn evaluate_claim(
    claim_id: impl Into<String>,
    material_claim: impl Into<String>,
    claim_type: impl Into<String>,
    material: bool,
    evidence: ClaimEvidence,
) -> Result<ClaimReport, OutcomeError> {
    let fields = (claim_id.into(), material_claim.into(), claim_type.into());
    validate_claim_input(&fields, &evidence)?;
    let support_status = determine_claim_status(&evidence);
    Ok(ClaimReport {
        claim_id: fields.0,
        material_claim: fields.1,
        claim_type: fields.2,
        material,
        support_status,
        evidence,
    })
}

pub fn assemble_authoritative_run_result(
    task: &TaskContract,
    actions: Vec<ActionOutcome>,
    claims: Vec<ClaimReport>,
) -> AuthoritativeRunResult {
    let criterion_outcomes = derive_criterion_outcomes(task, &actions);
    let material_errors = collect_errors(&actions);
    let partial_side_effects = collect_partial_side_effects(&actions);
    let unmet_success_criteria = find_unmet_criteria(task, &criterion_outcomes);
    let unresolved_uncertainty = collect_uncertainty(&actions, &claims);
    let status = determine_run_status(
        task,
        &actions,
        &claims,
        &criterion_outcomes,
        &partial_side_effects,
        &unresolved_uncertainty,
    );
    let summary = build_run_summary(
        status,
        &criterion_outcomes,
        &material_errors,
        &partial_side_effects,
        &unmet_success_criteria,
        &unresolved_uncertainty,
    );
    AuthoritativeRunResult {
        status,
        actions,
        claims,
        criterion_outcomes,
        material_errors,
        partial_side_effects,
        unmet_success_criteria,
        unresolved_uncertainty,
        summary,
    }
}

fn validate_action_input(
    task: &TaskContract,
    input: &ActionOutcomeInput,
) -> Result<(), OutcomeError> {
    validate_action_identity(task, input)?;
    validate_action_scope(task, input)?;
    validate_action_evidence(input)?;
    validate_postcondition_observations(task, input)
}

fn validate_action_identity(
    task: &TaskContract,
    input: &ActionOutcomeInput,
) -> Result<(), OutcomeError> {
    validate_nonempty(&input.action_id, OutcomeError::EmptyActionId)?;
    if input.task_id != task.task_id() {
        return Err(OutcomeError::TaskMismatch);
    }
    validate_nonempty(&input.run_id, OutcomeError::EmptyRunId)
}

fn validate_action_scope(
    task: &TaskContract,
    input: &ActionOutcomeInput,
) -> Result<(), OutcomeError> {
    if !task.permits_action(&input.action) {
        return Err(OutcomeError::ActionNotPermitted);
    }
    if !task.permits_tool(&input.tool) {
        return Err(OutcomeError::ToolNotPermitted);
    }
    validate_nonempty(&input.target, OutcomeError::EmptyTarget)
}

fn validate_action_evidence(input: &ActionOutcomeInput) -> Result<(), OutcomeError> {
    validate_evidence_refs(&input.evidence_refs)?;
    if input.evidence_digests.is_empty() {
        return Err(OutcomeError::MissingEvidenceDigest);
    }
    if input.evidence_refs.len() != input.evidence_digests.len() {
        return Err(OutcomeError::EvidenceBindingMismatch);
    }
    Ok(())
}

fn validate_postcondition_observations(
    task: &TaskContract,
    input: &ActionOutcomeInput,
) -> Result<(), OutcomeError> {
    if input.postcondition_observations.is_empty() {
        return Err(OutcomeError::MissingPostconditions);
    }
    validate_unique_observation_ids(&input.postcondition_observations)?;
    for observation in &input.postcondition_observations {
        validate_postcondition_observation(task, input, observation)?;
    }
    Ok(())
}

fn validate_unique_observation_ids(
    observations: &[PostconditionObservation],
) -> Result<(), OutcomeError> {
    let ids = observations
        .iter()
        .map(|observation| observation.id.as_str())
        .collect::<Vec<_>>();
    if ids.iter().copied().collect::<HashSet<_>>().len() == ids.len() {
        return Ok(());
    }
    Err(OutcomeError::DuplicatePostconditionObservation)
}

fn validate_postcondition_observation(
    task: &TaskContract,
    input: &ActionOutcomeInput,
    observation: &PostconditionObservation,
) -> Result<(), OutcomeError> {
    validate_nonempty(
        &observation.id,
        OutcomeError::EmptyPostconditionObservationId,
    )?;
    if task.postcondition(&observation.id).is_none() {
        return Err(OutcomeError::UnknownPostconditionObservation);
    }
    validate_evidence_refs(&observation.evidence_refs)?;
    validate_observation_evidence_binding(input, observation)?;
    validate_observed_state(observation)
}

fn validate_observation_evidence_binding(
    input: &ActionOutcomeInput,
    observation: &PostconditionObservation,
) -> Result<(), OutcomeError> {
    if observation.evidence_refs.len() != observation.evidence_digests.len() {
        return Err(OutcomeError::EvidenceBindingMismatch);
    }
    if observation
        .evidence_refs
        .iter()
        .all(|item| input.evidence_refs.contains(item))
        && observation
            .evidence_digests
            .iter()
            .all(|item| input.evidence_digests.contains(item))
    {
        return Ok(());
    }
    Err(OutcomeError::EvidenceBindingMismatch)
}

fn validate_observed_state(observation: &PostconditionObservation) -> Result<(), OutcomeError> {
    if observation.state != PostconditionObservationState::Observed {
        return Ok(());
    }
    let value = observation
        .observed_value
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or(OutcomeError::MissingObservedValue)?;
    if observation.evidence_refs.is_empty() || observation.evidence_digests.is_empty() {
        return Err(OutcomeError::MissingEvidenceDigest);
    }
    if observation
        .evidence_digests
        .contains(&Digest::of_text(value))
    {
        return Ok(());
    }
    Err(OutcomeError::EvidenceBindingMismatch)
}

fn validate_evidence_refs(items: &[String]) -> Result<(), OutcomeError> {
    if items.iter().any(|item| item.trim().is_empty()) {
        return Err(OutcomeError::EmptyEvidenceReference);
    }
    Ok(())
}

fn derive_postcondition_results(
    observations: &[PostconditionObservation],
) -> Vec<PostconditionResult> {
    observations
        .iter()
        .map(derive_postcondition_result)
        .collect()
}

fn derive_postcondition_result(observation: &PostconditionObservation) -> PostconditionResult {
    PostconditionResult {
        postcondition_id: observation.id.clone(),
        status: observation_result_status(observation.state),
        evidence_refs: observation.evidence_refs.clone(),
        evidence_digests: observation.evidence_digests.clone(),
    }
}

fn observation_result_status(state: PostconditionObservationState) -> PostconditionResultStatus {
    match state {
        PostconditionObservationState::Observed => PostconditionResultStatus::Satisfied,
        PostconditionObservationState::Failed => PostconditionResultStatus::Failed,
        PostconditionObservationState::Unknown | PostconditionObservationState::NotChecked => {
            PostconditionResultStatus::Unknown
        }
    }
}

fn unresolved_required_postconditions(
    task: &TaskContract,
    results: &[PostconditionResult],
) -> Vec<String> {
    task.expected_postconditions()
        .iter()
        .filter(|postcondition| postcondition.required)
        .filter(|postcondition| !postcondition_is_satisfied(&postcondition.id, results))
        .map(|postcondition| postcondition.id.clone())
        .collect()
}

fn determine_action_status(
    task: &TaskContract,
    input: &ActionOutcomeInput,
    results: &[PostconditionResult],
) -> ActionStatus {
    match input.tool_return_status {
        ToolReturnStatus::NotAttempted => ActionStatus::NotAttempted,
        ToolReturnStatus::Blocked => ActionStatus::Blocked,
        ToolReturnStatus::Failed => failed_action_status(input),
        ToolReturnStatus::TimedOut | ToolReturnStatus::Unknown => ActionStatus::Unknown,
        ToolReturnStatus::Succeeded => successful_tool_status(task, input, results),
    }
}

fn successful_tool_status(
    task: &TaskContract,
    input: &ActionOutcomeInput,
    results: &[PostconditionResult],
) -> ActionStatus {
    if required_result_failed(task, results) {
        return failed_action_status(input);
    }
    if required_result_unresolved(task, results) {
        return ActionStatus::Unknown;
    }
    if input.observed_effect.is_none() {
        return ActionStatus::Unknown;
    }
    ActionStatus::Succeeded
}

fn failed_action_status(input: &ActionOutcomeInput) -> ActionStatus {
    if input.observed_effect.is_some() || !input.partial_side_effects.is_empty() {
        return ActionStatus::Partial;
    }
    ActionStatus::Failed
}

fn required_result_failed(task: &TaskContract, results: &[PostconditionResult]) -> bool {
    task.expected_postconditions()
        .iter()
        .filter(|postcondition| postcondition.required)
        .any(|postcondition| {
            postcondition_result(&postcondition.id, results)
                .map(|result| result.status == PostconditionResultStatus::Failed)
                .unwrap_or(false)
        })
}

fn required_result_unresolved(task: &TaskContract, results: &[PostconditionResult]) -> bool {
    task.expected_postconditions()
        .iter()
        .filter(|postcondition| postcondition.required)
        .any(|postcondition| !postcondition_is_satisfied(&postcondition.id, results))
}

fn postcondition_is_satisfied(id: &str, results: &[PostconditionResult]) -> bool {
    postcondition_result(id, results)
        .map(|result| result.status == PostconditionResultStatus::Satisfied)
        .unwrap_or(false)
}

fn postcondition_result<'a>(
    id: &str,
    results: &'a [PostconditionResult],
) -> Option<&'a PostconditionResult> {
    results.iter().find(|result| result.postcondition_id == id)
}

fn derive_criterion_outcomes(
    task: &TaskContract,
    actions: &[ActionOutcome],
) -> Vec<CriterionOutcome> {
    task.success_criteria()
        .iter()
        .map(|criterion| derive_criterion_outcome(criterion, actions))
        .collect()
}

fn derive_criterion_outcome(
    criterion: &SuccessCriterion,
    actions: &[ActionOutcome],
) -> CriterionOutcome {
    let results = criterion
        .required_postcondition_ids
        .iter()
        .map(|id| aggregate_postcondition(id, actions))
        .collect::<Vec<_>>();
    CriterionOutcome {
        criterion_id: criterion.id.clone(),
        status: criterion_status(&results),
        supporting_postcondition_ids: satisfied_postcondition_ids(&results),
        evidence_refs: aggregated_evidence_refs(&results),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AggregatedPostconditionResult {
    postcondition_id: String,
    status: PostconditionResultStatus,
    evidence_refs: Vec<String>,
}

fn aggregate_postcondition(id: &str, actions: &[ActionOutcome]) -> AggregatedPostconditionResult {
    let observations = action_postcondition_results(id, actions);
    let Some(max_retry) = observations.iter().map(|item| item.0).max() else {
        return unknown_aggregated_postcondition(id);
    };
    let current = observations
        .into_iter()
        .filter(|item| item.0 == max_retry)
        .map(|item| item.1)
        .collect::<Vec<_>>();
    aggregate_current_results(id, &current)
}

fn action_postcondition_results<'a>(
    id: &str,
    actions: &'a [ActionOutcome],
) -> Vec<(u32, &'a PostconditionResult)> {
    actions
        .iter()
        .filter_map(|outcome| {
            postcondition_result(id, &outcome.postcondition_results)
                .map(|result| (outcome.input.retries, result))
        })
        .collect()
}

fn aggregate_current_results(
    id: &str,
    results: &[&PostconditionResult],
) -> AggregatedPostconditionResult {
    let statuses = results
        .iter()
        .map(|item| item.status)
        .collect::<HashSet<_>>();
    if statuses.len() != 1 {
        return unknown_aggregated_postcondition(id);
    }
    AggregatedPostconditionResult {
        postcondition_id: id.to_string(),
        status: results[0].status,
        evidence_refs: unique_evidence_refs(results),
    }
}

fn unknown_aggregated_postcondition(id: &str) -> AggregatedPostconditionResult {
    AggregatedPostconditionResult {
        postcondition_id: id.to_string(),
        status: PostconditionResultStatus::Unknown,
        evidence_refs: Vec::new(),
    }
}

fn unique_evidence_refs(results: &[&PostconditionResult]) -> Vec<String> {
    let mut seen = HashSet::new();
    results
        .iter()
        .flat_map(|result| result.evidence_refs.iter())
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

fn criterion_status(results: &[AggregatedPostconditionResult]) -> CriterionStatus {
    if results
        .iter()
        .any(|result| result.status == PostconditionResultStatus::Failed)
    {
        return CriterionStatus::Failed;
    }
    if results
        .iter()
        .any(|result| result.status != PostconditionResultStatus::Satisfied)
    {
        return CriterionStatus::Unknown;
    }
    CriterionStatus::Satisfied
}

fn satisfied_postcondition_ids(results: &[AggregatedPostconditionResult]) -> Vec<String> {
    results
        .iter()
        .filter(|result| result.status == PostconditionResultStatus::Satisfied)
        .map(|result| result.postcondition_id.clone())
        .collect()
}

fn aggregated_evidence_refs(results: &[AggregatedPostconditionResult]) -> Vec<String> {
    let mut seen = HashSet::new();
    results
        .iter()
        .flat_map(|result| result.evidence_refs.iter())
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

fn find_unmet_criteria(task: &TaskContract, outcomes: &[CriterionOutcome]) -> Vec<String> {
    task.success_criteria()
        .iter()
        .filter(|criterion| criterion.required)
        .filter(|criterion| !criterion_is_satisfied(&criterion.id, outcomes))
        .map(|criterion| criterion.id.clone())
        .collect()
}

fn criterion_is_satisfied(id: &str, outcomes: &[CriterionOutcome]) -> bool {
    outcomes
        .iter()
        .find(|outcome| outcome.criterion_id == id)
        .map(|outcome| outcome.status == CriterionStatus::Satisfied)
        .unwrap_or(false)
}

fn collect_errors(actions: &[ActionOutcome]) -> Vec<String> {
    actions
        .iter()
        .flat_map(|outcome| outcome.input.exact_errors.clone())
        .collect()
}

fn collect_partial_side_effects(actions: &[ActionOutcome]) -> Vec<String> {
    actions
        .iter()
        .flat_map(|outcome| outcome.input.partial_side_effects.clone())
        .collect()
}

fn collect_uncertainty(actions: &[ActionOutcome], claims: &[ClaimReport]) -> Vec<String> {
    let mut uncertainty = actions
        .iter()
        .flat_map(|outcome| outcome.input.remaining_uncertainty.clone())
        .collect::<Vec<_>>();
    uncertainty.extend(claim_uncertainty(claims));
    uncertainty
}

fn claim_uncertainty(claims: &[ClaimReport]) -> Vec<String> {
    claims
        .iter()
        .filter(|claim| claim.support_status == ClaimSupportStatus::Unverified)
        .map(|claim| format!("claim:{}:unverified", claim.claim_id))
        .collect()
}

fn determine_run_status(
    task: &TaskContract,
    actions: &[ActionOutcome],
    claims: &[ClaimReport],
    criteria: &[CriterionOutcome],
    partial_side_effects: &[String],
    uncertainty: &[String],
) -> ActionStatus {
    if !partial_side_effects.is_empty() || actions.iter().any(is_partial) {
        return ActionStatus::Partial;
    }
    if has_unverified_material_claim(claims) || !uncertainty.is_empty() {
        return ActionStatus::Unknown;
    }
    if required_criterion_failed(task, criteria) {
        return ActionStatus::Failed;
    }
    if required_criterion_unknown(task, criteria) || has_noncompletion_action_unknown(actions) {
        return ActionStatus::Unknown;
    }
    if actions.iter().any(is_blocked) {
        return ActionStatus::Blocked;
    }
    ActionStatus::Succeeded
}

fn required_criterion_failed(task: &TaskContract, outcomes: &[CriterionOutcome]) -> bool {
    required_criteria(task, outcomes).any(|outcome| outcome.status == CriterionStatus::Failed)
}

fn required_criterion_unknown(task: &TaskContract, outcomes: &[CriterionOutcome]) -> bool {
    required_criteria(task, outcomes).any(|outcome| outcome.status == CriterionStatus::Unknown)
}

fn required_criteria<'a>(
    task: &'a TaskContract,
    outcomes: &'a [CriterionOutcome],
) -> impl Iterator<Item = &'a CriterionOutcome> {
    outcomes.iter().filter(|outcome| {
        task.success_criteria()
            .iter()
            .any(|criterion| criterion.id == outcome.criterion_id && criterion.required)
    })
}

fn has_noncompletion_action_unknown(actions: &[ActionOutcome]) -> bool {
    actions.iter().any(|outcome| {
        outcome.status == ActionStatus::Unknown
            && outcome.input.tool_return_status != ToolReturnStatus::Succeeded
    })
}

fn has_unverified_material_claim(claims: &[ClaimReport]) -> bool {
    claims.iter().any(|claim| {
        claim.material
            && !matches!(
                claim.support_status,
                ClaimSupportStatus::Supported | ClaimSupportStatus::NotApplicable
            )
    })
}

fn is_partial(outcome: &ActionOutcome) -> bool {
    outcome.status == ActionStatus::Partial
}

fn is_blocked(outcome: &ActionOutcome) -> bool {
    outcome.status == ActionStatus::Blocked
}

fn build_run_summary(
    status: ActionStatus,
    criteria: &[CriterionOutcome],
    errors: &[String],
    side_effects: &[String],
    unmet: &[String],
    uncertainty: &[String],
) -> String {
    format!(
        "status={status:?}; criterion_outcomes={}; material_errors={}; partial_side_effects={}; unmet_success_criteria={}; unresolved_uncertainty={}",
        criterion_summary(criteria),
        errors.join("|"),
        side_effects.join("|"),
        unmet.join("|"),
        uncertainty.join("|")
    )
}

fn criterion_summary(criteria: &[CriterionOutcome]) -> String {
    criteria
        .iter()
        .map(|outcome| format!("{}:{:?}", outcome.criterion_id, outcome.status))
        .collect::<Vec<_>>()
        .join("|")
}

fn validate_claim_input(
    fields: &(String, String, String),
    evidence: &ClaimEvidence,
) -> Result<(), OutcomeError> {
    validate_nonempty(&fields.0, OutcomeError::EmptyClaimId)?;
    validate_nonempty(&fields.1, OutcomeError::EmptyClaimText)?;
    validate_nonempty(&fields.2, OutcomeError::EmptyClaimType)?;
    validate_claim_attribution(evidence)
}

fn validate_claim_attribution(evidence: &ClaimEvidence) -> Result<(), OutcomeError> {
    if evidence.source_identity.trim().is_empty() || evidence.source_version.trim().is_empty() {
        return Err(OutcomeError::MissingSourceIdentity);
    }
    if evidence.verifier_id.trim().is_empty() || evidence.verifier_version.trim().is_empty() {
        return Err(OutcomeError::MissingVerifierIdentity);
    }
    Ok(())
}

fn determine_claim_status(evidence: &ClaimEvidence) -> ClaimSupportStatus {
    if evidence.not_applicable {
        return ClaimSupportStatus::NotApplicable;
    }
    if evidence.contradicted || !evidence.contradictions.is_empty() {
        return ClaimSupportStatus::Contradicted;
    }
    if evidence.supported && !evidence.evidence_refs.is_empty() {
        return ClaimSupportStatus::Supported;
    }
    ClaimSupportStatus::Unverified
}

fn validate_nonempty(value: &str, error: OutcomeError) -> Result<(), OutcomeError> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{PostconditionRequirement, RetryPolicy, SuccessCriterion, TaskContractInput};

    fn task() -> TaskContract {
        TaskContract::new(TaskContractInput {
            task_id: "task-1".into(),
            objective: "write verified file".into(),
            success_criteria: vec![SuccessCriterion {
                id: "criterion-file".into(),
                description: "file verified".into(),
                required: true,
                required_postcondition_ids: vec!["post-file".into()],
            }],
            forbidden_outcomes: vec!["outside write".into()],
            permitted_actions: vec!["write_file".into()],
            permitted_tools: vec!["filesystem".into()],
            non_goals: vec!["deploy".into()],
            required_approval_points: vec!["before_write".into()],
            side_effect_budget: 1,
            retry_policy: RetryPolicy {
                max_attempts: 2,
                require_idempotency_after_possible_side_effect: true,
            },
            stop_conditions: vec!["criterion_satisfied".into()],
            expected_postconditions: vec![PostconditionRequirement {
                id: "post-file".into(),
                description: "read-back matches".into(),
                required: true,
            }],
            evidence_requirements: vec!["digest".into()],
        })
        .unwrap()
    }

    fn action(
        tool_status: ToolReturnStatus,
        observation_state: PostconditionObservationState,
    ) -> ActionOutcomeInput {
        let observed_value = "digest".to_string();
        let evidence_digest = Digest::of_text(&observed_value);
        ActionOutcomeInput {
            action_id: "action-1".into(),
            task_id: "task-1".into(),
            run_id: "run-1".into(),
            action: "write_file".into(),
            tool: "filesystem".into(),
            argument_digest: Digest::of_text("args"),
            target: "workspace/file".into(),
            recipient: None,
            tool_return_status: tool_status,
            observed_effect: Some("file exists".into()),
            postcondition_observations: vec![PostconditionObservation {
                id: "post-file".into(),
                state: observation_state,
                observed_value: Some(observed_value),
                evidence_refs: vec!["evidence-1".into()],
                evidence_digests: vec![evidence_digest.clone()],
            }],
            exact_errors: Vec::new(),
            partial_side_effects: Vec::new(),
            retries: 0,
            compensation: Vec::new(),
            remaining_uncertainty: Vec::new(),
            evidence_refs: vec!["evidence-1".into()],
            evidence_digests: vec![evidence_digest],
        }
    }

    fn supported_claim() -> ClaimReport {
        evaluate_claim(
            "claim-1",
            "file exists",
            "postcondition",
            true,
            ClaimEvidence {
                supported: true,
                contradicted: false,
                not_applicable: false,
                evidence_refs: vec!["evidence-1".into()],
                source_identity: "filesystem".into(),
                source_version: "1".into(),
                verifier_id: "read-back".into(),
                verifier_version: "1".into(),
                assumptions: Vec::new(),
                contradictions: Vec::new(),
                uncertainty_reason: None,
            },
        )
        .unwrap()
    }

    #[test]
    fn missing_required_postcondition_keeps_action_unknown() {
        let mut input = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        input.postcondition_observations[0].id = "optional".into();
        assert_eq!(
            evaluate_action_outcome(&task(), input),
            Err(OutcomeError::UnknownPostconditionObservation)
        );
    }

    #[test]
    fn duplicate_postcondition_observation_rejects() {
        let mut input = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        input
            .postcondition_observations
            .push(input.postcondition_observations[0].clone());
        assert_eq!(
            evaluate_action_outcome(&task(), input),
            Err(OutcomeError::DuplicatePostconditionObservation)
        );
    }

    #[test]
    fn passed_observation_without_evidence_rejects() {
        let mut input = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        input.postcondition_observations[0].evidence_refs.clear();
        input.postcondition_observations[0].evidence_digests.clear();
        assert_eq!(
            evaluate_action_outcome(&task(), input),
            Err(OutcomeError::MissingEvidenceDigest)
        );
    }

    #[test]
    fn passed_observation_without_value_rejects() {
        let mut input = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        input.postcondition_observations[0].observed_value = None;
        assert_eq!(
            evaluate_action_outcome(&task(), input),
            Err(OutcomeError::MissingObservedValue)
        );
    }

    #[test]
    fn failed_required_postcondition_prevents_success() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Failed,
            ),
        )
        .unwrap();
        assert_ne!(outcome.status(), ActionStatus::Succeeded);
    }

    #[test]
    fn unknown_required_postcondition_prevents_success() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Unknown,
            ),
        )
        .unwrap();
        assert_eq!(outcome.status(), ActionStatus::Unknown);
    }

    #[test]
    fn not_checked_required_postcondition_prevents_success() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::NotChecked,
            ),
        )
        .unwrap();
        assert_eq!(outcome.status(), ActionStatus::Unknown);
    }

    #[test]
    fn evidence_bearing_observation_allows_action_success() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Observed,
            ),
        )
        .unwrap();
        assert_eq!(outcome.status(), ActionStatus::Succeeded);
    }

    #[test]
    fn criterion_satisfaction_is_derived() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Observed,
            ),
        )
        .unwrap();
        let result =
            assemble_authoritative_run_result(&task(), vec![outcome], vec![supported_claim()]);
        assert_eq!(
            result.criterion_outcome("criterion-file").unwrap().status(),
            CriterionStatus::Satisfied
        );
    }

    #[test]
    fn authoritative_result_preserves_prior_error() {
        let mut first = action(
            ToolReturnStatus::Failed,
            PostconditionObservationState::NotChecked,
        );
        first.observed_effect = None;
        first.exact_errors = vec!["first attempt failed".into()];
        let failed = evaluate_action_outcome(&task(), first).unwrap();
        let mut retry = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        retry.retries = 1;
        let succeeded = evaluate_action_outcome(&task(), retry).unwrap();
        let result = assemble_authoritative_run_result(
            &task(),
            vec![failed, succeeded],
            vec![supported_claim()],
        );
        assert_eq!(result.status(), ActionStatus::Succeeded);
        assert!(result.summary().contains("first attempt failed"));
    }

    #[test]
    fn authoritative_result_preserves_prior_partial_side_effect() {
        let mut first = action(
            ToolReturnStatus::Failed,
            PostconditionObservationState::Failed,
        );
        first.partial_side_effects = vec!["partial file write".into()];
        let partial = evaluate_action_outcome(&task(), first).unwrap();
        let mut retry = action(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Observed,
        );
        retry.retries = 1;
        let succeeded = evaluate_action_outcome(&task(), retry).unwrap();
        let result = assemble_authoritative_run_result(
            &task(),
            vec![partial, succeeded],
            vec![supported_claim()],
        );
        assert_eq!(result.status(), ActionStatus::Partial);
        assert!(result
            .partial_side_effects()
            .contains(&"partial file write".into()));
    }

    #[test]
    fn identical_inputs_derive_identical_results() {
        let first = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Observed,
            ),
        );
        let second = evaluate_action_outcome(
            &task(),
            action(
                ToolReturnStatus::Succeeded,
                PostconditionObservationState::Observed,
            ),
        );
        assert_eq!(first, second);
    }
}
