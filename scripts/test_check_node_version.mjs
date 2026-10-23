#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { MINIMUM_NODE_RELEASES, requireSupportedNodeVersion, supportedNodeRange } from './check_node_version.mjs';

for (const version of ['22.23.4', '22.23.5', '22.24.0', '24.22.1', '24.22.2', '24.23.0']) {
  test(`supported stable release ${version} is accepted`, () => {
    assert.equal(requireSupportedNodeVersion(version), version);
  });
}

for (const version of ['22.23.3', '22.22.99', '24.22.0', '24.21.99', '20.99.99', '23.1.0', '26.12.0', '24.22.1-rc.1', '22']) {
  test(`unsupported release ${version} is rejected`, () => {
    assert.throws(() => requireSupportedNodeVersion(version));
  });
}

test('package and lockfile engine requirements match the runtime policy', () => {
  const manifest = JSON.parse(fs.readFileSync('ui/package.json', 'utf8'));
  const lock = JSON.parse(fs.readFileSync('ui/package-lock.json', 'utf8'));
  assert.equal(manifest.engines.node, supportedNodeRange());
  assert.equal(lock.packages[''].engines.node, supportedNodeRange());
});

test('workflow Node versions use approved exact security-floor pins', () => {
  const approved = new Set(Object.values(MINIMUM_NODE_RELEASES));
  let checked = 0;
  for (const name of fs.readdirSync('.github/workflows')) {
    const source = fs.readFileSync(path.join('.github/workflows', name), 'utf8');
    for (const match of source.matchAll(/^\s*node-version:\s*["']?([^"'\s#]+)["']?/gm)) {
      assert.ok(approved.has(match[1]), `${name}: unapproved Node version ${match[1]}`);
      checked += 1;
    }
  }
  assert.ok(checked > 0, 'expected explicit workflow runtime pins');
});
