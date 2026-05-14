//! Boundary, absence, capability, and status helper lists for the local operator shell.

use super::*;

pub fn local_session_evidence_export_absence_markers() -> LocalSessionEvidenceExportAbsenceMarkers {
    LocalSessionEvidenceExportAbsenceMarkers {
        provider_execution_absent: true,
        persistence_absent: true,
        release_absent: true,
        deployment_absent: true,
        signing_absent: true,
        publishing_absent: true,
        installer_absent: true,
        update_channel_absent: true,
        public_use_absent: true,
        readiness_approval_absent: true,
        marker_summary: vec![
            "provider execution absent".to_string(),
            "persistence absent".to_string(),
            "release absent".to_string(),
            "deployment absent".to_string(),
            "signing absent".to_string(),
            "publishing absent".to_string(),
            "installer absent".to_string(),
            "update-channel absent".to_string(),
            "public-use absent".to_string(),
            "readiness absent".to_string(),
        ],
    }
}

pub fn local_provider_adapter_dry_run_capability_surface(
) -> LocalProviderAdapterDryRunCapabilitySurface {
    LocalProviderAdapterDryRunCapabilitySurface {
        controlled_dry_run_only: true,
        deterministic_fake_adapter_only: true,
        no_real_model_execution: true,
        no_process_spawn: true,
        no_network: true,
        no_shell: true,
        no_secrets: true,
        no_provider_trust: true,
        no_candidate_materialization: true,
        no_action_execution: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Controlled adapter dry run only; only deterministic_fake_adapter can execute in Phase 154. No real model execution, process, network, shell, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.".to_string(),
    }
}

