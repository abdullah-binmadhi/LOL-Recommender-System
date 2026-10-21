import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { resolve } from 'node:path';
import test from 'node:test';
import {
  canonical, compareArtifacts, comparisonText, createReference, extractSection,
  fixtureDirectory, normalize, root, selectedSections
} from './reference.mjs';

const artifacts = createReference();
const manifest = artifacts.get('manifest.json');
const catalog = artifacts.get('catalog.json');
const questions = JSON.parse(readFileSync(resolve(root, 'src/data/questions.json'), 'utf8')).questions;
const profiles = [...artifacts.entries()].filter(([name]) => name.startsWith('profile-')).map(([, value]) => value);
const profile = id => profiles.find(value => value.id === id);
const sha256 = text => createHash('sha256').update(text).digest('hex');
const bounded = (value, min = 0, max = 100) => {
  assert.ok(Number.isFinite(value), 'Expected a finite number');
  assert.ok(value >= min && value <= max, `Expected value within ${min}..${max}`);
};

test('content boundaries fail closed; unrelated page content is not selected', () => {
  assert.equal(extractSection('before START code END after', 'START', 'END'), 'START code');
  assert.throws(() => extractSection('START START END', 'START', 'END'), /ambiguous/);
  assert.throws(() => extractSection('START', 'START', 'END'), /Missing/);
  assert.throws(() => extractSection('END START', 'START', 'END'), /Reversed/);
  const html = readFileSync(resolve(root, 'src/index.html'), 'utf8');
  const helper = readFileSync(resolve(root, 'src/analytics/FeatureExtractor.js'), 'utf8');
  const selected = selectedSections(html, helper);
  assert.deepEqual(selectedSections(html + '\n<!-- unrelated synthetic page content -->', helper), selected);
  for (const item of selected) {
    assert.ok(!/GEMINI_API_KEY|AIza[\w-]{20,}|<script|localStorage|fetch\s*\(|document\./.test(item.code), 'Selected source must exclude credentials and browser code');
    assert.equal(manifest.sections.find(section => section.name === item.name).sha256, sha256(item.code));
    assert.equal(item.path.startsWith('migration_files/'), false);
  }
});

test('all effective questions and realistic answers come from current question data', () => {
  assert.equal(questions.length, 12);
  assert.equal(manifest.effectiveQuestionCount, questions.length);
  assert.equal(artifacts.get('questions.json').length, questions.length);
  for (const capture of profiles.filter(value => value.kind === 'questionnaire-12')) {
    assert.equal(Object.keys(capture.answers).length, questions.length);
    for (const question of questions) assert.ok(question.options.includes(capture.answers[question.id]));
  }
  for (const input of manifest.inputs) {
    assert.equal(input.sha256, sha256(readFileSync(resolve(root, input.path), 'utf8')));
  }
});

test('12-question mapping mismatch is captured, not repaired', () => {
  const mage = profile('mage-mid');
  assert.equal(mage.features.pressure_response, mage.answers[6]); // Resource answer, not Q8.
  assert.equal(mage.features.aesthetic_preference, mage.answers[7]);
  assert.equal(mage.features.team_contribution, mage.answers[8]);
  assert.equal(mage.features.character_identity, mage.answers[9]);
  assert.equal(mage.features.problem_solving, mage.answers[10]);
  assert.equal(mage.features.difficulty, 5);
  assert.equal(mage.features.damage, 8);
  assert.ok(Math.abs(mage.features.burst - (8 * 0.7 + 7 * 0.3) / 3.33 / 3 * 100) < 1e-12);
  assert.equal(profile('marksman-bot').features.position, 'Bot');
  assert.equal(profile('no-preference-adc').features.position, 'ADC');
  assert.equal(profile('no-preference-adc').features.range, 'no preference');
  assert.equal(profile('empty-defaults').features.range, 'no_preference');
  assert.equal(profile('unknown-values').features.difficulty, 5);
  assert.equal(profile('unknown-values').features.playstyle, 'balanced');
  assert.equal(profile('unknown-values').features.role, 'Synthetic unknown option');
  assert.equal(profile('tank-top').features.difficulty, 2);
  assert.equal(profile('marksman-bot').features.difficulty, 7.5);
  assert.equal(profile('support-team').features.difficulty, 9.5);
  assert.equal(profile('empty-defaults').features.pressure_response, 'Stay calm and strategic');
  const changed = structuredClone(mage);
  changed.answers[11] = 'Synthetic alternate identity';
  changed.answers[12] = 'Synthetic alternate strategy';
  const altered = createReference({ profiles: [changed] }).get('profile-mage-mid.json');
  assert.equal(canonical(altered.features), canonical(mage.features));
  assert.equal(canonical(altered.scores), canonical(mage.scores));
});

test('catalog preserves effective JS key order, overwrites and missing derived data', () => {
  assert.equal(manifest.championCount, catalog.length);
  assert.equal(new Set(catalog.map(champion => champion.name)).size, catalog.length);
  assert.deepEqual(catalog.slice(0, 5).map(champion => champion.name), ['Jinx', 'Ashe', 'Yasuo', 'Lux', 'Garen']);
  assert.ok(catalog.length > 150);
  assert.equal(catalog.some(champion => Object.hasOwn(champion.data, 'derived_stats')), false);
  assert.equal(catalog.some(champion => Object.hasOwn(champion.data, 'herotype')), false);
  const mage = profile('mage-mid');
  const champion = catalog.find(item => item.name === 'Jinx').data;
  const features = mage.features;
  const ranges = ['difficulty', 'damage', 'toughness', 'mobility', 'control', 'utility'];
  const weights = [1, 1.2, 1.2, 1, 1, 1];
  let squares = ranges.reduce((sum, key, index) => sum + weights[index] * ((features[key] - champion[key]) / 9) ** 2, 0);
  squares += 0.8 * (features.burst / 100) ** 2 + 0.8 * (features.sustain / 100) ** 2 + 0.8 * (features.cc / 100) ** 2;
  squares += 2.5 + 1.5; // Jinx mismatches Mage/Mid; missing derived stats contribute zero.
  assert.ok(Math.abs(mage.scores.knn.Jinx.rawScore - Math.sqrt(squares)) < 1e-12);
});

test('actual generated forest exports every feature subset and bootstrap sample', () => {
  const forest = artifacts.get('forest.json');
  assert.equal(forest.length, 10);
  const names = new Set(catalog.map(champion => champion.name));
  const features = new Set(['role', 'difficulty', 'damage', 'toughness', 'mobility', 'control', 'utility']);
  for (const [index, tree] of forest.entries()) {
    assert.equal(tree.id, index);
    assert.equal(tree.featureIndices.length, 5);
    assert.equal(new Set(tree.featureIndices).size, 5);
    for (const feature of tree.featureIndices) assert.ok(features.has(feature));
    assert.equal(tree.bootstrapChampionNames.length, catalog.length);
    for (const name of tree.bootstrapChampionNames) assert.ok(names.has(name));
  }
  assert.ok(manifest.randomness.totalRandomCalls > catalog.length * forest.length);
  assert.ok(manifest.randomness.bootstrapRandomCalls > 0);
});

test('every champion has bounded scores, details, aggregate values and valid top10', () => {
  for (const capture of profiles) {
    for (const scores of Object.values(capture.scores)) {
      assert.deepEqual(Object.keys(scores), catalog.map(champion => champion.name));
      for (const result of Object.values(scores)) {
        bounded(result.score);
        assert.ok(Number.isFinite(result.rawScore));
        assert.ok(Array.isArray(result.details.contributingFactors));
        assert.ok(Array.isArray(result.details.matchedCriteria));
        assert.ok(Array.isArray(result.details.penalties));
      }
    }
    assert.equal(capture.aggregated.length, catalog.length);
    assert.equal(capture.top10.length, 10);
    assert.equal(new Set(capture.top10.map(item => item.championName)).size, 10);
    const roleCounts = {};
    for (const item of capture.top10) {
      assert.ok(capture.aggregated.some(value => canonical(item) === canonical(value)));
      const role = catalog.find(champion => champion.name === item.championName).data.role;
      roleCounts[role] = (roleCounts[role] ?? 0) + 1;
    }
    assert.ok(Object.values(roleCounts).every(count => count <= 3));
    for (const item of capture.aggregated) {
      bounded(item.average);
      bounded(item.weighted);
      assert.equal(item.average, item.weighted); // Production calls weighted mean "average".
      assert.ok(Math.abs(item.average - (item.randomForest * 0.4 + item.decisionTree * 0.3 + item.knn * 0.3)) < 1e-12);
    }
    const { relevantChampions, ...metrics } = capture.evaluation;
    assert.ok(relevantChampions.length >= 10 && relevantChampions.length <= 25);
    for (const value of Object.values(metrics)) bounded(value, 0, 1);
  }
});

test('fresh VM runs are deterministic, capability-free, and fixtures remain practical', () => {
  const second = createReference();
  for (const [name, value] of artifacts) assert.equal(canonical(value), canonical(second.get(name)));
  for (const capability of Object.values(manifest.capabilities)) assert.equal(capability, 'undefined');
  assert.throws(() => normalize({ score: NaN }), /Non-finite/);
  assert.throws(() => normalize(Infinity), /Non-finite/);
  assert.throws(() => normalize(undefined), /undefined/);
  assert.equal(canonical({ z: -0, a: [2, 1] }), '{"a":[2,1],"z":0}\n');
  let bytes = 0;
  for (const [name, value] of artifacts) {
    const text = canonical(value);
    bytes += Buffer.byteLength(text);
    assert.ok(!/AIza[\w-]{20,}|GEMINI_API_KEY|"(?:sessionId|email|timestamp)"\s*:|<script/.test(text), 'Fixture must exclude credentials, PII and time fields');
    assert.ok(Buffer.byteLength(text) < 512 * 1024, `${name} is too large`);
  }
  assert.ok(bytes < 3 * 1024 * 1024);
});

test('check mode matches committed fixtures and detects value, source-hash and file-set drift', () => {
  assert.deepEqual(compareArtifacts(artifacts), []);
  const changed = new Map(artifacts);
  const altered = structuredClone(profile('mage-mid'));
  altered.scores.knn.Jinx.score += 0.01;
  changed.set('profile-mage-mid.json', altered);
  assert.deepEqual(compareArtifacts(changed), ['Reference drift: profile-mage-mid.json']);
  const changedManifest = structuredClone(manifest);
  changedManifest.sections[0].sha256 = 'synthetic drift';
  changed.set('manifest.json', changedManifest);
  assert.ok(compareArtifacts(changed).includes('Reference drift: manifest.json'));
  changed.delete('catalog.json');
  assert.ok(compareArtifacts(changed).includes('Unexpected fixture: catalog.json'));
  changed.set('missing-synthetic.json', {});
  assert.ok(compareArtifacts(changed).includes('Missing fixture: missing-synthetic.json'));
  const runtimeOnly = structuredClone(manifest);
  runtimeOnly.randomness.runtime = { node: 'different-runtime', v8: 'different-runtime' };
  assert.equal(comparisonText('manifest.json', runtimeOnly), comparisonText('manifest.json', manifest));
  const files = readdirSync(fixtureDirectory).filter(name => name.endsWith('.json'));
  assert.equal(files.length, artifacts.size);
  assert.ok(files.reduce((sum, name) => sum + statSync(resolve(fixtureDirectory, name)).size, 0) < 3 * 1024 * 1024);
});
