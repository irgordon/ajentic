//! Deterministic validation for CHANGELOG version and date sequencing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogSequenceStatus {
    Sequential,
    NonSequential,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogSequenceError {
    DuplicateVersion,
    VersionRegression,
    NonMonotonicDate,
    InvalidVersionFormat,
    MissingVersion,
    MissingDate,
}

pub fn validate_changelog_sequence(
    changelog: &str,
) -> Result<ChangelogSequenceStatus, ChangelogSequenceError> {
    let mut previous_version: Option<(u64, u64, u64)> = None;
    let mut previous_date: Option<(u32, u32, u32)> = None;
    let mut seen_versions: Vec<(u64, u64, u64)> = Vec::new();

    for line in changelog.lines() {
        let heading = line.trim();
        if !heading.starts_with("##") {
            continue;
        }

        let (version_text, date_text) = parse_heading_parts(heading)?;
        let version = parse_version(version_text)?;
        let date = parse_date(date_text)?;

        if seen_versions.contains(&version) {
            return Err(ChangelogSequenceError::DuplicateVersion);
        }

        if let Some(previous) = previous_version {
            if version >= previous {
                return Err(ChangelogSequenceError::VersionRegression);
            }
        }

        if let Some(previous) = previous_date {
            if date > previous {
                return Err(ChangelogSequenceError::NonMonotonicDate);
            }
        }

        seen_versions.push(version);
        previous_version = Some(version);
        previous_date = Some(date);
    }

    Ok(ChangelogSequenceStatus::Sequential)
}

fn parse_heading_parts(heading: &str) -> Result<(&str, &str), ChangelogSequenceError> {
    let content = heading.trim_start_matches('#').trim();

    let separator = if content.contains(" - ") {
        " - "
    } else if content.contains(" — ") {
        " — "
    } else {
        return Err(ChangelogSequenceError::MissingDate);
    };

    let mut parts = content.splitn(2, separator);
    let version = parts
        .next()
        .ok_or(ChangelogSequenceError::MissingVersion)?
        .trim();
    let date = parts
        .next()
        .ok_or(ChangelogSequenceError::MissingDate)?
        .trim();

    if version.is_empty() {
        return Err(ChangelogSequenceError::MissingVersion);
    }

    if date.is_empty() {
        return Err(ChangelogSequenceError::MissingDate);
    }

    Ok((version, date))
}

fn parse_version(version_text: &str) -> Result<(u64, u64, u64), ChangelogSequenceError> {
    let raw = version_text
        .strip_prefix('v')
        .ok_or(ChangelogSequenceError::InvalidVersionFormat)?;
    let mut parts = raw.split('.');

    let major = parts
        .next()
        .ok_or(ChangelogSequenceError::InvalidVersionFormat)?
        .parse::<u64>()
        .map_err(|_| ChangelogSequenceError::InvalidVersionFormat)?;
    let minor = parts
        .next()
        .ok_or(ChangelogSequenceError::InvalidVersionFormat)?
        .parse::<u64>()
        .map_err(|_| ChangelogSequenceError::InvalidVersionFormat)?;
    let patch = parts
        .next()
        .ok_or(ChangelogSequenceError::InvalidVersionFormat)?
        .parse::<u64>()
        .map_err(|_| ChangelogSequenceError::InvalidVersionFormat)?;

    if parts.next().is_some() {
        return Err(ChangelogSequenceError::InvalidVersionFormat);
    }

    Ok((major, minor, patch))
}

fn parse_date(date_text: &str) -> Result<(u32, u32, u32), ChangelogSequenceError> {
    let mut parts = date_text.split('-');

    let year = parts
        .next()
        .ok_or(ChangelogSequenceError::MissingDate)?
        .parse::<u32>()
        .map_err(|_| ChangelogSequenceError::MissingDate)?;
    let month = parts
        .next()
        .ok_or(ChangelogSequenceError::MissingDate)?
        .parse::<u32>()
        .map_err(|_| ChangelogSequenceError::MissingDate)?;
    let day = parts
        .next()
        .ok_or(ChangelogSequenceError::MissingDate)?
        .parse::<u32>()
        .map_err(|_| ChangelogSequenceError::MissingDate)?;

    if parts.next().is_some() {
        return Err(ChangelogSequenceError::MissingDate);
    }

    if month == 0 || month > 12 || day == 0 || day > 31 {
        return Err(ChangelogSequenceError::MissingDate);
    }

    Ok((year, month, day))
}

#[cfg(test)]
mod tests {
    use super::{validate_changelog_sequence, ChangelogSequenceError, ChangelogSequenceStatus};

    #[test]
    fn sequential_versions_are_accepted() {
        let changelog = "## v0.0.3 - 2026-05-01\n## v0.0.2 - 2026-04-30\n## v0.0.1 - 2026-04-30";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Ok(ChangelogSequenceStatus::Sequential));
    }

    #[test]
    fn duplicate_version_is_rejected() {
        let changelog = "## v0.0.3 - 2026-05-01\n## v0.0.3 - 2026-04-30";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Err(ChangelogSequenceError::DuplicateVersion));
    }

    #[test]
    fn version_regression_is_rejected() {
        let changelog = "## v0.0.2 - 2026-05-01\n## v0.0.3 - 2026-04-30";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Err(ChangelogSequenceError::VersionRegression));
    }

    #[test]
    fn non_monotonic_date_is_rejected() {
        let changelog = "## v0.0.3 - 2026-04-30\n## v0.0.2 - 2026-05-01";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Err(ChangelogSequenceError::NonMonotonicDate));
    }

    #[test]
    fn invalid_version_format_is_rejected() {
        let changelog = "## 0.0.3 - 2026-05-01\n## v0.0.2 - 2026-04-30";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Err(ChangelogSequenceError::InvalidVersionFormat));
    }

    #[test]
    fn missing_date_is_rejected() {
        let changelog = "## v0.0.3\n## v0.0.2 - 2026-04-30";

        let result = validate_changelog_sequence(changelog);

        assert_eq!(result, Err(ChangelogSequenceError::MissingDate));
    }

    #[test]
    fn validation_is_deterministic() {
        let changelog = "## v0.0.3 - 2026-05-01\n## v0.0.2 - 2026-04-30\n## v0.0.1 - 2026-04-30";

        let first = validate_changelog_sequence(changelog);
        let second = validate_changelog_sequence(changelog);

        assert_eq!(first, second);
        assert_eq!(first, Ok(ChangelogSequenceStatus::Sequential));
    }
}
