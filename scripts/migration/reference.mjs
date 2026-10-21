import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';
import { syntheticProfiles } from './profiles.mjs';

export const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const fixtureDirectory = resolve(root, 'tests/fixtures/migration');
export const seed = 0x50484130;
const htmlPath = 'src/index.html';
const helperPath = 'src/analytics/FeatureExtractor.js';
const questionsPath = 'src/data/questions.json';

// Content boundaries, not line numbers or brace counting (templates contain braces).
// A boundary must occur exactly once; changes fail closed rather than widening a slice.
export function extractSection(source, startMarker, endMarker) {
  const uniqueIndex = marker => {
    const index = source.indexOf(marker);
    if (index < 0 || source.indexOf(marker, index + marker.length) !== -1) {
      throw new Error('Missing or ambiguous production section boundary');
    }
    return index;
  };
  const start = uniqueIndex(startMarker);
  const end = uniqueIndex(endMarker);
  if (end <= start) throw new Error('Reversed production section boundaries');
  return source.slice(start, end).trimEnd();
}

export function selectedSections(html, helper) {
  const sections = [
    ['catalog', 'const allChampions = {', '// Helper function to get champion image URL'],
    ['imageUrl', 'function getChampionImageUrl(championName) {', '// Test function to verify champion image URLs'],
    ['statsAndBootstrap', 'const championStatsDatabase = {', '// Questions Database - Loading from JSON file'],
    ['randomForest', 'class SimpleRandomForest {', 'class SimpleDecisionTree {'],
    ['decisionTree', 'class SimpleDecisionTree {', 'class SimpleKNN {'],
    ['knn', 'class SimpleKNN {', '// Score Aggregation System for Unified ML Recommendations'],
    ['aggregator', 'class ScoreAggregator {', '// Evaluation Metrics Class'],
    ['metrics', 'class EvaluationMetrics {', '/**\n         * Display evaluation metrics in a visually appealing section'],
    ['answerMapping', 'const userFeatures = {\n                // Core gameplay preferences', '// TASK 5.2: Run all 3 algorithms with predictAll()'],
    ['mappingHelpers', 'function mapDifficulty(answer) {', 'function displayResults(performanceStats) {'],
    ['questionProjection', 'questionsDatabase = data.questions.map(question => {', "\n            })\n            .catch(error => {\n                console.error('Failed to load questions from JSON"]
  ].map(([name, start, end]) => ({ name, path: htmlPath, start, end,
    code: (() => {
          try { return extractSection(html, start, end); }
          catch { throw new Error(`Invalid production boundaries for section: ${name}`); }
        })() }));
  sections.push({ name: 'featureExtractor', path: helperPath,
    start: 'class FeatureExtractor {', end: "if (typeof module !== 'undefined' && module.exports)",
    code: extractSection(helper, 'class FeatureExtractor {', "if (typeof module !== 'undefined' && module.exports)") });
  return sections;
}

export function normalize(value) {
  if (typeof value === 'number') {
    if (!Number.isFinite(value)) throw new Error('Non-finite reference number');
    const rounded = Number(value.toFixed(12));
    return Object.is(rounded, -0) ? 0 : rounded;
  }
  if (Array.isArray(value)) return value.map(normalize);
  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(Object.keys(value).sort().map(key => [key, normalize(value[key])]));
  }
  if (value === undefined) throw new Error('Unexpected undefined reference value');
  return value;
}
export const canonical = value => JSON.stringify(normalize(value)) + '\n';
const hash = text => createHash('sha256').update(text).digest('hex');