pub fn local_provider_adapter_dry_run_boundary_statuses(
) -> Vec<LocalProviderAdapterDryRunBoundaryStatus> {
    vec![
        LocalProviderAdapterDryRunBoundaryStatus::ControlledDryRunOnly,
        LocalProviderAdapterDryRunBoundaryStatus::DeterministicFakeAdapterOnly,
        LocalProviderAdapterDryRunBoundaryStatus::NoRealModelExecution,
        LocalProviderAdapterDryRunBoundaryStatus::NoProcessSpawn,
        LocalProviderAdapterDryRunBoundaryStatus::NoNetwork,
        LocalProviderAdapterDryRunBoundaryStatus::NoShell,
        LocalProviderAdapterDryRunBoundaryStatus::NoSecrets,
        LocalProviderAdapterDryRunBoundaryStatus::NoProviderTrust,
        LocalProviderAdapterDryRunBoundaryStatus::NoCandidateMaterialization,
        LocalProviderAdapterDryRunBoundaryStatus::NoActionExecution,
        LocalProviderAdapterDryRunBoundaryStatus::NoProductionPersistence,
        LocalProviderAdapterDryRunBoundaryStatus::NoReadinessEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoReleaseEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoDeploymentEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn local_provider_adapter_dry_run_effect_statuses(
) -> Vec<LocalProviderAdapterDryRunEffectStatus> {
    vec![
        LocalProviderAdapterDryRunEffectStatus::NoProviderTrust,
        LocalProviderAdapterDryRunEffectStatus::NoCandidateMaterialization,
        LocalProviderAdapterDryRunEffectStatus::NoActionExecution,
        LocalProviderAdapterDryRunEffectStatus::NoReadinessEffect,
        LocalProviderAdapterDryRunEffectStatus::NoReleaseEffect,
        LocalProviderAdapterDryRunEffectStatus::NoDeploymentEffect,
        LocalProviderAdapterDryRunEffectStatus::NoPublicUseEffect,
    ]
}

pub fn constrained_local_provider_invocation_capability_surface(
) -> ConstrainedLocalProviderInvocationCapabilitySurface {
    ConstrainedLocalProviderInvocationCapabilitySurface {
        constrained_local_invocation_only: true,
        allowlisted_provider_only: true,
        allowlisted_provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
        no_arbitrary_command: true,
        no_shell: true,
        no_network: true,
        no_cloud: true,
        no_secrets: true,
        no_provider_trust: true,
        no_candidate_materialization: true,
        no_action_execution: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Constrained local provider invocation only; only allowlisted_local_deterministic_provider is enabled in Phase 156. No arbitrary command, shell, network, cloud, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.".to_string(),
    }
}

pub fn constrained_local_provider_invocation_boundary_statuses(
) -> Vec<ConstrainedLocalProviderInvocationBoundaryStatus> {
    vec![
        ConstrainedLocalProviderInvocationBoundaryStatus::ConstrainedLocalInvocationOnly,
        ConstrainedLocalProviderInvocationBoundaryStatus::AllowlistedProviderOnly,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoArbitraryCommand,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoShell,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoNetwork,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoCloud,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoSecrets,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoProviderTrust,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoCandidateMaterialization,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoActionExecution,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoProductionPersistence,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoReadinessEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoReleaseEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoDeploymentEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn constrained_local_provider_invocation_effect_statuses(
) -> Vec<ConstrainedLocalProviderInvocationEffectStatus> {
    vec![
        ConstrainedLocalProviderInvocationEffectStatus::NoProviderTrust,
        ConstrainedLocalProviderInvocationEffectStatus::NoCandidateMaterialization,
        ConstrainedLocalProviderInvocationEffectStatus::NoActionExecution,
        ConstrainedLocalProviderInvocationEffectStatus::NoReadinessEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoReleaseEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoDeploymentEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoPublicUseEffect,
    ]
}

pub(super) fn local_session_restore_boundary_statuses() -> Vec<LocalSessionRestoreBoundaryStatus> {
    vec![
        LocalSessionRestoreBoundaryStatus::LocalRestoreProjectionOnly,
        LocalSessionRestoreBoundaryStatus::NoRecoveryPromotion,
        LocalSessionRestoreBoundaryStatus::NoReplayRepair,
        LocalSessionRestoreBoundaryStatus::NoProductionPersistenceClaim,
        LocalSessionRestoreBoundaryStatus::NoReadinessEffect,
        LocalSessionRestoreBoundaryStatus::NoReleaseEffect,
        LocalSessionRestoreBoundaryStatus::NoDeploymentEffect,
        LocalSessionRestoreBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn required_controlled_internal_trial_stop_conditions(
) -> Vec<ControlledInternalTrialStopCondition> {
    vec![
        ControlledInternalTrialStopCondition::StopOnProviderTrustClaim,
        ControlledInternalTrialStopCondition::StopOnReadinessClaim,
        ControlledInternalTrialStopCondition::StopOnReleaseOrDeploymentClaim,
        ControlledInternalTrialStopCondition::StopOnPublicUseClaim,
        ControlledInternalTrialStopCondition::StopOnActionAuthorizationClaim,
        ControlledInternalTrialStopCondition::StopOnReplayRepairOrRecoveryPromotionClaim,
        ControlledInternalTrialStopCondition::StopOnPackageValidationFailure,
        ControlledInternalTrialStopCondition::StopOnRestoreValidationFailure,
        ControlledInternalTrialStopCondition::StopOnWorkflowRejection,
        ControlledInternalTrialStopCondition::StopOnOperatorEscalation,
    ]
}

pub fn controlled_internal_trial_package_absence_markers(
) -> ControlledInternalTrialPackageAbsenceMarkers {
    ControlledInternalTrialPackageAbsenceMarkers {
        release_artifact_absent: true,
        deployment_artifact_absent: true,
        readiness_approval_absent: true,
        public_use_approval_absent: true,
        production_use_approval_absent: true,
        provider_trust_absent: true,
        action_authorization_absent: true,
        replay_repair_absent: true,
        recovery_promotion_absent: true,
        default_persistence_absent: true,
        marker_summary: vec![
            "no_release_artifact".to_string(),
            "no_deployment_artifact".to_string(),
            "no_readiness_approval".to_string(),
            "no_public_use_approval".to_string(),
            "no_production_approval".to_string(),
            "no_provider_trust".to_string(),
            "no_action_authorization".to_string(),
            "no_replay_repair".to_string(),
            "no_recovery_promotion".to_string(),
            "no_default_persistence".to_string(),
        ],
    }
}

pub fn controlled_internal_trial_package_boundary_statuses(
) -> Vec<ControlledInternalTrialPackageBoundaryStatus> {
    vec![
        ControlledInternalTrialPackageBoundaryStatus::LocalOnlyTrialPackage,
        ControlledInternalTrialPackageBoundaryStatus::NonPublicTrialPackage,
        ControlledInternalTrialPackageBoundaryStatus::NoReleaseArtifact,
        ControlledInternalTrialPackageBoundaryStatus::NoDeploymentArtifact,
        ControlledInternalTrialPackageBoundaryStatus::NoReadinessApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoReleaseApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoProductionApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoPublicUseApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoProviderTrust,
        ControlledInternalTrialPackageBoundaryStatus::NoActionAuthorization,
        ControlledInternalTrialPackageBoundaryStatus::NoReplayRepair,
        ControlledInternalTrialPackageBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn complete_local_operator_workflow_boundary_statuses(
) -> Vec<CompleteLocalOperatorWorkflowBoundaryStatus> {
    vec![
        CompleteLocalOperatorWorkflowBoundaryStatus::LocalBetaWorkflowOnly,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoProviderTrust,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoProductionApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoReleaseApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoDeploymentApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoPublicUseApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoActionExecution,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoReplayRepair,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn trial_runbook_boundary_statuses() -> Vec<TrialRunbookBoundaryStatus> {
    vec![
        TrialRunbookBoundaryStatus::LocalTrialGuidanceOnly,
        TrialRunbookBoundaryStatus::NonPublicTrialGuidance,
        TrialRunbookBoundaryStatus::NoTrialExecution,
        TrialRunbookBoundaryStatus::NoStopConditionAutomation,
        TrialRunbookBoundaryStatus::NoAuthorityActivation,
        TrialRunbookBoundaryStatus::NoReadinessApproval,
        TrialRunbookBoundaryStatus::NoReleaseApproval,
        TrialRunbookBoundaryStatus::NoDeploymentApproval,
        TrialRunbookBoundaryStatus::NoPublicUseApproval,
        TrialRunbookBoundaryStatus::NoProductionApproval,
        TrialRunbookBoundaryStatus::NoActionExecution,
        TrialRunbookBoundaryStatus::NoReplayRepair,
        TrialRunbookBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub(super) fn trial_runbook_capability_surface() -> TrialRunbookCapabilitySurface {
    TrialRunbookCapabilitySurface {
        local_only: true,
        non_public: true,
        executes_trial: false,
        automates_stop_conditions: false,
        activates_authority: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
        executes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
    }
}

pub fn trial_session_evidence_boundary_statuses() -> Vec<TrialSessionEvidenceBoundaryStatus> {
    vec![
        TrialSessionEvidenceBoundaryStatus::LocalTrialEvidenceOnly,
        TrialSessionEvidenceBoundaryStatus::NonPublicEvidence,
        TrialSessionEvidenceBoundaryStatus::EvidenceNotAuthority,
        TrialSessionEvidenceBoundaryStatus::NoTrialExecution,
        TrialSessionEvidenceBoundaryStatus::NoControlledHumanUseApproval,
        TrialSessionEvidenceBoundaryStatus::NoReadinessApproval,
        TrialSessionEvidenceBoundaryStatus::NoReleaseApproval,
        TrialSessionEvidenceBoundaryStatus::NoDeploymentApproval,
        TrialSessionEvidenceBoundaryStatus::NoPublicUseApproval,
        TrialSessionEvidenceBoundaryStatus::NoProductionApproval,
        TrialSessionEvidenceBoundaryStatus::NoProviderTrust,
        TrialSessionEvidenceBoundaryStatus::NoActionAuthorization,
        TrialSessionEvidenceBoundaryStatus::NoReplayRepair,
        TrialSessionEvidenceBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn trial_session_evidence_absence_markers() -> TrialSessionEvidenceAbsenceMarkers {
    TrialSessionEvidenceAbsenceMarkers {
        release_artifact_absent: true,
        deployment_artifact_absent: true,
        readiness_approval_absent: true,
        public_use_approval_absent: true,
        production_use_approval_absent: true,
        provider_trust_absent: true,
        action_authorization_absent: true,
        replay_repair_absent: true,
        recovery_promotion_absent: true,
        default_persistence_absent: true,
        automatic_save_absent: true,
        background_persistence_absent: true,
        network_sync_absent: true,
        marker_summary: vec![
            "release artifact absent".to_string(),
            "deployment artifact absent".to_string(),
            "readiness approval absent".to_string(),
            "public use approval absent".to_string(),
            "production use approval absent".to_string(),
            "provider trust absent".to_string(),
            "action authorization absent".to_string(),
            "replay repair absent".to_string(),
            "recovery promotion absent".to_string(),
            "default persistence absent".to_string(),
            "automatic save absent".to_string(),
            "background persistence absent".to_string(),
            "network sync absent".to_string(),
        ],
    }
}

pub fn controlled_internal_trial_execution_boundary_statuses(
) -> Vec<ControlledInternalTrialExecutionBoundaryStatus> {
    vec![
        ControlledInternalTrialExecutionBoundaryStatus::ControlledInternalTrialHarnessOnly,
        ControlledInternalTrialExecutionBoundaryStatus::LocalTrialExecutionOnly,
        ControlledInternalTrialExecutionBoundaryStatus::NonPublicTrialExecution,
        ControlledInternalTrialExecutionBoundaryStatus::NoControlledHumanUseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoReadinessApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoReleaseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoDeploymentApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoPublicUseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoProductionApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoProviderTrust,
        ControlledInternalTrialExecutionBoundaryStatus::NoActionAuthorization,
        ControlledInternalTrialExecutionBoundaryStatus::NoReplayRepair,
        ControlledInternalTrialExecutionBoundaryStatus::NoRecoveryPromotion,
        ControlledInternalTrialExecutionBoundaryStatus::NoStopConditionAutomation,
        ControlledInternalTrialExecutionBoundaryStatus::NoAutomatedEscalation,
    ]
}

pub(super) fn controlled_internal_trial_execution_capability_surface(
) -> ControlledInternalTrialExecutionCapabilitySurface {
    ControlledInternalTrialExecutionCapabilitySurface {
        local_only: true,
        non_public: true,
        approves_controlled_human_use: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
        trusts_provider_output: false,
        authorizes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
        automates_stop_conditions: false,
        automates_escalation: false,
    }
}
