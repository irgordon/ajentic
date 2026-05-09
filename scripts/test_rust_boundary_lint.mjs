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

function makeBaseFiles() {
  return {
    'core/src/api/mod.rs': 'mod persistence;\npub use persistence::*;\n',
    'core/src/api/persistence.rs': [
      'pub fn execute_local_persistence_plan() {}',
      'pub fn verify_persisted_record_paths() {}',
      'fn demo() {',
      '    let _ = std::fs::read_to_string("x");',
      '    let _ = File::create("x");',
      '    let _ = OpenOptions::new();',
      '}',
      '',
    ].join('\n'),
    'core/src/execution/mod.rs': 'pub struct ExecutionMarker;\n',
  };
}

async function setupCase(root, files) {
  for (const [rel, content] of Object.entries(files)) {
    await writeRust(root, rel, content);
  }
}

async function withCase(files, testBody) {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'rust-boundary-lint-'));

  try {
    await setupCase(root, files);
    await testBody(root, lintRustBoundaries(root));
  } finally {
    await fs.rm(root, { recursive: true, force: true });
  }
}

function expectPass(issues, name) {
  const errors = issues.filter((issue) => issue.level === 'error');
  assert.equal(errors.length, 0, `${name} should have no blocking errors`);
}

function expectFailContains(issues, token, name) {
  assert.ok(
    issues.some((issue) => issue.level === 'error' && issue.message.includes(token)),
    `${name} should fail containing '${token}'`,
  );
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

const tests = [
  {
    name: 'expected-pass: module-only api mod is accepted',
    run: async () => {
      await withCase(makeBaseFiles(), async (_root, issues) => {
        expectPass(issues, 'module-only api mod');
      });
    },
  },
  {
    name: 'expected-fail: api mod pub struct is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/mod.rs'] += 'pub struct X;\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "forbidden token 'pub struct'", 'api mod pub struct');
      });
    },
  },
  {
    name: 'expected-fail: api mod cfg test is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/mod.rs'] += '#[cfg(test)]\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "forbidden token '#[cfg(test)]'", 'api mod cfg test');
      });
    },
  },
  {
    name: 'expected-fail: execute_local_persistence_plan outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn x(){ execute_local_persistence_plan(); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'execute_local_persistence_plan'", 'execute_local outside persistence');
      });
    },
  },
  {
    name: 'expected-pass: execute_local_persistence_plan in persistence is accepted',
    run: async () => {
      await withCase(makeBaseFiles(), async (_root, issues) => {
        expectPass(issues, 'execute_local in persistence allowed');
      });
    },
  },
  {
    name: 'expected-fail: verify_persisted_record_paths outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn y(){ verify_persisted_record_paths(); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'verify_persisted_record_paths'", 'verify outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: std::fs outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'use std::fs;\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'std::fs' is only allowed", 'std::fs outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: fs::write outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ fs::write("x", "y"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'fs::write' is only allowed", 'fs::write outside persistence');
      });
    },
  },
  {
    name: 'expected-pass: filesystem helpers inside persistence are accepted',
    run: async () => {
      await withCase(makeBaseFiles(), async (_root, issues) => {
        expectPass(issues, 'filesystem helpers inside persistence allowed');
      });
    },
  },
  {
    name: 'expected-fail: File helper outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ let _ = File::create("x"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'File::' is only allowed", 'File helper outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: OpenOptions outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ let _ = OpenOptions::new(); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'OpenOptions' is only allowed", 'OpenOptions outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: create_dir outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ std::fs::create_dir("x"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'create_dir' is only allowed", 'create_dir outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: remove_file outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ std::fs::remove_file("x"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'remove_file' is only allowed", 'remove_file outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: set_permissions outside persistence is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'fn z(){ std::fs::set_permissions("x", p); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'set_permissions' is only allowed", 'set_permissions outside persistence');
      });
    },
  },
  {
    name: 'expected-fail: tokio runtime usage is rejected',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'tokio::runtime::Runtime::new();\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, 'tokio::', 'tokio forbidden');
      });
    },
  },
  {
    name: 'expected-fail: Command usage is rejected everywhere',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += 'Command::new("x");\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'Command::'", 'Command forbidden');
      });
    },
  },
  {
    name: 'expected-fail: Command usage is rejected in persistence too',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/persistence.rs'] += 'fn p(){ Command::new("x"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, "'Command::'", 'Command forbidden in persistence');
      });
    },
  },
  {
    name: 'expected-fail: std::process usage is rejected in persistence too',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/persistence.rs'] += 'fn p(){ std::process::Command::new("x"); }\n';

      await withCase(files, async (_root, issues) => {
        expectFailContains(issues, 'std::process::', 'std::process forbidden in persistence');
      });
    },
  },
  {
    name: 'expected-pass: forbidden tokens in strings and comments are ignored',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += [
        '// tokio::runtime::Runtime::new(); Command::new("x");',
        'const TEXT: &str = "std::net TcpStream async fn .await spawn(";',
        '',
      ].join('\n');

      await withCase(files, async (_root, issues) => {
        expectPass(issues, 'strings and comments ignored');
      });
    },
  },
  {
    name: 'expected-pass: forbidden tokens in raw strings are ignored',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += [
        'const RAW: &str = r#"std::fs::write Command::new("x") tokio::runtime::Runtime::new()"#;',
        'const RAW_HASHED: &str = r##"TcpStream std::process::Command spawn("#"##;',
        '',
      ].join('\n');

      await withCase(files, async (_root, issues) => {
        expectPass(issues, 'raw strings ignored');
      });
    },
  },
  {
    name: 'expected-pass: forbidden tokens in char literals are ignored',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/execution/mod.rs'] += [
        "const QUOTE: char = '\\'';",
        "const BACKSLASH: char = '\\\\';",
        '',
      ].join('\n');

      await withCase(files, async (_root, issues) => {
        expectPass(issues, 'char literals ignored');
      });
    },
  },
  {
    name: 'expected-fail: CLI exits nonzero and prints location diagnostics for violations',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/mod.rs'] += 'pub struct X;\n';

      await withCase(files, async (root) => {
        const cliResult = runLintCli(root);
        assert.notEqual(cliResult.status, 0, 'CLI should fail for violation');
        assert.match(cliResult.stderr, /^.+:\d+:\d+: .+/m, 'diagnostic should be path:line:column: message');
      });
    },
  },
  {
    name: 'expected-fail: multiple violations are reported together',
    run: async () => {
      const files = makeBaseFiles();
      files['core/src/api/mod.rs'] += 'pub struct X;\npub trait Y {}\n';
      files['core/src/execution/mod.rs'] += 'tokio::runtime::Runtime::new();\n';

      await withCase(files, async (_root, issues) => {
        assert.ok(
          issues.filter((issue) => issue.level === 'error').length >= 3,
          'should report multiple violations together',
        );
      });
    },
  },
  {
    name: 'harness-fail: partial pass count is rejected by result summary',
    run: async () => {
      const summary = summarizeResults([{ name: 'deliberate-success', passed: true }], 2);
      assert.equal(summary.ok, false, 'partial pass count must be rejected');
      assert.equal(summary.passedCount, 1, 'partial pass count should be preserved for diagnostics');
    },
  },
];

async function main() {
  const expectedTotal = tests.length;
  const results = [];

  for (const test of tests) {
    try {
      await test.run();
      results.push({ name: test.name, passed: true });
    } catch (error) {
      results.push({ name: test.name, passed: false, error });
    }
  }

  const summary = summarizeResults(results, expectedTotal);
  if (!summary.ok) {
    console.error(`Rust boundary lint self-tests failed (${summary.passedCount}/${expectedTotal}).`);

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

  console.log(`Rust boundary lint self-tests passed (${summary.passedCount}/${expectedTotal}).`);
  process.exit(0);
}

await main().catch((error) => {
  console.error(`Rust boundary lint self-tests failed before completion: ${error?.stack ?? error}`);
  process.exit(1);
});
