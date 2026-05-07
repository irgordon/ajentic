import { behaviorTests } from "./submissionBoundary.behavior.test";

let passed = 0;

for (const test of behaviorTests) {
  test.run();
  passed += 1;
  console.log(`ok ${passed} - ${test.name}`);
}

console.log(`UI API behavior tests passed (${passed}/${behaviorTests.length}).`);
