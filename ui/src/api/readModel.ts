import { uiReadModelFixture } from "./fixtures";
import type { UiReadModel } from "./projections";

export const UI_READ_MODEL_IS_SYNC = true as const;
export const UI_READ_MODEL_IS_FIXTURE_BACKED = true as const;
export const UI_IS_REQUEST_PREVIEW_ONLY = true as const;

export function getUiReadModel(): UiReadModel {
  return uiReadModelFixture;
}
