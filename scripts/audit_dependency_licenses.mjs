#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';

const REVIEW_PATH = 'docs/contracts/development-license-review.json';
const REVIEWED_LICENSES = new Set(['MIT', 'BSD-3-Clause', 'Apache-2.0']);

export function auditDependencyLicenses(root = process.cwd()) {
  const lock = readJson(path.join(root, 'ui/package-lock.json'));
  const review = readJson(path.join(root, REVIEW_PATH));
  validateReviewInventory(lock.packages, review);
  return review.map(entry => auditInstalledLicense(root, lock.packages[entry.path], entry));
}

export function validateReviewInventory(packages, review) {
  if (!packages || !Array.isArray(review)) throw new Error('Malformed license inventory');
  const paths = review.map(entry => entry.path);
  if (new Set(paths).size !== paths.length) throw new Error('Duplicate license review entries');
  const locked = Object.keys(packages).filter(key => key !== '');
  if (locked.length !== paths.length || locked.some(key => !paths.includes(key))) {
    throw new Error('Every locked dependency requires an explicit license review entry');
  }
}

function auditInstalledLicense(root, locked, review) {
  const directory = path.join(root, 'ui', review.path);
  const installed = readJson(path.join(directory, 'package.json'));
  const digest = hashFile(path.join(directory, review.license_file));
  validateLicenseReview(locked, installed, review, digest);
  return { package: `${installed.name}@${installed.version}`, declared: installed.license, reviewed: review.reviewed_license };
}

export function validateLicenseReview(locked, installed, review, digest) {
  if (locked.version !== review.version || installed.version !== review.version) {
    throw new Error(`${review.path}: dependency version requires license re-review`);
  }
  if (locked.license !== review.declared_license || installed.license !== review.declared_license) {
    throw new Error(`${review.path}: declared license changed`);
  }
  if (!REVIEWED_LICENSES.has(review.reviewed_license) || digest !== review.license_sha256) {
    throw new Error(`${review.path}: license text changed or lacks review`);
  }
  if (review.declared_license !== review.reviewed_license && !review.note?.trim()) {
    throw new Error(`${review.path}: license discrepancy requires an explanation`);
  }
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function hashFile(file) {
  return createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

function runCli() {
  try {
    const rows = auditDependencyLicenses();
    for (const row of rows) console.log(`${row.package}: declared=${row.declared}; reviewed=${row.reviewed}`);
    console.log(`License inventory passed (${rows.length} locked dependencies; shipped license hashes checked).`);
  } catch (error) {
    console.error(`License inventory failed: ${error.message}`);
    process.exitCode = 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) runCli();
