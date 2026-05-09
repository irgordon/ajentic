#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const DEFAULT_ROOT = process.cwd();

const API_MOD_PATH = 'core/src/api/mod.rs';
const PERSISTENCE_PATH = 'core/src/api/persistence.rs';

const API_MOD_FORBIDDEN = [
  'pub enum',
  'pub struct',
  'pub trait',
  'pub fn',
  'impl',
  '#[cfg(test)]',
  'fn code(',
];

const ONLY_IN_PERSISTENCE = [
  'execute_local_persistence_plan',
  'verify_persisted_record_paths',
  'std::fs',
  'fs::write',
  'File::',
  'OpenOptions',
  'read_to_string',
  'read_dir',
  'create_dir',
  'create_dir_all',
  'remove_file',
  'remove_dir',
  'remove_dir_all',
  'copy',
  'write(',
  'write!',
  'writeln!',
  'rename',
  'set_permissions',
  'sync_all',
  'flush',
];

const GLOBAL_FORBIDDEN_REGEX = [
  /\bstd::net\b/g,
  /\bTcpStream\b/g,
  /\bUdpSocket\b/g,
  /\breqwest\b/g,
  /\bureq\b/g,
  /\bhyper\b/g,
  /\btokio::/g,
  /\basync\s+fn\b/g,
  /\.await\b/g,
  /\basync-std\b/g,
  /\bfutures::/g,
  /\bCommand::/g,
  /\bstd::process::/g,
  /\bthread::/g,
  /\bspawn\s*\(/g,
];

export function collectRustFiles(rootDir) {
  const files = [];

  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const filePath = path.join(dir, entry.name);

      if (entry.isDirectory()) {
        if (['target', '.git', 'node_modules'].includes(entry.name)) {
          continue;
        }

        walk(filePath);
        continue;
      }

      if (entry.isFile() && filePath.endsWith('.rs')) {
        files.push(filePath);
      }
    }
  };

  walk(rootDir);
  return files.sort();
}

function rawStringEnd(input, startIndex) {
  if (input[startIndex] !== 'r') {
    return null;
  }

  let i = startIndex + 1;
  let hashes = 0;

  while (input[i] === '#') {
    hashes += 1;
    i += 1;
  }

  if (input[i] !== '"') {
    return null;
  }

  const terminator = `"${'#'.repeat(hashes)}`;
  const contentStart = i + 1;
  const end = input.indexOf(terminator, contentStart);

  if (end === -1) {
    return input.length;
  }

  return end + terminator.length;
}

function stripStringsAndComments(input) {
  let output = '';
  let i = 0;
  let state = 'code';

  while (i < input.length) {
    const ch = input[i];
    const next = input[i + 1] || '';

    if (state === 'code') {
      const rawEnd = rawStringEnd(input, i);
      if (rawEnd !== null) {
        while (i < rawEnd) {
          output += input[i] === '\n' ? '\n' : ' ';
          i += 1;
        }
        continue;
      }

      if (ch === '/' && next === '/') {
        state = 'line_comment';
        output += '  ';
        i += 2;
        continue;
      }

      if (ch === '/' && next === '*') {
        state = 'block_comment';
        output += '  ';
        i += 2;
        continue;
      }

      if (ch === '"') {
        state = 'string';
        output += ' ';
        i += 1;
        continue;
      }

      if (ch === "'") {
        state = 'char';
        output += ' ';
        i += 1;
        continue;
      }

      output += ch;
      i += 1;
      continue;
    }

    if (state === 'line_comment') {
      if (ch === '\n') {
        state = 'code';
        output += '\n';
      } else {
        output += ' ';
      }

      i += 1;
      continue;
    }

    if (state === 'block_comment') {
      if (ch === '*' && next === '/') {
        state = 'code';
        output += '  ';
        i += 2;
      } else {
        output += ch === '\n' ? '\n' : ' ';
        i += 1;
      }

      continue;
    }

    if (state === 'string') {
      if (ch === '\\') {
        output += '  ';
        i += 2;
        continue;
      }

      if (ch === '"') {
        state = 'code';
        output += ' ';
        i += 1;
        continue;
      }

      output += ch === '\n' ? '\n' : ' ';
      i += 1;
      continue;
    }

    if (state === 'char') {
      if (ch === '\\') {
        output += '  ';
        i += 2;
        continue;
      }

      if (ch === "'") {
        state = 'code';
        output += ' ';
        i += 1;
        continue;
      }

      output += ch === '\n' ? '\n' : ' ';
      i += 1;
      continue;
    }
  }

  return output;
}

