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

export function auditBundledAssets(root = process.cwd()) {
  const directory = path.join(root, 'ui/public');
  const inventory = readJson(path.join(directory, 'third-party-assets.json'));
  const files = fs.readdirSync(directory, { recursive: true }).filter(file => fs.statSync(path.join(directory, file)).isFile() && file !== 'third-party-assets.json');
  validateAssetInventory(files, inventory.assets);
  for (const asset of inventory.assets) {
    if (hashFile(path.join(directory, asset.file)) !== asset.sha256) throw new Error(`${asset.file}: bundled asset changed; re-review its source and notice`);
  }
  return inventory.assets;
}

export function validateAssetInventory(files, assets) {
  if (!Array.isArray(assets)) throw new Error('Malformed bundled asset inventory');
  const names = assets.map(asset => asset.file);
  const content = files.filter(file => !['.DS_Store', '.gitkeep'].includes(path.basename(file)));
  if (new Set(names).size !== names.length || names.length !== content.length || content.some(file => !names.includes(file))) throw new Error('Every bundled asset requires one review entry');
  for (const asset of assets) {
    if (!['OFL-1.1', 'ISC AND MIT'].includes(asset.license) || !/^[a-f0-9]{64}$/.test(asset.sha256)) throw new Error('Unreviewed bundled asset license or digest');
    if (asset.file.includes('..') || path.isAbsolute(asset.file)) throw new Error('Bundled asset path must remain local');
    if (!asset.source?.startsWith('https://raw.githubusercontent.com/')) throw new Error('Bundled asset requires a recorded upstream source');
  }
  for (const notice of ['fonts/Inter-LICENSE.txt', 'fonts/Courier-Prime-OFL.txt', 'icons/Lucide-LICENSE.txt']) {
    if (!names.includes(notice)) throw new Error('Bundled assets require retained font and icon notices');
  }
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
    const assets = auditBundledAssets();
    console.log(`Bundled asset inventory passed (${assets.length} font, icon, and notice hashes checked).`);
  } catch (error) {
    console.error(`License inventory failed: ${error.message}`);
    process.exitCode = 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) runCli();
