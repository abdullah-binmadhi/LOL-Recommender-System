const assert = require('assert');
const FeatureExtractor = require('./FeatureExtractor.js');

console.log("Running FeatureExtractor Tests...");

// Mock attributes
const assassinAttrs = { damage: 3, toughness: 1, control: 1, mobility: 3, utility: 1, difficulty: 2 };
const tankAttrs = { damage: 1, toughness: 3, control: 3, mobility: 1, utility: 2, difficulty: 1 };

// Defined Formulas (Expected Behavior)
// Burst = (damage * 2 + mobility) / 3 * 10 (Scaled to 0-100 approx, max damage=3, mobility=3 -> (6+3)/3 = 3 -> *33.3?
// Let's stick to the 0-3 scale from JSON or normalize to 0-100?
// The JSON values are 0-3.
// Let's normalize output to 0-100.
// Max raw score for Burst (3,3) = (3*2 + 3)/3 = 3.
// So result * 33.33.
// Let's use simpler logic:
// Burst = (damage * 0.7 + mobility * 0.3) / 3 * 100
// Sustain = (toughness * 0.7 + utility * 0.3) / 3 * 100
// CC = (control / 3) * 100

function runTests() {
    let passed = 0;
    let failed = 0;

    try {
        console.log("Test 1: Calculate Burst Score");
        // Assassin: (3 * 0.7 + 3 * 0.3) = 2.1 + 0.9 = 3.0. Normalized: 100.
        const burst = FeatureExtractor.calculateBurst(assassinAttrs);
        assert.strictEqual(Math.round(burst), 100, `Expected assassin burst to be 100, got ${burst}`);
        console.log("  PASS");
        passed++;
    } catch (e) {
        console.error("  FAIL: " + e.message);
        failed++;
    }

    try {
        console.log("Test 2: Calculate Sustain Score");
        // Tank: (3 * 0.7 + 2 * 0.3) = 2.1 + 0.6 = 2.7. Normalized: (2.7 / 3) * 100 = 90.
        const sustain = FeatureExtractor.calculateSustain(tankAttrs);
        assert.strictEqual(Math.round(sustain), 90, `Expected tank sustain to be 90, got ${sustain}`);
        console.log("  PASS");
        passed++;
    } catch (e) {
        console.error("  FAIL: " + e.message);
        failed++;
    }

    try {
        console.log("Test 3: Calculate CC Score");
        // Tank: Control 3. Normalized: 100.
        const cc = FeatureExtractor.calculateCC(tankAttrs);
        assert.strictEqual(Math.round(cc), 100, `Expected tank CC to be 100, got ${cc}`);
        console.log("  PASS");
        passed++;
    } catch (e) {
        console.error("  FAIL: " + e.message);
        failed++;
    }

    console.log(`\nResults: ${passed} Passed, ${failed} Failed`);
    if (failed > 0) process.exit(1);
}

runTests();
