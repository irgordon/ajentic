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
pub enum CandidateCreationError {
    AdapterResponseInvalid(AdapterProtocolError),
    AdapterStatusNotCompleted {
        status: crate::execution::adapter_protocol::AdapterStatus,
    },
    MissingRunId,
    MissingDomainId,
    MissingObjectiveId,
    MissingConstraintsId,
    MissingContentRef,
    MissingGenerationMetadataRef,
    MissingRawOutputRef,
    MissingStructuredOutputRef,
    EmptyOutputText,
    InvalidCandidateIdInput {
        run_id: String,
        candidate_request_id: String,
    },
}

impl fmt::Display for CandidateCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdapterResponseInvalid(error) => write!(f, "adapter response invalid: {error}"),
            Self::AdapterStatusNotCompleted { status } => {
                write!(f, "adapter status must be Completed for candidate creation, found: {status:?}")
            }
            Self::MissingRunId => write!(f, "run_id is required"),
            Self::MissingDomainId => write!(f, "domain_id is required"),
            Self::MissingObjectiveId => write!(f, "objective_id is required"),
            Self::MissingConstraintsId => write!(f, "constraints_id is required"),
            Self::MissingContentRef => write!(f, "content_ref is required"),
            Self::MissingGenerationMetadataRef => write!(f, "generation_metadata_ref is required"),
            Self::MissingRawOutputRef => write!(f, "raw_output_ref is required"),
            Self::MissingStructuredOutputRef => write!(f, "structured_output_ref is required"),
            Self::EmptyOutputText => write!(f, "output_text is empty"),
            Self::InvalidCandidateIdInput { run_id, candidate_request_id } => write!(
                f,
                "candidate id input is invalid: run_id='{run_id}', candidate_request_id='{candidate_request_id}'"
            ),
        }
    }
}

impl std::error::Error for CandidateCreationError {}
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
