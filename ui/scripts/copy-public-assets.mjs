import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const source = fileURLToPath(new URL('../public/', import.meta.url));
const destination = path.resolve(process.argv[2] ?? 'dist');
const inventory = JSON.parse(fs.readFileSync(path.join(source, 'third-party-assets.json'), 'utf8'));

for (const relative of [...inventory.assets.map(asset => asset.file), 'third-party-assets.json']) copyAsset(relative);

function copyAsset(relative) {
  if (path.isAbsolute(relative) || relative.split(/[\\/]/).includes('..')) throw new Error('Public assets must stay inside their source/destination roots');
  const target = path.join(destination, relative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(source, relative), target);
}
