#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const localTypescriptPath = path.resolve('ui/node_modules/typescript/lib/typescript.js');
const globalTypescriptPath = path.resolve('/root/.nvm/versions/node/v22.21.1/lib/node_modules/typescript/lib/typescript.js');
const typescriptPath = fs.existsSync(localTypescriptPath) ? localTypescriptPath : globalTypescriptPath;
const ts = await import(pathToFileURL(typescriptPath).href);
const typescript = ts.default ?? ts;

const cliTargets = process.argv.slice(2);
// Production default remains ui/src. Tests may pass explicit target roots.
const targetRoots = cliTargets.length > 0 ? cliTargets.map((entry) => path.resolve(entry)) : [path.resolve('ui/src')];

const forbiddenJsxHandlers = new Set([
  'onClick',
  'onSubmit',
  'onChange',
  'onInput',
  'onKeyDown',
  'onKeyUp',
]);
const forbiddenImportModules = new Set(['http', 'https', 'net', 'ws', 'axios', 'node-fetch']);

function collectSourceFiles(rootDirs) {
  const files = [];

  function walk(currentDir) {
    const entries = fs.readdirSync(currentDir, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(currentDir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
      } else if (entry.isFile() && (fullPath.endsWith('.ts') || fullPath.endsWith('.tsx'))) {
        files.push(fullPath);
      }
    }
  }

  for (const rootDir of rootDirs) {
    if (fs.existsSync(rootDir)) {
      walk(rootDir);
    }
  }

  return files.sort();
}

function locationOf(sourceFile, node) {
  const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
  return `${path.relative(process.cwd(), sourceFile.fileName)}:${line + 1}:${character + 1}`;
}

function report(violations, sourceFile, node, message) {
  violations.push(`${locationOf(sourceFile, node)}: ${message}`);
}

function isIdentifierNamed(node, name) {
  return typescript.isIdentifier(node) && node.text === name;
}

function jsxOwnerElement(node) {
  if (!node?.parent) return null;
  const owner = node.parent.parent;
  if (owner && (typescript.isJsxOpeningElement(owner) || typescript.isJsxSelfClosingElement(owner))) return owner;
  return null;
}

const files = collectSourceFiles(targetRoots);
const violations = [];

for (const filePath of files) {
  const content = fs.readFileSync(filePath, 'utf8');
  const sourceFile = typescript.createSourceFile(
    filePath,
    content,
    typescript.ScriptTarget.Latest,
    true,
    filePath.endsWith('.tsx') ? typescript.ScriptKind.TSX : typescript.ScriptKind.TS,
  );

  function visit(node) {
    if (typescript.isCallExpression(node)) {
      const expr = node.expression;
      if (isIdentifierNamed(expr, 'fetch')) report(violations, sourceFile, node, 'Forbidden call: fetch(...)');
      if (isIdentifierNamed(expr, 'setInterval')) report(violations, sourceFile, node, 'Forbidden call: setInterval(...)');
      if (isIdentifierNamed(expr, 'setTimeout')) report(violations, sourceFile, node, 'Forbidden call: setTimeout(...)');
    }

    if (typescript.isNewExpression(node) && typescript.isIdentifier(node.expression)) {
      if (node.expression.text === 'WebSocket') report(violations, sourceFile, node, 'Forbidden constructor: new WebSocket(...)');
      if (node.expression.text === 'EventSource') report(violations, sourceFile, node, 'Forbidden constructor: new EventSource(...)');
    }

    if (typescript.isIdentifier(node) && (node.text === 'localStorage' || node.text === 'sessionStorage')) {
      report(violations, sourceFile, node, `Forbidden reference: ${node.text}`);
    }

    if (typescript.isImportDeclaration(node) && typescript.isStringLiteral(node.moduleSpecifier)) {
      const moduleName = node.moduleSpecifier.text;
      if (forbiddenImportModules.has(moduleName)) report(violations, sourceFile, node, `Forbidden import module: ${moduleName}`);
    }

    if (typescript.isBinaryExpression(node) && node.operatorToken.kind === typescript.SyntaxKind.EqualsToken) {
      if (typescript.isPropertyAccessExpression(node.left) && forbiddenJsxHandlers.has(node.left.name.text)) {
        report(violations, sourceFile, node.left, `Forbidden assignment target: ${node.left.name.text}`);
      }
      if (typescript.isElementAccessExpression(node.left) && typescript.isStringLiteral(node.left.argumentExpression)) {
        const key = node.left.argumentExpression.text;
        if (forbiddenJsxHandlers.has(key)) report(violations, sourceFile, node.left, `Forbidden assignment target: ${key}`);
      }
    }

    if (typescript.isPropertyAssignment(node)) {
      const key = typescript.isIdentifier(node.name)
        ? node.name.text
        : typescript.isStringLiteral(node.name)
          ? node.name.text
          : '';
      if (forbiddenJsxHandlers.has(key)) report(violations, sourceFile, node.name, `Forbidden object property: ${key}`);
    }

    if (typescript.isJsxAttribute(node)) {
      const attr = node.name.text;
      if (forbiddenJsxHandlers.has(attr)) report(violations, sourceFile, node.name, `Forbidden JSX event-handler attribute: ${attr}`);
      const owner = jsxOwnerElement(node);
      if (attr === 'href' && owner) {
        const tag = owner.tagName.getText(sourceFile);
        if (tag === 'a') report(violations, sourceFile, node.name, 'Forbidden JSX anchor href on <a>');
      }
      if (attr === 'type') {
        const initializer = node.initializer;
        if (initializer && typescript.isStringLiteral(initializer) && initializer.text === 'submit' && owner) {
          const tag = owner.tagName.getText(sourceFile);
          if (tag === 'input') report(violations, sourceFile, node.name, 'Forbidden JSX submit input element: <input type="submit">');
        }
      }
    }

    if (typescript.isJsxOpeningElement(node) || typescript.isJsxSelfClosingElement(node)) {
      const tag = node.tagName.getText(sourceFile);
      if (tag === 'form' || tag === 'button') report(violations, sourceFile, node.tagName, `Forbidden JSX intrinsic element: <${tag}>`);
    }

    typescript.forEachChild(node, visit);
  }

  visit(sourceFile);
}

if (violations.length > 0) {
  for (const violation of violations) console.error(violation);
  console.error(`UI boundary lint failed with ${violations.length} violation(s).`);
  process.exit(1);
}

console.log(`UI boundary lint passed (${files.length} files checked).`);
