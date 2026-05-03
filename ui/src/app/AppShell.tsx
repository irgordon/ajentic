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
    `- Run: ${readModel.run.title} (${readModel.run.runId})`,
    `- Run status: ${readModel.run.status}`,
    `- Lifecycle: ${readModel.lifecycle.summary}`,
    `- Execution decision: ${readModel.run.executionDecision}`,
    `- Promotion decision: ${readModel.run.promotionDecision}`,
    `- Replay: ${readModel.replay.summary}`,
    `- Output trust: rawOutputTrusted=${readModel.output.rawOutputTrusted}`
  ].join("\n");

  return [header, "", "Primary navigation", navigation, "", "Main content", overview, "", "Projection summary", projectionSummary, "", "Planned routes", routes].join("\n");
}
