#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const localTypescriptPath = path.resolve('ui/node_modules/typescript/lib/typescript.js');

const nodeVersionGlobalTypescriptPath = path.resolve(
  path.dirname(process.execPath),
  '..',
  'lib',
  'node_modules',
  'typescript',
  'lib',
  'typescript.js',
);

const fallbackGlobalTypescriptPath = path.resolve(
  '/root/.nvm/versions/node/v22.21.1/lib/node_modules/typescript/lib/typescript.js',
);

const typescriptPath = fs.existsSync(localTypescriptPath)
  ? localTypescriptPath
  : fs.existsSync(nodeVersionGlobalTypescriptPath)
    ? nodeVersionGlobalTypescriptPath
    : fallbackGlobalTypescriptPath;

if (!fs.existsSync(typescriptPath)) {
  console.error(
    `Unable to locate TypeScript runtime. Checked: ${localTypescriptPath}, ${nodeVersionGlobalTypescriptPath}, ${fallbackGlobalTypescriptPath}`,
  );
  process.exit(1);
}

const ts = await import(pathToFileURL(typescriptPath).href);
const typescript = ts.default ?? ts;

const cliTargets = process.argv.slice(2);

// Production default remains ui/src. Tests may pass explicit target roots.
const targetRoots = cliTargets.length > 0
  ? cliTargets.map((entry) => path.resolve(entry))
  : [path.resolve('ui/src')];

const forbiddenJsxHandlers = new Set([
  'onClick',
  'onSubmit',
  'onChange',
  'onInput',
  'onKeyDown',
  'onKeyUp',
]);

const forbiddenImportModules = new Set([
  'http',
  'https',
  'net',
  'ws',
  'axios',
  'node-fetch',
]);

const forbiddenCallIdentifiers = new Set([
  'fetch',
  'setInterval',
  'setTimeout',

  // Authority-shaped UI calls.
  'approveReadiness',
  'approveProductionCandidate',
  'approveReleaseCandidate',
  'approvePublicUse',
  'approveProductionHumanUse',
  'approveDeployment',
  'executeAction',
  'executeOperatorAction',
  'promoteRecovery',
  'repairReplay',
  'trustProviderOutput',
  'promoteProviderOutput',
  'expandPersistenceAuthority',
  'approvePersistence',
  'rewriteGovernance',
  'grantPolicyAuthority',
]);

const forbiddenConstructorIdentifiers = new Set([
  'WebSocket',
  'EventSource',
  'XMLHttpRequest',
]);

const forbiddenIdentifierReferences = new Set([
  'localStorage',
  'sessionStorage',
]);

const forbiddenAuthorityProperties = new Set([
  'readinessApproved',
  'productionCandidateApproved',
  'releaseCandidateApproved',
  'publicUseApproved',
  'productionHumanUseApproved',
  'deploymentApproved',
  'deploymentAutomationEnabled',
  'releaseArtifactCreated',
  'installerEnabled',
  'updateChannelEnabled',
  'signingEnabled',
  'publishingEnabled',
  'productionDeploymentEnabled',
  'publicReleaseEnabled',
  'providerTrustGranted',
  'providerOutputPromoted',
  'persistenceAuthorityExpanded',
  'replayRepaired',
  'recoveryPromoted',
  'actionExecuted',
  'governanceAuthorityRewritten',
  'policyAuthorityGranted',

  // snake_case variants used by Rust-facing projections or fixtures.
  'readiness_approved',
  'production_candidate_approved',
  'release_candidate_approved',
  'public_use_approved',
  'production_human_use_approved',
  'deployment_approved',
  'deployment_automation_enabled',
  'release_artifact_created',
  'installer_enabled',
  'update_channel_enabled',
  'signing_enabled',
  'publishing_enabled',
  'production_deployment_enabled',
  'public_release_enabled',
  'provider_trust_granted',
  'provider_output_promoted',
  'persistence_authority_expanded',
  'replay_repaired',
  'recovery_promoted',
  'action_executed',
  'governance_authority_rewritten',
  'policy_authority_granted',
]);

