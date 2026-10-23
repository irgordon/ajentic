use ajentic_core::execution::{
    evaluate_run_budget, RetrySafety, RunBudget, RunBudgetDecision, RunBudgetReason, RunUsage,
};

fn budget() -> RunBudget {
    RunBudget {
        max_model_turns: 10,
        max_tool_calls: 10,
        max_identical_calls: 2,
        max_retries_per_failure_class: 2,
        max_tokens: 1_000,
        max_wall_clock_millis: 60_000,
        max_side_effects: 2,
        max_cost_microunits: 1_000,
        max_turns_without_new_evidence: 2,
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

#[test]
fn repeated_calls_without_new_evidence_stop() {
    let mut usage = usage();
    usage.repeated_identical_calls = 2;
    let report = evaluate_run_budget(
        &budget(),
        &usage,
        &RetrySafety {
            retry_requested: false,
            preceding_attempt_may_have_side_effects: false,
            idempotency_proved: false,
            side_effect_absence_verified: false,
            compensation_approved: false,
        },
    );
    assert_eq!(report.decision, RunBudgetDecision::Stop);
    assert_eq!(report.reason, RunBudgetReason::IdenticalCallLimit);
}

#[test]
fn unsafe_non_idempotent_retry_escalates() {
    let report = evaluate_run_budget(
        &budget(),
        &usage(),
        &RetrySafety {
            retry_requested: true,
            preceding_attempt_may_have_side_effects: true,
            idempotency_proved: false,
            side_effect_absence_verified: false,
            compensation_approved: false,
        },
    );
    assert_eq!(report.decision, RunBudgetDecision::Escalate);
}
