//! Deterministic validation for CHANGELOG entry contract structure.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChangelogValidationStatus {
    Valid,
    Invalid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChangelogValidationError {
    MissingVersionHeading,
    InvalidVersionHeading,
    MissingStatusLine,
    MissingAddedSection,
    MissingNotesSection,
    StatusOnVersionHeading,
    DuplicateVersion,
    NonSequentialVersion,
    MissingDate,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

pub fn validate_changelog_text(
    changelog: &str,
) -> Result<ChangelogValidationStatus, ChangelogValidationError> {
    let mut entries = Vec::new();

    for line in changelog.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            entries.push(trimmed);
        }
    }

    if entries.is_empty() {
        return Err(ChangelogValidationError::MissingVersionHeading);
    }

    let mut previous_version: Option<Version> = None;
    let mut seen_versions: Vec<Version> = Vec::new();

    for heading in entries {
        if heading.contains("**Status:**") {
            return Err(ChangelogValidationError::StatusOnVersionHeading);
        }

        let (version, date) = parse_version_heading(heading)?;

        if date.is_empty() {
            return Err(ChangelogValidationError::MissingDate);
        }

        if seen_versions.contains(&version) {
            return Err(ChangelogValidationError::DuplicateVersion);
        }
        seen_versions.push(version);

        if let Some(previous) = previous_version {
            if version >= previous {
                return Err(ChangelogValidationError::NonSequentialVersion);
            }
        }

        previous_version = Some(version);
    }

    for heading in changelog
        .lines()
        .filter(|line| line.trim().starts_with("## "))
    {
        let start = changelog
            .find(heading)
            .ok_or(ChangelogValidationError::InvalidVersionHeading)?;
        let tail = &changelog[start + heading.len()..];
        let next_idx = tail.find("\n## ").unwrap_or(tail.len());
        let body = &tail[..next_idx];

        if !body
            .lines()
            .any(|line| line.trim() == "**Status:**" || line.trim().starts_with("**Status:** "))
        {
            return Err(ChangelogValidationError::MissingStatusLine);
        }

        if !body.lines().any(|line| line.trim() == "### Added") {
            return Err(ChangelogValidationError::MissingAddedSection);
        }

        if !body.lines().any(|line| line.trim() == "### Notes") {
            return Err(ChangelogValidationError::MissingNotesSection);
        }
    }

    Ok(ChangelogValidationStatus::Valid)
}

fn parse_version_heading(heading: &str) -> Result<(Version, &str), ChangelogValidationError> {
    let rest = heading
        .strip_prefix("## v")
        .ok_or(ChangelogValidationError::InvalidVersionHeading)?;

    let separator = " - ";
    let split_index = rest
        .find(separator)
        .ok_or(ChangelogValidationError::MissingDate)?;

    let version_text = &rest[..split_index];
    let date_text = &rest[split_index + separator.len()..].trim();

    if !is_valid_date(date_text) {
        return Err(ChangelogValidationError::MissingDate);
    }

    let mut parts = version_text.split('.');
    let major = parse_part(parts.next())?;
    let minor = parse_part(parts.next())?;
    let patch = parse_part(parts.next())?;

    if parts.next().is_some() {
        return Err(ChangelogValidationError::InvalidVersionHeading);
    }

    Ok((
        Version {
            major,
            minor,
            patch,
        },
        date_text,
    ))
}

fn parse_part(part: Option<&str>) -> Result<u64, ChangelogValidationError> {
    let text = part.ok_or(ChangelogValidationError::InvalidVersionHeading)?;
    text.parse::<u64>()
        .map_err(|_| ChangelogValidationError::InvalidVersionHeading)
}

fn is_valid_date(date_text: &str) -> bool {
    if date_text.len() != 10 {
        return false;
    }

    let bytes = date_text.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if index == 4 || index == 7 {
            if *byte != b'-' {
                return false;
            }
            continue;
        }

        if !byte.is_ascii_digit() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{validate_changelog_text, ChangelogValidationError, ChangelogValidationStatus};

    const VALID_CHANGELOG: &str = r#"# Changelog

## v0.0.2 - 2026-05-01
**Status:** Phase 2 capability description.

### Added
- Added item.

### Notes
- Note item.

## v0.0.1 - 2026-04-30
**Status:** Phase 1 capability description.

### Added
- Added item.

### Notes
- Note item.
"#;

    #[test]
    fn valid_changelog_entry_is_accepted() {
        let result = validate_changelog_text(VALID_CHANGELOG)
            .expect("valid changelog entries should pass validation");

        assert_eq!(result, ChangelogValidationStatus::Valid);
    }

    #[test]
    fn status_on_version_heading_is_rejected() {
        let fixture = r#"## v0.0.1 - 2026-05-01 **Status:** bad
### Added
- item
### Notes
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(
            result,
            Err(ChangelogValidationError::StatusOnVersionHeading)
        );
    }

    #[test]
    fn missing_status_line_is_rejected() {
        let fixture = r#"## v0.0.1 - 2026-05-01
### Added
- item
### Notes
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::MissingStatusLine));
    }

    #[test]
    fn missing_added_section_is_rejected() {
        let fixture = r#"## v0.0.1 - 2026-05-01
**Status:** Phase 1 capability description.
### Notes
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::MissingAddedSection));
    }

    #[test]
    fn missing_notes_section_is_rejected() {
        let fixture = r#"## v0.0.1 - 2026-05-01
**Status:** Phase 1 capability description.
### Added
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::MissingNotesSection));
    }

    #[test]
    fn duplicate_version_is_rejected() {
        let fixture = r#"## v0.0.1 - 2026-05-01
**Status:** Phase 1 capability description.
### Added
- one
### Notes
- one

## v0.0.1 - 2026-04-30
**Status:** Phase 1 capability description.
### Added
- two
### Notes
- two
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::DuplicateVersion));
    }

    #[test]
    fn invalid_version_heading_is_rejected() {
        let fixture = r#"## version 0.0.1 - 2026-05-01
**Status:** Phase 1 capability description.
### Added
- item
### Notes
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::InvalidVersionHeading));
    }

    #[test]
    fn missing_date_is_rejected() {
        let fixture = r#"## v0.0.1
**Status:** Phase 1 capability description.
### Added
- item
### Notes
- item
"#;

        let result = validate_changelog_text(fixture);

        assert_eq!(result, Err(ChangelogValidationError::MissingDate));
    }

    #[test]
    fn validation_is_deterministic() {
        let first = validate_changelog_text(VALID_CHANGELOG);
        let second = validate_changelog_text(VALID_CHANGELOG);

        assert_eq!(first, second);
        assert_eq!(first, Ok(ChangelogValidationStatus::Valid));
    }
}
