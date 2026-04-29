export const reviewSnapshots = [
  {
    runId: "run-001",
    candidate: {
      id: "candidate-001",
      lifecycleState: "PromotedTier1",
      runId: "run-001",
      objectiveId: "objective-compile-safety",
      constraintsId: "constraints-standard",
      domainId: "domain-core-rust",
      adapterName: "mock-adapter",
      outputSummary: "Static sample output. Display only; untrusted by default."
    },
    evaluations: [
      {
        id: "eval-001",
        evaluatorId: "safety-evaluator",
        status: "Pass",
        evidenceRef: "ev://safety/001",
        failureReasons: []
      },
      {
        id: "eval-002",
        evaluatorId: "correctness-evaluator",
        status: "Fail",
        evidenceRef: "ev://correctness/002",
        failureReasons: ["Missing acceptance criterion proof."]
      }
    ],
    governance: {
      id: "gov-001",
      status: "Blocked",
      requiredEvaluatorsSatisfied: false,
      policyChecks: [
        {
          id: "policy-required-evidence",
          status: "Blocked",
          evidenceRef: "ev://policy/missing",
          failureReasons: ["Required evidence reference missing for one evaluator."]
        },
        {
          id: "policy-risk-threshold",
          status: "Unknown",
          evidenceRef: "missing",
          failureReasons: ["Risk summary unavailable."]
        }
      ],
      evidenceRefs: ["ev://safety/001", "ev://correctness/002"],
      blockedReasons: ["Required evaluators not fully satisfied."],
      failureReasons: ["One required evaluator failed."]
    },
    promotion: {
      id: "prom-001",
      status: "Denied",
      fromState: "Passed",
      toState: "Rejected",
      requiredChecksPassed: false,
      evidenceRefs: ["ev://correctness/002"],
      denialReasons: ["Governance blocked; promotion eligibility is Rust-owned and unmet."]
    },
    ledgerSummary: "Ledger: Rust-owned append-only facts (displayed sample, not UI-created).",
    auditSummary: "Audit: Rust-derived explanation (displayed sample, not UI-created).",
    replaySummary: "Replay: Rust-owned reconstruction from facts (displayed sample, not UI-created)."
  }
];

export const lifecycleLegend = [
  "Created",
  "Evaluating",
  "Failed",
  "Blocked",
  "Passed",
  "PromotedTier1",
  "Rejected",
  "Unknown"
];

export const statusLegend = ["Pass", "Fail", "Blocked", "Unknown"];
