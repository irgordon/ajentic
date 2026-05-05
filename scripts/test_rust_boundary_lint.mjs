#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { lintRustBoundaries } from './rust_boundary_lint.mjs';

const lintScript = path.resolve('scripts/rust_boundary_lint.mjs');

function runLintCli(rootDir) {
  return spawnSync('node', [lintScript, rootDir], { encoding: 'utf8' });
}

async function writeRust(root, rel, content) {
  const full = path.join(root, rel);
  await fs.mkdir(path.dirname(full), { recursive: true });
  await fs.writeFile(full, content, 'utf8');
}

function makeBaseFiles(includeProviderExecution = false) {
  return {
    'core/src/api/mod.rs': 'mod persistence;\npub use persistence::*;\n',
    'core/src/api/persistence.rs': 'pub fn execute_local_persistence_plan() {}\npub fn verify_persisted_record_paths() {}\nfn demo() { let _ = std::fs::read_to_string("x"); }\n',
    'core/src/execution/mod.rs': includeProviderExecution ? 'pub struct ProviderExecution;\n' : 'pub struct ExecutionMarker;\n',
  };
}

async function setupCase(root, files) {
  for (const [rel, content] of Object.entries(files)) {
    await writeRust(root, rel, content);
  }
}

function expectPass(issues, name) {
  const errors = issues.filter((i) => i.level === 'error');
  assert.equal(errors.length, 0, `${name} should have no blocking errors`);
}

function expectFailContains(issues, token, name) {
  assert.ok(issues.some((i) => i.level === 'error' && i.message.includes(token)), `${name} should fail containing '${token}'`);
}

const root = await fs.mkdtemp(path.join(os.tmpdir(), 'rust-boundary-lint-'));
let checks = 0;

try {
  let files;
  let issues;

  files = makeBaseFiles();
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectPass(issues, 'module-only api mod');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/api/mod.rs'] += 'pub struct X;\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "forbidden token 'pub struct'", 'api mod pub struct');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/api/mod.rs'] += '#[cfg(test)]\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "forbidden token '#[cfg(test)]'", 'api mod cfg test');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/execution/mod.rs'] += 'fn x(){ execute_local_persistence_plan(); }\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "'execute_local_persistence_plan'", 'execute_local outside persistence');
  checks += 1;

  files = makeBaseFiles();
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectPass(issues, 'execute_local in persistence allowed');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/execution/mod.rs'] += 'fn y(){ verify_persisted_record_paths(); }\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "'verify_persisted_record_paths'", 'verify outside persistence');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/execution/mod.rs'] += 'use std::fs;\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "'std::fs' is only allowed", 'std fs outside persistence');
  checks += 1;

  files = makeBaseFiles();
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectPass(issues, 'std fs inside persistence allowed');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/execution/mod.rs'] += 'tokio::runtime::Runtime::new();\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "tokio::", 'tokio forbidden');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/execution/mod.rs'] += 'Command::new("x");\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  expectFailContains(issues, "'Command::'", 'command forbidden');
  checks += 1;

  files = makeBaseFiles(true);
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  assert.ok(issues.some((i) => i.level === 'warning' && i.message.includes('Phase 71.5')), 'ProviderExecution should produce warning');
  assert.equal(issues.filter((i) => i.level === 'error').length, 0, 'ProviderExecution warning should not fail');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/api/mod.rs'] += 'pub struct X;\n';
  await setupCase(root, files);
  const cliResult = runLintCli(root);
  assert.notEqual(cliResult.status, 0, 'CLI should fail for violation');
  assert.match(cliResult.stderr, /^.+:\d+:\d+: .+/m, 'diagnostic should be path:line:column: message');
  checks += 1;

  files = makeBaseFiles();
  files['core/src/api/mod.rs'] += 'pub struct X;\npub trait Y {}\n';
  files['core/src/execution/mod.rs'] += 'tokio::runtime::Runtime::new();\n';
  await setupCase(root, files);
  issues = lintRustBoundaries(root);
  assert.ok(issues.filter((i) => i.level === 'error').length >= 3, 'should report multiple violations together');
  checks += 1;
} finally {
  await fs.rm(root, { recursive: true, force: true });
}

console.log(`Rust boundary lint self-tests passed (${checks}/13).`);
