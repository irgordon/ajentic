#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub id: String,
    pub status: ExecutionStatus,
}

impl ExecutionPlan {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: ExecutionStatus::NotStarted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecision {
    Allowed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecisionReason {
    ReadyForExecution,
    LifecycleNotPassed,
    PolicyNotAllowed,
    ValidationNotPassed,
    ReplayNotReady,
}

impl ExecutionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForExecution => "ready_for_execution",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::PolicyNotAllowed => "policy_not_allowed",
            Self::ValidationNotPassed => "validation_not_passed",
            Self::ReplayNotReady => "replay_not_ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionDecisionReport {
    pub decision: ExecutionDecision,
    pub reason: ExecutionDecisionReason,
}

impl ExecutionDecisionReport {
    pub fn allowed() -> Self {
        Self {
            decision: ExecutionDecision::Allowed,
            reason: ExecutionDecisionReason::ReadyForExecution,
        }
    }

    pub fn blocked(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Blocked,
            reason,
        }
    }

    pub fn rejected(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Rejected,
            reason,
        }
    }
}

pub fn decide_execution(
    lifecycle: crate::state::LifecycleState,
    policy: &crate::policy::PolicyResult,
    validation: &crate::validation::ValidationResult,
    replay: &crate::replay::ReplayReport,
) -> ExecutionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed);
    }

    if policy.decision != crate::policy::PolicyDecision::Allowed {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed);
    }

    if validation.status != crate::validation::ValidationStatus::Pass {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed);
    }

    if replay.readiness != crate::replay::ReplayReadiness::Ready {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady);
    }

    ExecutionDecisionReport::allowed()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionDecision {
    Allowed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionDecisionReason {
    ReadyForTier1Promotion,
    LifecycleNotPassed,
    ExecutionNotAllowed,
}

impl PromotionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForTier1Promotion => "ready_for_tier_1_promotion",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::ExecutionNotAllowed => "execution_not_allowed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionDecisionReport {
    pub decision: PromotionDecision,
    pub reason: PromotionDecisionReason,
}

impl PromotionDecisionReport {
    pub fn allowed() -> Self {
        Self {
            decision: PromotionDecision::Allowed,
            reason: PromotionDecisionReason::ReadyForTier1Promotion,
        }
    }

    pub fn blocked(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Blocked,
            reason,
        }
    }

    pub fn rejected(reason: PromotionDecisionReason) -> Self {
        Self {
            decision: PromotionDecision::Rejected,
            reason,
        }
    }
}

pub fn decide_promotion(
    lifecycle: crate::state::LifecycleState,
    execution: &ExecutionDecisionReport,
) -> PromotionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed);
    }

    if execution.decision != ExecutionDecision::Allowed {
        return PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed);
    }

    PromotionDecisionReport::allowed()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionRecordError {
    PromotionNotAllowed,
    LedgerEventInvalid,
}

impl PromotionRecordError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PromotionNotAllowed => "promotion_not_allowed",
            Self::LedgerEventInvalid => "ledger_event_invalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionRecord {
    pub event: crate::ledger::LedgerEvent,
}

pub fn build_promotion_record(
    event_id: impl Into<String>,
    revision: u64,
    actor: crate::ledger::LedgerActor,
    evidence_refs: Vec<String>,
    payload_summary: impl Into<String>,
    promotion: &PromotionDecisionReport,
) -> Result<PromotionRecord, PromotionRecordError> {
    if promotion.decision != PromotionDecision::Allowed {
        return Err(PromotionRecordError::PromotionNotAllowed);
    }

    let payload = crate::ledger::LedgerPayload::with_lifecycle_transition(
        payload_summary,
        crate::state::LifecycleState::PromotedTier1,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;

    let event = crate::ledger::LedgerEvent::new(
        event_id,
        revision,
        crate::ledger::LedgerEventType::StateTransition,
        actor,
        evidence_refs,
        payload,
    )
    .map_err(|_| PromotionRecordError::LedgerEventInvalid)?;

    Ok(PromotionRecord { event })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionAppendError {
    LedgerAppendFailed,
}

impl PromotionAppendError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LedgerAppendFailed => "ledger_append_failed",
        }
    }
}

