use ajentic_core::api::{
    integration_output_to_provider_output, integration_request_to_provider_request,
    local_runtime_config_allows_authority_bypass, IntegrationOutput, IntegrationOutputStatus,
    IntegrationRequest, IntegrationSourceKind, IntegrationTrust, LocalProviderMode,
    LocalRuntimeConfig, LocalRuntimeMode, LocalWorkspaceMetadata, RuntimeSafetyDefaults,
    RuntimeSafetyLevel,
};
use ajentic_core::execution::provider_output_is_authoritative;

fn main() {
    let summary = run_from_args(std::env::args());
    println!("{summary}");
}

fn run_from_args<I>(args: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let mut command = args.into_iter();
    let _program = command.next();

    match command.next().as_deref() {
        None | Some("dry-run") => run_dry_run_summary(),
        Some(_) => safe_usage_summary(),
    }
}

fn run_dry_run_summary() -> String {
    let runtime_config = LocalRuntimeConfig::new(
        "phase-46-dry-run-config",
        LocalRuntimeMode::DryRun,
        LocalProviderMode::Disabled,
        RuntimeSafetyLevel::Strict,
        LocalWorkspaceMetadata::new(
            "fixture-workspace",
            "opaque://fixture/workspace",
            "fixture-operator",
        )
        .expect("fixture workspace metadata must be valid"),
        RuntimeSafetyDefaults::strict(),
    )
    .expect("fixture runtime config must be valid");

    let request = IntegrationRequest::new(
        "fixture-request",
        IntegrationSourceKind::LocalLlm,
        "compose deterministic fixture summary",
        "phase-46 dry-run fixture context",
    )
    .expect("fixture integration request must be valid");

    let output = IntegrationOutput::new_untrusted(
        "fixture-output",
        request.id.clone(),
        IntegrationSourceKind::LocalLlm,
        "candidate fixture output",
        IntegrationOutputStatus::Received,
    )
    .expect("fixture integration output must be valid");

    let provider_request = integration_request_to_provider_request(&request)
        .expect("fixture provider request mapping must be valid");
    let provider_output = integration_output_to_provider_output(&output)
        .expect("fixture provider output mapping must be valid");

    let authority_bypass_enabled = local_runtime_config_allows_authority_bypass(&runtime_config);
    let provider_output_authoritative = provider_output_is_authoritative(&provider_output);

    format!(
        "AJENTIC Phase 46 local CLI dry-run summary\nprovider output remains untrusted: {}\nno files were read or written\nno provider/model was called\nno persistence occurred\nrelease-candidate readiness is not claimed\nproduction readiness is not claimed\nruntime mode: {:?}\nprovider mode: {:?}\nworkspace metadata id: {}\nauthority bypass enabled: {}\nprovider request id: {}\nprovider output id: {}\nintegration request id: {}\nintegration output trust is untrusted: {}",
        !provider_output_authoritative,
        runtime_config.runtime_mode,
        runtime_config.provider_mode,
        runtime_config.workspace.workspace_id,
        authority_bypass_enabled,
        provider_request.id,
        provider_output.id,
        request.id,
        output.trust == IntegrationTrust::Untrusted,
    )
}

fn safe_usage_summary() -> String {
    "safe usage: ajentic_core [dry-run]\nno files were read or written\nno provider/model was called\nno persistence occurred\nrelease-candidate readiness is not claimed\nproduction readiness is not claimed"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{run_dry_run_summary, run_from_args};

    #[test]
    fn dry_run_summary_mentions_untrusted_provider_output() {
        assert!(run_dry_run_summary().contains("provider output remains untrusted"));
    }

    #[test]
    fn dry_run_summary_mentions_no_files_read_or_written() {
        assert!(run_dry_run_summary().contains("no files were read or written"));
    }

    #[test]
    fn dry_run_summary_mentions_no_provider_or_model_called() {
        assert!(run_dry_run_summary().contains("no provider/model was called"));
    }

    #[test]
    fn cli_dry_run_does_not_use_stub_provider() {
        assert!(!run_dry_run_summary().contains("stub-output:"));
    }

    #[test]
    fn cli_dry_run_still_says_no_provider_or_model_called() {
        assert!(run_dry_run_summary().contains("no provider/model was called"));
    }

    #[test]
    fn cli_dry_run_does_not_use_local_provider_config() {
        let summary = run_dry_run_summary();
        assert!(!summary.contains("LocalProcess"));
        assert!(!summary.contains("LocalHttp"));
        assert!(!summary.contains("local-preview-adapter"));
    }

    #[test]
    fn dry_run_summary_mentions_no_persistence() {
        assert!(run_dry_run_summary().contains("no persistence occurred"));
    }

    #[test]
    fn cli_dry_run_summary_still_says_no_persistence() {
        assert!(run_dry_run_summary().contains("no persistence occurred"));
    }

    #[test]
    fn cli_dry_run_does_not_call_persistence_plan() {
        let summary = run_dry_run_summary();
        assert!(!summary.contains("physical_write_not_implemented"));
    }

    #[test]
    fn dry_run_summary_does_not_claim_release_candidate_readiness() {
        assert!(run_dry_run_summary().contains("release-candidate readiness is not claimed"));
    }

    #[test]
    fn dry_run_summary_does_not_claim_production_readiness() {
        assert!(run_dry_run_summary().contains("production readiness is not claimed"));
    }

    #[test]
    fn dry_run_summary_is_deterministic() {
        assert_eq!(run_dry_run_summary(), run_dry_run_summary());
    }

    #[test]
    fn cli_dry_run_does_not_submit_operator_intent() {
        let summary = run_dry_run_summary();
        assert!(!summary.contains("operator intent"));
        assert!(!summary.contains("intent submission"));
    }

    #[test]
    fn ui_preview_controls_are_not_wired_by_this_phase() {
        let summary = run_dry_run_summary();
        assert!(!summary.contains("preview control"));
        assert!(!summary.contains("ui control"));
    }

    #[test]
    fn cli_dry_run_does_not_call_local_harness_workflow() {
        let summary = run_dry_run_summary();
        assert!(!summary.contains("workflow completed in memory"));
    }

    #[test]
    fn unknown_command_usage_is_safe() {
        let output = run_from_args(["ajentic_core".to_string(), "unknown".to_string()]);
        assert!(output.contains("safe usage: ajentic_core [dry-run]"));
        assert!(output.contains("no files were read or written"));
        assert!(output.contains("no provider/model was called"));
    }
}
