// Display-only typedefs for the non-authoritative visibility surface.
// These types document UI rendering shape only.
// Rust-owned records remain authoritative.

/** @typedef {"Created"|"Evaluating"|"Failed"|"Blocked"|"Passed"|"PromotedTier1"|"Rejected"|"Unknown"} LifecycleState */
/** @typedef {"Pass"|"Fail"|"Blocked"|"Unknown"} ReviewStatus */
/** @typedef {"Approved"|"Denied"} PromotionStatus */

/**
 * @typedef {Object} CandidateView
 * @property {string} id
 * @property {LifecycleState} lifecycleState
 * @property {string} runId
 * @property {string} objectiveId
 * @property {string} constraintsId
 * @property {string} domainId
 * @property {string} adapterName
 * @property {string} outputSummary
 */

/**
 * @typedef {Object} EvaluationResultView
 * @property {string} id
 * @property {string} evaluatorId
 * @property {ReviewStatus} status
 * @property {string} evidenceRef
 * @property {string[]} failureReasons
 */

/**
 * @typedef {Object} PolicyCheckView
 * @property {string} id
 * @property {ReviewStatus} status
 * @property {string} evidenceRef
 * @property {string[]} failureReasons
 */

/**
 * @typedef {Object} GovernanceView
 * @property {string} id
 * @property {ReviewStatus} status
 * @property {boolean} requiredEvaluatorsSatisfied
 * @property {PolicyCheckView[]} policyChecks
 * @property {string[]} evidenceRefs
 * @property {string[]} blockedReasons
 * @property {string[]} failureReasons
 */

/**
 * @typedef {Object} PromotionDecisionView
 * @property {string} id
 * @property {PromotionStatus} status
 * @property {LifecycleState} fromState
 * @property {LifecycleState} toState
 * @property {boolean} requiredChecksPassed
 * @property {string[]} evidenceRefs
 * @property {string[]} denialReasons
 */

/**
 * @typedef {Object} ReviewSnapshot
 * @property {string} runId
 * @property {CandidateView} candidate
 * @property {EvaluationResultView[]} evaluations
 * @property {GovernanceView} governance
 * @property {PromotionDecisionView} promotion
 * @property {string} ledgerSummary
 * @property {string} auditSummary
 * @property {string} replaySummary
 */
