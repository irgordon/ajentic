import { getUiReadModel } from "../api/readModel";
import { navigationItems } from "./navigation";
import { routeMetadata } from "./routes";
import { OverviewScreen } from "../screens/OverviewScreen";

export function AppShell(): string {
  const readModel = getUiReadModel();

  const header = ["AJENTIC", "Deterministic control layer for model-driven work."].join("\n");

  const navigation = navigationItems.map((item) => `- ${item.label}: ${item.description}`).join("\n");

  const routes = routeMetadata.map((route) => `- ${route.label}: ${route.description}`).join("\n");

  const overview = OverviewScreen();

  const projectionSummary = [
    "Run and trust summary (read-only)",
    `- Run: ${readModel.run.title} (${readModel.run.runId})`,
    `- Status text: ${readModel.run.status}`,
    `- Lifecycle summary: ${readModel.lifecycle.summary}`,
    `- Execution decision (text): ${readModel.run.executionDecision}`,
    `- Promotion decision (text): ${readModel.run.promotionDecision}`,
    `- Replay summary: ${readModel.replay.summary}`,
    `- Authority text: ${readModel.run.authority}`,
    `- Trust text: rawOutputTrusted=${readModel.output.rawOutputTrusted} (raw model output remains untrusted until Rust validation)`
  ].join("\n");

  return [header, "", "Primary navigation", navigation, "", "Main content", overview, "", "Projection summary", projectionSummary, "", "Planned routes", routes].join("\n");
}
