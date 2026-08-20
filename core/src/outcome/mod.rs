use crate::integrity::Digest;
use crate::task::TaskContract;

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
pub enum PostconditionStatus {
    Passed,
    Failed,
    Unknown,
    NotChecked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostconditionCheck {
    pub id: String,
    pub required: bool,
    pub status: PostconditionStatus,
    pub observed_value: Option<String>,
    pub evidence_refs: Vec<String>,
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
    pub postconditions: Vec<PostconditionCheck>,
    pub exact_errors: Vec<String>,
    pub partial_side_effects: Vec<String>,
    pub retries: u32,
    pub compensation: Vec<String>,
    pub remaining_uncertainty: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub satisfied_criterion_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutcome {
    input: ActionOutcomeInput,
    status: ActionStatus,
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
    EmptyEvidenceReference,
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
            Self::EmptyEvidenceReference => "empty_evidence_reference",
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

    pub fn satisfied_criterion_ids(&self) -> &[String] {
        &self.input.satisfied_criterion_ids
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
    let status = determine_action_status(&input);
    Ok(ActionOutcome { input, status })
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
    let material_errors = collect_errors(&actions);
    let partial_side_effects = collect_partial_side_effects(&actions);
    let unmet_success_criteria = find_unmet_criteria(task, &actions);
    let unresolved_uncertainty = collect_uncertainty(&actions, &claims);
    let status = determine_run_status(
        &actions,
        &claims,
        &partial_side_effects,
        &unmet_success_criteria,
    );
    let summary = build_run_summary(
        status,
        &material_errors,
        &partial_side_effects,
        &unmet_success_criteria,
        &unresolved_uncertainty,
    );
    AuthoritativeRunResult {
        status,
        actions,
        claims,
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
    validate_action_evidence(input)
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
    if input.postconditions.is_empty() {
        return Err(OutcomeError::MissingPostconditions);
    }
    if input
        .evidence_refs
        .iter()
        .any(|item| item.trim().is_empty())
    {
        return Err(OutcomeError::EmptyEvidenceReference);
    }
    Ok(())
}

fn determine_action_status(input: &ActionOutcomeInput) -> ActionStatus {
    match input.tool_return_status {
        ToolReturnStatus::NotAttempted => ActionStatus::NotAttempted,
        ToolReturnStatus::Blocked => ActionStatus::Blocked,
        ToolReturnStatus::Failed => failed_action_status(input),
        ToolReturnStatus::TimedOut | ToolReturnStatus::Unknown => ActionStatus::Unknown,
        ToolReturnStatus::Succeeded => successful_tool_status(input),
    }
}

fn failed_action_status(input: &ActionOutcomeInput) -> ActionStatus {
    if input.observed_effect.is_some() || !input.partial_side_effects.is_empty() {
        return ActionStatus::Partial;
    }
    ActionStatus::Failed
}

fn successful_tool_status(input: &ActionOutcomeInput) -> ActionStatus {
    if has_failed_required_postcondition(input) {
        return failed_action_status(input);
    }
    if has_unverified_required_postcondition(input) {
        return ActionStatus::Unknown;
    }
    if input.observed_effect.is_none() || input.evidence_refs.is_empty() {
        return ActionStatus::Unknown;
    }
    ActionStatus::Succeeded
}

fn has_failed_required_postcondition(input: &ActionOutcomeInput) -> bool {
    input
        .postconditions
        .iter()
        .any(|item| item.required && item.status == PostconditionStatus::Failed)
}

fn has_unverified_required_postcondition(input: &ActionOutcomeInput) -> bool {
    input.postconditions.iter().any(|item| {
        item.required
            && matches!(
                item.status,
                PostconditionStatus::Unknown | PostconditionStatus::NotChecked
            )
    })
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

fn find_unmet_criteria(task: &TaskContract, actions: &[ActionOutcome]) -> Vec<String> {
    task.success_criteria()
        .iter()
        .filter(|criterion| criterion.required)
        .filter(|criterion| !criterion_is_satisfied(&criterion.id, actions))
        .map(|criterion| criterion.id.clone())
        .collect()
}

fn criterion_is_satisfied(criterion_id: &str, actions: &[ActionOutcome]) -> bool {
    actions.iter().any(|outcome| {
        outcome.status == ActionStatus::Succeeded
            && outcome
                .input
                .satisfied_criterion_ids
                .iter()
                .any(|id| id == criterion_id)
    })
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
    actions: &[ActionOutcome],
    claims: &[ClaimReport],
    partial_side_effects: &[String],
    unmet_criteria: &[String],
) -> ActionStatus {
    if !partial_side_effects.is_empty() || actions.iter().any(is_partial) {
        return ActionStatus::Partial;
    }
    if has_unverified_material_claim(claims) || actions.iter().any(is_unknown) {
        return ActionStatus::Unknown;
    }
    if !unmet_criteria.is_empty() {
        return ActionStatus::Failed;
    }
    if actions.iter().any(is_blocked) {
        return ActionStatus::Blocked;
    }
    ActionStatus::Succeeded
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

fn is_unknown(outcome: &ActionOutcome) -> bool {
    outcome.status == ActionStatus::Unknown
}

fn is_blocked(outcome: &ActionOutcome) -> bool {
    outcome.status == ActionStatus::Blocked
}

fn build_run_summary(
    status: ActionStatus,
    errors: &[String],
    side_effects: &[String],
    unmet: &[String],
    uncertainty: &[String],
) -> String {
    format!(
        "status={status:?}; material_errors={}; partial_side_effects={}; unmet_success_criteria={}; unresolved_uncertainty={}",
        errors.join("|"),
        side_effects.join("|"),
        unmet.join("|"),
        uncertainty.join("|")
    )
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

    fn action(status: ToolReturnStatus, postcondition: PostconditionStatus) -> ActionOutcomeInput {
        ActionOutcomeInput {
            action_id: "action-1".into(),
            task_id: "task-1".into(),
            run_id: "run-1".into(),
            action: "write_file".into(),
            tool: "filesystem".into(),
            argument_digest: Digest::of_text("args"),
            target: "workspace/file".into(),
            recipient: None,
            tool_return_status: status,
            observed_effect: Some("file exists".into()),
            postconditions: vec![PostconditionCheck {
                id: "post-file".into(),
                required: true,
                status: postcondition,
                observed_value: Some("digest".into()),
                evidence_refs: vec!["evidence-1".into()],
            }],
            exact_errors: Vec::new(),
            partial_side_effects: Vec::new(),
            retries: 0,
            compensation: Vec::new(),
            remaining_uncertainty: Vec::new(),
            evidence_refs: vec!["evidence-1".into()],
            satisfied_criterion_ids: vec!["criterion-file".into()],
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
    fn tool_success_without_postcondition_is_unknown() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(ToolReturnStatus::Succeeded, PostconditionStatus::NotChecked),
        )
        .unwrap();
        assert_eq!(outcome.status(), ActionStatus::Unknown);
    }

    #[test]
    fn observed_postcondition_allows_success() {
        let outcome = evaluate_action_outcome(
            &task(),
            action(ToolReturnStatus::Succeeded, PostconditionStatus::Passed),
        )
        .unwrap();
        assert_eq!(outcome.status(), ActionStatus::Succeeded);
    }

    #[test]
    fn authoritative_result_preserves_prior_error() {
        let mut first = action(ToolReturnStatus::Failed, PostconditionStatus::NotChecked);
        first.observed_effect = None;
        first.exact_errors = vec!["first attempt failed".into()];
        first.satisfied_criterion_ids.clear();
        let failed = evaluate_action_outcome(&task(), first).unwrap();
        let succeeded = evaluate_action_outcome(
            &task(),
            action(ToolReturnStatus::Succeeded, PostconditionStatus::Passed),
        )
        .unwrap();
        let result = assemble_authoritative_run_result(
            &task(),
            vec![failed, succeeded],
            vec![supported_claim()],
        );
        assert!(result
            .material_errors()
            .contains(&"first attempt failed".into()));
        assert!(result.summary().contains("first attempt failed"));
    }
}
