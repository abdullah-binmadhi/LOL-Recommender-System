import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const wasmPath = resolve(root, 'target/wasm32-unknown-unknown/release/recommender_core.wasm');
const fixtures = resolve(root, 'tests/fixtures/migration');
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {});
const wasm = instance.exports;
const decoder = new TextDecoder();

function recommend(answers) {
    const input = new TextEncoder().encode(JSON.stringify(answers));
    const pointer = wasm.scoring_alloc(input.byteLength);
    new Uint8Array(wasm.memory.buffer, pointer, input.byteLength).set(input);
    try {
        const status = wasm.scoring_recommend(pointer, input.byteLength);
        if (status !== 0) {
            const message = decoder.decode(new Uint8Array(
                wasm.memory.buffer, wasm.scoring_error_ptr(), wasm.scoring_error_len()
            ).slice());
            return { status, message };
        }
        const bytes = new Uint8Array(
            wasm.memory.buffer, wasm.scoring_result_ptr(), wasm.scoring_result_len()
        ).slice();
        return { status, result: JSON.parse(decoder.decode(bytes)) };
    } finally {
        wasm.scoring_dealloc(pointer, input.byteLength);
    }
}

function assertNumericParity(actual, expected, path = '$') {
    if (typeof actual === 'number' && typeof expected === 'number') {
        assert.ok(Math.abs(actual - expected) <= 1e-9, `${path}: ${actual} != ${expected}`);
    } else if (Array.isArray(actual) && Array.isArray(expected)) {
        assert.equal(actual.length, expected.length, `${path}: length`);
        actual.forEach((value, index) => assertNumericParity(value, expected[index], `${path}[${index}]`));
    } else if (actual && expected && typeof actual === 'object' && typeof expected === 'object') {
        assert.deepEqual(Object.keys(actual).sort(), Object.keys(expected).sort(), `${path}: keys`);
        for (const key of Object.keys(expected)) assertNumericParity(actual[key], expected[key], `${path}.${key}`);
    } else {
        assert.deepEqual(actual, expected, path);
    }
}

test('browser-target WebAssembly matches all profile reference outputs', () => {
    const profiles = readdirSync(fixtures).filter(name => /^profile-.*\.json$/.test(name));
    assert.equal(profiles.length, 8);
    for (const name of profiles) {
        const profile = JSON.parse(readFileSync(resolve(fixtures, name), 'utf8'));
        const { status, result, message } = recommend(profile.answers);
        assert.equal(status, 0, `${name}: ${message}`);
        for (const key of ['features', 'scores', 'aggregated', 'top10', 'evaluation']) {
            assertNumericParity(result[key], profile[key], `${profile.id}.${key}`);
        }
    }
});

test('questionnaire scoring and evaluation are routed through Rust/Wasm', () => {
    const html = readFileSync(resolve(root, 'src/index.html'), 'utf8');
    const start = html.indexOf('async function runAllAlgorithms()');
    const end = html.indexOf('function mapDifficulty(', start);
    assert.ok(start >= 0 && end > start, 'Rust scoring entry point is present');
    const scoringPath = html.slice(start, end);
    assert.match(scoringPath, /await scoreAnswers\(answers\)/);
    assert.doesNotMatch(scoringPath, /getCachedScores\(|ScoreAggregator\.|EvaluationMetrics\./);
    assert.doesNotMatch(html, /const algorithms = \{/);
});

test('browser adapter loads the Wasm module and returns a full recommendation result', async () => {
    const originalFetch = globalThis.fetch;
    globalThis.fetch = async () => new Response(readFileSync(wasmPath), {
        status: 200,
        headers: { 'Content-Type': 'application/wasm' }
    });
    try {
        const { scoreAnswers } = await import('../../src/wasm/scoring.mjs');
        const profile = JSON.parse(readFileSync(resolve(fixtures, 'profile-mage-mid.json'), 'utf8'));
        const result = await scoreAnswers(profile.answers);
        assertNumericParity(result.features, profile.features, 'adapter.features');
        assertNumericParity(result.top10, profile.top10, 'adapter.top10');
        assertNumericParity(result.evaluation, {
            relevantChampions: profile.evaluation.relevantChampions,
            precisionAt1: profile.evaluation.precisionAt1,
            precisionAt3: profile.evaluation.precisionAt3,
            precisionAt10: profile.evaluation.precisionAt10,
            recallAt10: profile.evaluation.recallAt10,
            f1At10: profile.evaluation.f1At10,
            mrr: profile.evaluation.mrr
        }, 'adapter.evaluation');
    } finally {
        globalThis.fetch = originalFetch;
    }
});

test('WebAssembly reports malformed, oversized, and non-object quiz input without trapping', () => {
    const malformed = new TextEncoder().encode('{');
    let pointer = wasm.scoring_alloc(malformed.length);
    new Uint8Array(wasm.memory.buffer, pointer, malformed.length).set(malformed);
    try {
        assert.equal(wasm.scoring_recommend(pointer, malformed.length), 1);
        assert.match(decoder.decode(new Uint8Array(wasm.memory.buffer, wasm.scoring_error_ptr(), wasm.scoring_error_len()).slice()), /valid JSON/);
    } finally {
        wasm.scoring_dealloc(pointer, malformed.length);
    }

    assert.equal(wasm.scoring_recommend(0, 1024 * 1024 + 1), 1);
    assert.equal(wasm.scoring_recommend(0, 2), 1);
    pointer = wasm.scoring_alloc(2);
    new Uint8Array(wasm.memory.buffer, pointer, 2).set(new TextEncoder().encode('[]'));
    try {
        assert.equal(wasm.scoring_recommend(pointer, 2), 2);
        assert.match(decoder.decode(new Uint8Array(wasm.memory.buffer, wasm.scoring_error_ptr(), wasm.scoring_error_len()).slice()), /could not be scored/);
    } finally {
        wasm.scoring_dealloc(pointer, 2);
    }
});
