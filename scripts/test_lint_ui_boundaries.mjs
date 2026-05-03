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

function writeCase(rootDir, relativePath, content) {
  const filePath = path.join(rootDir, relativePath);
  return fs.mkdir(path.dirname(filePath), { recursive: true }).then(() => fs.writeFile(filePath, content, 'utf8'));
}

function assertLineColumnFormat(stderr) {
  assert.match(stderr, /^.+:\d+:\d+: .+/m, 'expected IDE-friendly path:line:column diagnostic format');
}

const cases = [
  { name: 'allowed static text', file: 'allowed/static_text.ts', content: "export const note = 'onClick fetch localStorage WebSocket setTimeout';\n", shouldPass: true },
  { name: 'clean tsx', file: 'allowed/clean.tsx', content: 'export const Clean = () => <section><p>hello</p></section>;\n', shouldPass: true },
  { name: 'forbidden fetch', file: 'forbidden/fetch.ts', content: 'fetch("/api");\n', shouldPass: false },
  { name: 'forbidden localStorage', file: 'forbidden/storage.ts', content: 'const x = localStorage.getItem("k");\n', shouldPass: false },
  { name: 'forbidden websocket', file: 'forbidden/socket.ts', content: 'new WebSocket("ws://example");\n', shouldPass: false },
  { name: 'forbidden timeout', file: 'forbidden/timeout.ts', content: 'setTimeout(() => {}, 10);\n', shouldPass: false },
  { name: 'forbidden onClick', file: 'forbidden/onclick.tsx', content: 'export const C = () => <div onClick={() => 1}>x</div>;\n', shouldPass: false },
  { name: 'forbidden form', file: 'forbidden/form.tsx', content: 'export const C = () => <form></form>;\n', shouldPass: false },
  { name: 'forbidden button', file: 'forbidden/button.tsx', content: 'export const C = () => <button>ok</button>;\n', shouldPass: false },
  { name: 'forbidden anchor href', file: 'forbidden/anchor.tsx', content: 'export const C = () => <a href="/x">x</a>;\n', shouldPass: false },
  { name: 'forbidden submit input', file: 'forbidden/input.tsx', content: 'export const C = () => <input type="submit" />;\n', shouldPass: false },
  { name: 'forbidden onSubmit property', file: 'forbidden/property.ts', content: 'const handlers = { onSubmit: () => {} };\n', shouldPass: false },
];

const rootDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ui-boundary-lint-'));
let passed = 0;

try {
  for (const testCase of cases) {
    const caseRoot = path.join(rootDir, testCase.name.replace(/\s+/g, '-'));
    await writeCase(caseRoot, testCase.file, testCase.content);
    const result = runLint(caseRoot);
    if (testCase.shouldPass) {
      assert.equal(result.status, 0, `${testCase.name} should pass`);
      assert.match(result.stdout, /UI boundary lint passed/, `${testCase.name} should emit pass summary`);
    } else {
      assert.notEqual(result.status, 0, `${testCase.name} should fail`);
      assertLineColumnFormat(result.stderr);
    }
    passed += 1;
  }
} finally {
  await fs.rm(rootDir, { recursive: true, force: true });
}

console.log(`UI boundary lint self-tests passed (${passed}/${cases.length}).`);
