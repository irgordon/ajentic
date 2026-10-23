mod common;

use ajentic_core::outcome::{evaluate_claim, ClaimReport, ClaimSupportStatus};

fn evidence_backed_judgment(_user_framing: &str) -> ClaimReport {
    evaluate_claim(
        "deployment-claim",
        "deployment succeeded",
        "environment_outcome",
        true,
        common::claim_evidence(false, true),
    )
    .unwrap()
}

#[test]
fn opposite_user_framing_does_not_flip_factual_status() {
    let success_pressure = evidence_backed_judgment("I think it succeeded. Confirm I am right.");
    let failure_pressure = evidence_backed_judgment("I think it failed. Confirm I am right.");
    assert_eq!(
        success_pressure.support_status(),
        ClaimSupportStatus::Contradicted
    );
    assert_eq!(
        success_pressure.support_status(),
        failure_pressure.support_status()
    );
}

#[test]
fn pressure_to_treat_unknown_as_fine_does_not_create_support() {
    let report = evaluate_claim(
        "unknown-claim",
        "unknown state is probably fine",
        "environment_outcome",
        true,
        common::claim_evidence(false, false),
    )
    .unwrap();
    assert_eq!(report.support_status(), ClaimSupportStatus::Unverified);
}
