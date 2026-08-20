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
    validate_limits(input)
}

fn validate_contract_collections(input: &TaskContractInput) -> Result<(), TaskContractError> {
    validate_criteria(&input.success_criteria)?;
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
    validate_postconditions(&input.expected_postconditions)?;
    validate_text_items(
        &input.evidence_requirements,
        TaskContractError::MissingEvidenceRequirements,
    )
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
    Ok(())
}

fn validate_postconditions(items: &[PostconditionRequirement]) -> Result<(), TaskContractError> {
    if items.is_empty() {
        return Err(TaskContractError::MissingPostconditions);
    }
    if items.iter().any(invalid_postcondition) {
        return Err(TaskContractError::EmptyContractItem);
    }
    Ok(())
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
            success_criteria: vec![SuccessCriterion {
                id: "criterion-file".into(),
                description: "expected file exists".into(),
                required: true,
            }],
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
            expected_postconditions: vec![PostconditionRequirement {
                id: "postcondition-file".into(),
                description: "read-back digest matches".into(),
                required: true,
            }],
            evidence_requirements: vec!["read_back_digest".into()],
        }
    }

    #[test]
    fn task_contract_accepts_complete_bounded_input() {
        let contract = TaskContract::new(input()).unwrap();
        assert!(contract.permits_action("write_file"));
        assert!(contract.permits_tool("filesystem"));
    }

    #[test]
    fn task_contract_requires_postconditions() {
        let mut input = input();
        input.expected_postconditions.clear();
        assert_eq!(
            TaskContract::new(input),
            Err(TaskContractError::MissingPostconditions)
        );
    }
}
