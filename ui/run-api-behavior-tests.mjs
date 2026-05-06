#!/usr/bin/env node
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';

const selfTestMode = process.argv.includes('--self-test-failure-propagation');

function assertCondition(condition, message) {
  if (!condition) throw new Error(message);
}

async function runBehaviorTests(tests, options = {}) {
  let passed = 0;
  const failures = [];

  for (const test of tests) {
    try {
      test.run();
      passed += 1;
      if (!options.silent) console.log(`ok ${passed} - ${test.name}`);
    } catch (error) {
      failures.push({ name: test.name, error });
      if (!options.silent) {
        console.error(`not ok ${passed + failures.length} - ${test.name}`);
        console.error(error instanceof Error ? error.stack : String(error));
      }
    }
  }

  return { passed, failed: failures.length };
}

async function proveFailurePropagation() {
  const result = await runBehaviorTests([
    {
      name: 'intentional_failure_propagation_probe',
      run: () => assertCondition(false, 'intentional failure propagation probe')
    }
  ], { silent: true });
  assertCondition(result.failed === 1, 'self-test expected the harness to count one failure');
  assertCondition(result.passed === 0, 'self-test expected no passing tests');
  console.log('UI API behavior harness failure propagation proof passed.');
}

async function compileBehaviorTests(outDir) {
  const compile = spawnSync('tsc', [
    '--target', 'ES2020',
    '--module', 'commonjs',
    '--moduleResolution', 'node',
    '--strict',
    '--esModuleInterop',
    '--skipLibCheck',
    '--rootDir', 'src',
    '--outDir', outDir,
    '--noEmit', 'false',
    'src/api/projections.ts',
    'src/api/fixtures.ts',
    'src/api/readModel.ts',
    'src/api/submissionBoundary.behavior.test.ts'
  ], { cwd: process.cwd(), encoding: 'utf8' });

  if (compile.status !== 0) {
    if (compile.stdout) process.stdout.write(compile.stdout);
    if (compile.stderr) process.stderr.write(compile.stderr);
    throw new Error(`TypeScript behavior test compile failed with exit code ${compile.status ?? 'unknown'}`);
  }
}

async function runCompiledBehaviorTests() {
  const outDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ajentic-ui-api-behavior-'));
  try {
    await compileBehaviorTests(outDir);
    const requireFromRunner = createRequire(import.meta.url);
    const testModule = requireFromRunner(path.join(outDir, 'api', 'submissionBoundary.behavior.test.js'));
    const tests = testModule.behaviorTests;
    assertCondition(Array.isArray(tests), 'compiled behavior test module did not export behaviorTests');
    const result = await runBehaviorTests(tests);
    if (result.failed > 0) {
      throw new Error(`UI API behavior tests failed (${result.failed}/${tests.length}).`);
    }
    console.log(`UI API behavior tests passed (${result.passed}/${tests.length}).`);
  } finally {
    await fs.rm(outDir, { recursive: true, force: true });
  }
}

if (selfTestMode) {
  await proveFailurePropagation();
} else {
  await runCompiledBehaviorTests();
}