function matchesToken(text, token) {
  const matches = [];
  const lines = text.split(/\r?\n/);

  for (let lineIndex = 0; lineIndex < lines.length; lineIndex += 1) {
    let start = 0;

    while (true) {
      const index = lines[lineIndex].indexOf(token, start);
      if (index === -1) {
        break;
      }

      matches.push({
        line: lineIndex + 1,
        column: index + 1,
        snippet: token,
      });

      start = index + token.length;
    }
  }

  return matches;
}

function matchesRegex(text, regex) {
  const matches = [];
  const lines = text.split(/\r?\n/);

  for (let lineIndex = 0; lineIndex < lines.length; lineIndex += 1) {
    regex.lastIndex = 0;
    let match;

    while ((match = regex.exec(lines[lineIndex])) !== null) {
      matches.push({
        line: lineIndex + 1,
        column: match.index + 1,
        snippet: match[0],
      });

      if (match.index === regex.lastIndex) {
        regex.lastIndex += 1;
      }
    }
  }

  return matches;
}

function pushIssue(issues, level, relPath, match, message) {
  issues.push({
    level,
    relPath,
    line: match.line,
    column: match.column,
    message,
  });
}

export function lintRustBoundaries(rootDirArg) {
  const root = path.resolve(rootDirArg || DEFAULT_ROOT);
  const issues = [];

  for (const absolutePath of collectRustFiles(root)) {
    const relPath = path.relative(root, absolutePath).replace(/\\/g, '/');
    const scanned = stripStringsAndComments(fs.readFileSync(absolutePath, 'utf8'));

    if (relPath === API_MOD_PATH) {
      for (const token of API_MOD_FORBIDDEN) {
        for (const match of matchesToken(scanned, token)) {
          pushIssue(
            issues,
            'error',
            relPath,
            match,
            `api/mod.rs must remain module/re-export only; forbidden token '${token}'`,
          );
        }
      }
    }

    for (const token of ONLY_IN_PERSISTENCE) {
      if (relPath === PERSISTENCE_PATH) {
        continue;
      }

      for (const match of matchesToken(scanned, token)) {
        pushIssue(
          issues,
          'error',
          relPath,
          match,
          `'${token}' is only allowed in ${PERSISTENCE_PATH}`,
        );
      }
    }

    for (const regex of GLOBAL_FORBIDDEN_REGEX) {
      for (const match of matchesRegex(scanned, regex)) {
        pushIssue(
          issues,
          'error',
          relPath,
          match,
          `forbidden Rust boundary token '${match.snippet}'`,
        );
      }
    }
  }

  return issues;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const root = process.argv[2] ? path.resolve(process.argv[2]) : DEFAULT_ROOT;
  const fileCount = collectRustFiles(root).length;
  const issues = lintRustBoundaries(root);
  const errors = issues.filter((issue) => issue.level === 'error');
  const warnings = issues.filter((issue) => issue.level === 'warning');

  for (const issue of issues) {
    const prefix = issue.level === 'warning' ? 'warning: ' : '';
    const output = `${issue.relPath}:${issue.line}:${issue.column}: ${prefix}${issue.message}`;

    if (issue.level === 'warning') {
      console.warn(output);
    } else {
      console.error(output);
    }
  }

  if (errors.length) {
    console.error(
      `Rust boundary lint failed with ${errors.length} error(s) and ${warnings.length} warning(s).`,
    );
    process.exit(1);
  }

  console.log(`Rust boundary lint passed (${fileCount} files checked, ${warnings.length} warning(s)).`);
}
