import { copyFileSync, existsSync, mkdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const built = resolve(root, 'target/wasm32-unknown-unknown/release/recommender_core.wasm');
const outputDirectory = resolve(root, 'public/wasm');
const output = resolve(outputDirectory, 'lol_scoring_core.wasm');

const build = spawnSync('cargo', [
    'build', '--release', '--locked', '--target', 'wasm32-unknown-unknown',
    '--package', 'recommender-core'
], { cwd: root, stdio: 'inherit' });
if (build.error) throw build.error;
if (build.status !== 0) process.exit(build.status ?? 1);
if (!existsSync(built)) throw new Error('Cargo completed without producing the expected Wasm module.');

mkdirSync(outputDirectory, { recursive: true });
copyFileSync(built, output);
console.log(`Built browser scoring module: ${output.slice(root.length + 1)}`);
