#!/usr/bin/env node
import path from 'node:path';
import { pathToFileURL } from 'node:url';

export const MINIMUM_NODE_RELEASES = Object.freeze({ 22: '22.23.4', 24: '24.22.1' });
const VERSION_PATTERN = /^v?(\d+)\.(\d+)\.(\d+)$/;

export function requireSupportedNodeVersion(version = process.versions.node) {
  const actual = parseRelease(version);
  const minimum = MINIMUM_NODE_RELEASES[actual[0]];
  if (!minimum || compareReleases(actual, parseRelease(minimum)) < 0) {
    throw new Error(`Unsupported Node.js ${version}; use ${supportedNodeRange()} (supported LTS lines).`);
  }
  return version;
}

export function supportedNodeRange() {
  return Object.values(MINIMUM_NODE_RELEASES).map(version => `^${version}`).join(' || ');
}

function parseRelease(version) {
  const match = typeof version === 'string' ? VERSION_PATTERN.exec(version) : null;
  if (!match) throw new Error('Node.js must be a stable major.minor.patch release');
  return match.slice(1).map(Number);
}

function compareReleases(actual, minimum) {
  for (let index = 0; index < minimum.length; index += 1) {
    const difference = actual[index] - minimum[index];
    if (difference !== 0) return difference;
  }
  return 0;
}

function runCli() {
  try {
    console.log(`Node.js security floor passed (${requireSupportedNodeVersion()}).`);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) runCli();
