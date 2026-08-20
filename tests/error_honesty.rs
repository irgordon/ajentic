mod common;

use ajentic_core::outcome::{
    assemble_authoritative_run_result, evaluate_action_outcome, evaluate_claim, ActionStatus,
    PostconditionStatus, ToolReturnStatus,
};

#[test]
fn later_success_does_not_erase_earlier_error() {
    let task = common::task_contract();
    let mut first = common::action_input(ToolReturnStatus::Failed, PostconditionStatus::NotChecked);
    first.observed_effect = None;
    first.exact_errors = vec!["write failed: permission denied".into()];
    first.satisfied_criterion_ids.clear();
    let failed = evaluate_action_outcome(&task, first).unwrap();
    let succeeded = evaluate_action_outcome(
        &task,
        common::action_input(ToolReturnStatus::Succeeded, PostconditionStatus::Passed),
    )
    .unwrap();
    let claim = evaluate_claim(
        "claim-1",
        "file verified",
        "postcondition",
        true,
        common::claim_evidence(true, false),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task, vec![failed, succeeded], vec![claim]);
    assert_eq!(result.status(), ActionStatus::Succeeded);
    assert!(result.summary().contains("permission denied"));
}

#[test]
fn timeout_after_possible_side_effect_remains_unknown() {
    let task = common::task_contract();
    let mut input = common::action_input(ToolReturnStatus::TimedOut, PostconditionStatus::Unknown);
    input.exact_errors = vec!["request timed out after transmission".into()];
    input.remaining_uncertainty = vec!["remote side effect may have occurred".into()];
    let outcome = evaluate_action_outcome(&task, input).unwrap();
    let result = assemble_authoritative_run_result(&task, vec![outcome], Vec::new());
    assert_eq!(result.status(), ActionStatus::Unknown);
    assert!(result.summary().contains("request timed out"));
    assert!(result.summary().contains("side effect may have occurred"));
}
