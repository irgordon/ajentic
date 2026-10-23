#!/usr/bin/env node
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { auditDependencyLicenses, validateLicenseReview, validateReviewInventory } from './audit_dependency_licenses.mjs';

const DECLARED = { version: '1.0.0', license: 'MIT' };
const REVIEW = { path: 'node_modules/fixture', version: '1.0.0', declared_license: 'MIT', reviewed_license: 'MIT', license_sha256: 'digest' };

test('installed dependency licenses match the complete reviewed inventory', () => {
  const rows = auditDependencyLicenses();
  assert.equal(rows.find(row => row.package === 'fast-uri@3.0.1').reviewed, 'BSD-3-Clause');
});

test('a new locked dependency requires a license review', () => {
  assert.throws(() => validateReviewInventory({ '': {}, 'node_modules/new': DECLARED }, []), /explicit license review/);
});

test('changed license text cannot reuse the prior review', () => {
  assert.throws(() => validateLicenseReview(DECLARED, DECLARED, REVIEW, 'changed'), /license text changed/);
});

test('changed dependency versions require license re-review', () => {
  assert.throws(() => validateLicenseReview({ ...DECLARED, version: '2.0.0' }, DECLARED, REVIEW, 'digest'), /version requires/);
});

test('a metadata and license-text discrepancy requires a recorded explanation', () => {
  const review = { ...REVIEW, reviewed_license: 'BSD-3-Clause' };
  assert.throws(() => validateLicenseReview(DECLARED, DECLARED, review, 'digest'), /explanation/);
});