export function createReference({ profiles } = {}) {
  const html = readFileSync(resolve(root, htmlPath), 'utf8');
  const helper = readFileSync(resolve(root, helperPath), 'utf8');
  const questionsText = readFileSync(resolve(root, questionsPath), 'utf8');
  const questions = JSON.parse(questionsText).questions;
  const sections = selectedSections(html, helper);
  for (const { code } of sections) {
    if (/GEMINI_API_KEY|AIza[\w-]{20,}|<script|localStorage|fetch\s*\(|document\./.test(code)) {
      throw new Error('Selected source contains excluded credentials or browser code; refusing evaluation');
    }
  }
  const section = name => sections.find(item => item.name === name).code;
  const context = vm.createContext(Object.create(null), {
    name: 'phase-0-reference', codeGeneration: { strings: false, wasm: false }
  });
  const run = code => new vm.Script(code, { filename: 'selected-reference.js' })
    .runInContext(context, { timeout: 10_000 });
  // No host functions/objects are injected. Only JSON strings cross the boundary.
  // vm is a controlled evaluator for trusted local source, not a hostile-code sandbox.
  run(`
    const console = Object.freeze({ log() {}, warn() {}, error() {} });
    const Date = undefined, performance = undefined;
    let randomState = ${seed}, randomCalls = 0;
    Object.defineProperty(Math, 'random', { configurable: false, writable: false, value: () => {
      randomCalls++;
      randomState = (Math.imul(1664525, randomState) + 1013904223) >>> 0;
      return randomState / 4294967296;
    }});
    ${section('catalog')}
    ${section('imageUrl')}
    ${section('statsAndBootstrap')}
    const bootstrapRandomCalls = randomCalls;
    ${section('featureExtractor')}
    ${section('randomForest')}
    ${section('decisionTree')}
    ${section('knn')}
    ${section('aggregator')}
    ${section('metrics')}
    ${section('mappingHelpers')}
    function convertAnswers(answers) {
      ${section('answerMapping')}
      return userFeatures;
    }
    const algorithms = {
      randomForest: new SimpleRandomForest(),
      decisionTree: new SimpleDecisionTree(),
      knn: new SimpleKNN()
    };
    algorithms.randomForest.initializeChampionData();
    const data = JSON.parse(${JSON.stringify(questionsText)});
    let questionsDatabase;
    ${section('questionProjection')}
    function compactAggregate(value) {
      const { details, ...scores } = value;
      return scores;
    }
    function capture(profile) {
      const features = convertAnswers(profile.answers);
      const scores = Object.fromEntries(Object.entries(algorithms)
        .map(([name, algorithm]) => [name, algorithm.predictAll(features)]));
      const aggregated = ScoreAggregator.aggregateScores(scores.randomForest, scores.decisionTree, scores.knn);
      const top10 = ScoreAggregator.selectTop10(aggregated);
      const results = { top10 };
      const recommended = EvaluationMetrics.getRecommendedChampions(results);
      const relevant = EvaluationMetrics.calculateUserRelevance(profile.answers, allChampions, results);
      return { ...profile, features, scores,
        aggregated: Object.values(aggregated).map(compactAggregate),
        top10: top10.map(compactAggregate),
        evaluation: {
          relevantChampions: [...relevant],
          precisionAt1: EvaluationMetrics.precisionAtK(recommended, relevant, 1),
          precisionAt3: EvaluationMetrics.precisionAtK(recommended, relevant, 3),
          precisionAt10: EvaluationMetrics.precisionAtK(recommended, relevant, 10),
          recallAt10: EvaluationMetrics.recallAtK(recommended, relevant, 10),
          f1At10: EvaluationMetrics.f1ScoreAtK(recommended, relevant, 10),
          mrr: EvaluationMetrics.meanReciprocalRank(recommended, relevant)
        }
      };
    }
  `);
  const read = expression => JSON.parse(run(`JSON.stringify(${expression})`));
  const catalog = read('Object.entries(allChampions).map(([name, data]) => ({ name, data }))');
  const forest = read(`algorithms.randomForest.trees.map(tree => ({
    id: tree.id, featureIndices: tree.featureIndices,
    bootstrapChampionNames: tree.sample.map(([name]) => name)
  }))`);
  const effectiveQuestions = read('questionsDatabase');
  const generatedProfiles = profiles ?? syntheticProfiles(questions);
  const captures = generatedProfiles.map(profile => read(`capture(JSON.parse(${JSON.stringify(JSON.stringify(profile))}))`));
  const manifest = {
    schemaVersion: 1,
    source: htmlPath,
    helperRepair: { path: helperPath, reason: 'Provide missing FeatureExtractor binding to selected scoring code only; production unmodified.' },
    normalization: 'Object keys sorted; array order retained; finite numbers rounded to 12 decimal places; no timestamps.',
    aggregationEncoding: 'Aggregated/top10 omit duplicate details; retrieve details from scores[algorithm][championName].details.',
    randomness: { seed, algorithm: 'LCG uint32: imul(1664525, state) + 1013904223; / 2^32',
      bootstrapRandomCalls: read('bootstrapRandomCalls'), totalRandomCalls: read('randomCalls'),
      runtime: { node: process.versions.node, v8: process.versions.v8 },
      caveat: 'Production random-comparator Array.sort is engine-dependent. Exported feature subsets are authoritative, not RNG emulation.' },
    inputs: [
      { path: questionsPath, sha256: hash(questionsText) },
      { path: helperPath, sha256: hash(helper) }
    ],
    sections: sections.map(({ name, path, start, end, code }) => ({ name, path, start, end, sha256: hash(code) })),
    effectiveQuestionCount: effectiveQuestions.length,
    championCount: catalog.length,
    profileIds: captures.map(profile => profile.id),
    capabilities: read(`({ process: typeof process, require: typeof require, fetch: typeof fetch,
      document: typeof document, window: typeof window, localStorage: typeof localStorage,
      Date: typeof Date, performance: typeof performance })`)
  };
  // Runtime metadata is informational: changing Node alone must not produce drift.
  // Runtime-dependent changes to the actual forest/scores still fail comparison.
  const artifacts = new Map([
    ['manifest.json', manifest], ['catalog.json', catalog], ['forest.json', forest],
    ['questions.json', effectiveQuestions],
    ...captures.map(profile => [`profile-${profile.id}.json`, profile])
  ]);
  return artifacts;
}

export function comparisonText(name, value) {
  if (name === 'manifest.json') {
    value = structuredClone(value);
    delete value.randomness.runtime;
  }
  return canonical(value);
}

export function compareArtifacts(artifacts, directory = fixtureDirectory) {
  const expected = [...artifacts.keys()].sort();
  const actual = readdirSync(directory).filter(name => name.endsWith('.json')).sort();
  const drift = actual.filter(name => !artifacts.has(name)).map(name => `Unexpected fixture: ${name}`);
  for (const name of expected) {
    if (!actual.includes(name)) { drift.push(`Missing fixture: ${name}`); continue; }
    const stored = JSON.parse(readFileSync(resolve(directory, name), 'utf8'));
    if (comparisonText(name, stored) !== comparisonText(name, artifacts.get(name))) {
      drift.push(`Reference drift: ${name}`);
    }
  }
  return drift;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const mode = process.argv[2] ?? '--check';
    if (!['--write', '--check'].includes(mode) || process.argv.length > 3) {
      throw new Error('Usage: node scripts/migration/reference.mjs [--write|--check]');
    }
    const artifacts = createReference();
    if (mode === '--write') {
      for (const [name, value] of artifacts) writeFileSync(resolve(fixtureDirectory, name), canonical(value));
      console.log(`Wrote ${artifacts.size} sanitized reference fixtures.`);
    } else {
      const drift = compareArtifacts(artifacts);
      if (drift.length) throw new Error(drift.join('\n'));
      console.log(`Reference check passed: ${artifacts.size} fixtures match.`);
    }
  } catch (error) {
    // Never dump selected source, VM stack snippets, or the full HTML on failure.
    console.error(error.message);
    process.exitCode = 1;
  }
}
