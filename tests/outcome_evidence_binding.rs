use ajentic_core::integrity::Digest;
use ajentic_core::outcome::{
    assemble_authoritative_run_result, evaluate_action_outcome, ActionOutcomeInput, ActionStatus,
    CriterionStatus, OutcomeError, PostconditionObservation, PostconditionObservationState,
    ToolReturnStatus,
};
use ajentic_core::task::{
    PostconditionRequirement, RetryPolicy, SuccessCriterion, TaskContract, TaskContractInput,
};

fn task() -> TaskContract {
    TaskContract::new(TaskContractInput {
        task_id: "task-1".into(),
        objective: "satisfy A and B".into(),
        success_criteria: vec![
            SuccessCriterion {
                id: "criterion-c".into(),
                description: "A and B are observed".into(),
                required: true,
                required_postcondition_ids: vec!["post-a".into(), "post-b".into()],
            },
            SuccessCriterion {
                id: "criterion-optional".into(),
                description: "optional observation".into(),
                required: false,
                required_postcondition_ids: vec!["post-optional".into()],
            },
        ],
        forbidden_outcomes: vec!["unbound completion".into()],
        permitted_actions: vec!["observe".into()],
        permitted_tools: vec!["observer".into()],
        non_goals: vec!["external truth".into()],
        required_approval_points: vec!["before_observation".into()],
        side_effect_budget: 1,
        retry_policy: RetryPolicy {
            max_attempts: 3,
            require_idempotency_after_possible_side_effect: true,
        },
        stop_conditions: vec!["criterion_satisfied".into()],
        expected_postconditions: vec![
            postcondition("post-a", true),
            postcondition("post-b", true),
            postcondition("post-optional", false),
        ],
        evidence_requirements: vec!["observation_digest".into()],
    })
    .unwrap()
}

fn postcondition(id: &str, required: bool) -> PostconditionRequirement {
    PostconditionRequirement {
        id: id.into(),
        description: format!("observation for {id}"),
        required,
    }
}

fn observation(id: &str, state: PostconditionObservationState) -> PostconditionObservation {
    let value = format!("observed:{id}");
    PostconditionObservation {
        id: id.into(),
        state,
        observed_value: Some(value.clone()),
        evidence_refs: vec![format!("evidence:{id}")],
        evidence_digests: vec![Digest::of_text(&value)],
    }
}

fn action(
    action_id: &str,
    retries: u32,
    observations: Vec<PostconditionObservation>,
) -> ActionOutcomeInput {
    let evidence_refs = observations
        .iter()
        .flat_map(|item| item.evidence_refs.clone())
        .collect();
    let evidence_digests = observations
        .iter()
        .flat_map(|item| item.evidence_digests.clone())
        .collect();
    ActionOutcomeInput {
        action_id: action_id.into(),
        task_id: "task-1".into(),
        run_id: "run-1".into(),
        action: "observe".into(),
        tool: "observer".into(),
        argument_digest: Digest::of_text(action_id),
        target: "bounded-fixture".into(),
        recipient: None,
        tool_return_status: ToolReturnStatus::Succeeded,
        observed_effect: Some("observation recorded".into()),
        postcondition_observations: observations,
        exact_errors: Vec::new(),
        partial_side_effects: Vec::new(),
        retries,
        compensation: Vec::new(),
        remaining_uncertainty: Vec::new(),
        evidence_refs,
        evidence_digests,
    }
}

#[test]
fn one_of_two_required_postconditions_keeps_completion_unknown() {
    let outcome = evaluate_action_outcome(
        &task(),
        action(
            "action-a",
            0,
            vec![observation(
                "post-a",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task(), vec![outcome], Vec::new());
    assert_eq!(
        result.criterion_outcome("criterion-c").unwrap().status(),
        CriterionStatus::Unknown
    );
    assert_ne!(result.status(), ActionStatus::Succeeded);
    assert!(result
        .unmet_success_criteria()
        .contains(&"criterion-c".into()));
}

#[test]
fn two_actions_collectively_satisfy_mapped_postconditions() {
    let first = evaluate_action_outcome(
        &task(),
        action(
            "action-a",
            0,
            vec![observation(
                "post-a",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let second = evaluate_action_outcome(
        &task(),
        action(
            "action-b",
            0,
            vec![observation(
                "post-b",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task(), vec![first, second], Vec::new());
    assert_eq!(
        result.criterion_outcome("criterion-c").unwrap().status(),
        CriterionStatus::Satisfied
    );
    assert_eq!(result.status(), ActionStatus::Succeeded);
}

#[test]
fn optional_postcondition_cannot_replace_missing_required_postcondition() {
    let outcome = evaluate_action_outcome(
        &task(),
        action(
            "action-optional",
            0,
            vec![
                observation("post-a", PostconditionObservationState::Observed),
                observation("post-optional", PostconditionObservationState::Observed),
            ],
        ),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task(), vec![outcome], Vec::new());
    assert_eq!(
        result.criterion_outcome("criterion-c").unwrap().status(),
        CriterionStatus::Unknown
    );
}

#[test]
fn failed_mapped_postcondition_keeps_criterion_unsatisfied() {
    let first = evaluate_action_outcome(
        &task(),
        action(
            "action-a",
            0,
            vec![observation(
                "post-a",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let second = evaluate_action_outcome(
        &task(),
        action(
            "action-b",
            0,
            vec![observation("post-b", PostconditionObservationState::Failed)],
        ),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task(), vec![first, second], Vec::new());
    assert_eq!(
        result.criterion_outcome("criterion-c").unwrap().status(),
        CriterionStatus::Failed
    );
    assert_ne!(result.status(), ActionStatus::Succeeded);
}

#[test]
fn same_retry_conflict_remains_unknown() {
    let failed = evaluate_action_outcome(
        &task(),
        action(
            "action-a-failed",
            0,
            vec![observation("post-a", PostconditionObservationState::Failed)],
        ),
    )
    .unwrap();
    let passed = evaluate_action_outcome(
        &task(),
        action(
            "action-a-passed",
            0,
            vec![observation(
                "post-a",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let b = evaluate_action_outcome(
        &task(),
        action(
            "action-b",
            0,
            vec![observation(
                "post-b",
                PostconditionObservationState::Observed,
            )],
        ),
    )
    .unwrap();
    let result = assemble_authoritative_run_result(&task(), vec![failed, passed, b], Vec::new());
    assert_eq!(
        result.criterion_outcome("criterion-c").unwrap().status(),
        CriterionStatus::Unknown
    );
}

#[test]
fn observed_value_digest_mismatch_rejects_outcome() {
    let mut observation = observation("post-a", PostconditionObservationState::Observed);
    observation.evidence_digests = vec![Digest::of_text("different")];
    let result = evaluate_action_outcome(&task(), action("action-a", 0, vec![observation]));
    assert_eq!(result, Err(OutcomeError::EvidenceBindingMismatch));
}

#[test]
fn identical_inputs_produce_identical_criterion_results() {
    let build = || {
        let a = evaluate_action_outcome(
            &task(),
            action(
                "action-a",
                0,
                vec![observation(
                    "post-a",
                    PostconditionObservationState::Observed,
                )],
            ),
        )
        .unwrap();
        let b = evaluate_action_outcome(
            &task(),
            action(
                "action-b",
                0,
                vec![observation(
                    "post-b",
                    PostconditionObservationState::Observed,
                )],
            ),
        )
        .unwrap();
        assemble_authoritative_run_result(&task(), vec![a, b], Vec::new())
    };
    assert_eq!(build(), build());
}
