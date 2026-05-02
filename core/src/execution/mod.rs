#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub id: String,
    pub status: ExecutionStatus,
}

impl ExecutionPlan {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: ExecutionStatus::NotStarted,
        }
    }
}
