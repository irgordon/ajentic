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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterProtocolError {
    ProtocolVersionMismatch { expected: String, found: String },
    RunIdMismatch { expected: String, found: String },
    CandidateRequestIdMismatch { expected: String, found: String },
    EmptyAdapterName,
    EmptyAdapterVersion,
    OutputTooLarge { max: usize, actual: usize },
    ParseFailed,
    SubprocessFailed,
    SubprocessNoOutput,
    InvalidStatus { value: String },
}

impl fmt::Display for AdapterProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProtocolVersionMismatch { expected, found } => write!(
                f,
                "adapter protocol version mismatch: expected '{expected}', found '{found}'"
            ),
            Self::RunIdMismatch { expected, found } => {
                write!(
                    f,
                    "adapter run_id mismatch: expected '{expected}', found '{found}'"
                )
            }
            Self::CandidateRequestIdMismatch { expected, found } => write!(
                f,
                "adapter candidate_request_id mismatch: expected '{expected}', found '{found}'"
            ),
            Self::EmptyAdapterName => write!(f, "adapter_name is empty"),
            Self::EmptyAdapterVersion => write!(f, "adapter_version is empty"),
            Self::OutputTooLarge { max, actual } => {
                write!(f, "adapter output too large: max {max} bytes, got {actual}")
            }
            Self::ParseFailed => write!(f, "failed to parse adapter response"),
            Self::SubprocessFailed => write!(f, "adapter subprocess execution failed"),
            Self::SubprocessNoOutput => write!(f, "adapter subprocess returned no output"),
            Self::InvalidStatus { value } => write!(f, "invalid adapter status: '{value}'"),
        }
    }
}

impl std::error::Error for AdapterProtocolError {}
