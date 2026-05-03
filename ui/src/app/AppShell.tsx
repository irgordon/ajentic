import { navigationItems } from "./navigation";
import { routeMetadata } from "./routes";
import { OverviewScreen } from "../screens/OverviewScreen";

export function AppShell(): string {
  const header = [
    "AJENTIC",
    "Deterministic control layer for model-driven work."
  ].join("\n");

  const navigation = navigationItems
    .map((item) => `- ${item.label}: ${item.description}`)
    .join("\n");

  const routes = routeMetadata
    .map((route) => `- ${route.label}: ${route.description}`)
    .join("\n");

  const overview = OverviewScreen();

  return [
    header,
    "",
    "Primary navigation",
    navigation,
    "",
    "Main content",
    overview,
    "",
    "Planned routes",
    routes
  ].join("\n");
}
