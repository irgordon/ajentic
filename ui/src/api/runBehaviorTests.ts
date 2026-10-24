import { behaviorTests } from "./submissionBoundary.behavior.test.js";
import { reviewWorkspaceTests } from "../app/reviewWorkspace.behavior.test.js";

let passed = 0;

const allTests = [...behaviorTests, ...reviewWorkspaceTests];
for (const test of allTests) {
  test.run();
  passed += 1;
  console.log(`ok ${passed} - ${test.name}`);
}

console.log(`UI API behavior tests passed (${passed}/${allTests.length}).`);
