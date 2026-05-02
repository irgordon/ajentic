#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStatus {
    Active,
    Disabled,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    ProjectFact,
    Convention,
    OperatorNote,
    RunObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    EmptyMemoryId,
    EmptyContent,
    EmptyProvenanceSource,
    EmptyProvenanceCreatedBy,
    EmptyProvenanceCreatedAt,
    EmptyProvenanceReason,
    EmptySnapshotId,
    EmptySnapshotCreatedAt,
}

impl MemoryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyMemoryId => "empty_memory_id",
            Self::EmptyContent => "empty_content",
            Self::EmptyProvenanceSource => "empty_provenance_source",
            Self::EmptyProvenanceCreatedBy => "empty_provenance_created_by",
            Self::EmptyProvenanceCreatedAt => "empty_provenance_created_at",
            Self::EmptyProvenanceReason => "empty_provenance_reason",
            Self::EmptySnapshotId => "empty_snapshot_id",
            Self::EmptySnapshotCreatedAt => "empty_snapshot_created_at",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryProvenance {
    pub source: String,
    pub created_by: String,
    pub created_at: String,
    pub reason: String,
}

impl MemoryProvenance {
    pub fn new(
        source: impl Into<String>,
        created_by: impl Into<String>,
        created_at: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, MemoryError> {
        let source = source.into();
        let created_by = created_by.into();
        let created_at = created_at.into();
        let reason = reason.into();

        if source.is_empty() {
            return Err(MemoryError::EmptyProvenanceSource);
        }
        if created_by.is_empty() {
            return Err(MemoryError::EmptyProvenanceCreatedBy);
        }
        if created_at.is_empty() {
            return Err(MemoryError::EmptyProvenanceCreatedAt);
        }
        if reason.is_empty() {
            return Err(MemoryError::EmptyProvenanceReason);
        }

        Ok(Self {
            source,
            created_by,
            created_at,
            reason,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEntry {
    pub id: String,
    pub memory_type: MemoryType,
    pub content: String,
    pub provenance: MemoryProvenance,
    pub status: MemoryStatus,
}

impl MemoryEntry {
    pub fn new(
        id: impl Into<String>,
        memory_type: MemoryType,
        content: impl Into<String>,
        provenance: MemoryProvenance,
        status: MemoryStatus,
    ) -> Result<Self, MemoryError> {
        let id = id.into();
        let content = content.into();

        if id.is_empty() {
            return Err(MemoryError::EmptyMemoryId);
        }
        if content.is_empty() {
            return Err(MemoryError::EmptyContent);
        }

        Ok(Self {
            id,
            memory_type,
            content,
            provenance,
            status,
        })
    }

    pub fn is_active(&self) -> bool {
        self.status == MemoryStatus::Active
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub id: String,
    pub created_at: String,
    pub entries: Vec<MemoryEntry>,
}

impl MemorySnapshot {
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<String>,
        entries: Vec<MemoryEntry>,
    ) -> Result<Self, MemoryError> {
        let id = id.into();
        let created_at = created_at.into();

        if id.is_empty() {
            return Err(MemoryError::EmptySnapshotId);
        }
        if created_at.is_empty() {
            return Err(MemoryError::EmptySnapshotCreatedAt);
        }

        Ok(Self {
            id,
            created_at,
            entries,
        })
    }

    pub fn active_entries(&self) -> Vec<&MemoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.is_active())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_provenance() -> MemoryProvenance {
        MemoryProvenance::new("caller", "operator", "2026-05-02T00:00:00Z", "baseline")
            .expect("valid provenance")
    }

    fn make_entry(id: &str, status: MemoryStatus) -> MemoryEntry {
        MemoryEntry::new(
            id,
            MemoryType::ProjectFact,
            format!("content-{id}"),
            valid_provenance(),
            status,
        )
        .expect("valid memory entry")
    }

    #[test]
    fn memory_provenance_requires_source() {
        let result = MemoryProvenance::new("", "operator", "2026-05-02T00:00:00Z", "baseline");
        assert_eq!(result, Err(MemoryError::EmptyProvenanceSource));
    }

    #[test]
    fn memory_provenance_requires_created_by() {
        let result = MemoryProvenance::new("caller", "", "2026-05-02T00:00:00Z", "baseline");
        assert_eq!(result, Err(MemoryError::EmptyProvenanceCreatedBy));
    }

    #[test]
    fn memory_provenance_requires_created_at() {
        let result = MemoryProvenance::new("caller", "operator", "", "baseline");
        assert_eq!(result, Err(MemoryError::EmptyProvenanceCreatedAt));
    }

    #[test]
    fn memory_provenance_requires_reason() {
        let result = MemoryProvenance::new("caller", "operator", "2026-05-02T00:00:00Z", "");
        assert_eq!(result, Err(MemoryError::EmptyProvenanceReason));
    }

    #[test]
    fn memory_entry_requires_id() {
        let result = MemoryEntry::new(
            "",
            MemoryType::ProjectFact,
            "content",
            valid_provenance(),
            MemoryStatus::Active,
        );
        assert_eq!(result, Err(MemoryError::EmptyMemoryId));
    }

    #[test]
    fn memory_entry_requires_content() {
        let result = MemoryEntry::new(
            "mem-1",
            MemoryType::ProjectFact,
            "",
            valid_provenance(),
            MemoryStatus::Active,
        );
        assert_eq!(result, Err(MemoryError::EmptyContent));
    }

    #[test]
    fn memory_entry_active_status_is_active() {
        let entry = make_entry("mem-1", MemoryStatus::Active);
        assert!(entry.is_active());
    }

    #[test]
    fn memory_entry_disabled_status_is_not_active() {
        let entry = make_entry("mem-1", MemoryStatus::Disabled);
        assert!(!entry.is_active());
    }

    #[test]
    fn memory_entry_rejected_status_is_not_active() {
        let entry = make_entry("mem-1", MemoryStatus::Rejected);
        assert!(!entry.is_active());
    }

    #[test]
    fn memory_snapshot_requires_id() {
        let result = MemorySnapshot::new(
            "",
            "2026-05-02T00:00:00Z",
            vec![make_entry("mem-1", MemoryStatus::Active)],
        );
        assert_eq!(result, Err(MemoryError::EmptySnapshotId));
    }

    #[test]
    fn memory_snapshot_requires_created_at() {
        let result = MemorySnapshot::new(
            "snap-1",
            "",
            vec![make_entry("mem-1", MemoryStatus::Active)],
        );
        assert_eq!(result, Err(MemoryError::EmptySnapshotCreatedAt));
    }

    #[test]
    fn memory_snapshot_preserves_entry_order() {
        let entries = vec![
            make_entry("mem-1", MemoryStatus::Disabled),
            make_entry("mem-2", MemoryStatus::Active),
            make_entry("mem-3", MemoryStatus::Rejected),
        ];
        let snapshot =
            MemorySnapshot::new("snap-1", "2026-05-02T00:00:00Z", entries).expect("snapshot");

        assert_eq!(snapshot.entries[0].id, "mem-1");
        assert_eq!(snapshot.entries[1].id, "mem-2");
        assert_eq!(snapshot.entries[2].id, "mem-3");
    }

    #[test]
    fn memory_snapshot_active_entries_filters_active_only() {
        let entries = vec![
            make_entry("mem-1", MemoryStatus::Disabled),
            make_entry("mem-2", MemoryStatus::Active),
            make_entry("mem-3", MemoryStatus::Rejected),
        ];
        let snapshot =
            MemorySnapshot::new("snap-1", "2026-05-02T00:00:00Z", entries).expect("snapshot");

        let active_entries = snapshot.active_entries();
        assert_eq!(active_entries.len(), 1);
        assert_eq!(active_entries[0].id, "mem-2");
    }

    #[test]
    fn memory_snapshot_active_entries_preserves_order() {
        let entries = vec![
            make_entry("mem-1", MemoryStatus::Active),
            make_entry("mem-2", MemoryStatus::Disabled),
            make_entry("mem-3", MemoryStatus::Active),
        ];
        let snapshot =
            MemorySnapshot::new("snap-1", "2026-05-02T00:00:00Z", entries).expect("snapshot");

        let active_entries = snapshot.active_entries();
        assert_eq!(active_entries[0].id, "mem-1");
        assert_eq!(active_entries[1].id, "mem-3");
    }

    #[test]
    fn memory_snapshot_deterministic_for_fixed_inputs() {
        let entries = vec![
            make_entry("mem-1", MemoryStatus::Active),
            make_entry("mem-2", MemoryStatus::Disabled),
        ];

        let snapshot_a = MemorySnapshot::new("snap-1", "2026-05-02T00:00:00Z", entries.clone())
            .expect("snapshot");
        let snapshot_b =
            MemorySnapshot::new("snap-1", "2026-05-02T00:00:00Z", entries).expect("snapshot");

        assert_eq!(snapshot_a, snapshot_b);
    }

    #[test]
    fn memory_error_codes_are_stable() {
        let cases = vec![
            (MemoryError::EmptyMemoryId, "empty_memory_id"),
            (MemoryError::EmptyContent, "empty_content"),
            (
                MemoryError::EmptyProvenanceSource,
                "empty_provenance_source",
            ),
            (
                MemoryError::EmptyProvenanceCreatedBy,
                "empty_provenance_created_by",
            ),
            (
                MemoryError::EmptyProvenanceCreatedAt,
                "empty_provenance_created_at",
            ),
            (
                MemoryError::EmptyProvenanceReason,
                "empty_provenance_reason",
            ),
            (MemoryError::EmptySnapshotId, "empty_snapshot_id"),
            (
                MemoryError::EmptySnapshotCreatedAt,
                "empty_snapshot_created_at",
            ),
        ];

        for (error, expected_code) in cases {
            assert_eq!(error.code(), expected_code);
        }
    }
}
