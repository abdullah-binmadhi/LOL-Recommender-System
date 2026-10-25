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

// Ensure the root index.html serves the full WASM application directly without redirect latency
const rustIndex = path.join(dist, 'rust-preview', 'index.html');
if (fs.existsSync(rustIndex)) {
  fs.copyFileSync(rustIndex, path.join(dist, 'index.html'));
}

// Copy supabase_bridge.js to both root dist and rust-preview to ensure scripts load without 404
const bridgeSrc = path.resolve('web/supabase_bridge.js');
if (fs.existsSync(bridgeSrc)) {
  fs.copyFileSync(bridgeSrc, path.join(dist, 'supabase_bridge.js'));
  fs.copyFileSync(bridgeSrc, path.join(dist, 'rust-preview', 'supabase_bridge.js'));
  const previewDir = path.resolve('rust-preview');
  if (fs.existsSync(previewDir)) {
    fs.copyFileSync(bridgeSrc, path.join(previewDir, 'supabase_bridge.js'));
  }
}

console.log('Vercel static distribution successfully prepared in dist/');
