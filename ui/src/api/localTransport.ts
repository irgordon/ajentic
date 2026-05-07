export type LocalUiRustTransportOperation =
  | "review_state"
  | "dry_run"
  | "workflow_review_escalation_query"
  | "review_interaction"
  | "unsupported"
  | "authority_escalation"
  | "provider_execution"
  | "persistence_write"
  | "durable_append"
  | "export_write"
  | "replay_repair"
  | "recovery_promotion"
  | "action_execution";

export type LocalUiRustTransportStatus = "accepted" | "rejected";

export type LocalUiRustTransportReason =
  | "review_state_returned"
  | "dry_run_returned"
  | "workflow_review_escalation_returned"
  | "review_interaction_preview_returned"
  | "malformed_input_rejected"
  | "oversized_input_rejected"
  | "non_local_request_rejected"
  | "unsupported_operation_rejected"
  | "authority_bearing_request_rejected"
  | "provider_execution_rejected"
  | "persistence_rejected"
  | "durable_append_rejected"
  | "export_rejected"
  | "replay_repair_rejected"
  | "recovery_promotion_rejected"
  | "action_execution_rejected"
  | "invalid_workflow_review_escalation_rejected";

export type LocalUiRustTransportRequest = Readonly<{
  requestId: string;
  operation: LocalUiRustTransportOperation;
  localOnly: boolean;
  workflowState: string;
  reviewState: string;
  escalationState: string;
  payloadSummary: string;
}>;

export type LocalUiRustTransportResponse = Readonly<{
  transportVersion: "phase-104-local-transport-v1";
  requestId: string;
  status: LocalUiRustTransportStatus;
  reason: LocalUiRustTransportReason;
  localOnly: true;
  nonAuthoritative: true;
  deterministic: true;
  providerExecutionEnabled: false;
  persistenceEnabled: false;
  durableAppendEnabled: false;
  exportEnabled: false;
  replayRepairEnabled: false;
  recoveryPromotionEnabled: false;
  actionExecutionEnabled: false;
  workflowState: string;
  reviewState: string;
  escalationState: string;
  dryRunSummary: string;
  summary: string;
}>;

export type LocalUiRustTransportStartupReport = Readonly<{
  status: "started" | "rejected";
  reason: "local_only_loopback_ready" | "remote_or_public_bind_rejected" | "empty_bind_rejected";
  bindHost: string;
  localOnly: boolean;
  publicNetworkExposed: false;
  authenticatedRemoteAccess: false;
  backgroundExecutionEnabled: false;
  providerExecutionEnabled: false;
  persistenceEnabled: false;
  replayRepairEnabled: false;
  recoveryPromotionEnabled: false;
  actionExecutionEnabled: false;
  summary: string;
}>;

const LOCAL_TRANSPORT_VERSION = "phase-104-local-transport-v1" as const;
const MAX_LOCAL_TRANSPORT_PAYLOAD_BYTES = 4096;

export function startBoundedLocalUiRustTransport(bindHost = "127.0.0.1"): LocalUiRustTransportStartupReport {
  const trimmed = bindHost.trim();
  if (trimmed.length === 0) return startupReport("rejected", "empty_bind_rejected", bindHost, false);
  if (!["127.0.0.1", "localhost", "::1"].includes(bindHost)) {
    return startupReport("rejected", "remote_or_public_bind_rejected", bindHost, false);
  }
  return startupReport("started", "local_only_loopback_ready", bindHost, true);
}

export function encodeLocalUiRustTransportRequest(request: LocalUiRustTransportRequest): string {
  return [
    `request_id=${request.requestId}`,
    `operation=${request.operation}`,
    `local_only=${request.localOnly ? "true" : "false"}`,
    `workflow_state=${request.workflowState}`,
    `review_state=${request.reviewState}`,
    `escalation_state=${request.escalationState}`,
    `payload_summary=${request.payloadSummary}`
  ].join("\n");
}

export function handleLocalUiRustTransportPayload(payload: string): LocalUiRustTransportResponse {
  if (payload.length > MAX_LOCAL_TRANSPORT_PAYLOAD_BYTES) return rejection("unknown", "oversized_input_rejected");
  const request = parsePayload(payload);
  if (request === null) return rejection("unknown", "malformed_input_rejected");
  return handleLocalUiRustTransportRequest(request);
}

export function handleLocalUiRustTransportRequest(request: LocalUiRustTransportRequest): LocalUiRustTransportResponse {
  if (!request.localOnly) return rejection(request.requestId, "non_local_request_rejected");

  if (request.operation === "review_state") {
    return acceptance(request, "review_state_returned", "review state returned through local non-authoritative transport prototype");
  }
  if (request.operation === "dry_run") {
    return acceptance(request, "dry_run_returned", "dry-run read-model data returned without provider, persistence, recovery, replay repair, or action execution");
  }
  if (request.operation === "workflow_review_escalation_query") {
    if (!validWorkflowReviewEscalation(request)) {
      return rejection(request.requestId, "invalid_workflow_review_escalation_rejected");
    }
    return acceptance(request, "workflow_review_escalation_returned", "workflow, review, and escalation state returned deterministically for operator review");
  }
  if (request.operation === "review_interaction") {
    return acceptance(request, "review_interaction_preview_returned", "bounded review interaction preview returned; no automatic workflow approval occurs");
  }

  const rejectionReasons: Record<LocalUiRustTransportOperation, LocalUiRustTransportReason> = {
    review_state: "review_state_returned",
    dry_run: "dry_run_returned",
    workflow_review_escalation_query: "workflow_review_escalation_returned",
    review_interaction: "review_interaction_preview_returned",
    unsupported: "unsupported_operation_rejected",
    authority_escalation: "authority_bearing_request_rejected",
    provider_execution: "provider_execution_rejected",
    persistence_write: "persistence_rejected",
    durable_append: "durable_append_rejected",
    export_write: "export_rejected",
    replay_repair: "replay_repair_rejected",
    recovery_promotion: "recovery_promotion_rejected",
    action_execution: "action_execution_rejected"
  };

  return rejection(request.requestId, rejectionReasons[request.operation]);
}

