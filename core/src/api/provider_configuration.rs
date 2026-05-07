use std::collections::{BTreeMap, BTreeSet};

pub const MAX_PROVIDER_CONFIGURATION_PAYLOAD_BYTES: usize = 8192;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationExecutionPosture {
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationTrustPosture {
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationReadinessPosture {
    NotReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationTransportPosture {
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationType {
    LocalOnlyDeclared,
}

impl ProviderConfigurationType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "local_only_declared" => Some(Self::LocalOnlyDeclared),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalOnlyDeclared => "local_only_declared",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderCapabilityDeclaration {
    ConfigurationReview,
    TextGenerationDeclared,
    EmbeddingDeclared,
}

impl ProviderCapabilityDeclaration {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "configuration_review" => Some(Self::ConfigurationReview),
            "text_generation_declared" => Some(Self::TextGenerationDeclared),
            "embedding_declared" => Some(Self::EmbeddingDeclared),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::ConfigurationReview => "configuration_review",
            Self::TextGenerationDeclared => "text_generation_declared",
            Self::EmbeddingDeclared => "embedding_declared",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderIsolationDeclaration {
    LocalOnly,
    NoNetwork,
    NoFilesystem,
    NoBackgroundExecution,
}

impl ProviderIsolationDeclaration {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "local_only" => Some(Self::LocalOnly),
            "no_network" => Some(Self::NoNetwork),
            "no_filesystem" => Some(Self::NoFilesystem),
            "no_background_execution" => Some(Self::NoBackgroundExecution),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::NoNetwork => "no_network",
            Self::NoFilesystem => "no_filesystem",
            Self::NoBackgroundExecution => "no_background_execution",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderResourceLimits {
    pub timeout_ms: u32,
    pub max_prompt_bytes: u32,
    pub max_context_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConfiguration {
    pub provider_id: String,
    pub provider_type: ProviderConfigurationType,
    pub capabilities: Vec<ProviderCapabilityDeclaration>,
    pub resources: ProviderResourceLimits,
    pub isolation: Vec<ProviderIsolationDeclaration>,
    pub execution_posture: ProviderConfigurationExecutionPosture,
    pub transport_posture: ProviderConfigurationTransportPosture,
    pub trust_posture: ProviderConfigurationTrustPosture,
    pub readiness_posture: ProviderConfigurationReadinessPosture,
    pub local_only: bool,
    pub auto_select: bool,
    pub fallback_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConfigurationBundle {
    pub providers: Vec<ProviderConfiguration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderConfigurationRejectionReason {
    MalformedConfiguration,
    OversizedConfiguration,
    MissingRequiredField,
    InvalidProviderIdentifier,
    UnsupportedProviderType,
    InvalidCapabilityDeclaration,
    InvalidTimeoutResourceDeclaration,
    InvalidIsolationDeclaration,
    ExecutionEnabledRejected,
    TransportEnabledRejected,
    TrustEnabledRejected,
    ReadinessClaimRejected,
    DuplicateProviderIdentifier,
    AutoSelectionRejected,
    FallbackRejected,
    AuthorityBearingConfigurationRejected,
}

impl ProviderConfigurationRejectionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MalformedConfiguration => "malformed_configuration",
            Self::OversizedConfiguration => "oversized_configuration",
            Self::MissingRequiredField => "missing_required_field",
            Self::InvalidProviderIdentifier => "invalid_provider_identifier",
            Self::UnsupportedProviderType => "unsupported_provider_type",
            Self::InvalidCapabilityDeclaration => "invalid_capability_declaration",
            Self::InvalidTimeoutResourceDeclaration => "invalid_timeout_resource_declaration",
            Self::InvalidIsolationDeclaration => "invalid_isolation_declaration",
            Self::ExecutionEnabledRejected => "execution_enabled_rejected",
            Self::TransportEnabledRejected => "transport_enabled_rejected",
            Self::TrustEnabledRejected => "trust_enabled_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::DuplicateProviderIdentifier => "duplicate_provider_identifier",
            Self::AutoSelectionRejected => "auto_selection_rejected",
            Self::FallbackRejected => "fallback_rejected",
            Self::AuthorityBearingConfigurationRejected => {
                "authority_bearing_configuration_rejected"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConfigurationValidationReport {
    pub status: ProviderConfigurationStatus,
    pub reasons: Vec<ProviderConfigurationRejectionReason>,
    pub provider_count: usize,
    pub execution_enabled: bool,
    pub transport_enabled: bool,
    pub provider_trusted: bool,
    pub local_only: bool,
    pub readiness_approved: bool,
    pub mutates_authority: bool,
    pub persists_provider_state: bool,
    pub appends_audit_or_ledger: bool,
    pub summary: String,
}

impl ProviderConfigurationValidationReport {
    fn accepted(provider_count: usize, local_only: bool) -> Self {
        Self {
            status: ProviderConfigurationStatus::Accepted,
            reasons: Vec::new(),
            provider_count,
            execution_enabled: false,
            transport_enabled: false,
            provider_trusted: false,
            local_only,
            readiness_approved: false,
            mutates_authority: false,
            persists_provider_state: false,
            appends_audit_or_ledger: false,
            summary: "provider configuration accepted as deterministic, disabled, untrusted, non-ready configuration only; no provider execution is performed".to_string(),
        }
    }

    fn rejected(
        reasons: BTreeSet<ProviderConfigurationRejectionReason>,
        provider_count: usize,
    ) -> Self {
        Self {
            status: ProviderConfigurationStatus::Rejected,
            reasons: reasons.into_iter().collect(),
            provider_count,
            execution_enabled: false,
            transport_enabled: false,
            provider_trusted: false,
            local_only: false,
            readiness_approved: false,
            mutates_authority: false,
            persists_provider_state: false,
            appends_audit_or_ledger: false,
            summary: "provider configuration rejected fail-closed; provider execution, transport, trust, readiness, persistence, audit append, and ledger append remain disabled".to_string(),
        }
    }
}

pub fn validate_provider_configuration_bundle(
    bundle: &ProviderConfigurationBundle,
) -> ProviderConfigurationValidationReport {
    let mut reasons = BTreeSet::new();
    if bundle.providers.is_empty() {
        reasons.insert(ProviderConfigurationRejectionReason::MissingRequiredField);
    }

    let mut ids = BTreeSet::new();
    for provider in &bundle.providers {
        validate_provider_configuration(provider, &mut reasons);
        if !ids.insert(provider.provider_id.clone()) {
            reasons.insert(ProviderConfigurationRejectionReason::DuplicateProviderIdentifier);
        }
    }

    if reasons.is_empty() {
        ProviderConfigurationValidationReport::accepted(
            bundle.providers.len(),
            bundle.providers.iter().all(|provider| provider.local_only),
        )
    } else {
        ProviderConfigurationValidationReport::rejected(reasons, bundle.providers.len())
    }
}

fn validate_provider_configuration(
    provider: &ProviderConfiguration,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) {
    if !provider_identifier_is_valid(&provider.provider_id) {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidProviderIdentifier);
    }
    if provider.capabilities.is_empty() {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidCapabilityDeclaration);
    }
    if provider.resources.timeout_ms == 0
        || provider.resources.timeout_ms > 30_000
        || provider.resources.max_prompt_bytes == 0
        || provider.resources.max_prompt_bytes > 65_536
        || provider.resources.max_context_bytes == 0
        || provider.resources.max_context_bytes > 262_144
    {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidTimeoutResourceDeclaration);
    }
    if !provider
        .isolation
        .contains(&ProviderIsolationDeclaration::LocalOnly)
        || !provider
            .isolation
            .contains(&ProviderIsolationDeclaration::NoNetwork)
        || !provider
            .isolation
            .contains(&ProviderIsolationDeclaration::NoBackgroundExecution)
    {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidIsolationDeclaration);
    }
    if provider.execution_posture != ProviderConfigurationExecutionPosture::Disabled {
        reasons.insert(ProviderConfigurationRejectionReason::ExecutionEnabledRejected);
    }
    if provider.transport_posture != ProviderConfigurationTransportPosture::Disabled {
        reasons.insert(ProviderConfigurationRejectionReason::TransportEnabledRejected);
    }
    if provider.trust_posture != ProviderConfigurationTrustPosture::Untrusted {
        reasons.insert(ProviderConfigurationRejectionReason::TrustEnabledRejected);
    }
    if provider.readiness_posture != ProviderConfigurationReadinessPosture::NotReady {
        reasons.insert(ProviderConfigurationRejectionReason::ReadinessClaimRejected);
    }
    if !provider.local_only {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidIsolationDeclaration);
    }
    if provider.auto_select {
        reasons.insert(ProviderConfigurationRejectionReason::AutoSelectionRejected);
    }
    if provider.fallback_enabled {
        reasons.insert(ProviderConfigurationRejectionReason::FallbackRejected);
    }
}

pub fn provider_identifier_is_valid(provider_id: &str) -> bool {
    let bytes = provider_id.as_bytes();
    if bytes.len() < 3 || bytes.len() > 64 {
        return false;
    }
    bytes.iter().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-' || *byte == b'_'
    }) && bytes[0].is_ascii_lowercase()
        && bytes[bytes.len() - 1].is_ascii_lowercase()
}

pub fn parse_provider_configuration_payload(
    payload: &str,
) -> ProviderConfigurationValidationReport {
    if payload.len() > MAX_PROVIDER_CONFIGURATION_PAYLOAD_BYTES {
        let mut reasons = BTreeSet::new();
        reasons.insert(ProviderConfigurationRejectionReason::OversizedConfiguration);
        return ProviderConfigurationValidationReport::rejected(reasons, 0);
    }

    match parse_provider_blocks(payload) {
        Ok(bundle) => validate_provider_configuration_bundle(&bundle),
        Err(reasons) => ProviderConfigurationValidationReport::rejected(reasons, 0),
    }
}

fn parse_provider_blocks(
    payload: &str,
) -> Result<ProviderConfigurationBundle, BTreeSet<ProviderConfigurationRejectionReason>> {
    let mut reasons = BTreeSet::new();
    let mut blocks: Vec<BTreeMap<String, String>> = Vec::new();
    let mut current: Option<BTreeMap<String, String>> = None;

    for raw_line in payload.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "provider" {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            current = Some(BTreeMap::new());
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || current.is_none() {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            continue;
        }
        if authority_bearing_key_or_value(key, value) {
            reasons.insert(
                ProviderConfigurationRejectionReason::AuthorityBearingConfigurationRejected,
            );
        }
        if !provider_configuration_key_is_allowed(key) {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
        }
        if let Some(block) = current.as_mut() {
            if block.insert(key.to_string(), value.to_string()).is_some() {
                reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            }
        }
    }

    if let Some(block) = current.take() {
        blocks.push(block);
    }
    if blocks.is_empty() {
        reasons.insert(ProviderConfigurationRejectionReason::MissingRequiredField);
    }
    if !reasons.is_empty() {
        return Err(reasons);
    }

    let mut providers = Vec::new();
    for block in blocks {
        match provider_from_block(&block) {
            Ok(provider) => providers.push(provider),
            Err(block_reasons) => reasons.extend(block_reasons),
        }
    }
    if reasons.is_empty() {
        Ok(ProviderConfigurationBundle { providers })
    } else {
        Err(reasons)
    }
}

fn provider_from_block(
    block: &BTreeMap<String, String>,
) -> Result<ProviderConfiguration, BTreeSet<ProviderConfigurationRejectionReason>> {
    let mut reasons = BTreeSet::new();
    let provider_id = required_value(block, "id", &mut reasons).unwrap_or_default();
    let provider_type = required_value(block, "type", &mut reasons)
        .and_then(ProviderConfigurationType::parse)
        .unwrap_or_else(|| {
            reasons.insert(ProviderConfigurationRejectionReason::UnsupportedProviderType);
            ProviderConfigurationType::LocalOnlyDeclared
        });
    let capabilities = parse_list(block, "capabilities", &mut reasons, |value| {
        ProviderCapabilityDeclaration::parse(value)
            .ok_or(ProviderConfigurationRejectionReason::InvalidCapabilityDeclaration)
    });
    let isolation = parse_list(block, "isolation", &mut reasons, |value| {
        ProviderIsolationDeclaration::parse(value)
            .ok_or(ProviderConfigurationRejectionReason::InvalidIsolationDeclaration)
    });
    let resources = ProviderResourceLimits {
        timeout_ms: parse_u32(block, "timeout_ms", &mut reasons),
        max_prompt_bytes: parse_u32(block, "max_prompt_bytes", &mut reasons),
        max_context_bytes: parse_u32(block, "max_context_bytes", &mut reasons),
    };
    parse_disabled(block, "execution_enabled", &mut reasons);
    let execution_posture = ProviderConfigurationExecutionPosture::Disabled;
    parse_disabled(block, "transport_enabled", &mut reasons);
    let transport_posture = ProviderConfigurationTransportPosture::Disabled;
    let trust_posture = parse_untrusted(block, &mut reasons);
    let readiness_posture = parse_not_ready(block, &mut reasons);
    let local_only = parse_true(block, "local_only", &mut reasons);
    let auto_select = parse_false(block, "auto_select", &mut reasons);
    let fallback_enabled = parse_false(block, "fallback_enabled", &mut reasons);

    if !provider_identifier_is_valid(provider_id) {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidProviderIdentifier);
    }
    if resources.timeout_ms == 0
        || resources.timeout_ms > 30_000
        || resources.max_prompt_bytes == 0
        || resources.max_prompt_bytes > 65_536
        || resources.max_context_bytes == 0
        || resources.max_context_bytes > 262_144
    {
        reasons.insert(ProviderConfigurationRejectionReason::InvalidTimeoutResourceDeclaration);
    }

    if !reasons.is_empty() {
        return Err(reasons);
    }

    Ok(ProviderConfiguration {
        provider_id: provider_id.to_string(),
        provider_type,
        capabilities,
        resources,
        isolation,
        execution_posture,
        transport_posture,
        trust_posture,
        readiness_posture,
        local_only,
        auto_select,
        fallback_enabled,
    })
}

fn required_value<'a>(
    block: &'a BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> Option<&'a str> {
    match block.get(key) {
        Some(value) => Some(value.as_str()),
        None => {
            reasons.insert(ProviderConfigurationRejectionReason::MissingRequiredField);
            None
        }
    }
}

fn parse_list<T, F>(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
    parse_item: F,
) -> Vec<T>
where
    T: Ord,
    F: Fn(&str) -> Result<T, ProviderConfigurationRejectionReason>,
{
    let Some(value) = required_value(block, key, reasons) else {
        return Vec::new();
    };
    let mut parsed = BTreeSet::new();
    for item in value.split(',').map(str::trim) {
        if item.is_empty() {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            continue;
        }
        match parse_item(item) {
            Ok(parsed_item) => {
                parsed.insert(parsed_item);
            }
            Err(reason) => {
                reasons.insert(reason);
            }
        }
    }
    parsed.into_iter().collect()
}

fn parse_u32(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> u32 {
    let Some(value) = required_value(block, key, reasons) else {
        return 0;
    };
    match value.parse::<u32>() {
        Ok(parsed) => parsed,
        Err(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::InvalidTimeoutResourceDeclaration);
            0
        }
    }
}

fn parse_disabled(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> bool {
    match required_value(block, key, reasons) {
        Some("false") => true,
        Some("true") => {
            reasons.insert(if key == "execution_enabled" {
                ProviderConfigurationRejectionReason::ExecutionEnabledRejected
            } else {
                ProviderConfigurationRejectionReason::TransportEnabledRejected
            });
            false
        }
        Some(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            false
        }
        None => false,
    }
}

fn parse_untrusted(
    block: &BTreeMap<String, String>,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> ProviderConfigurationTrustPosture {
    match required_value(block, "untrusted", reasons) {
        Some("true") => ProviderConfigurationTrustPosture::Untrusted,
        Some("false") => {
            reasons.insert(ProviderConfigurationRejectionReason::TrustEnabledRejected);
            ProviderConfigurationTrustPosture::Untrusted
        }
        Some(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            ProviderConfigurationTrustPosture::Untrusted
        }
        None => ProviderConfigurationTrustPosture::Untrusted,
    }
}

fn parse_not_ready(
    block: &BTreeMap<String, String>,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> ProviderConfigurationReadinessPosture {
    match required_value(block, "readiness", reasons) {
        Some("not_ready") => ProviderConfigurationReadinessPosture::NotReady,
        Some(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::ReadinessClaimRejected);
            ProviderConfigurationReadinessPosture::NotReady
        }
        None => ProviderConfigurationReadinessPosture::NotReady,
    }
}

fn parse_true(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> bool {
    match required_value(block, key, reasons) {
        Some("true") => true,
        Some("false") => {
            reasons.insert(ProviderConfigurationRejectionReason::InvalidIsolationDeclaration);
            false
        }
        Some(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            false
        }
        None => false,
    }
}

fn parse_false(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<ProviderConfigurationRejectionReason>,
) -> bool {
    match required_value(block, key, reasons) {
        Some("false") => false,
        Some("true") => {
            reasons.insert(if key == "auto_select" {
                ProviderConfigurationRejectionReason::AutoSelectionRejected
            } else {
                ProviderConfigurationRejectionReason::FallbackRejected
            });
            true
        }
        Some(_) => {
            reasons.insert(ProviderConfigurationRejectionReason::MalformedConfiguration);
            true
        }
        None => true,
    }
}

fn provider_configuration_key_is_allowed(key: &str) -> bool {
    matches!(
        key,
        "id" | "type"
            | "capabilities"
            | "timeout_ms"
            | "max_prompt_bytes"
            | "max_context_bytes"
            | "isolation"
            | "execution_enabled"
            | "transport_enabled"
            | "local_only"
            | "untrusted"
            | "readiness"
            | "auto_select"
            | "fallback_enabled"
    )
}

fn authority_bearing_key_or_value(key: &str, value: &str) -> bool {
    let key = key.to_ascii_lowercase();
    let value = value.to_ascii_lowercase();
    let combined = format!("{key}={value}");
    [
        "api_key",
        "secret",
        "token",
        "execute",
        "infer",
        "stream",
        "persist",
        "append_ledger",
        "append_audit",
        "repair_replay",
        "replay_repair",
        "promote_recovery",
        "mutates_authority",
    ]
    .iter()
    .any(|needle| combined.contains(needle))
}

pub fn provider_configuration_executes_provider(
    _report: &ProviderConfigurationValidationReport,
) -> bool {
    false
}

pub fn provider_configuration_uses_transport(
    _report: &ProviderConfigurationValidationReport,
) -> bool {
    false
}

pub fn provider_configuration_trusts_provider(
    _report: &ProviderConfigurationValidationReport,
) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_payload(id: &str) -> String {
        format!(
            "provider\nid={id}\ntype=local_only_declared\ncapabilities=configuration_review,text_generation_declared\ntimeout_ms=1000\nmax_prompt_bytes=2048\nmax_context_bytes=8192\nisolation=local_only,no_network,no_filesystem,no_background_execution\nexecution_enabled=false\ntransport_enabled=false\nlocal_only=true\nuntrusted=true\nreadiness=not_ready\nauto_select=false\nfallback_enabled=false\n"
        )
    }

    fn reason_codes(report: &ProviderConfigurationValidationReport) -> Vec<&'static str> {
        report.reasons.iter().map(|reason| reason.code()).collect()
    }

    #[test]
    fn phase_106_accepts_valid_provider_configuration_as_disabled_contract_only() {
        let report = parse_provider_configuration_payload(&valid_payload("provider_alpha"));
        assert_eq!(report.status, ProviderConfigurationStatus::Accepted);
        assert_eq!(report.provider_count, 1);
        assert!(!report.execution_enabled);
        assert!(!provider_configuration_executes_provider(&report));
        assert!(!report.transport_enabled);
        assert!(!provider_configuration_uses_transport(&report));
        assert!(!report.provider_trusted);
        assert!(!provider_configuration_trusts_provider(&report));
        assert!(!report.readiness_approved);
        assert!(!report.mutates_authority);
        assert!(!report.persists_provider_state);
        assert!(!report.appends_audit_or_ledger);
    }

    #[test]
    fn phase_106_malformed_provider_configs_fail_closed() {
        let report = parse_provider_configuration_payload("id=no-provider-header");
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderConfigurationRejectionReason::MalformedConfiguration));
        assert!(!report.execution_enabled);
    }

    #[test]
    fn phase_106_missing_required_fields_fail_closed() {
        let report = parse_provider_configuration_payload("provider\nid=abc\n");
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderConfigurationRejectionReason::MissingRequiredField));
    }

    #[test]
    fn phase_106_invalid_provider_identifiers_fail_closed() {
        let report = parse_provider_configuration_payload(&valid_payload("Provider.Bad"));
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert_eq!(
            report.reasons,
            vec![ProviderConfigurationRejectionReason::InvalidProviderIdentifier]
        );
    }

    #[test]
    fn phase_106_invalid_capability_declarations_fail_closed() {
        let payload = valid_payload("provider_alpha").replace(
            "capabilities=configuration_review,text_generation_declared",
            "capabilities=network_socket",
        );
        let report = parse_provider_configuration_payload(&payload);
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderConfigurationRejectionReason::InvalidCapabilityDeclaration));
    }

    #[test]
    fn phase_106_invalid_timeout_resource_declarations_fail_closed() {
        let payload = valid_payload("provider_alpha").replace("timeout_ms=1000", "timeout_ms=0");
        let report = parse_provider_configuration_payload(&payload);
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert_eq!(
            report.reasons,
            vec![ProviderConfigurationRejectionReason::InvalidTimeoutResourceDeclaration]
        );
    }

    #[test]
    fn phase_106_execution_enabled_flags_reject() {
        let payload = valid_payload("provider_alpha")
            .replace("execution_enabled=false", "execution_enabled=true");
        let report = parse_provider_configuration_payload(&payload);
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderConfigurationRejectionReason::ExecutionEnabledRejected));
        assert!(!report.execution_enabled);
    }

    #[test]
    fn phase_106_unsupported_provider_types_reject() {
        let payload = valid_payload("provider_alpha")
            .replace("type=local_only_declared", "type=https_remote_provider");
        let report = parse_provider_configuration_payload(&payload);
        assert_eq!(report.status, ProviderConfigurationStatus::Rejected);
        assert!(report
            .reasons
            .contains(&ProviderConfigurationRejectionReason::UnsupportedProviderType));
    }

    #[test]
    fn phase_106_duplicate_identifiers_reject_deterministically() {
        let payload = format!(
            "{}\n{}",
            valid_payload("provider_alpha"),
            valid_payload("provider_alpha")
        );
        let first = parse_provider_configuration_payload(&payload);
        let second = parse_provider_configuration_payload(&payload);
        assert_eq!(first.status, ProviderConfigurationStatus::Rejected);
        assert_eq!(first.reasons, second.reasons);
        assert_eq!(
            first.reasons,
            vec![ProviderConfigurationRejectionReason::DuplicateProviderIdentifier]
        );
    }

    #[test]
    fn phase_106_validation_reason_ordering_is_deterministic() {
        let payload = valid_payload("Provider.Bad")
            .replace(
                "capabilities=configuration_review,text_generation_declared",
                "capabilities=socket",
            )
            .replace("timeout_ms=1000", "timeout_ms=0")
            .replace("execution_enabled=false", "execution_enabled=true")
            .replace("auto_select=false", "auto_select=true");
        let report = parse_provider_configuration_payload(&payload);
        assert_eq!(
            reason_codes(&report),
            vec![
                "invalid_provider_identifier",
                "invalid_capability_declaration",
                "invalid_timeout_resource_declaration",
                "execution_enabled_rejected",
                "auto_selection_rejected",
            ]
        );
    }

    #[test]
    fn phase_106_provider_trust_and_transport_remain_disabled() {
        let report = parse_provider_configuration_payload(&valid_payload("provider_alpha"));
        assert!(!report.transport_enabled);
        assert!(!report.provider_trusted);
        assert!(!report.execution_enabled);
    }
}
