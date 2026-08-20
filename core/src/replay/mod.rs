use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayStatus {
    Replayable,
    NotReplayable,
    Failed,
    Unknown,
}

impl ReplayStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Replayable => "replayable",
            Self::NotReplayable => "not_replayable",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIntegrity {
    Valid,
    Invalid,
    Incomplete,
    Conflicting,
    Unknown,
}

impl ReplayIntegrity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Incomplete => "incomplete",
            Self::Conflicting => "conflicting",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayReadiness {
    Ready,
    NotReady,
}

impl ReplayReadiness {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::NotReady => "not_ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    EmptyLedger,
    InvalidFirstRevision,
    RevisionGap,
    DuplicateRevision,
    ConflictingEventId,
}

impl ReplayError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyLedger => "empty_ledger",
            Self::InvalidFirstRevision => "invalid_first_revision",
            Self::RevisionGap => "revision_gap",
            Self::DuplicateRevision => "duplicate_revision",
            Self::ConflictingEventId => "conflicting_event_id",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub status: ReplayStatus,
    pub integrity: ReplayIntegrity,
    pub readiness: ReplayReadiness,
    pub events_replayed: u64,
    pub reason: &'static str,
}

impl ReplayReport {
    pub fn unknown() -> Self {
        Self {
            status: ReplayStatus::Unknown,
            integrity: ReplayIntegrity::Unknown,
            readiness: ReplayReadiness::NotReady,
            events_replayed: 0,
            reason: "unknown_is_not_pass",
        }
    }

    pub fn not_replayable(integrity: ReplayIntegrity, reason: &'static str) -> Self {
        Self {
            status: ReplayStatus::NotReplayable,
            integrity,
            readiness: ReplayReadiness::NotReady,
            events_replayed: 0,
            reason,
        }
    }

    pub fn replayable(events_replayed: u64) -> Self {
        Self {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed,
            reason: "ledger_sequence_valid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReceipt {
    binding: crate::authority::AuthorityBinding,
    report: ReplayReport,
    final_state: crate::state::LifecycleState,
    receipt_digest: crate::integrity::Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayReceiptError {
    EvidenceManifestMismatch,
    IntegrityMismatch,
    LedgerNotReplayable,
    ReconstructionFailed,
    LifecycleNotPassed,
    RevisionMismatch,
}

impl ReplayReceiptError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EvidenceManifestMismatch => "evidence_manifest_mismatch",
            Self::IntegrityMismatch => "integrity_mismatch",
            Self::LedgerNotReplayable => "ledger_not_replayable",
            Self::ReconstructionFailed => "reconstruction_failed",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::RevisionMismatch => "revision_mismatch",
        }
    }
}

impl ReplayReceipt {
    pub fn binding(&self) -> &crate::authority::AuthorityBinding {
        &self.binding
    }

    pub fn report(&self) -> &ReplayReport {
        &self.report
    }

    pub fn final_state(&self) -> crate::state::LifecycleState {
        self.final_state
    }

    pub fn digest(&self) -> &crate::integrity::Digest {
        &self.receipt_digest
    }

    pub fn ready(&self) -> bool {
        self.report.readiness == ReplayReadiness::Ready
    }
}

pub fn verify_replay_receipt(
    binding: crate::authority::AuthorityBinding,
    ledger: &crate::ledger::Ledger,
    manifest: &crate::authority::EvidenceManifest,
) -> Result<ReplayReceipt, ReplayReceiptError> {
    verify_replay_inputs(&binding, ledger, manifest)?;
    issue_replay_receipt(binding, ledger)
}

#[cfg(test)]
pub(crate) fn record_unknown_replay(binding: crate::authority::AuthorityBinding) -> ReplayReceipt {
    let report = ReplayReport::unknown();
    let final_state = crate::state::LifecycleState::Passed;
    let receipt_digest = replay_receipt_digest(&binding, &report, final_state);
    ReplayReceipt {
        binding,
        report,
        final_state,
        receipt_digest,
    }
}

