#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const lintScript = path.resolve('scripts/lint_ui_boundaries.mjs');

function runLint(targetDir) {
  return spawnSync('node', [lintScript, targetDir], { encoding: 'utf8' });
}

async function writeCase(rootDir, relativePath, content) {
  const filePath = path.join(rootDir, relativePath);
  await fs.mkdir(path.dirname(filePath), { recursive: true });
  await fs.writeFile(filePath, content, 'utf8');
}

function assertLineColumnFormat(stderr) {
  assert.match(stderr, /^.+:\d+:\d+: .+/m, 'expected IDE-friendly path:line:column diagnostic format');
}

function summarizeResults(results, expectedTotal) {
  const failedNames = results.filter((result) => !result.passed).map((result) => result.name);
  const passedCount = results.length - failedNames.length;

  return {
    passedCount,
    failedNames,
    ok: failedNames.length === 0 && passedCount === expectedTotal && results.length === expectedTotal,
  };
}

const cases = [
  {
    name: 'allowed static text',
    file: 'allowed/static_text.ts',
    content: "export const note = 'onClick fetch localStorage WebSocket setTimeout readiness_approved deployment_approved';\n",
    shouldPass: true,
  },
  {
    name: 'allowed clean tsx',
    file: 'allowed/clean.tsx',
    content: 'export const Clean = () => <section><p>hello</p></section>;\n',
    shouldPass: true,
  },
  {
    name: 'allowed descriptive projection object',
    file: 'allowed/projection.ts',
    content: [
      'export const projection = {',
      '  nonAuthoritative: true,',
      '  readinessApproved: false,',
      '  productionCandidateApproved: false,',
      '  actionExecuted: false,',
      '};',
      '',
    ].join('\n'),
    shouldPass: true,
  },

  {
    name: 'forbidden fetch',
    file: 'forbidden/fetch.ts',
    content: 'fetch("/api");\n',
    shouldPass: false,
  },
  {
    name: 'forbidden XMLHttpRequest',
    file: 'forbidden/xhr.ts',
    content: 'const req = new XMLHttpRequest();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden localStorage',
    file: 'forbidden/storage.ts',
    content: 'const x = localStorage.getItem("k");\n',
    shouldPass: false,
  },
  {
    name: 'forbidden sessionStorage',
    file: 'forbidden/session-storage.ts',
    content: 'const x = sessionStorage.getItem("k");\n',
    shouldPass: false,
  },
  {
    name: 'forbidden websocket',
    file: 'forbidden/socket.ts',
    content: 'new WebSocket("ws://example");\n',
    shouldPass: false,
  },
  {
    name: 'forbidden event source',
    file: 'forbidden/event-source.ts',
    content: 'new EventSource("/events");\n',
    shouldPass: false,
  },
  {
    name: 'forbidden timeout',
    file: 'forbidden/timeout.ts',
    content: 'setTimeout(() => {}, 10);\n',
    shouldPass: false,
  },
  {
    name: 'forbidden interval',
    file: 'forbidden/interval.ts',
    content: 'setInterval(() => {}, 10);\n',
    shouldPass: false,
  },
  {
    name: 'forbidden promise timer',
    file: 'forbidden/promise.ts',
    content: 'Promise.resolve().then(() => 1);\n',
    shouldPass: false,
  },

  {
    name: 'forbidden onClick',
    file: 'forbidden/onclick.tsx',
    content: 'export const C = () => <div onClick={() => 1}>x</div>;\n',
    shouldPass: false,
  },
  {
    name: 'forbidden onSubmit property',
    file: 'forbidden/property.ts',
    content: 'const handlers = { onSubmit: () => {} };\n',
    shouldPass: false,
  },
  {
    name: 'forbidden form',
    file: 'forbidden/form.tsx',
    content: 'export const C = () => <form></form>;\n',
    shouldPass: false,
  },
  {
    name: 'forbidden button',
    file: 'forbidden/button.tsx',
    content: 'export const C = () => <button>ok</button>;\n',
    shouldPass: false,
  },
  {
    name: 'forbidden anchor href',
    file: 'forbidden/anchor.tsx',
    content: 'export const C = () => <a href="/x">x</a>;\n',
    shouldPass: false,
  },
  {
    name: 'forbidden submit input',
    file: 'forbidden/input.tsx',
    content: 'export const C = () => <input type="submit" />;\n',
    shouldPass: false,
  },

  {
    name: 'forbidden readiness approval call',
    file: 'forbidden/readiness.ts',
    content: 'approveReadiness();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden production candidate approval call',
    file: 'forbidden/production-candidate.ts',
    content: 'approveProductionCandidate();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden release candidate approval call',
    file: 'forbidden/release-candidate.ts',
    content: 'approveReleaseCandidate();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden action execution call',
    file: 'forbidden/action.ts',
    content: 'executeAction();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden recovery promotion call',
    file: 'forbidden/recovery.ts',
    content: 'promoteRecovery();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden replay repair call',
    file: 'forbidden/replay.ts',
    content: 'repairReplay();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden provider trust call',
    file: 'forbidden/provider-trust.ts',
    content: 'trustProviderOutput();\n',
    shouldPass: false,
  },
  {
    name: 'forbidden deployment approval call',
    file: 'forbidden/deployment.ts',
    content: 'approveDeployment();\n',
    shouldPass: false,
  },
];

const harnessTests = [
  {
    name: 'harness-fail: partial pass count is rejected by result summary',
    run: async () => {
      const summary = summarizeResults([{ name: 'deliberate-success', passed: true }], 2);
      assert.equal(summary.ok, false, 'partial pass count must be rejected');
      assert.equal(summary.passedCount, 1, 'partial pass count should be preserved for diagnostics');
    },
  },
];

const rootDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ui-boundary-lint-'));
const results = [];

try {
  for (const testCase of cases) {
    const caseRoot = path.join(rootDir, testCase.name.replace(/\s+/g, '-'));
    await writeCase(caseRoot, testCase.file, testCase.content);

    try {
      const result = runLint(caseRoot);

      if (testCase.shouldPass) {
        assert.equal(result.status, 0, `${testCase.name} should pass\nstderr:\n${result.stderr}`);
        assert.match(result.stdout, /UI boundary lint passed/, `${testCase.name} should emit pass summary`);
      } else {
        assert.notEqual(result.status, 0, `${testCase.name} should fail`);
        assertLineColumnFormat(result.stderr);
      }

      results.push({ name: testCase.name, passed: true });
    } catch (error) {
      results.push({ name: testCase.name, passed: false, error });
    }
  }

  for (const harnessTest of harnessTests) {
    try {
      await harnessTest.run();
      results.push({ name: harnessTest.name, passed: true });
    } catch (error) {
      results.push({ name: harnessTest.name, passed: false, error });
    }
  }
} finally {
  await fs.rm(rootDir, { recursive: true, force: true });
}

const expectedTotal = cases.length + harnessTests.length;
const summary = summarizeResults(results, expectedTotal);

if (!summary.ok) {
  console.error(`UI boundary lint self-tests failed (${summary.passedCount}/${expectedTotal}).`);

  for (const result of results) {
    if (!result.passed) {
      console.error(`- ${result.name}: ${result.error?.stack ?? result.error}`);
    }
  }

  if (results.length !== expectedTotal) {
    console.error(`- harness: executed ${results.length} test(s), expected ${expectedTotal}.`);
  }

  process.exit(1);
}

console.log(`UI boundary lint self-tests passed (${summary.passedCount}/${expectedTotal}).`);
process.exit(0);