pub fn append_promotion_record(
    ledger: &crate::ledger::Ledger,
    record: PromotionRecord,
) -> Result<crate::ledger::Ledger, PromotionAppendError> {
    ledger
        .append(record.event)
        .map_err(|_| PromotionAppendError::LedgerAppendFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{PolicyDecision, PolicyResult};
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::LifecycleState;
    use crate::validation::{ValidationResult, ValidationStatus};

    fn ready_policy() -> PolicyResult {
        PolicyResult::allowed("required_policy_evidence_present")
    }

    fn ready_validation() -> ValidationResult {
        ValidationResult::pass("validation_evidence_passed")
    }

    fn ready_replay() -> ReplayReport {
        ReplayReport::replayable(2)
    }

    #[test]
    fn execution_decision_reason_codes_are_stable() {
        assert_eq!(
            ExecutionDecisionReason::ReadyForExecution.code(),
            "ready_for_execution"
        );
        assert_eq!(
            ExecutionDecisionReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::PolicyNotAllowed.code(),
            "policy_not_allowed"
        );
        assert_eq!(
            ExecutionDecisionReason::ValidationNotPassed.code(),
            "validation_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::ReplayNotReady.code(),
            "replay_not_ready"
        );
    }

    #[test]
    fn execution_allows_when_all_inputs_are_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(report, ExecutionDecisionReport::allowed());
    }

    #[test]
    fn execution_rejects_when_lifecycle_not_passed() {
        let report = decide_execution(
            LifecycleState::Created,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_policy_not_allowed() {
        let report = decide_execution(
            LifecycleState::Passed,
            &PolicyResult::unknown(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_blocks_when_validation_not_passed() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_replay_not_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ReplayReport::unknown(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady)
        );
    }

    #[test]
    fn execution_priority_lifecycle_before_policy() {
        let report = decide_execution(
            LifecycleState::Created,
            &PolicyResult::unknown(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_priority_policy_before_validation() {
        let report = decide_execution(
            LifecycleState::Passed,
            &PolicyResult::unknown(),
            &ValidationResult::unknown(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_priority_validation_before_replay() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
            &ReplayReport::unknown(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_does_not_require_reason_strings_for_decision() {
        let policy_a = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_a",
        };
        let policy_b = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_b",
        };

        let validation_a = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_a",
        };
        let validation_b = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_b",
        };

        let replay_a = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 1,
            reason: "replay_reason_a",
        };
        let replay_b = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 99,
            reason: "replay_reason_b",
        };

        let report_a =
            decide_execution(LifecycleState::Passed, &policy_a, &validation_a, &replay_a);
        let report_b =
            decide_execution(LifecycleState::Passed, &policy_b, &validation_b, &replay_b);

        assert_eq!(report_a, ExecutionDecisionReport::allowed());
        assert_eq!(report_b, ExecutionDecisionReport::allowed());
        assert_eq!(report_a, report_b);
    }

    #[test]
    fn promotion_decision_reason_codes_are_stable() {
        assert_eq!(
            PromotionDecisionReason::ReadyForTier1Promotion.code(),
            "ready_for_tier_1_promotion"
        );
        assert_eq!(
            PromotionDecisionReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
        assert_eq!(
            PromotionDecisionReason::ExecutionNotAllowed.code(),
            "execution_not_allowed"
        );
    }

    #[test]
    fn promotion_allows_when_lifecycle_passed_and_execution_allowed() {
        let report = decide_promotion(LifecycleState::Passed, &ExecutionDecisionReport::allowed());

        assert_eq!(report, PromotionDecisionReport::allowed());
    }

    #[test]
    fn promotion_rejects_when_lifecycle_created() {
        let report = decide_promotion(LifecycleState::Created, &ExecutionDecisionReport::allowed());

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_rejects_when_lifecycle_promoted_tier_1() {
        let report = decide_promotion(
            LifecycleState::PromotedTier1,
            &ExecutionDecisionReport::allowed(),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_blocks_when_execution_blocked() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed)
        );
    }

    #[test]
    fn promotion_blocks_when_execution_rejected() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed)
        );
    }

    #[test]
    fn promotion_priority_lifecycle_before_execution() {
        let report = decide_promotion(
            LifecycleState::Created,
            &ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed),
        );

        assert_eq!(
            report,
            PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn promotion_decision_uses_execution_decision_not_reason() {
        let report = decide_promotion(
            LifecycleState::Passed,
            &ExecutionDecisionReport {
                decision: ExecutionDecision::Allowed,
                reason: ExecutionDecisionReason::PolicyNotAllowed,
            },
        );

        assert_eq!(report, PromotionDecisionReport::allowed());
    }

    #[test]
    fn promotion_does_not_mutate_or_transition_harness_state() {
        let state = crate::state::HarnessState {
            lifecycle: LifecycleState::Passed,
            revision: 7,
        };

        let report = decide_promotion(state.lifecycle, &ExecutionDecisionReport::allowed());

        assert_eq!(report, PromotionDecisionReport::allowed());
        assert_eq!(state.lifecycle, LifecycleState::Passed);
        assert_eq!(state.revision, 7);
    }

    fn ledger_actor() -> crate::ledger::LedgerActor {
        crate::ledger::LedgerActor::new(crate::ledger::LedgerActorType::Human, "operator-1")
            .expect("actor should be valid")
    }

    #[test]
    fn promotion_record_error_codes_are_stable() {
        assert_eq!(
            PromotionRecordError::PromotionNotAllowed.code(),
            "promotion_not_allowed"
        );
        assert_eq!(
            PromotionRecordError::LedgerEventInvalid.code(),
            "ledger_event_invalid"
        );
    }

    #[test]
    fn promotion_record_rejects_blocked_promotion() {
        let result = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::blocked(PromotionDecisionReason::ExecutionNotAllowed),
        );

        assert_eq!(result, Err(PromotionRecordError::PromotionNotAllowed));
    }

    #[test]
    fn promotion_record_rejects_rejected_promotion() {
        let result = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::rejected(PromotionDecisionReason::LifecycleNotPassed),
        );

        assert_eq!(result, Err(PromotionRecordError::PromotionNotAllowed));
    }

    #[test]
    fn promotion_record_builds_state_transition_event_for_allowed_promotion() {
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(
            record.event.event_type,
            crate::ledger::LedgerEventType::StateTransition
        );
    }

    #[test]
    fn promotion_record_payload_targets_promoted_tier_1() {
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(
            record.event.payload.lifecycle_transition,
            Some(LifecycleState::PromotedTier1)
        );
    }

    #[test]
    fn promotion_record_preserves_caller_revision() {
        let record = build_promotion_record(
            "evt-1",
            42,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.revision, 42);
    }

    #[test]
    fn promotion_record_preserves_caller_actor() {
        let actor = ledger_actor();
        let record = build_promotion_record(
            "evt-1",
            1,
            actor.clone(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.actor.id, actor.id);
        assert_eq!(record.event.actor.actor_type, actor.actor_type);
    }

    #[test]
    fn promotion_record_preserves_evidence_refs() {
        let evidence_refs = vec![
            "evidence-1".to_string(),
            "evidence-2".to_string(),
            "evidence-3".to_string(),
        ];
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            evidence_refs.clone(),
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(record.event.evidence_refs, evidence_refs);
    }

    #[test]
    fn promotion_record_fails_on_invalid_ledger_event() {
        let blocked = build_promotion_record(
            "",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(blocked, Err(PromotionRecordError::LedgerEventInvalid));

        let zero_revision = build_promotion_record(
            "evt-1",
            0,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(zero_revision, Err(PromotionRecordError::LedgerEventInvalid));

        let no_evidence = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec![],
            "summary",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(no_evidence, Err(PromotionRecordError::LedgerEventInvalid));

        let empty_summary = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "",
            &PromotionDecisionReport::allowed(),
        );
        assert_eq!(empty_summary, Err(PromotionRecordError::LedgerEventInvalid));
    }

    #[test]
    fn promotion_record_uses_decision_not_reason() {
        let report = PromotionDecisionReport {
            decision: PromotionDecision::Allowed,
            reason: PromotionDecisionReason::LifecycleNotPassed,
        };
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &report,
        );

        assert!(record.is_ok());
    }

    #[test]
    fn promotion_record_does_not_append_to_ledger() {
        let ledger = crate::ledger::Ledger::empty();
        let _ = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert!(ledger.events().is_empty());
    }

    #[test]
    fn promotion_record_does_not_transition_harness_state() {
        let state = crate::state::HarnessState {
            revision: 10,
            lifecycle: LifecycleState::Passed,
        };

        let _ = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("allowed promotion should build record");

        assert_eq!(state.revision, 10);
        assert_eq!(state.lifecycle, LifecycleState::Passed);
    }
    #[test]
    fn promotion_append_error_codes_are_stable() {
        assert_eq!(
            PromotionAppendError::LedgerAppendFailed.code(),
            "ledger_append_failed"
        );
    }

    #[test]
    fn promotion_append_succeeds_on_empty_ledger_with_revision_one() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(ledger.events().len(), 0);
        assert_eq!(next.events().len(), 1);
    }

    #[test]
    fn promotion_append_preserves_event_shape() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");
        let appended = &next.events()[0];

        assert_eq!(
            appended.event_type,
            crate::ledger::LedgerEventType::StateTransition
        );
        assert_eq!(
            appended.payload.lifecycle_transition,
            Some(LifecycleState::PromotedTier1)
        );
    }

    #[test]
    fn promotion_append_preserves_existing_ledger_order() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-2",
            2,
            actor,
            vec!["evidence-2".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let next = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(next.events().len(), 2);
        assert_eq!(next.events()[0].id, "evt-seed");
        assert_eq!(next.events()[1].id, "evt-2");
    }

    #[test]
    fn promotion_append_requires_valid_next_revision() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-3",
            3,
            actor,
            vec!["evidence-3".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let result = append_promotion_record(&ledger, record);

        assert_eq!(result, Err(PromotionAppendError::LedgerAppendFailed));
    }

    #[test]
    fn promotion_append_failure_does_not_mutate_ledger() {
        let actor = ledger_actor();
        let payload = crate::ledger::LedgerPayload::new("seed-summary").expect("payload valid");
        let seed_event = crate::ledger::LedgerEvent::new(
            "evt-seed",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor.clone(),
            vec!["evidence-seed".to_string()],
            payload,
        )
        .expect("seed event valid");
        let ledger = crate::ledger::Ledger::empty()
            .append(seed_event)
            .expect("seed append should succeed");

        let record = build_promotion_record(
            "evt-3",
            3,
            actor,
            vec!["evidence-3".to_string()],
            "promotion-summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let _ = append_promotion_record(&ledger, record);

        assert_eq!(ledger.events().len(), 1);
        assert_eq!(ledger.events()[0].id, "evt-seed");
        assert_eq!(ledger.events()[0].revision, 1);
    }

    #[test]
    fn promotion_append_does_not_transition_harness_state() {
        let state = crate::state::HarnessState {
            revision: 8,
            lifecycle: LifecycleState::Passed,
        };
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let _ = append_promotion_record(&ledger, record).expect("append should succeed");

        assert_eq!(state.lifecycle, LifecycleState::Passed);
        assert_eq!(state.revision, 8);
    }

    #[test]
    fn promotion_append_does_not_require_replay_readiness() {
        let ledger = crate::ledger::Ledger::empty();
        let record = build_promotion_record(
            "evt-1",
            1,
            ledger_actor(),
            vec!["evidence-1".to_string()],
            "summary",
            &PromotionDecisionReport::allowed(),
        )
        .expect("record should build");

        let result = append_promotion_record(&ledger, record);

        assert!(result.is_ok());
    }
}
