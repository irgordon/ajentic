//! Errors module.

use std::fmt;
use std::path::PathBuf;

use crate::candidate::lifecycle::CandidateLifecycleState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CandidateLifecycleError {
    InvalidTransition {
        from: CandidateLifecycleState,
        to: CandidateLifecycleState,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StaticRunValidationError {
    RunDirectoryMissing { path: PathBuf },
    RunPathIsNotDirectory { path: PathBuf },
    RequiredFileMissing { path: PathBuf },
    RequiredFileEmpty { path: PathBuf },
    RequiredMarkerMissing { path: PathBuf, marker: &'static str },
    ReadFailed { path: PathBuf },
}

impl fmt::Display for StaticRunValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RunDirectoryMissing { path } => {
                write!(f, "run directory is missing: {}", path.display())
            }
            Self::RunPathIsNotDirectory { path } => {
                write!(f, "run path is not a directory: {}", path.display())
            }
            Self::RequiredFileMissing { path } => {
                write!(f, "required file is missing: {}", path.display())
            }
            Self::RequiredFileEmpty { path } => {
                write!(f, "required file is empty: {}", path.display())
            }
            Self::RequiredMarkerMissing { path, marker } => {
                write!(
                    f,
                    "required marker '{marker}' is missing in: {}",
                    path.display()
                )
            }
            Self::ReadFailed { path } => {
                write!(f, "failed to read required file: {}", path.display())
            }
        }
    }
}
