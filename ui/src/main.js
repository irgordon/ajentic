import { reviewSnapshots, lifecycleLegend, statusLegend } from "./sample_state.js";
import { renderApp } from "./render.js";

const root = document.getElementById("app");

if (!root) {
  throw new Error("Missing #app root element.");
}

renderApp(root, reviewSnapshots, lifecycleLegend, statusLegend);
