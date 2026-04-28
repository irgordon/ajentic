//! Static run directory validation.

use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::StaticRunValidationError;

pub const REQUIRED_RUN_FILES: &[RequiredRunFile] = &[
    RequiredRunFile {
        name: "objective.yaml",
        required_marker: "objective:",
    },
    RequiredRunFile {
        name: "constraints.yaml",
        required_marker: "constraints:",
    },
    RequiredRunFile {
        name: "domain.yaml",
        required_marker: "domain:",
    },
    RequiredRunFile {
        name: "policy.yaml",
        required_marker: "policy:",
    },
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RequiredRunFile {
    pub name: &'static str,
    pub required_marker: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticRunFileStatus {
    pub name: &'static str,
    pub byte_len: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticRunInspection {
    pub run_dir: PathBuf,
    pub files: Vec<StaticRunFileStatus>,
}

pub fn validate_static_run_dir(
    run_dir: &Path,
) -> Result<StaticRunInspection, StaticRunValidationError> {
    if !run_dir.exists() {
        return Err(StaticRunValidationError::RunDirectoryMissing {
            path: run_dir.to_path_buf(),
        });
    }

    if !run_dir.is_dir() {
        return Err(StaticRunValidationError::RunPathIsNotDirectory {
            path: run_dir.to_path_buf(),
        });
    }

    let mut files = Vec::with_capacity(REQUIRED_RUN_FILES.len());

    for required_file in REQUIRED_RUN_FILES {
        let file_path = run_dir.join(required_file.name);
        if !file_path.exists() {
            return Err(StaticRunValidationError::RequiredFileMissing { path: file_path });
        }

        let file_contents =
            fs::read_to_string(&file_path).map_err(|_| StaticRunValidationError::ReadFailed {
                path: file_path.clone(),
            })?;

        if file_contents.trim().is_empty() {
            return Err(StaticRunValidationError::RequiredFileEmpty { path: file_path });
        }

        if !file_contents.contains(required_file.required_marker) {
            return Err(StaticRunValidationError::RequiredMarkerMissing {
                path: file_path,
                marker: required_file.required_marker,
            });
        }

        let byte_len = file_contents.len();
        files.push(StaticRunFileStatus {
            name: required_file.name,
            byte_len,
        });
    }

    Ok(StaticRunInspection {
        run_dir: run_dir.to_path_buf(),
        files,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::validate_static_run_dir;
    use crate::errors::StaticRunValidationError;

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock must be after unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!("ajentic_phase3_{name}_{nanos}"));
            fs::create_dir_all(&path).expect("test directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_file(path: &Path, content: &str) {
        fs::write(path, content).expect("test file should be written");
    }

    fn write_valid_files(run_dir: &Path) {
        write_file(
            &run_dir.join("objective.yaml"),
            "objective:\n  name: test objective\n",
        );
        write_file(
            &run_dir.join("constraints.yaml"),
            "constraints:\n  limits: []\n",
        );
        write_file(&run_dir.join("domain.yaml"), "domain:\n  kind: test\n");
        write_file(&run_dir.join("policy.yaml"), "policy:\n  mode: strict\n");
    }

    #[test]
    fn valid_run_directory_passes_static_validation() {
        let inspection = validate_static_run_dir(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../examples/minimal_run"
        )))
        .expect("minimal run example should pass static validation");

        assert_eq!(inspection.files.len(), 4);
    }

    #[test]
    fn missing_run_directory_fails() {
        let path = Path::new("examples/does_not_exist");
        let error = validate_static_run_dir(path).expect_err("missing directory should fail");

        assert_eq!(
            error,
            StaticRunValidationError::RunDirectoryMissing {
                path: path.to_path_buf()
            }
        );
    }

    #[test]
    fn run_path_is_file_fails() {
        let test_dir = TestDir::new("path_is_file");
        let file_path = test_dir.path().join("not_directory.txt");
        write_file(&file_path, "content");

        let error = validate_static_run_dir(&file_path).expect_err("file path should fail");

        assert_eq!(
            error,
            StaticRunValidationError::RunPathIsNotDirectory { path: file_path }
        );
    }

    #[test]
    fn missing_required_file_fails() {
        let test_dir = TestDir::new("missing_required_file");
        write_valid_files(test_dir.path());
        fs::remove_file(test_dir.path().join("objective.yaml"))
            .expect("objective file should be removed");

        let missing_path = test_dir.path().join("objective.yaml");
        let error = validate_static_run_dir(test_dir.path())
            .expect_err("missing required file should fail");

        assert_eq!(
            error,
            StaticRunValidationError::RequiredFileMissing { path: missing_path }
        );
    }

    #[test]
    fn empty_required_file_fails() {
        let test_dir = TestDir::new("empty_required_file");
        write_valid_files(test_dir.path());
        write_file(&test_dir.path().join("constraints.yaml"), "   \n\t   \n");

        let error =
            validate_static_run_dir(test_dir.path()).expect_err("empty required file should fail");

        assert_eq!(
            error,
            StaticRunValidationError::RequiredFileEmpty {
                path: test_dir.path().join("constraints.yaml")
            }
        );
    }

    #[test]
    fn missing_marker_fails() {
        let test_dir = TestDir::new("missing_marker");
        write_valid_files(test_dir.path());
        write_file(&test_dir.path().join("domain.yaml"), "name: test-domain\n");

        let error = validate_static_run_dir(test_dir.path())
            .expect_err("missing required marker should fail");

        assert_eq!(
            error,
            StaticRunValidationError::RequiredMarkerMissing {
                path: test_dir.path().join("domain.yaml"),
                marker: "domain:"
            }
        );
    }
}