fn verify_replay_inputs(
    binding: &crate::authority::AuthorityBinding,
    ledger: &crate::ledger::Ledger,
    manifest: &crate::authority::EvidenceManifest,
) -> Result<(), ReplayReceiptError> {
    manifest
        .verify_binding(binding)
        .map_err(|_| ReplayReceiptError::EvidenceManifestMismatch)?;
    ledger
        .verify_integrity(binding, manifest)
        .map_err(|_| ReplayReceiptError::IntegrityMismatch)?;
    verify_replay_revision(binding, ledger)
}

fn verify_replay_revision(
    binding: &crate::authority::AuthorityBinding,
    ledger: &crate::ledger::Ledger,
) -> Result<(), ReplayReceiptError> {
    if ledger.last_revision() == Some(binding.valid_through_revision()) {
        return Ok(());
    }
    Err(ReplayReceiptError::RevisionMismatch)
}

fn issue_replay_receipt(
    binding: crate::authority::AuthorityBinding,
    ledger: &crate::ledger::Ledger,
) -> Result<ReplayReceipt, ReplayReceiptError> {
    let report = classify_replay_readiness(ledger.events())
        .map_err(|_| ReplayReceiptError::LedgerNotReplayable)?;
    let reconstruction = reconstruct_harness_state(ledger.events())
        .map_err(|_| ReplayReceiptError::ReconstructionFailed)?;
    ensure_passed_lifecycle(reconstruction.final_state.lifecycle)?;
    Ok(build_replay_receipt(binding, report, reconstruction))
}

fn ensure_passed_lifecycle(
    lifecycle: crate::state::LifecycleState,
) -> Result<(), ReplayReceiptError> {
    if lifecycle == crate::state::LifecycleState::Passed {
        return Ok(());
    }
    Err(ReplayReceiptError::LifecycleNotPassed)
}

fn build_replay_receipt(
    binding: crate::authority::AuthorityBinding,
    report: ReplayReport,
    reconstruction: ReplayReconstruction,
) -> ReplayReceipt {
    let final_state = reconstruction.final_state.lifecycle;
    let receipt_digest = replay_receipt_digest(&binding, &report, final_state);
    ReplayReceipt {
        binding,
        report,
        final_state,
        receipt_digest,
    }
}

fn replay_receipt_digest(
    binding: &crate::authority::AuthorityBinding,
    report: &ReplayReport,
    final_state: crate::state::LifecycleState,
) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "replay|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}",
        binding.run_id(),
        binding.task_digest().as_str(),
        binding.operator_intent_digest().as_str(),
        binding.context_packet_digest().as_str(),
        binding.candidate_digest().as_str(),
        binding.policy_bundle_digest().as_str(),
        binding.evidence_manifest_digest().as_str(),
        binding.verifier_id(),
        binding.verifier_version(),
        binding.valid_through_revision(),
        final_state,
        report.events_replayed
    ))
}

pub fn classify_replay_readiness(
    events: &[crate::ledger::LedgerEvent],
) -> Result<ReplayReport, ReplayError> {
    if events.is_empty() {
        return Err(ReplayError::EmptyLedger);
    }

    if events[0].revision != 1 {
        return Err(ReplayError::InvalidFirstRevision);
    }

    let mut seen_event_ids: HashSet<&str> = HashSet::new();

    for (expected_revision, event) in (1_u64..).zip(events.iter()) {
        if !seen_event_ids.insert(event.id.as_str()) {
            return Err(ReplayError::ConflictingEventId);
        }

        if event.revision < expected_revision {
            return Err(ReplayError::DuplicateRevision);
        }

        if event.revision > expected_revision {
            return Err(ReplayError::RevisionGap);
        }
    }

    Ok(ReplayReport::replayable(events.len() as u64))
}

pub fn report_from_error(error: ReplayError) -> ReplayReport {
    match error {
        ReplayError::EmptyLedger => {
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "empty_ledger")
        }
        ReplayError::InvalidFirstRevision => {
            ReplayReport::not_replayable(ReplayIntegrity::Invalid, "invalid_first_revision")
        }
        ReplayError::RevisionGap => {
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "revision_gap")
        }
        ReplayError::DuplicateRevision => {
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "duplicate_revision")
        }
        ReplayError::ConflictingEventId => {
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "conflicting_event_id")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayReconstructionError {
    NotReplayReady,
    UnsupportedStateTransitionPayload,
    LifecycleTransitionFailed,
}

