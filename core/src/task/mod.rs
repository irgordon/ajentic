use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskContract {
    task_id: String,
    objective: String,
    success_criteria: Vec<SuccessCriterion>,
    forbidden_outcomes: Vec<String>,
    permitted_actions: Vec<String>,
    permitted_tools: Vec<String>,
    non_goals: Vec<String>,
    required_approval_points: Vec<String>,
    side_effect_budget: u32,
    retry_policy: RetryPolicy,
    stop_conditions: Vec<String>,
    expected_postconditions: Vec<PostconditionRequirement>,
    evidence_requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskContractInput {
    pub task_id: String,
    pub objective: String,
    pub success_criteria: Vec<SuccessCriterion>,
    pub forbidden_outcomes: Vec<String>,
    pub permitted_actions: Vec<String>,
    pub permitted_tools: Vec<String>,
    pub non_goals: Vec<String>,
    pub required_approval_points: Vec<String>,
    pub side_effect_budget: u32,
    pub retry_policy: RetryPolicy,
    pub stop_conditions: Vec<String>,
    pub expected_postconditions: Vec<PostconditionRequirement>,
    pub evidence_requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessCriterion {
    pub id: String,
    pub description: String,
    pub required: bool,
    pub required_postcondition_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostconditionRequirement {
    pub id: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub require_idempotency_after_possible_side_effect: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskContractError {
    EmptyTaskId,
    EmptyObjective,
    MissingSuccessCriteria,
    MissingForbiddenOutcomes,
    MissingPermittedActions,
    MissingPermittedTools,
    MissingNonGoals,
    MissingApprovalPoints,
    InvalidSideEffectBudget,
    InvalidRetryPolicy,
    MissingStopConditions,
    MissingPostconditions,
    MissingEvidenceRequirements,
    EmptyContractItem,
    DuplicateSuccessCriterionId,
    DuplicatePostconditionId,
    MissingCriterionPostconditionBinding,
    EmptyPostconditionReference,
    UnknownPostconditionReference,
    DuplicatePostconditionReference,
    UnboundRequiredPostcondition,
    RequiredCriterionReferencesOptionalPostcondition,
}

impl TaskContractError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EmptyTaskId => "empty_task_id",
            Self::EmptyObjective => "empty_objective",
            Self::MissingSuccessCriteria => "missing_success_criteria",
            Self::MissingForbiddenOutcomes => "missing_forbidden_outcomes",
            Self::MissingPermittedActions => "missing_permitted_actions",
            Self::MissingPermittedTools => "missing_permitted_tools",
            Self::MissingNonGoals => "missing_non_goals",
            Self::MissingApprovalPoints => "missing_approval_points",
            Self::InvalidSideEffectBudget => "invalid_side_effect_budget",
            Self::InvalidRetryPolicy => "invalid_retry_policy",
            Self::MissingStopConditions => "missing_stop_conditions",
            Self::MissingPostconditions => "missing_postconditions",
            Self::MissingEvidenceRequirements => "missing_evidence_requirements",
            Self::EmptyContractItem => "empty_contract_item",
            Self::DuplicateSuccessCriterionId => "duplicate_success_criterion_id",
            Self::DuplicatePostconditionId => "duplicate_postcondition_id",
            Self::MissingCriterionPostconditionBinding => "missing_criterion_postcondition_binding",
            Self::EmptyPostconditionReference => "empty_postcondition_reference",
            Self::UnknownPostconditionReference => "unknown_postcondition_reference",
            Self::DuplicatePostconditionReference => "duplicate_postcondition_reference",
            Self::UnboundRequiredPostcondition => "unbound_required_postcondition",
            Self::RequiredCriterionReferencesOptionalPostcondition => {
                "required_criterion_references_optional_postcondition"
            }
        }
    }
}

impl TaskContract {
    pub fn new(input: TaskContractInput) -> Result<Self, TaskContractError> {
        validate_task_contract(&input)?;
        Ok(Self::from_validated(input))
    }

    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    pub fn objective(&self) -> &str {
        &self.objective
    }

    pub fn success_criteria(&self) -> &[SuccessCriterion] {
        &self.success_criteria
    }

    pub fn expected_postconditions(&self) -> &[PostconditionRequirement] {
        &self.expected_postconditions
    }

    pub fn postcondition(&self, id: &str) -> Option<&PostconditionRequirement> {
        self.expected_postconditions
            .iter()
            .find(|item| item.id == id)
    }

    pub fn side_effect_budget(&self) -> u32 {
        self.side_effect_budget
    }

    pub fn retry_policy(&self) -> RetryPolicy {
        self.retry_policy
    }

    pub fn permits_action(&self, action: &str) -> bool {
        self.permitted_actions.iter().any(|item| item == action)
    }

    pub fn permits_tool(&self, tool: &str) -> bool {
        self.permitted_tools.iter().any(|item| item == tool)
    }

    pub fn forbidden_outcomes(&self) -> &[String] {
        &self.forbidden_outcomes
    }

    pub fn non_goals(&self) -> &[String] {
        &self.non_goals
    }

    pub fn required_approval_points(&self) -> &[String] {
        &self.required_approval_points
    }

    pub fn stop_conditions(&self) -> &[String] {
        &self.stop_conditions
    }

    pub fn evidence_requirements(&self) -> &[String] {
        &self.evidence_requirements
    }

    fn from_validated(input: TaskContractInput) -> Self {
        Self {
            task_id: input.task_id,
            objective: input.objective,
            success_criteria: input.success_criteria,
            forbidden_outcomes: input.forbidden_outcomes,
            permitted_actions: input.permitted_actions,
            permitted_tools: input.permitted_tools,
            non_goals: input.non_goals,
            required_approval_points: input.required_approval_points,
            side_effect_budget: input.side_effect_budget,
            retry_policy: input.retry_policy,
            stop_conditions: input.stop_conditions,
            expected_postconditions: input.expected_postconditions,
            evidence_requirements: input.evidence_requirements,
        }
    }
}

fn validate_task_contract(input: &TaskContractInput) -> Result<(), TaskContractError> {
    validate_required_text(&input.task_id, TaskContractError::EmptyTaskId)?;
    validate_required_text(&input.objective, TaskContractError::EmptyObjective)?;
    validate_contract_collections(input)?;
    validate_contract_mappings(input)?;
    validate_limits(input)
}

fn validate_contract_collections(input: &TaskContractInput) -> Result<(), TaskContractError> {
    validate_criteria(&input.success_criteria)?;
    validate_postconditions(&input.expected_postconditions)?;
    validate_supporting_collections(input)
}

fn validate_supporting_collections(input: &TaskContractInput) -> Result<(), TaskContractError> {
    validate_text_items(
        &input.forbidden_outcomes,
        TaskContractError::MissingForbiddenOutcomes,
    )?;
    validate_text_items(
        &input.permitted_actions,
        TaskContractError::MissingPermittedActions,
    )?;
    validate_text_items(
        &input.permitted_tools,
        TaskContractError::MissingPermittedTools,
    )?;
    validate_text_items(&input.non_goals, TaskContractError::MissingNonGoals)?;
    validate_text_items(
        &input.required_approval_points,
        TaskContractError::MissingApprovalPoints,
    )?;
    validate_text_items(
        &input.stop_conditions,
        TaskContractError::MissingStopConditions,
    )?;
    validate_text_items(
        &input.evidence_requirements,
        TaskContractError::MissingEvidenceRequirements,
    )
}

fn validate_contract_mappings(input: &TaskContractInput) -> Result<(), TaskContractError> {
    let postconditions = postcondition_ids(&input.expected_postconditions);
    validate_criterion_mappings(&input.success_criteria, &postconditions, input)?;
    validate_required_postcondition_coverage(input)
}

fn validate_criterion_mappings(
    criteria: &[SuccessCriterion],
    postcondition_ids: &HashSet<&str>,
    input: &TaskContractInput,
) -> Result<(), TaskContractError> {
    for criterion in criteria {
        validate_criterion_mapping(criterion, postcondition_ids, input)?;
    }
    Ok(())
}

fn validate_criterion_mapping(
    criterion: &SuccessCriterion,
    postcondition_ids: &HashSet<&str>,
    input: &TaskContractInput,
) -> Result<(), TaskContractError> {
    validate_mapping_presence(criterion)?;
    validate_mapping_references(criterion, postcondition_ids)?;
    validate_required_mapping(criterion, input)
}

fn validate_mapping_presence(criterion: &SuccessCriterion) -> Result<(), TaskContractError> {
    if criterion.required_postcondition_ids.is_empty() {
        return Err(TaskContractError::MissingCriterionPostconditionBinding);
    }
    if criterion
        .required_postcondition_ids
        .iter()
        .any(|item| item.trim().is_empty())
    {
        return Err(TaskContractError::EmptyPostconditionReference);
    }
    Ok(())
}

fn validate_mapping_references(
    criterion: &SuccessCriterion,
    postcondition_ids: &HashSet<&str>,
) -> Result<(), TaskContractError> {
    let mapped = criterion
        .required_postcondition_ids
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if mapped.iter().copied().collect::<HashSet<_>>().len() != mapped.len() {
        return Err(TaskContractError::DuplicatePostconditionReference);
    }
    if mapped.iter().any(|item| !postcondition_ids.contains(item)) {
        return Err(TaskContractError::UnknownPostconditionReference);
    }
    Ok(())
}

fn validate_required_mapping(
    criterion: &SuccessCriterion,
    input: &TaskContractInput,
) -> Result<(), TaskContractError> {
    if !criterion.required {
        return Ok(());
    }
    if criterion.required_postcondition_ids.iter().all(|id| {
        input
            .expected_postconditions
            .iter()
            .any(|postcondition| postcondition.id == *id && postcondition.required)
    }) {
        return Ok(());
    }
    Err(TaskContractError::RequiredCriterionReferencesOptionalPostcondition)
}

fn validate_required_postcondition_coverage(
    input: &TaskContractInput,
) -> Result<(), TaskContractError> {
    let bound = input
        .success_criteria
        .iter()
        .filter(|criterion| criterion.required)
        .flat_map(|criterion| criterion.required_postcondition_ids.iter())
        .map(String::as_str)
        .collect::<HashSet<_>>();
    if input
        .expected_postconditions
        .iter()
        .filter(|postcondition| postcondition.required)
        .all(|postcondition| bound.contains(postcondition.id.as_str()))
    {
        return Ok(());
    }
    Err(TaskContractError::UnboundRequiredPostcondition)
}

fn validate_limits(input: &TaskContractInput) -> Result<(), TaskContractError> {
    if input.side_effect_budget == 0 {
        return Err(TaskContractError::InvalidSideEffectBudget);
    }
    if input.retry_policy.max_attempts == 0 {
        return Err(TaskContractError::InvalidRetryPolicy);
    }
    Ok(())
}

fn validate_criteria(items: &[SuccessCriterion]) -> Result<(), TaskContractError> {
    if items.is_empty() {
        return Err(TaskContractError::MissingSuccessCriteria);
    }
    if items.iter().any(invalid_criterion) {
        return Err(TaskContractError::EmptyContractItem);
    }
    validate_unique_criterion_ids(items)
}

fn validate_postconditions(items: &[PostconditionRequirement]) -> Result<(), TaskContractError> {
    if items.is_empty() {
        return Err(TaskContractError::MissingPostconditions);
    }
    if items.iter().any(invalid_postcondition) {
        return Err(TaskContractError::EmptyContractItem);
    }
    validate_unique_postcondition_ids(items)
}

fn validate_unique_criterion_ids(items: &[SuccessCriterion]) -> Result<(), TaskContractError> {
    let ids = items
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    if ids.iter().copied().collect::<HashSet<_>>().len() == ids.len() {
        return Ok(());
    }
    Err(TaskContractError::DuplicateSuccessCriterionId)
}

fn validate_unique_postcondition_ids(
    items: &[PostconditionRequirement],
) -> Result<(), TaskContractError> {
    let ids = items
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    if ids.iter().copied().collect::<HashSet<_>>().len() == ids.len() {
        return Ok(());
    }
    Err(TaskContractError::DuplicatePostconditionId)
}

fn validate_text_items(
    items: &[String],
    missing: TaskContractError,
) -> Result<(), TaskContractError> {
    if items.is_empty() {
        return Err(missing);
    }
    if items.iter().any(|item| item.trim().is_empty()) {
        return Err(TaskContractError::EmptyContractItem);
    }
    Ok(())
}

fn postcondition_ids(items: &[PostconditionRequirement]) -> HashSet<&str> {
    items.iter().map(|item| item.id.as_str()).collect()
}

fn invalid_criterion(item: &SuccessCriterion) -> bool {
    item.id.trim().is_empty() || item.description.trim().is_empty()
}

fn invalid_postcondition(item: &PostconditionRequirement) -> bool {
    item.id.trim().is_empty() || item.description.trim().is_empty()
}

fn validate_required_text(value: &str, error: TaskContractError) -> Result<(), TaskContractError> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> TaskContractInput {
        TaskContractInput {
            task_id: "task-1".into(),
            objective: "write and verify a file".into(),
            success_criteria: vec![criterion("criterion-file", true, &["postcondition-file"])],
            forbidden_outcomes: vec!["write outside workspace".into()],
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
            expected_postconditions: vec![postcondition("postcondition-file", true)],
            evidence_requirements: vec!["read_back_digest".into()],
        }
    }

    fn criterion(id: &str, required: bool, postconditions: &[&str]) -> SuccessCriterion {
        SuccessCriterion {
            id: id.into(),
            description: format!("criterion {id}"),
            required,
            required_postcondition_ids: postconditions.iter().map(|item| (*item).into()).collect(),
        }
    }

    fn postcondition(id: &str, required: bool) -> PostconditionRequirement {
        PostconditionRequirement {
            id: id.into(),
            description: format!("postcondition {id}"),
            required,
        }
    }

    #[test]
    fn task_contract_accepts_complete_bounded_input() {
        let contract = TaskContract::new(input()).unwrap();
        assert_eq!(
            contract.success_criteria()[0].required_postcondition_ids,
            vec!["postcondition-file"]
        );
    }

    #[test]
    fn duplicate_success_criterion_ids_reject() {
        let mut input = input();
        input
            .success_criteria
            .push(criterion("criterion-file", false, &["postcondition-file"]));
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::DuplicateSuccessCriterionId)
        );
    }

    #[test]
    fn duplicate_postcondition_ids_reject() {
        let mut input = input();
        input
            .expected_postconditions
            .push(postcondition("postcondition-file", false));
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::DuplicatePostconditionId)
        );
    }

    #[test]
    fn criterion_without_postcondition_mapping_rejects() {
        let mut input = input();
        input.success_criteria[0].required_postcondition_ids.clear();
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::MissingCriterionPostconditionBinding)
        );
    }

    #[test]
    fn unknown_postcondition_reference_rejects() {
        let mut input = input();
        input.success_criteria[0].required_postcondition_ids = vec!["unknown".into()];
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::UnknownPostconditionReference)
        );
    }

    #[test]
    fn duplicate_postcondition_reference_rejects() {
        let mut input = input();
        input.success_criteria[0].required_postcondition_ids =
            vec!["postcondition-file".into(), "postcondition-file".into()];
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::DuplicatePostconditionReference)
        );
    }

    #[test]
    fn unbound_required_postcondition_rejects() {
        let mut input = input();
        input
            .expected_postconditions
            .push(postcondition("postcondition-permissions", true));
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::UnboundRequiredPostcondition)
        );
    }

    #[test]
    fn required_criterion_referencing_optional_postcondition_rejects() {
        let mut input = input();
        input.expected_postconditions[0].required = false;
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::RequiredCriterionReferencesOptionalPostcondition)
        );
    }

    #[test]
    fn empty_postcondition_reference_rejects() {
        let mut input = input();
        input.success_criteria[0].required_postcondition_ids = vec![String::new()];
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::EmptyPostconditionReference)
        );
    }

    #[test]
    fn task_contract_validation_is_deterministic() {
        assert_eq!(TaskContract::new(input()), TaskContract::new(input()));
    }
}
