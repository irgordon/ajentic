#![allow(dead_code)]

use ajentic_core::authority::{
    AuthorityBinding, AuthorityBindingInput, EvidenceManifest, EvidenceReference,
};
use ajentic_core::execution::AuthorityEvaluationEvidence;
use ajentic_core::integrity::Digest;
use ajentic_core::ledger::{
    Ledger, LedgerActor, LedgerActorType, LedgerEvent, LedgerEventType, LedgerPayload, LedgerSeal,
};
use ajentic_core::outcome::{
    ActionOutcomeInput, ClaimEvidence, PostconditionCheck, PostconditionStatus, ToolReturnStatus,
};
use ajentic_core::policy::{evaluate_policy, PolicyEvidence, PolicyReceipt};
use ajentic_core::replay::{verify_replay_receipt, ReplayReceipt};
use ajentic_core::state::LifecycleState;
use ajentic_core::task::{
    PostconditionRequirement, RetryPolicy, SuccessCriterion, TaskContract, TaskContractInput,
};
use ajentic_core::validation::{evaluate_validation, ValidationEvidence, ValidationReceipt};

pub struct ReceiptBundle {
    pub binding: AuthorityBinding,
    pub manifest: EvidenceManifest,
    pub validation: ValidationReceipt,
    pub policy: PolicyReceipt,
    pub replay: ReplayReceipt,
    pub ledger: Ledger,
    pub evaluation_evidence: AuthorityEvaluationEvidence,
}

pub fn receipt_bundle(run_id: &str, candidate: &str) -> ReceiptBundle {
    let manifest = evidence_manifest();
    let binding = authority_binding(run_id, candidate, &manifest, 2);
    let evaluation_evidence = AuthorityEvaluationEvidence::new(
        ValidationEvidence::new(true, true, true, false, manifest.clone()),
        PolicyEvidence::new(true, true, false),
    );
    let validation = evaluate_validation(binding.clone(), evaluation_evidence.validation());
    let policy = evaluate_policy(binding.clone(), evaluation_evidence.policy(), &validation);
    let ledger = sealed_passed_ledger(&binding, &validation, &policy);
    let replay = verify_replay_receipt(binding.clone(), &ledger, &manifest).unwrap();
    ReceiptBundle {
        binding,
        manifest,
        validation,
        policy,
        replay,
        ledger,
        evaluation_evidence,
    }
}

pub fn evidence_manifest() -> EvidenceManifest {
    EvidenceManifest::new(vec![
        EvidenceReference::new("evidence-1", Digest::of_text("created-to-evaluating")).unwrap(),
        EvidenceReference::new("evidence-2", Digest::of_text("evaluating-to-passed")).unwrap(),
    ])
    .unwrap()
}

pub fn authority_binding(
    run_id: &str,
    candidate: &str,
    manifest: &EvidenceManifest,
    revision: u64,
) -> AuthorityBinding {
    AuthorityBinding::new(AuthorityBindingInput {
        run_id: run_id.into(),
        task_digest: Digest::of_text("task"),
        operator_intent_digest: Digest::of_text("intent"),
        context_packet_digest: Digest::of_text("context"),
        candidate_digest: Digest::of_text(candidate),
        policy_bundle_digest: Digest::of_text("policy"),
        evidence_manifest_digest: manifest.digest().clone(),
        verifier_id: "test-verifier".into(),
        verifier_version: "1.0.0".into(),
        valid_through_revision: revision,
    })
    .unwrap()
}

pub fn passing_validation(
    binding: AuthorityBinding,
    manifest: EvidenceManifest,
) -> ValidationReceipt {
    evaluate_validation(
        binding,
        &ValidationEvidence::new(true, true, true, false, manifest),
    )
}

pub fn passing_policy(binding: AuthorityBinding, validation: &ValidationReceipt) -> PolicyReceipt {
    evaluate_policy(binding, &PolicyEvidence::new(true, true, false), validation)
}

pub fn raw_passed_ledger() -> Ledger {
    Ledger::empty()
        .append(state_event("event-1", 1, LifecycleState::Evaluating))
        .unwrap()
        .append(state_event("event-2", 2, LifecycleState::Passed))
        .unwrap()
}

pub fn sealed_passed_ledger(
    binding: &AuthorityBinding,
    validation: &ValidationReceipt,
    policy: &PolicyReceipt,
) -> Ledger {
    raw_passed_ledger()
        .seal(&LedgerSeal {
            binding: binding.clone(),
            actor_authorization_ref: "actor-authorization".into(),
            validation_receipt_ref: validation.digest().as_str().into(),
            policy_receipt_ref: policy.digest().as_str().into(),
            schema_version: "v1.0.0".into(),
            verifier_version: "1.0.0".into(),
        })
        .unwrap()
}

pub fn state_event(id: &str, revision: u64, lifecycle: LifecycleState) -> LedgerEvent {
    LedgerEvent::new(
        id,
        revision,
        LedgerEventType::StateTransition,
        LedgerActor::new(LedgerActorType::System, "system").unwrap(),
        vec![format!("evidence-{revision}")],
        LedgerPayload::with_lifecycle_transition("transition", lifecycle).unwrap(),
    )
    .unwrap()
}

pub fn task_contract() -> TaskContract {
    TaskContract::new(TaskContractInput {
        task_id: "task-1".into(),
        objective: "write and verify a file".into(),
        success_criteria: vec![SuccessCriterion {
            id: "criterion-file".into(),
            description: "file digest matches".into(),
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
    })
    .unwrap()
}

pub fn action_input(
    tool_status: ToolReturnStatus,
    postcondition_status: PostconditionStatus,
) -> ActionOutcomeInput {
    ActionOutcomeInput {
        action_id: "action-1".into(),
        task_id: "task-1".into(),
        run_id: "run-1".into(),
        action: "write_file".into(),
        tool: "filesystem".into(),
        argument_digest: Digest::of_text("path=file;content=expected"),
        target: "workspace/file".into(),
        recipient: None,
        tool_return_status: tool_status,
        observed_effect: Some("file exists".into()),
        postconditions: vec![PostconditionCheck {
            id: "postcondition-file".into(),
            required: true,
            status: postcondition_status,
            observed_value: Some("sha256:expected".into()),
            evidence_refs: vec!["read-back-1".into()],
        }],
        exact_errors: Vec::new(),
        partial_side_effects: Vec::new(),
        retries: 0,
        compensation: Vec::new(),
        remaining_uncertainty: Vec::new(),
        evidence_refs: vec!["read-back-1".into()],
        satisfied_criterion_ids: vec!["criterion-file".into()],
    }
}

pub fn claim_evidence(supported: bool, contradicted: bool) -> ClaimEvidence {
    ClaimEvidence {
        supported,
        contradicted,
        not_applicable: false,
        evidence_refs: if supported {
            vec!["read-back-1".into()]
        } else {
            Vec::new()
        },
        source_identity: "filesystem-read-back".into(),
        source_version: "1".into(),
        verifier_id: "postcondition-verifier".into(),
        verifier_version: "1".into(),
        assumptions: Vec::new(),
        contradictions: if contradicted {
            vec!["digest mismatch".into()]
        } else {
            Vec::new()
        },
        uncertainty_reason: if supported || contradicted {
            None
        } else {
            Some("no evidence".into())
        },
    }
}
