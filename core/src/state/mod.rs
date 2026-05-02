#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Created,
    Evaluating,
    Failed,
    Blocked,
    Passed,
    PromotedTier1,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessState {
    pub revision: u64,
    pub lifecycle: LifecycleState,
}

impl HarnessState {
    pub fn genesis() -> Self {
        Self {
            revision: 0,
            lifecycle: LifecycleState::Created,
        }
    }
}
