import { renderLocalRuntimeReviewSurface } from "../api/localRuntimeReview";
import { getUiReadModel } from "../api/readModel";
import { navigationItems } from "./navigation";
import { routeMetadata } from "./routes";
import { OverviewScreen } from "../screens/OverviewScreen";
import { renderLocalHelpEntryText } from "./help";

export function AppShell(): string {
  const readModel = getUiReadModel();

  const header = ["AJENTIC", "Deterministic control layer for model-driven work."].join("\n");

  const navigation = navigationItems.map((item) => `- ${item.label}: ${item.description}`).join("\n");

  const routes = routeMetadata.map((route) => `- ${route.label}: ${route.description}`).join("\n");

  const overview = OverviewScreen();
  const runtimeReviewSurface = renderLocalRuntimeReviewSurface();
  const localHelp = renderLocalHelpEntryText();

  const projectionSummary = [
    "Run and trust summary (read-only)",
    `- Run: ${readModel.application.run.title} (${readModel.application.run.runId})`,
    `- Status text: ${readModel.application.run.status}`,
    `- Lifecycle summary: ${readModel.application.lifecycle.summary}`,
    `- Execution decision (text): ${readModel.application.run.executionDecision}`,
    `- Promotion decision (text): ${readModel.application.run.promotionDecision}`,
    `- Replay summary: ${readModel.application.replay.summary}`,
    `- Authority text: ${readModel.application.run.authority}`,
    `- Safety summary: ${readModel.application.safety.summary}`,
    `- Safety defaults: allowProviderNetwork=${readModel.application.safety.allowProviderNetwork}, allowFileIo=${readModel.application.safety.allowFileIo}, allowUiMutation=${readModel.application.safety.allowUiMutation}`,
    `- Provider trust: ${readModel.application.provider.outputTrust} (non-authoritative=${!readModel.application.provider.authoritative})`,
    `- Integration trust: ${readModel.application.integration.outputTrust} (non-authoritative=${!readModel.application.integration.authoritative})`,
    `- Trust text: rawOutputTrusted=${readModel.application.output.rawOutputTrusted} (raw model output remains untrusted until Rust validation)`
  ].join("\n");

  return [header, "", "Primary navigation", navigation, "", "Local runtime review", runtimeReviewSurface, "", "Local help", localHelp, "", "Main content", overview, "", "Projection summary", projectionSummary, "", "Planned routes", routes].join("\n");
}
