#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

const require = createRequire(new URL('../ui/package.json', import.meta.url));
const Ajv2020 = require('ajv/dist/2020.js');
const addFormats = require('ajv-formats');
const DIALECT = 'https://json-schema.org/draft/2020-12/schema';
const SCHEMA_SUFFIX = '.schema.json';
const DEFAULT_FIXTURES = 'tests/schema-fixtures.json';

export function validateRepositorySchemas(root = process.cwd()) {
  const catalog = loadSchemaCatalog(root);
  const validator = compileSchemaCatalog(catalog);
  const fixtureCount = validatePayloadFixtures(validator, root);
  return { schemaCount: catalog.length, fixtureCount };
}

export function compileSchemaCatalog(catalog) {
  const validator = createSchemaValidator();
  for (const entry of catalog) registerSchema(validator, entry);
  for (const entry of catalog) compileRegisteredSchema(validator, entry);
  return validator;
}

function loadSchemaCatalog(root) {
  const files = listSchemaFiles(path.join(root, 'schemas'));
  if (files.length === 0) throw new Error('No repository schemas found');
  return files.map(file => ({ file, schema: readJson(file) }));
}

function createSchemaValidator() {
  const validator = new Ajv2020({ allErrors: true, strictTypes: false, strictRequired: false });
  addFormats(validator);
  return validator;
}

function registerSchema(validator, { file, schema }) {
  if (schema.$schema !== DIALECT) throw new Error(`${file}: unsupported schema dialect`);
  if (!schema.$id) throw new Error(`${file}: missing schema $id`);
  validator.addSchema(schema);
}

function compileRegisteredSchema(validator, { file, schema }) {
  if (!validator.getSchema(schema.$id)) throw new Error(`${file}: schema did not compile`);
}

function validatePayloadFixtures(validator, root) {
  const fixtures = readJson(path.join(root, DEFAULT_FIXTURES));
  if (!Array.isArray(fixtures) || fixtures.length === 0) throw new Error('No schema fixtures found');
  for (const fixture of fixtures) validatePayloadFixture(validator, fixture);
  return fixtures.length;
}

export function validatePayloadFixture(validator, fixture) {
  validateFixtureShape(fixture);
  const validate = validator.getSchema(fixture.schema);
  if (!validate) throw new Error(`${fixture.name}: unknown schema ${fixture.schema}`);
  const valid = validate(fixture.data);
  if (valid !== fixture.valid) {
    throw new Error(`${fixture.name}: expected valid=${fixture.valid}; ${validator.errorsText(validate.errors)}`);
  }
}

function validateFixtureShape(fixture) {
  if (!fixture || typeof fixture.name !== 'string' || !fixture.name.trim()) {
    throw new Error('Schema fixture requires a name');
  }
  if (typeof fixture.schema !== 'string' || typeof fixture.valid !== 'boolean' || !Object.hasOwn(fixture, 'data')) {
    throw new Error(`${fixture.name}: malformed fixture contract`);
  }
}

function listSchemaFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) return listSchemaFiles(file);
    return entry.isFile() && entry.name.endsWith(SCHEMA_SUFFIX) ? [file] : [];
  }).sort();
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function runCli() {
  try {
    const result = validateRepositorySchemas(process.argv[2]);
    console.log(`Schema validation passed (${result.schemaCount} schemas, ${result.fixtureCount} payload cases).`);
  } catch (error) {
    console.error(`Schema validation failed: ${error.message}`);
    process.exitCode = 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) runCli();
