#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';
import { compileSchemaCatalog, validateCoverageManifest, validatePayloadFixture, validateRepositorySchemas } from './validate_schemas.mjs';

const DIALECT = 'https://json-schema.org/draft/2020-12/schema';
const ID = 'https://example.invalid/review.schema.json';

function catalog(properties = {}) {
  return [{ file: 'fixture.schema.json', schema: { $schema: DIALECT, $id: ID, ...properties } }];
}

test('all repository schemas compile and payload fixtures match their expected results', () => {
  const result = validateRepositorySchemas();
  assert.ok(result.schemaCount > 0);
  assert.ok(result.fixtureCount > 0);
});

test('invalid metaschema keywords fail validation despite being valid JSON', () => {
  assert.throws(() => compileSchemaCatalog(catalog({ type: 'not-a-json-type' })));
});

test('strict type-context validation remains enabled', () => {
  assert.throws(() => compileSchemaCatalog(catalog({ minItems: 1 })), /strictTypes/);
});

test('strict required-property validation remains enabled', () => {
  assert.throws(() => compileSchemaCatalog(catalog({ type: 'object', required: ['missing'] })), /strictRequired/);
});

test('a new schema without a coverage decision fails the gate', () => {
  assert.throws(() => validateCoverageManifest(catalog(), [], []), /Missing schema coverage/);
});

test('payload coverage requires both acceptance and rejection cases', () => {
  const manifest = [{ schema: ID, status: 'payload-cases' }];
  const fixtures = [{ schema: ID, valid: true, data: {} }];
  assert.throws(() => validateCoverageManifest(catalog(), fixtures, manifest), /valid and invalid/);
});

test('compile-only coverage requires an explicit reason', () => {
  assert.throws(() => validateCoverageManifest(catalog(), [], [{ schema: ID, status: 'compile-only' }]), /reason/);
});

test('stale coverage entries cannot hide deleted schemas', () => {
  assert.throws(() => validateCoverageManifest([], [], [{ schema: ID, status: 'compile-only', reason: 'gap' }]), /Unknown coverage/);
});

test('missing references fail closed without network retrieval', () => {
  assert.throws(() => compileSchemaCatalog(catalog({ $ref: 'missing.schema.json' })));
});

test('duplicate schema identities are rejected', () => {
  const entries = catalog();
  assert.throws(() => compileSchemaCatalog([...entries, ...entries]));
});

test('a payload expectation mismatch fails the gate', () => {
  const validator = compileSchemaCatalog(catalog({ type: 'string' }));
  assert.throws(() => validatePayloadFixture(validator, { name: 'mismatch', schema: ID, valid: true, data: 42 }));
});

test('fixtures cannot silently skip validation through an unknown schema', () => {
  const validator = compileSchemaCatalog(catalog());
  assert.throws(() => validatePayloadFixture(validator, { name: 'unknown', schema: 'missing', valid: false, data: {} }));
});

test('fixtures require explicit boolean expectations', () => {
  const validator = compileSchemaCatalog(catalog());
  assert.throws(() => validatePayloadFixture(validator, { name: 'bad', schema: ID, valid: 'false', data: {} }));
});

test('date-time formats are checked rather than accepted as arbitrary strings', () => {
  const validator = compileSchemaCatalog(catalog({ type: 'string', format: 'date-time' }));
  validatePayloadFixture(validator, { name: 'bad date', schema: ID, valid: false, data: 'yesterday' });
  validatePayloadFixture(validator, { name: 'valid date', schema: ID, valid: true, data: '2026-10-23T12:00:00Z' });
});

test('the CLI returns nonzero for an invalid schema', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ajentic-schema-cli-'));
  try {
    fs.mkdirSync(path.join(root, 'schemas'));
    fs.writeFileSync(path.join(root, 'schemas/bad.schema.json'), JSON.stringify(catalog({ type: 'invalid-type' })[0].schema));
    const result = spawnSync(process.execPath, ['scripts/validate_schemas.mjs', root], { encoding: 'utf8', timeout: 10000 });
    assert.equal(result.status, 1);
    assert.match(result.stderr, /Schema validation failed/);
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
});
