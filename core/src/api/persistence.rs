use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistencePayloadKind {
    LedgerSnapshot,
    RunRecord,
    AuditProjection,
    MemorySnapshot,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceWriteMode {
    CreateNew,
    ReplaceExisting,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceAtomicity {
    Required,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPersistencePlan {
    pub plan_id: String,
    pub target_path: String,
    pub temp_path: String,
    pub expected_revision: Option<u64>,
    pub payload_kind: LocalPersistencePayloadKind,
    pub write_mode: LocalPersistenceWriteMode,
    pub atomicity: LocalPersistenceAtomicity,
}

impl LocalPersistencePlan {
    pub fn new(
        plan_id: impl Into<String>,
        target_path: impl Into<String>,
        temp_path: impl Into<String>,
        expected_revision: Option<u64>,
        payload_kind: LocalPersistencePayloadKind,
        write_mode: LocalPersistenceWriteMode,
        atomicity: LocalPersistenceAtomicity,
    ) -> Self {
        Self {
            plan_id: plan_id.into(),
            target_path: target_path.into(),
            temp_path: temp_path.into(),
            expected_revision,
            payload_kind,
            write_mode,
            atomicity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPersistenceValidation {
    pub plan_id: String,
    pub valid: bool,
    pub reason: LocalPersistenceValidationReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceValidationReason {
    Valid,
    EmptyPlanId,
    EmptyTargetPath,
    EmptyTempPath,
    TargetAndTempPathSame,
    MissingExpectedRevision,
    UnknownPayloadKind,
    UnknownWriteMode,
    AtomicityNotRequired,
    PathLooksLikeSecret,
}

impl LocalPersistenceValidationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::EmptyPlanId => "empty_plan_id",
            Self::EmptyTargetPath => "empty_target_path",
            Self::EmptyTempPath => "empty_temp_path",
            Self::TargetAndTempPathSame => "target_and_temp_path_same",
            Self::MissingExpectedRevision => "missing_expected_revision",
            Self::UnknownPayloadKind => "unknown_payload_kind",
            Self::UnknownWriteMode => "unknown_write_mode",
            Self::AtomicityNotRequired => "atomicity_not_required",
            Self::PathLooksLikeSecret => "path_looks_like_secret",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceError {
    InvalidPlan,
    PhysicalWriteNotImplemented,
}

impl LocalPersistenceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPlan => "invalid_plan",
            Self::PhysicalWriteNotImplemented => "physical_write_not_implemented",
        }
    }
}

pub fn validate_local_persistence_plan(plan: &LocalPersistencePlan) -> LocalPersistenceValidation {
    let reason = if plan.plan_id.is_empty() {
        LocalPersistenceValidationReason::EmptyPlanId
    } else if plan.target_path.is_empty() {
        LocalPersistenceValidationReason::EmptyTargetPath
    } else if plan.temp_path.is_empty() {
        LocalPersistenceValidationReason::EmptyTempPath
    } else if plan.target_path == plan.temp_path {
        LocalPersistenceValidationReason::TargetAndTempPathSame
    } else if plan.expected_revision.is_none() {
        LocalPersistenceValidationReason::MissingExpectedRevision
    } else if plan.payload_kind == LocalPersistencePayloadKind::Unknown {
        LocalPersistenceValidationReason::UnknownPayloadKind
    } else if plan.write_mode == LocalPersistenceWriteMode::Unknown {
        LocalPersistenceValidationReason::UnknownWriteMode
    } else if plan.atomicity != LocalPersistenceAtomicity::Required {
        LocalPersistenceValidationReason::AtomicityNotRequired
    } else if contains_secret_marker(&plan.target_path) || contains_secret_marker(&plan.temp_path) {
        LocalPersistenceValidationReason::PathLooksLikeSecret
    } else {
        LocalPersistenceValidationReason::Valid
    };

    LocalPersistenceValidation {
        plan_id: plan.plan_id.clone(),
        valid: reason == LocalPersistenceValidationReason::Valid,
        reason,
    }
}

pub fn local_persistence_plan_allows_hidden_write(plan: &LocalPersistencePlan) -> bool {
    !validate_local_persistence_plan(plan).valid
}

pub fn execute_local_persistence_plan(
    plan: &LocalPersistencePlan,
    _payload_bytes: &[u8],
) -> Result<(), LocalPersistenceError> {
    let validation = validate_local_persistence_plan(plan);
    if !validation.valid {
        return Err(LocalPersistenceError::InvalidPlan);
    }
    Err(LocalPersistenceError::PhysicalWriteNotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_plan() -> LocalPersistencePlan {
        LocalPersistencePlan::new(
            "plan-1",
            "state/target.bin",
            "state/temp.bin",
            Some(3),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        )
    }

    #[test]
    fn invalid_plan_fails_closed_before_write_path() {
        assert_eq!(
            execute_local_persistence_plan(
                &LocalPersistencePlan::new(
                    "",
                    "t",
                    "x",
                    Some(1),
                    LocalPersistencePayloadKind::LedgerSnapshot,
                    LocalPersistenceWriteMode::CreateNew,
                    LocalPersistenceAtomicity::Required
                ),
                b"payload"
            ),
            Err(LocalPersistenceError::InvalidPlan)
        );
    }
    #[test]
    fn valid_plan_returns_not_implemented() {
        assert_eq!(
            execute_local_persistence_plan(&valid_plan(), b"payload"),
            Err(LocalPersistenceError::PhysicalWriteNotImplemented)
        );
    }
    #[test]
    fn same_target_temp_precedes_expected_revision() {
        let v = validate_local_persistence_plan(&LocalPersistencePlan::new(
            "p",
            "same",
            "same",
            None,
            LocalPersistencePayloadKind::Unknown,
            LocalPersistenceWriteMode::Unknown,
            LocalPersistenceAtomicity::Unknown,
        ));
        assert_eq!(
            v.reason,
            LocalPersistenceValidationReason::TargetAndTempPathSame
        );
    }
    #[test]
    fn missing_expected_revision_precedes_payload_write_atomicity_checks() {
        let v = validate_local_persistence_plan(&LocalPersistencePlan::new(
            "p",
            "t",
            "x",
            None,
            LocalPersistencePayloadKind::Unknown,
            LocalPersistenceWriteMode::Unknown,
            LocalPersistenceAtomicity::Unknown,
        ));
        assert_eq!(
            v.reason,
            LocalPersistenceValidationReason::MissingExpectedRevision
        );
    }
    #[test]
    fn secret_path_detection_is_deterministic_when_reached() {
        let mut p = valid_plan();
        p.target_path = "secret-target".into();
        assert_eq!(
            validate_local_persistence_plan(&p).reason,
            LocalPersistenceValidationReason::PathLooksLikeSecret
        );
    }
    #[test]
    fn hidden_write_helper_true_for_invalid_false_for_valid() {
        assert!(local_persistence_plan_allows_hidden_write(
            &LocalPersistencePlan::new(
                "",
                "t",
                "x",
                Some(1),
                LocalPersistencePayloadKind::LedgerSnapshot,
                LocalPersistenceWriteMode::CreateNew,
                LocalPersistenceAtomicity::Required
            )
        ));
        assert!(!local_persistence_plan_allows_hidden_write(&valid_plan()));
    }
    #[test]
    fn execute_plan_does_not_inspect_payload_bytes() {
        assert_eq!(
            execute_local_persistence_plan(&valid_plan(), &[0, 1, 2]),
            execute_local_persistence_plan(&valid_plan(), b"different bytes")
        );
    }
}
