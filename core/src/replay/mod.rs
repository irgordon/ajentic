#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayStatus {
    Replayable,
    NotReplayable,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIntegrity {
    Valid,
    Invalid,
    Incomplete,
    Conflicting,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub status: ReplayStatus,
    pub integrity: ReplayIntegrity,
    pub events_replayed: u64,
}

impl ReplayReport {
    pub fn unknown() -> Self {
        Self {
            status: ReplayStatus::Unknown,
            integrity: ReplayIntegrity::Unknown,
            events_replayed: 0,
        }
    }
}