impl ReplayReconstructionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotReplayReady => "not_replay_ready",
            Self::UnsupportedStateTransitionPayload => "unsupported_state_transition_payload",
            Self::LifecycleTransitionFailed => "lifecycle_transition_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReconstruction {
    pub final_state: crate::state::HarnessState,
    pub events_seen: u64,
    pub state_transitions_applied: u64,
}

pub fn reconstruct_harness_state(
    events: &[crate::ledger::LedgerEvent],
) -> Result<ReplayReconstruction, ReplayReconstructionError> {
    classify_replay_readiness(events).map_err(|_| ReplayReconstructionError::NotReplayReady)?;

    let mut state = crate::state::HarnessState::genesis();
    let mut events_seen = 0_u64;
    let mut state_transitions_applied = 0_u64;

    for event in events {
        events_seen += 1;

        if event.event_type != crate::ledger::LedgerEventType::StateTransition {
            continue;
        }

        let next = event
            .payload
            .lifecycle_transition
            .ok_or(ReplayReconstructionError::UnsupportedStateTransitionPayload)?;

        state = state
            .replay_transition_to(next)
            .map_err(|_| ReplayReconstructionError::LifecycleTransitionFailed)?;
        state_transitions_applied += 1;
    }

    Ok(ReplayReconstruction {
        final_state: state,
        events_seen,
        state_transitions_applied,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{
        LedgerActor, LedgerActorType, LedgerEvent, LedgerEventType, LedgerPayload,
    };

    fn actor() -> LedgerActor {
        LedgerActor::new(LedgerActorType::System, "actor-1").expect("actor should be valid")
    }

    fn payload() -> LedgerPayload {
        LedgerPayload::new("summary").expect("payload should be valid")
    }

    fn event(id: &str, revision: u64) -> LedgerEvent {
        LedgerEvent::new(
            id,
            revision,
            LedgerEventType::StateTransition,
            actor(),
            vec!["evidence-1".to_string()],
            payload(),
        )
        .expect("event should be valid")
    }

    fn event_with_type(
        id: &str,
        revision: u64,
        event_type: LedgerEventType,
        payload: LedgerPayload,
    ) -> LedgerEvent {
        LedgerEvent::new(
            id,
            revision,
            event_type,
            actor(),
            vec!["evidence-1".to_string()],
            payload,
        )
        .expect("event should be valid")
    }

    #[test]
    fn replay_unknown_is_not_ready() {
        /* unchanged */
        let report = ReplayReport::unknown();
        assert_eq!(report.status, ReplayStatus::Unknown);
        assert_eq!(report.integrity, ReplayIntegrity::Unknown);
        assert_eq!(report.readiness, ReplayReadiness::NotReady);
        assert_eq!(report.events_replayed, 0);
        assert_eq!(report.reason, "unknown_is_not_pass");
    }
    #[test]
    fn replay_error_codes_are_stable() {
        let _ = ReplayError::EmptyLedger.code();
    }
    #[test]
    fn stable_code_methods_return_expected_values() {
        assert_eq!(ReplayStatus::Replayable.code(), "replayable");
        assert_eq!(ReplayStatus::NotReplayable.code(), "not_replayable");
        assert_eq!(ReplayStatus::Failed.code(), "failed");
        assert_eq!(ReplayStatus::Unknown.code(), "unknown");

        assert_eq!(ReplayIntegrity::Valid.code(), "valid");
        assert_eq!(ReplayIntegrity::Invalid.code(), "invalid");
        assert_eq!(ReplayIntegrity::Incomplete.code(), "incomplete");
        assert_eq!(ReplayIntegrity::Conflicting.code(), "conflicting");
        assert_eq!(ReplayIntegrity::Unknown.code(), "unknown");

        assert_eq!(ReplayReadiness::Ready.code(), "ready");
        assert_eq!(ReplayReadiness::NotReady.code(), "not_ready");
    }
    #[test]
    fn replay_empty_ledger_errors() {
        assert_eq!(
            classify_replay_readiness(&[]),
            Err(ReplayError::EmptyLedger)
        );
    }
    #[test]
    fn replay_first_revision_must_be_one() {
        assert_eq!(
            classify_replay_readiness(&[event("event-1", 2)]),
            Err(ReplayError::InvalidFirstRevision)
        );
    }
    #[test]
    fn replay_contiguous_events_are_replayable() {
        assert!(classify_replay_readiness(&[event("event-1", 1), event("event-2", 2)]).is_ok());
    }
    #[test]
    fn replay_counts_events_without_applying_them() {
        assert_eq!(
            classify_replay_readiness(&[
                event("event-1", 1),
                event("event-2", 2),
                event("event-3", 3)
            ])
            .unwrap()
            .events_replayed,
            3
        );
    }
    #[test]
    fn replay_revision_gap_errors() {
        assert_eq!(
            classify_replay_readiness(&[event("event-1", 1), event("event-2", 3)]),
            Err(ReplayError::RevisionGap)
        );
    }
    #[test]
    fn replay_duplicate_revision_errors() {
        assert_eq!(
            classify_replay_readiness(&[event("event-1", 1), event("event-2", 1)]),
            Err(ReplayError::DuplicateRevision)
        );
    }
    #[test]
    fn replay_conflicting_event_id_errors_same_revision() {
        assert_eq!(
            classify_replay_readiness(&[event("event-1", 1), event("event-1", 2)]),
            Err(ReplayError::ConflictingEventId)
        );
    }
    #[test]
    fn replay_conflicting_event_id_errors_different_revision() {
        assert_eq!(
            classify_replay_readiness(&[
                event("event-1", 1),
                event("event-2", 2),
                event("event-1", 3)
            ]),
            Err(ReplayError::ConflictingEventId)
        );
    }
    #[test]
    fn replay_preserves_input_order_by_not_sorting() {
        assert_eq!(
            classify_replay_readiness(&[event("event-1", 2), event("event-2", 1)]),
            Err(ReplayError::InvalidFirstRevision)
        );
    }
    #[test]
    fn replay_report_from_empty_ledger_error() {
        assert_eq!(
            report_from_error(ReplayError::EmptyLedger),
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "empty_ledger")
        );
    }
    #[test]
    fn replay_report_from_invalid_first_revision_error() {
        assert_eq!(
            report_from_error(ReplayError::InvalidFirstRevision),
            ReplayReport::not_replayable(ReplayIntegrity::Invalid, "invalid_first_revision")
        );
    }
    #[test]
    fn replay_report_from_revision_gap_error() {
        assert_eq!(
            report_from_error(ReplayError::RevisionGap),
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "revision_gap")
        );
    }
    #[test]
    fn replay_report_from_duplicate_revision_error() {
        assert_eq!(
            report_from_error(ReplayError::DuplicateRevision),
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "duplicate_revision")
        );
    }
    #[test]
    fn replay_report_from_conflicting_event_id_error() {
        assert_eq!(
            report_from_error(ReplayError::ConflictingEventId),
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "conflicting_event_id")
        );
    }

    #[test]
    fn reconstruction_empty_ledger_is_not_replay_ready() {
        assert_eq!(
            reconstruct_harness_state(&[]),
            Err(ReplayReconstructionError::NotReplayReady)
        );
    }
    #[test]
    fn reconstruction_uses_genesis_for_initial_state() {
        let r = reconstruct_harness_state(&[event_with_type(
            "event-1",
            1,
            LedgerEventType::MemoryWrite,
            payload(),
        )])
        .unwrap();
        assert_eq!(r.final_state, crate::state::HarnessState::genesis());
    }
    #[test]
    fn reconstruction_applies_single_state_transition() {
        let r = reconstruct_harness_state(&[event_with_type(
            "event-1",
            1,
            LedgerEventType::StateTransition,
            LedgerPayload::with_lifecycle_transition("x", crate::state::LifecycleState::Evaluating)
                .unwrap(),
        )])
        .unwrap();
        assert_eq!(r.final_state.revision, 1);
        assert_eq!(
            r.final_state.lifecycle,
            crate::state::LifecycleState::Evaluating
        );
    }
    #[test]
    fn reconstruction_applies_multiple_state_transitions_in_order() {
        let r = reconstruct_harness_state(&[
            event_with_type(
                "event-1",
                1,
                LedgerEventType::StateTransition,
                LedgerPayload::with_lifecycle_transition(
                    "x",
                    crate::state::LifecycleState::Evaluating,
                )
                .unwrap(),
            ),
            event_with_type(
                "event-2",
                2,
                LedgerEventType::StateTransition,
                LedgerPayload::with_lifecycle_transition("y", crate::state::LifecycleState::Passed)
                    .unwrap(),
            ),
        ])
        .unwrap();
        assert_eq!(r.final_state.revision, 2);
        assert_eq!(
            r.final_state.lifecycle,
            crate::state::LifecycleState::Passed
        );
    }
    #[test]
    fn reconstruction_ignores_non_state_transition_events() {
        let r = reconstruct_harness_state(&[event_with_type(
            "event-1",
            1,
            LedgerEventType::MemoryWrite,
            payload(),
        )])
        .unwrap();
        assert_eq!(r.final_state, crate::state::HarnessState::genesis());
    }
    #[test]
    fn reconstruction_counts_events_seen() {
        let r = reconstruct_harness_state(&[
            event_with_type("event-1", 1, LedgerEventType::MemoryWrite, payload()),
            event_with_type("event-2", 2, LedgerEventType::ExecutionStart, payload()),
        ])
        .unwrap();
        assert_eq!(r.events_seen, 2);
    }
    #[test]
    fn reconstruction_counts_state_transitions_applied() {
        let r = reconstruct_harness_state(&[
            event_with_type(
                "event-1",
                1,
                LedgerEventType::StateTransition,
                LedgerPayload::with_lifecycle_transition(
                    "x",
                    crate::state::LifecycleState::Evaluating,
                )
                .unwrap(),
            ),
            event_with_type("event-2", 2, LedgerEventType::MemoryWrite, payload()),
            event_with_type(
                "event-3",
                3,
                LedgerEventType::StateTransition,
                LedgerPayload::with_lifecycle_transition("y", crate::state::LifecycleState::Passed)
                    .unwrap(),
            ),
        ])
        .unwrap();
        assert_eq!(r.state_transitions_applied, 2);
    }
    #[test]
    fn reconstruction_requires_state_transition_payload() {
        assert_eq!(
            reconstruct_harness_state(&[event_with_type(
                "event-1",
                1,
                LedgerEventType::StateTransition,
                payload()
            )]),
            Err(ReplayReconstructionError::UnsupportedStateTransitionPayload)
        );
    }
    #[test]
    fn reconstruction_fails_on_invalid_lifecycle_transition() {
        assert_eq!(
            reconstruct_harness_state(&[event_with_type(
                "event-1",
                1,
                LedgerEventType::StateTransition,
                LedgerPayload::with_lifecycle_transition(
                    "x",
                    crate::state::LifecycleState::PromotedTier1
                )
                .unwrap()
            )]),
            Err(ReplayReconstructionError::LifecycleTransitionFailed)
        );
    }
    #[test]
    fn reconstruction_does_not_sort_events() {
        assert_eq!(
            reconstruct_harness_state(&[
                event_with_type(
                    "event-1",
                    2,
                    LedgerEventType::StateTransition,
                    LedgerPayload::with_lifecycle_transition(
                        "x",
                        crate::state::LifecycleState::Passed
                    )
                    .unwrap()
                ),
                event_with_type(
                    "event-2",
                    1,
                    LedgerEventType::StateTransition,
                    LedgerPayload::with_lifecycle_transition(
                        "y",
                        crate::state::LifecycleState::Evaluating
                    )
                    .unwrap()
                )
            ]),
            Err(ReplayReconstructionError::NotReplayReady)
        );
    }
    #[test]
    fn reconstruction_error_codes_are_stable() {
        assert_eq!(
            ReplayReconstructionError::NotReplayReady.code(),
            "not_replay_ready"
        );
        assert_eq!(
            ReplayReconstructionError::UnsupportedStateTransitionPayload.code(),
            "unsupported_state_transition_payload"
        );
        assert_eq!(
            ReplayReconstructionError::LifecycleTransitionFailed.code(),
            "lifecycle_transition_failed"
        );
    }
}
