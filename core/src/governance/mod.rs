//! Governance module.

pub mod contract;
pub mod precheck;
pub mod promotion;
pub mod runtime;

pub use precheck::GovernanceReviewInput;
pub use promotion::{
    decide_promotion, promote_candidate, PromotionDecisionRecord, PromotionStatus,
};
pub use runtime::{review_governance, GovernanceResultRecord, GovernanceStatus};
