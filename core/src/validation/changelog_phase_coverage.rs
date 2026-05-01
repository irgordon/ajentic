//! Deterministic validation for CHANGELOG phase coverage continuity.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogPhaseCoverageStatus {
    Continuous,
    GapDetected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogPhaseCoverageError {
    MissingPhase,
    DuplicatePhase,
    NonSequentialPhase,
    InvalidPhaseNumber,
    MissingStatusLine,
}

pub fn validate_changelog_phase_coverage(
    changelog: &str,
) -> Result<ChangelogPhaseCoverageStatus, ChangelogPhaseCoverageError> {
    let phase_numbers = parse_phase_numbers(changelog)?;

    if phase_numbers.is_empty() {
        return Err(ChangelogPhaseCoverageError::MissingStatusLine);
    }

    let mut seen_phases: Vec<u64> = Vec::new();
    let mut previous_phase: Option<u64> = None;

    for phase in phase_numbers {
        if seen_phases.contains(&phase) {
            return Err(ChangelogPhaseCoverageError::DuplicatePhase);
        }

        if let Some(previous) = previous_phase {
            if phase > previous {
                return Err(ChangelogPhaseCoverageError::NonSequentialPhase);
            }

            if previous - phase != 1 {
                return Err(ChangelogPhaseCoverageError::MissingPhase);
            }
        }

        seen_phases.push(phase);
        previous_phase = Some(phase);
    }

    Ok(ChangelogPhaseCoverageStatus::Continuous)
}

fn parse_phase_numbers(changelog: &str) -> Result<Vec<u64>, ChangelogPhaseCoverageError> {
    let mut phase_numbers = Vec::new();

    for line in changelog.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("**Status:**") {
            continue;
        }

        let phase_number = parse_phase_number(trimmed)?;
        phase_numbers.push(phase_number);
    }

    Ok(phase_numbers)
}

fn parse_phase_number(status_line: &str) -> Result<u64, ChangelogPhaseCoverageError> {
    let after_status = status_line
        .strip_prefix("**Status:**")
        .ok_or(ChangelogPhaseCoverageError::MissingStatusLine)?
        .trim();

    let mut tokens = after_status.split_whitespace();

    let phase_token = tokens
        .next()
        .ok_or(ChangelogPhaseCoverageError::MissingStatusLine)?;

    if phase_token != "Phase" {
        return Err(ChangelogPhaseCoverageError::MissingStatusLine);
    }

    let number_token = tokens
        .next()
        .ok_or(ChangelogPhaseCoverageError::InvalidPhaseNumber)?;

    let normalized = number_token
        .trim_end_matches(|character: char| !character.is_ascii_digit())
        .trim();

    if normalized.is_empty() {
        return Err(ChangelogPhaseCoverageError::InvalidPhaseNumber);
    }

    let phase_number = normalized
        .parse::<u64>()
        .map_err(|_| ChangelogPhaseCoverageError::InvalidPhaseNumber)?;

    if phase_number == 0 {
        return Err(ChangelogPhaseCoverageError::InvalidPhaseNumber);
    }

    Ok(phase_number)
}

#[cfg(test)]
mod tests {
    use super::{
        validate_changelog_phase_coverage, ChangelogPhaseCoverageError,
        ChangelogPhaseCoverageStatus,
    };

    #[test]
    fn continuous_phase_sequence_is_accepted() {
        let changelog = "**Status:** Phase 3 something.\n**Status:** Phase 2 something.\n**Status:** Phase 1 something.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Ok(ChangelogPhaseCoverageStatus::Continuous));
    }

    #[test]
    fn missing_phase_is_rejected() {
        let changelog = "**Status:** Phase 4 something.\n**Status:** Phase 2 something.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Err(ChangelogPhaseCoverageError::MissingPhase));
    }

    #[test]
    fn duplicate_phase_is_rejected() {
        let changelog = "**Status:** Phase 3 something.\n**Status:** Phase 3 something.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Err(ChangelogPhaseCoverageError::DuplicatePhase));
    }

    #[test]
    fn non_sequential_phase_is_rejected() {
        let changelog = "**Status:** Phase 2 something.\n**Status:** Phase 3 something.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Err(ChangelogPhaseCoverageError::NonSequentialPhase));
    }

    #[test]
    fn invalid_phase_number_is_rejected() {
        let changelog = "**Status:** Phase x something.\n**Status:** Phase 1 something.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Err(ChangelogPhaseCoverageError::InvalidPhaseNumber));
    }

    #[test]
    fn missing_status_line_is_rejected() {
        let changelog = "## v0.0.1 - 2026-05-01\n### Added\n- Entry without status.";

        let result = validate_changelog_phase_coverage(changelog);

        assert_eq!(result, Err(ChangelogPhaseCoverageError::MissingStatusLine));
    }

    #[test]
    fn validation_is_deterministic() {
        let changelog = "**Status:** Phase 3 something.\n**Status:** Phase 2 something.\n**Status:** Phase 1 something.";

        let first = validate_changelog_phase_coverage(changelog);
        let second = validate_changelog_phase_coverage(changelog);

        assert_eq!(first, second);
        assert_eq!(first, Ok(ChangelogPhaseCoverageStatus::Continuous));
    }
}
