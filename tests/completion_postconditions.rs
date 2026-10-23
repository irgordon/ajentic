mod common;

use ajentic_core::outcome::{
    assemble_authoritative_run_result, evaluate_action_outcome, evaluate_claim, ActionStatus,
    PostconditionObservationState, ToolReturnStatus,
};

#[test]
fn tool_success_without_read_back_is_not_completion() {
    let task = common::task_contract();
    let outcome = evaluate_action_outcome(
        &task,
        common::action_input(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::NotChecked,
        ),
    )
    .unwrap();
    assert_eq!(outcome.status(), ActionStatus::Unknown);
}

#[test]
fn failed_mandatory_postcondition_prevents_completion() {
    let task = common::task_contract();
    let outcome = evaluate_action_outcome(
        &task,
        common::action_input(
            ToolReturnStatus::Succeeded,
            PostconditionObservationState::Failed,
        ),
    )
    .unwrap();
    let claim = evaluate_claim(
        "claim-1",
        "file verified",
        "postcondition",
        true,
        common::claim_evidence(false, true),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task, vec![outcome], vec![claim]);
    assert_ne!(result.status(), ActionStatus::Succeeded);
    assert!(result
        .unmet_success_criteria()
        .contains(&"criterion-file".into()));
}
