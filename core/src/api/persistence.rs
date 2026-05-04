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
    EmptyPayload,
    TargetAlreadyExists,
    TempWriteFailed,
    TempSyncFailed,
    AtomicRenameFailed,
}

impl LocalPersistenceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPlan => "invalid_plan",
            Self::PhysicalWriteNotImplemented => "physical_write_not_implemented",
            Self::EmptyPayload => "empty_payload",
            Self::TargetAlreadyExists => "target_already_exists",
            Self::TempWriteFailed => "temp_write_failed",
            Self::TempSyncFailed => "temp_sync_failed",
            Self::AtomicRenameFailed => "atomic_rename_failed",
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
    payload_bytes: &[u8],
) -> Result<(), LocalPersistenceError> {
    let validation = validate_local_persistence_plan(plan);
    if !validation.valid {
        return Err(LocalPersistenceError::InvalidPlan);
    }
    if payload_bytes.is_empty() {
        return Err(LocalPersistenceError::EmptyPayload);
    }
    if plan.write_mode == LocalPersistenceWriteMode::CreateNew
        && std::path::Path::new(&plan.target_path).exists()
    {
        return Err(LocalPersistenceError::TargetAlreadyExists);
    }

    let mut temp_file = std::fs::File::create(&plan.temp_path)
        .map_err(|_| LocalPersistenceError::TempWriteFailed)?;
    use std::io::Write;
    temp_file
        .write_all(payload_bytes)
        .map_err(|_| LocalPersistenceError::TempWriteFailed)?;
    temp_file
        .flush()
        .map_err(|_| LocalPersistenceError::TempSyncFailed)?;
    temp_file
        .sync_all()
        .map_err(|_| LocalPersistenceError::TempSyncFailed)?;
    drop(temp_file);

    std::fs::rename(&plan.temp_path, &plan.target_path)
        .map_err(|_| LocalPersistenceError::AtomicRenameFailed)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

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

    fn test_root(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        p.push(format!(
            "ajentic_phase61_{name}_{}_{}",
            std::process::id(),
            unique
        ));
        p
    }

    fn build_plan(root: &Path, mode: LocalPersistenceWriteMode) -> LocalPersistencePlan {
        LocalPersistencePlan::new(
            "plan-1",
            root.join("target.bin").to_string_lossy().to_string(),
            root.join("temp.bin").to_string_lossy().to_string(),
            Some(1),
            LocalPersistencePayloadKind::LedgerSnapshot,
            mode,
            LocalPersistenceAtomicity::Required,
        )
    }

    #[test]
    fn persistence_error_codes_are_stable_after_physical_write() {
        assert_eq!(LocalPersistenceError::InvalidPlan.code(), "invalid_plan");
        assert_eq!(
            LocalPersistenceError::PhysicalWriteNotImplemented.code(),
            "physical_write_not_implemented"
        );
        assert_eq!(LocalPersistenceError::EmptyPayload.code(), "empty_payload");
        assert_eq!(
            LocalPersistenceError::TargetAlreadyExists.code(),
            "target_already_exists"
        );
        assert_eq!(
            LocalPersistenceError::TempWriteFailed.code(),
            "temp_write_failed"
        );
        assert_eq!(
            LocalPersistenceError::TempSyncFailed.code(),
            "temp_sync_failed"
        );
        assert_eq!(
            LocalPersistenceError::AtomicRenameFailed.code(),
            "atomic_rename_failed"
        );
    }

    #[test]
    fn execute_persistence_plan_rejects_invalid_plan_before_payload_check() {
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
                &[]
            ),
            Err(LocalPersistenceError::InvalidPlan)
        );
    }

    #[test]
    fn execute_persistence_plan_rejects_empty_payload() {
        let root = test_root("empty_payload");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        assert_eq!(
            execute_local_persistence_plan(&plan, &[]),
            Err(LocalPersistenceError::EmptyPayload)
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_writes_payload_to_target_via_temp_path() {
        let root = test_root("writes_payload");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        let payload = b"abc123";
        execute_local_persistence_plan(&plan, payload).expect("write ok");
        assert_eq!(fs::read(&plan.target_path).expect("read target"), payload);
        assert!(!PathBuf::from(&plan.temp_path).exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_replaces_existing_target_when_write_mode_replace_existing() {
        let root = test_root("replace_existing");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::ReplaceExisting);
        fs::write(&plan.target_path, b"old").expect("seed target");
        execute_local_persistence_plan(&plan, b"new").expect("write ok");
        assert_eq!(fs::read(&plan.target_path).expect("read target"), b"new");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_create_new_fails_if_target_exists() {
        let root = test_root("create_new_exists");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        fs::write(&plan.target_path, b"existing").expect("seed target");
        assert_eq!(
            execute_local_persistence_plan(&plan, b"new"),
            Err(LocalPersistenceError::TargetAlreadyExists)
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_fails_when_temp_parent_missing() {
        let root = test_root("temp_parent_missing");
        fs::create_dir_all(&root).expect("mkdir");
        let missing = root.join("missing");
        let plan = LocalPersistencePlan::new(
            "plan-1",
            root.join("target.bin").to_string_lossy().to_string(),
            missing.join("temp.bin").to_string_lossy().to_string(),
            Some(1),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        );
        assert_eq!(
            execute_local_persistence_plan(&plan, b"payload"),
            Err(LocalPersistenceError::TempWriteFailed)
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_fails_when_target_parent_missing() {
        let root = test_root("target_parent_missing");
        fs::create_dir_all(&root).expect("mkdir");
        let missing = root.join("missing");
        let plan = LocalPersistencePlan::new(
            "plan-1",
            missing.join("target.bin").to_string_lossy().to_string(),
            root.join("temp.bin").to_string_lossy().to_string(),
            Some(1),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        );
        assert_eq!(
            execute_local_persistence_plan(&plan, b"payload"),
            Err(LocalPersistenceError::AtomicRenameFailed)
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_does_not_create_parent_directories() {
        let root = test_root("no_parent_create");
        let plan = LocalPersistencePlan::new(
            "plan-1",
            root.join("a/target.bin").to_string_lossy().to_string(),
            root.join("b/temp.bin").to_string_lossy().to_string(),
            Some(1),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        );
        assert_eq!(
            execute_local_persistence_plan(&plan, b"payload"),
            Err(LocalPersistenceError::TempWriteFailed)
        );
        assert!(!root.exists());
    }

    #[test]
    fn execute_persistence_plan_does_not_infer_payload_kind_from_extension() {
        let root = test_root("no_infer_kind");
        fs::create_dir_all(&root).expect("mkdir");
        let mut plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        plan.target_path = root.join("state.json").to_string_lossy().to_string();
        plan.temp_path = root.join("state.tmp").to_string_lossy().to_string();
        plan.payload_kind = LocalPersistencePayloadKind::RunRecord;
        execute_local_persistence_plan(&plan, b"not-json").expect("write ok");
        assert_eq!(fs::read(&plan.target_path).expect("read"), b"not-json");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_does_not_inspect_payload_bytes() {
        let root = test_root("no_inspect_payload");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::ReplaceExisting);
        execute_local_persistence_plan(&plan, &[0, 1, 2, 3]).expect("write ok");
        assert_eq!(fs::read(&plan.target_path).expect("read"), vec![0, 1, 2, 3]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_does_not_serialize_local_application_state() {
        let root = test_root("no_serialize_state");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        execute_local_persistence_plan(&plan, b"caller-supplied-bytes").expect("write ok");
        assert_eq!(
            fs::read(&plan.target_path).expect("read"),
            b"caller-supplied-bytes"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_does_not_repair_replay() {
        let root = test_root("no_replay_repair");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        execute_local_persistence_plan(&plan, b"replay-bytes").expect("write ok");
        assert_eq!(fs::read(&plan.target_path).expect("read"), b"replay-bytes");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn execute_persistence_plan_is_only_physical_write_boundary() {
        let root = test_root("only_boundary");
        fs::create_dir_all(&root).expect("mkdir");
        let plan = build_plan(&root, LocalPersistenceWriteMode::CreateNew);
        execute_local_persistence_plan(&plan, b"payload").expect("write ok");
        assert!(PathBuf::from(&plan.target_path).exists());
        let _ = fs::remove_dir_all(&root);
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
}