function collectSourceFiles(rootDirs) {
  const files = [];

  function walk(currentDir) {
    const entries = fs.readdirSync(currentDir, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = path.join(currentDir, entry.name);

      if (entry.isDirectory()) {
        if (['node_modules', 'dist', 'build', 'coverage', '.git'].includes(entry.name)) {
          continue;
        }

        walk(fullPath);
        continue;
      }

      if (entry.isFile() && (fullPath.endsWith('.ts') || fullPath.endsWith('.tsx'))) {
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

function identifierText(node) {
  if (!node) {
    return '';
  }

  if (typescript.isIdentifier(node) || typescript.isStringLiteral(node) || typescript.isNumericLiteral(node)) {
    return node.text;
  }

  return '';
}

function propertyNameText(name) {
  if (!name) {
    return '';
  }

  if (typescript.isIdentifier(name) || typescript.isStringLiteral(name) || typescript.isNumericLiteral(name)) {
    return name.text;
  }

  return '';
}

function jsxOwnerElement(node) {
  if (!node?.parent) {
    return null;
  }

  const owner = node.parent.parent;

  if (
    owner
    && (typescript.isJsxOpeningElement(owner) || typescript.isJsxSelfClosingElement(owner))
  ) {
    return owner;
  }

  return null;
}

function isFalseLiteral(node) {
  return node.kind === typescript.SyntaxKind.FalseKeyword;
}

function isExplicitDisabledString(node) {
  return typescript.isStringLiteral(node)
    && ['false', 'disabled', 'denied', 'not_approved', 'not-approved'].includes(node.text);
}

function isAllowedDescriptiveFalseProperty(node) {
  if (!typescript.isPropertyAssignment(node)) {
    return false;
  }

  const key = propertyNameText(node.name);

  if (!forbiddenAuthorityProperties.has(key)) {
    return false;
  }

  return isFalseLiteral(node.initializer) || isExplicitDisabledString(node.initializer);
}

function checkAuthorityPropertyAssignment(violations, sourceFile, node) {
  if (!typescript.isPropertyAssignment(node)) {
    return;
  }

  const key = propertyNameText(node.name);

  if (!forbiddenAuthorityProperties.has(key)) {
    return;
  }

  if (isAllowedDescriptiveFalseProperty(node)) {
    return;
  }

  report(
    violations,
    sourceFile,
    node.name,
    `Forbidden authority-granting object property: ${key}`,
  );
}

function checkAuthorityAssignmentTarget(violations, sourceFile, node) {
  if (!typescript.isBinaryExpression(node) || node.operatorToken.kind !== typescript.SyntaxKind.EqualsToken) {
    return;
  }

  const left = node.left;

  if (typescript.isPropertyAccessExpression(left)) {
    const key = left.name.text;

    if (forbiddenJsxHandlers.has(key)) {
      report(violations, sourceFile, left, `Forbidden assignment target: ${key}`);
    }

    if (forbiddenAuthorityProperties.has(key)) {
      report(violations, sourceFile, left, `Forbidden authority assignment target: ${key}`);
    }
  }

  if (typescript.isElementAccessExpression(left) && typescript.isStringLiteral(left.argumentExpression)) {
    const key = left.argumentExpression.text;

    if (forbiddenJsxHandlers.has(key)) {
      report(violations, sourceFile, left, `Forbidden assignment target: ${key}`);
    }

    if (forbiddenAuthorityProperties.has(key)) {
      report(violations, sourceFile, left, `Forbidden authority assignment target: ${key}`);
    }
  }
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

      if (typescript.isIdentifier(expr) && forbiddenCallIdentifiers.has(expr.text)) {
        report(violations, sourceFile, node, `Forbidden call: ${expr.text}(...)`);
      }

      if (typescript.isPropertyAccessExpression(expr)) {
        const property = expr.name.text;

        if (forbiddenCallIdentifiers.has(property)) {
          report(violations, sourceFile, node, `Forbidden method call: ${property}(...)`);
        }
      }

      if (
        typescript.isPropertyAccessExpression(expr)
        && typescript.isCallExpression(expr.expression)
        && typescript.isPropertyAccessExpression(expr.expression.expression)
        && typescript.isIdentifier(expr.expression.expression.expression)
        && expr.expression.expression.expression.text === 'Promise'
        && expr.expression.expression.name.text === 'resolve'
        && expr.name.text === 'then'
      ) {
        report(violations, sourceFile, node, 'Forbidden Promise microtask scheduling: Promise.resolve(...).then(...)');
      }
    }

    if (typescript.isNewExpression(node) && typescript.isIdentifier(node.expression)) {
      if (forbiddenConstructorIdentifiers.has(node.expression.text)) {
        report(violations, sourceFile, node, `Forbidden constructor: new ${node.expression.text}(...)`);
      }
    }

    if (typescript.isIdentifier(node) && forbiddenIdentifierReferences.has(node.text)) {
      report(violations, sourceFile, node, `Forbidden reference: ${node.text}`);
    }

    if (typescript.isImportDeclaration(node) && typescript.isStringLiteral(node.moduleSpecifier)) {
      const moduleName = node.moduleSpecifier.text;

      if (forbiddenImportModules.has(moduleName)) {
        report(violations, sourceFile, node, `Forbidden import module: ${moduleName}`);
      }
    }

    checkAuthorityAssignmentTarget(violations, sourceFile, node);

    if (typescript.isPropertyAssignment(node)) {
      const key = propertyNameText(node.name);

      if (forbiddenJsxHandlers.has(key)) {
        report(violations, sourceFile, node.name, `Forbidden object property: ${key}`);
      }

      checkAuthorityPropertyAssignment(violations, sourceFile, node);
    }

    if (typescript.isShorthandPropertyAssignment(node)) {
      const key = identifierText(node.name);

      if (forbiddenAuthorityProperties.has(key)) {
        report(violations, sourceFile, node.name, `Forbidden authority shorthand property: ${key}`);
      }
    }

    if (typescript.isJsxAttribute(node)) {
      const attr = node.name.text;

      if (forbiddenJsxHandlers.has(attr)) {
        report(violations, sourceFile, node.name, `Forbidden JSX event-handler attribute: ${attr}`);
      }

      const owner = jsxOwnerElement(node);

      if (attr === 'href' && owner) {
        const tag = owner.tagName.getText(sourceFile);
        if (tag === 'a') {
          report(violations, sourceFile, node.name, 'Forbidden JSX anchor href on <a>');
        }
      }

      if (attr === 'type') {
        const initializer = node.initializer;

        if (initializer && typescript.isStringLiteral(initializer) && initializer.text === 'submit' && owner) {
          const tag = owner.tagName.getText(sourceFile);

          if (tag === 'input') {
            report(violations, sourceFile, node.name, 'Forbidden JSX submit input element: <input type="submit">');
          }
        }
      }
    }

    if (typescript.isJsxOpeningElement(node) || typescript.isJsxSelfClosingElement(node)) {
      const tag = node.tagName.getText(sourceFile);

      if (tag === 'form' || tag === 'button') {
        report(violations, sourceFile, node.tagName, `Forbidden JSX intrinsic element: <${tag}>`);
      }
    }

    typescript.forEachChild(node, visit);
  }

  visit(sourceFile);
}

if (violations.length > 0) {
  for (const violation of violations) {
    console.error(violation);
  }

  console.error(`UI boundary lint failed with ${violations.length} violation(s).`);
  process.exit(1);
}

console.log(`UI boundary lint passed (${files.length} files checked).`);
