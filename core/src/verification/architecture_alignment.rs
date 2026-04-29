//! Architecture alignment verification status vocabularies.
//!
//! These enums are deterministic, closed-set, descriptive outputs only.
//! They must not be used as authoritative lifecycle, governance, promotion,
//! replay, or persistence control surfaces.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureAlignmentStatus {
    Aligned,
    DriftDetected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityBoundaryStatus {
    Preserved,
    ViolationDetected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeterminismVerificationStatus {
    Deterministic,
    NonDeterministic,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailClosedVerificationStatus {
    Enforced,
    ViolationDetected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentationAlignmentStatus {
    Consistent,
    MismatchDetected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverallAlignmentStatus {
    Verified,
    Violation,
    Unknown,
}
