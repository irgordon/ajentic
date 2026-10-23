#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunBudget {
    pub max_model_turns: u32,
    pub max_tool_calls: u32,
    pub max_identical_calls: u32,
    pub max_retries_per_failure_class: u32,
    pub max_tokens: u64,
    pub max_wall_clock_millis: u64,
    pub max_side_effects: u32,
    pub max_cost_microunits: u64,
    pub max_turns_without_new_evidence: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunUsage {
    pub model_turns: u32,
    pub tool_calls: u32,
    pub repeated_identical_calls: u32,
    pub retries_for_failure_class: u32,
    pub tokens: u64,
    pub wall_clock_millis: u64,
    pub side_effects: u32,
    pub cost_microunits: u64,
    pub turns_without_new_evidence: u32,
    pub cancellation_requested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetrySafety {
    pub retry_requested: bool,
    pub preceding_attempt_may_have_side_effects: bool,
    pub idempotency_proved: bool,
    pub side_effect_absence_verified: bool,
    pub compensation_approved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunBudgetDecision {
    Continue,
    Stop,
    Escalate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunBudgetReason {
    WithinBudget,
    CancellationRequested,
    ModelTurnLimit,
    ToolCallLimit,
    IdenticalCallLimit,
    RetryLimit,
    TokenLimit,
    WallClockLimit,
    SideEffectLimit,
    CostLimit,
    NoNewEvidenceLimit,
    UnsafeSideEffectRetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunBudgetReport {
    pub decision: RunBudgetDecision,
    pub reason: RunBudgetReason,
}

pub fn evaluate_run_budget(
    budget: &RunBudget,
    usage: &RunUsage,
    retry: &RetrySafety,
) -> RunBudgetReport {
    if usage.cancellation_requested {
        return stopped(RunBudgetReason::CancellationRequested);
    }
    if let Some(reason) = exceeded_budget_reason(budget, usage) {
        return stopped(reason);
    }
    if retry_is_unsafe(retry) {
        return escalated(RunBudgetReason::UnsafeSideEffectRetry);
    }
    continued()
}

fn exceeded_budget_reason(budget: &RunBudget, usage: &RunUsage) -> Option<RunBudgetReason> {
    model_or_tool_limit(budget, usage)
        .or_else(|| repetition_or_retry_limit(budget, usage))
        .or_else(|| resource_limit(budget, usage))
        .or_else(|| operational_limit(budget, usage))
}

fn model_or_tool_limit(budget: &RunBudget, usage: &RunUsage) -> Option<RunBudgetReason> {
    if usage.model_turns >= budget.max_model_turns {
        return Some(RunBudgetReason::ModelTurnLimit);
    }
    if usage.tool_calls >= budget.max_tool_calls {
        return Some(RunBudgetReason::ToolCallLimit);
    }
    None
}

fn repetition_or_retry_limit(budget: &RunBudget, usage: &RunUsage) -> Option<RunBudgetReason> {
    if usage.repeated_identical_calls >= budget.max_identical_calls {
        return Some(RunBudgetReason::IdenticalCallLimit);
    }
    if usage.retries_for_failure_class >= budget.max_retries_per_failure_class {
        return Some(RunBudgetReason::RetryLimit);
    }
    None
}

fn resource_limit(budget: &RunBudget, usage: &RunUsage) -> Option<RunBudgetReason> {
    if usage.tokens >= budget.max_tokens {
        return Some(RunBudgetReason::TokenLimit);
    }
    if usage.wall_clock_millis >= budget.max_wall_clock_millis {
        return Some(RunBudgetReason::WallClockLimit);
    }
    if usage.cost_microunits >= budget.max_cost_microunits {
        return Some(RunBudgetReason::CostLimit);
    }
    None
}

fn operational_limit(budget: &RunBudget, usage: &RunUsage) -> Option<RunBudgetReason> {
    if usage.side_effects >= budget.max_side_effects {
        return Some(RunBudgetReason::SideEffectLimit);
    }
    if usage.turns_without_new_evidence >= budget.max_turns_without_new_evidence {
        return Some(RunBudgetReason::NoNewEvidenceLimit);
    }
    None
}

fn retry_is_unsafe(retry: &RetrySafety) -> bool {
    retry.retry_requested
        && retry.preceding_attempt_may_have_side_effects
        && !retry.idempotency_proved
        && !retry.side_effect_absence_verified
        && !retry.compensation_approved
}

fn stopped(reason: RunBudgetReason) -> RunBudgetReport {
    RunBudgetReport {
        decision: RunBudgetDecision::Stop,
        reason,
    }
}

fn escalated(reason: RunBudgetReason) -> RunBudgetReport {
    RunBudgetReport {
        decision: RunBudgetDecision::Escalate,
        reason,
    }
}

fn continued() -> RunBudgetReport {
    RunBudgetReport {
        decision: RunBudgetDecision::Continue,
        reason: RunBudgetReason::WithinBudget,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget() -> RunBudget {
        RunBudget {
            max_model_turns: 10,
            max_tool_calls: 10,
            max_identical_calls: 3,
            max_retries_per_failure_class: 2,
            max_tokens: 1_000,
            max_wall_clock_millis: 60_000,
            max_side_effects: 2,
            max_cost_microunits: 1_000,
            max_turns_without_new_evidence: 3,
        }
    }

    fn usage() -> RunUsage {
        RunUsage {
            model_turns: 1,
            tool_calls: 1,
            repeated_identical_calls: 0,
            retries_for_failure_class: 0,
            tokens: 10,
            wall_clock_millis: 10,
            side_effects: 0,
            cost_microunits: 10,
            turns_without_new_evidence: 0,
            cancellation_requested: false,
        }
    }

    fn retry() -> RetrySafety {
        RetrySafety {
            retry_requested: false,
            preceding_attempt_may_have_side_effects: false,
            idempotency_proved: false,
            side_effect_absence_verified: false,
            compensation_approved: false,
        }
    }

    #[test]
    fn budget_allows_bounded_run() {
        assert_eq!(
            evaluate_run_budget(&budget(), &usage(), &retry()).decision,
            RunBudgetDecision::Continue
        );
    }

    #[test]
    fn unsafe_side_effect_retry_escalates() {
        let retry = RetrySafety {
            retry_requested: true,
            preceding_attempt_may_have_side_effects: true,
            ..retry()
        };
        let report = evaluate_run_budget(&budget(), &usage(), &retry);
        assert_eq!(report.decision, RunBudgetDecision::Escalate);
        assert_eq!(report.reason, RunBudgetReason::UnsafeSideEffectRetry);
    }

    #[test]
    fn no_new_evidence_stops() {
        let mut usage = usage();
        usage.turns_without_new_evidence = 3;
        let report = evaluate_run_budget(&budget(), &usage, &retry());
        assert_eq!(report.reason, RunBudgetReason::NoNewEvidenceLimit);
    }
}
