import fs from 'node:fs';
import path from 'node:path';

const dist = path.resolve('dist');
if (fs.existsSync(dist)) {
  fs.rmSync(dist, { recursive: true, force: true });
}
fs.mkdirSync(dist, { recursive: true });

const copyList = ['index.html', 'analytics.html', 'rust-preview', 'public', 'src', 'web'];
for (const item of copyList) {
  if (fs.existsSync(item)) {
    fs.cpSync(item, path.join(dist, item), { recursive: true });
  }
}

console.log('Vercel static distribution successfully prepared in dist/');
