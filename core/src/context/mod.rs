#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextViewType {
    Code,
    Memory,
    Docs,
    Schema,
    Diff,
    Task,
    Example,
    System,
    ToolOutput,
    ModelOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSlice {
    pub id: String,
    pub view_type: ContextViewType,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPacket {
    pub id: String,
    pub slices: Vec<ContextSlice>,
}

impl ContextPacket {
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            slices: Vec::new(),
        }
    }
}
