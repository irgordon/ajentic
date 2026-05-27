#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityClaimClassification {
    DenialMarker,
    PositiveAuthorityClaim,
    Neutral,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityClaimDecision {
    Allowed,
    Rejected,
}

pub fn classify_authority_claim(token: &str) -> AuthorityClaimClassification {
    if token.is_empty() {
        return AuthorityClaimClassification::Neutral;
    }

    if denial_markers().contains(&token) {
        return AuthorityClaimClassification::DenialMarker;
    }

    if positive_authority_markers().contains(&token) {
        return AuthorityClaimClassification::PositiveAuthorityClaim;
    }

    if token.contains('_') {
        AuthorityClaimClassification::Unknown
    } else {
        AuthorityClaimClassification::Neutral
    }
}

pub fn classify_authority_tokens(tokens: &[String]) -> AuthorityClaimDecision {
    for token in tokens {
        match classify_authority_claim(token) {
            AuthorityClaimClassification::PositiveAuthorityClaim
            | AuthorityClaimClassification::Unknown => return AuthorityClaimDecision::Rejected,
            AuthorityClaimClassification::DenialMarker | AuthorityClaimClassification::Neutral => {}
        }
    }

    AuthorityClaimDecision::Allowed
}

pub fn positive_authority_markers() -> &'static [&'static str] {
    &[
        "release_candidate_approved",
        "release_candidate_ready",
        "release_ready",
        "production_ready",
        "production_candidate_approved",
        "deployment_ready",
        "deployment_enabled",
        "public_use_ready",
        "package_approved",
        "package_created",
        "release_package_created",
        "public_package_created",
        "rehearsal_approved",
        "approval_granted",
        "release_artifact_created",
        "public_artifact_created",
        "signing_enabled",
        "signature_created",
        "artifact_signed",
        "signed_release",
        "published_release",
        "installer_enabled",
        "update_channel_enabled",
        "public_distribution_enabled",
        "public_download_created",
        "github_release_created",
        "release_tag_created",
        "provider_output_trusted",
        "action_authorized",
        "replay_repaired",
        "recovery_promoted",
    ]
}

pub fn denial_markers() -> &'static [&'static str] {
    &[
        "release_candidate_status_not_approved",
        "release_readiness_not_approved",
        "production_status_not_approved",
        "public_use_not_approved",
        "release_artifact_not_created",
        "public_artifact_not_created",
        "deployment_artifact_not_created",
        "no_signature_created",
        "no_signing",
        "no_publishing",
        "no_deployment_artifact",
        "no_public_distribution",
        "no_public_download",
        "no_github_release",
        "no_release_tag",
        "no_installer_activation",
        "no_update_channel_activation",
        "no_provider_trust",
        "no_action_authorization",
        "no_replay_repair",
        "no_recovery_promotion",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_positive_markers_rejected() {
        for token in [
            "signature_created",
            "signing_enabled",
            "release_candidate_approved",
            "provider_output_trusted",
            "action_authorized",
        ] {
            assert_eq!(
                classify_authority_claim(token),
                AuthorityClaimClassification::PositiveAuthorityClaim
            );
            assert_eq!(
                classify_authority_tokens(&[token.to_string()]),
                AuthorityClaimDecision::Rejected
            );
        }
    }

    #[test]
    fn required_denial_markers_allowed() {
        for token in [
            "no_signature_created",
            "release_artifact_not_created",
            "public_artifact_not_created",
            "release_candidate_status_not_approved",
        ] {
            assert_eq!(
                classify_authority_claim(token),
                AuthorityClaimClassification::DenialMarker
            );
            assert_eq!(
                classify_authority_tokens(&[token.to_string()]),
                AuthorityClaimDecision::Allowed
            );
        }
    }

    #[test]
    fn unknown_fails_closed_and_casing_mutation_rejected() {
        assert_eq!(
            classify_authority_claim("weird_authority_signal"),
            AuthorityClaimClassification::Unknown
        );
        assert_eq!(
            classify_authority_tokens(&["weird_authority_signal".to_string()]),
            AuthorityClaimDecision::Rejected
        );
        assert_eq!(
            classify_authority_claim("Signature_Created"),
            AuthorityClaimClassification::Unknown
        );
        assert_eq!(
            classify_authority_tokens(&["Signature_Created".to_string()]),
            AuthorityClaimDecision::Rejected
        );
    }

    #[test]
    fn token_boundary_collisions_do_not_break_denial_markers() {
        assert_eq!(
            classify_authority_claim("no_signature_created"),
            AuthorityClaimClassification::DenialMarker
        );
        assert_eq!(
            classify_authority_claim("signature_created"),
            AuthorityClaimClassification::PositiveAuthorityClaim
        );
        assert_eq!(
            classify_authority_tokens(&["no_signature_created".to_string()]),
            AuthorityClaimDecision::Allowed
        );
        assert_eq!(
            classify_authority_tokens(&["signature_created".to_string()]),
            AuthorityClaimDecision::Rejected
        );
    }
}
