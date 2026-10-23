mod common;

use ajentic_core::outcome::{
    assemble_authoritative_run_result, evaluate_action_outcome, evaluate_claim, ActionStatus,
    PostconditionObservationState, ToolReturnStatus,
};

#[test]
fn later_success_does_not_erase_earlier_error() {
    let task = common::task_contract();
    let mut first = common::action_input(
        ToolReturnStatus::Failed,
        PostconditionObservationState::NotChecked,
    );
    first.observed_effect = None;
    first.exact_errors = vec!["write failed: permission denied".into()];
    let failed = evaluate_action_outcome(&task, first).unwrap();
    let mut retry = common::action_input(
        ToolReturnStatus::Succeeded,
        PostconditionObservationState::Observed,
    );
    retry.retries = 1;
    let succeeded = evaluate_action_outcome(&task, retry).unwrap();
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
    let mut input = common::action_input(
        ToolReturnStatus::TimedOut,
        PostconditionObservationState::Unknown,
    );
    input.exact_errors = vec!["request timed out after transmission".into()];
    input.remaining_uncertainty = vec!["remote side effect may have occurred".into()];
    let outcome = evaluate_action_outcome(&task, input).unwrap();
    let result = assemble_authoritative_run_result(&task, vec![outcome], Vec::new());
    assert_eq!(result.status(), ActionStatus::Unknown);
    assert!(result.summary().contains("request timed out"));
    assert!(result.summary().contains("side effect may have occurred"));
}
