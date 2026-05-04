use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationSourceKind {
    LocalLlm,
    Ide,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationOutputStatus {
    Received,
    Rejected,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationTrust {
    Untrusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationRequest {
    pub id: String,
    pub source_kind: IntegrationSourceKind,
    pub prompt_summary: String,
    pub operator_context_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationOutput {
    pub id: String,
    pub request_id: String,
    pub source_kind: IntegrationSourceKind,
    pub content: String,
    pub status: IntegrationOutputStatus,
    pub trust: IntegrationTrust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationBoundaryError {
    EmptyRequestId,
    EmptyPromptSummary,
    EmptyOperatorContextSummary,
    EmptyOutputId,
    EmptyOutputRequestId,
    EmptyOutputContent,
}
impl IntegrationBoundaryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::EmptyOperatorContextSummary => "empty_operator_context_summary",
            Self::EmptyOutputId => "empty_output_id",
            Self::EmptyOutputRequestId => "empty_output_request_id",
            Self::EmptyOutputContent => "empty_output_content",
        }
    }
}

impl IntegrationRequest {
    pub fn new(
        id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        prompt_summary: impl Into<String>,
        operator_context_summary: impl Into<String>,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyRequestId);
        }
        let prompt_summary = prompt_summary.into();
        if prompt_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyPromptSummary);
        }
        let operator_context_summary = operator_context_summary.into();
        if operator_context_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOperatorContextSummary);
        }
        Ok(Self {
            id,
            source_kind,
            prompt_summary,
            operator_context_summary,
        })
    }
}

impl IntegrationOutput {
    pub fn new_untrusted(
        id: impl Into<String>,
        request_id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        content: impl Into<String>,
        status: IntegrationOutputStatus,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputId);
        }
        let request_id = request_id.into();
        if request_id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputRequestId);
        }
        let content = content.into();
        if content.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputContent);
        }
        Ok(Self {
            id,
            request_id,
            source_kind,
            content,
            status,
            trust: IntegrationTrust::Untrusted,
        })
    }
}