function parsePayload(payload: string): LocalUiRustTransportRequest | null {
  if (payload.trim().length === 0 || payload.includes("\0")) return null;
  const fields = new Map<string, string>();
  for (const line of payload.split("\n")) {
    const separator = line.indexOf("=");
    if (separator <= 0) return null;
    const key = line.slice(0, separator);
    const value = line.slice(separator + 1);
    if (key.trim() !== key || value.trim() !== value) return null;
    fields.set(key, value);
  }

  const requestId = fields.get("request_id");
  const operation = fields.get("operation");
  const localOnly = fields.get("local_only");
  const workflowState = fields.get("workflow_state");
  const reviewState = fields.get("review_state");
  const escalationState = fields.get("escalation_state");
  const payloadSummary = fields.get("payload_summary") ?? "none";
  if (!requestId || !operation || !localOnly || !workflowState || !reviewState || !escalationState || !payloadSummary) return null;
  if (localOnly !== "true" && localOnly !== "false") return null;

  return {
    requestId,
    operation: parseOperation(operation),
    localOnly: localOnly === "true",
    workflowState,
    reviewState,
    escalationState,
    payloadSummary
  };
}

function parseOperation(operation: string): LocalUiRustTransportOperation {
  if (operation === "provider_execute" || operation === "model_execute") return "provider_execution";
  if (operation === "persist" || operation === "persistence") return "persistence_write";
  if (operation === "append_ledger" || operation === "append_audit") return "durable_append";
  if (operation === "export") return "export_write";
  if (operation === "repair_replay") return "replay_repair";
  if (operation === "promote_recovery") return "recovery_promotion";
  if (operation === "execute_action") return "action_execution";
  if (operation === "authority" || operation === "auto_approve") return "authority_escalation";
  if (isLocalUiRustTransportOperation(operation)) return operation;
  return "unsupported";
}

function isLocalUiRustTransportOperation(operation: string): operation is LocalUiRustTransportOperation {
  return [
    "review_state",
    "dry_run",
    "workflow_review_escalation_query",
    "review_interaction",
    "unsupported",
    "authority_escalation",
    "provider_execution",
    "persistence_write",
    "durable_append",
    "export_write",
    "replay_repair",
    "recovery_promotion",
    "action_execution"
  ].includes(operation);
}

function validWorkflowReviewEscalation(request: LocalUiRustTransportRequest): boolean {
  return ["idle", "draft", "review", "blocked", "completed"].includes(request.workflowState)
    && ["not_started", "pending", "in_review", "accepted_for_review", "rejected"].includes(request.reviewState)
    && ["none", "operator_required", "blocked", "manual_review_required"].includes(request.escalationState);
}

function acceptance(request: LocalUiRustTransportRequest, reason: LocalUiRustTransportReason, summary: string): LocalUiRustTransportResponse {
  return response(request.requestId, "accepted", reason, request.workflowState, request.reviewState, request.escalationState, summary);
}

function rejection(requestId: string, reason: LocalUiRustTransportReason): LocalUiRustTransportResponse {
  return response(requestId, "rejected", reason, "blocked", "rejected", "manual_review_required", `request rejected fail-closed: ${reason}`);
}

function response(
  requestId: string,
  status: LocalUiRustTransportStatus,
  reason: LocalUiRustTransportReason,
  workflowState: string,
  reviewState: string,
  escalationState: string,
  summary: string
): LocalUiRustTransportResponse {
  return {
    transportVersion: LOCAL_TRANSPORT_VERSION,
    requestId,
    status,
    reason,
    localOnly: true,
    nonAuthoritative: true,
    deterministic: true,
    providerExecutionEnabled: false,
    persistenceEnabled: false,
    durableAppendEnabled: false,
    exportEnabled: false,
    replayRepairEnabled: false,
    recoveryPromotionEnabled: false,
    actionExecutionEnabled: false,
    workflowState,
    reviewState,
    escalationState,
    dryRunSummary: "dry-run transport is deterministic and side-effect free",
    summary
  };
}

function startupReport(
  status: "started" | "rejected",
  reason: LocalUiRustTransportStartupReport["reason"],
  bindHost: string,
  localOnly: boolean
): LocalUiRustTransportStartupReport {
  return {
    status,
    reason,
    bindHost,
    localOnly,
    publicNetworkExposed: false,
    authenticatedRemoteAccess: false,
    backgroundExecutionEnabled: false,
    providerExecutionEnabled: false,
    persistenceEnabled: false,
    replayRepairEnabled: false,
    recoveryPromotionEnabled: false,
    actionExecutionEnabled: false,
    summary: "bounded local-only UI-to-Rust transport prototype; deterministic review communication only; non-authoritative and side-effect free"
  };
}
