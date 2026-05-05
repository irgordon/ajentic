#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const DEFAULT_ROOT = process.cwd();
const API_MOD_PATH = 'core/src/api/mod.rs';
const PERSISTENCE_PATH = 'core/src/api/persistence.rs';

const API_MOD_FORBIDDEN = ['pub enum', 'pub struct', 'pub trait', 'pub fn', 'impl', '#[cfg(test)]', 'fn code('];
const ONLY_IN_PERSISTENCE = ['execute_local_persistence_plan', 'verify_persisted_record_paths', 'std::fs', 'File::', 'read_to_string', 'read_dir', 'write(', 'write!', 'writeln!', 'rename', 'sync_all', 'flush'];

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

const EXECUTION_MOD_FORBIDDEN_REGEX = [
  /\bstd::fs\b/g,
  /\bstd::net\b/g,
  /\breqwest\b/g,
  /\bureq\b/g,
  /\bhyper\b/g,
  /\btokio::/g,
  /\basync\s+fn\b/g,
  /\.await\b/g,
  /\bCommand::/g,
  /\bstd::process::/g,
  /\bthread::/g,
  /\bspawn\s*\(/g,
];

export function collectRustFiles(rootDir) { const files=[]; const walk=(d)=>{ for (const e of fs.readdirSync(d,{withFileTypes:true})) { const f=path.join(d,e.name); if(e.isDirectory()){ if(['target','.git','node_modules'].includes(e.name)) continue; walk(f);} else if(e.isFile()&&f.endsWith('.rs')) files.push(f);} }; walk(rootDir); return files.sort(); }

function stripStringsAndComments(input){let out='',i=0,state='code';while(i<input.length){const ch=input[i],n=input[i+1]||'';if(state==='code'){if(ch==='/'&&n==='/'){state='line';out+='  ';i+=2;continue;}if(ch==='/'&&n==='*'){state='block';out+='  ';i+=2;continue;}if(ch==='"'){state='string';out+=' ';i++;continue;}out+=ch;i++;continue;}if(state==='line'){if(ch==='\n'){state='code';out+='\n';}else out+=' ';i++;continue;}if(state==='block'){if(ch==='*'&&n=== '/'){state='code';out+='  ';i+=2;}else{out+=ch==='\n'?'\n':' ';i++;}continue;}if(state==='string'){if(ch==='\\'){out+='  ';i+=2;continue;}if(ch==='"'){state='code';out+=' ';i++;continue;}out+=ch==='\n'?'\n':' ';i++;}}return out;}

function matchesToken(text, token){const out=[]; const lines=text.split(/\r?\n/); for(let i=0;i<lines.length;i++){let s=0;while(true){const idx=lines[i].indexOf(token,s);if(idx===-1) break;out.push({line:i+1,column:idx+1,snippet:token});s=idx+token.length;}} return out;}
function matchesRegex(text, regex){const out=[]; const lines=text.split(/\r?\n/); for(let i=0;i<lines.length;i++){regex.lastIndex=0; let m; while((m=regex.exec(lines[i]))!==null){out.push({line:i+1,column:m.index+1,snippet:m[0]}); if(m.index===regex.lastIndex) regex.lastIndex++;}} return out;}

export function lintRustBoundaries(rootDirArg){const root=path.resolve(rootDirArg||DEFAULT_ROOT);const issues=[];for(const abs of collectRustFiles(root)){const rel=path.relative(root,abs).replace(/\\/g,'/');const scanned=stripStringsAndComments(fs.readFileSync(abs,'utf8')); const push=(level,m,msg)=>issues.push({level,relPath:rel,line:m.line,column:m.column,message:msg});
if(rel===API_MOD_PATH){for(const t of API_MOD_FORBIDDEN) for(const m of matchesToken(scanned,t)) push('error',m,`api/mod.rs must remain module/re-export only; forbidden token '${t}'`);} 
for(const t of ONLY_IN_PERSISTENCE){ if(rel!==PERSISTENCE_PATH) for(const m of matchesToken(scanned,t)) push('error',m,`'${t}' is only allowed in ${PERSISTENCE_PATH}`);} 
for(const r of GLOBAL_FORBIDDEN_REGEX) { if(rel===PERSISTENCE_PATH && r.source.includes('std::process')) continue; for(const m of matchesRegex(scanned,r)) push('error',m,`forbidden Rust boundary token '${m.snippet}'`);} }
return issues;}

if (import.meta.url === `file://${process.argv[1]}`) {
  const root = process.argv[2] ? path.resolve(process.argv[2]) : DEFAULT_ROOT;
  const files = collectRustFiles(root).length;
  const issues = lintRustBoundaries(root);
  const errors = issues.filter((i) => i.level === 'error');
  const warnings = issues.filter((i) => i.level === 'warning');
  for (const i of issues) console[i.level === 'warning' ? 'warn' : 'error'](`${i.relPath}:${i.line}:${i.column}: ${i.level === 'warning' ? 'warning: ' : ''}${i.message}`);
  if (errors.length) { console.error(`Rust boundary lint failed with ${errors.length} error(s) and ${warnings.length} warning(s).`); process.exit(1);} 
  console.log(`Rust boundary lint passed (${files} files checked, ${warnings.length} warning(s)).`);
}
