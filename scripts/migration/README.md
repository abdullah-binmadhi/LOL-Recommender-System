# Phase 0: production reference harness

Node built-ins only; no install/build step. Run from the project root (Node 22+ recommended):

```sh
node scripts/migration/reference.mjs --write
node scripts/migration/reference.mjs --check
node --test scripts/migration/reference.test.mjs
```

With no argument, the generator defaults to **check**, not write. Review drift before intentionally regenerating. Output goes only to the existing `tests/fixtures/migration/` directory. No directory is created by the runner. Check is read-only, compares normalized JSON, and exits nonzero for changed values/provenance, missing JSON files, unexpected JSON files, or invalid extraction. Write does not delete stale files; remove obsolete fixtures explicitly after review if profile IDs change.

## Reference boundary

The source of truth is the **current working-tree** `src/index.html`, not `migration_files/`, external champion JSON, or an assumed pristine commit. Existing production edits are intentionally reflected in the fixtures. `reference.mjs` selects uniquely occurring content boundaries for:

- Inline champion literal (evaluated with native object-literal duplicate-key behavior).
- Image URL helper, stats table, generated stats, and catalog initialization.
- Three algorithm classes, score aggregator, and evaluation metrics class.
- Only the feature-conversion statements inside `runAllAlgorithms`, its mapping helpers, and the successful JSON-to-question projection.

Boundaries are explicit content markers, not fragile line numbers, regex brace parsing, or execution of a whole page script. Missing, duplicate, or reversed markers fail closed. Delimiters and SHA-256 hashes of the exact selected text are recorded in `manifest.json`, plus full hashes of `src/data/questions.json` and `src/analytics/FeatureExtractor.js`. No full HTML is copied, emitted, or hashed. Unrelated UI/API changes are intentionally outside this check's scope.

**Only dependency repair:** the existing `src/analytics/FeatureExtractor.js` class is evaluated in the harness VM to supply the otherwise missing `FeatureExtractor` binding used by production feature conversion. Its CommonJS export wrapper is not needed/evaluated. No source is rewritten and no champion derived features are manufactured. This is a scoring reference with a documented helper repair, not a claim that the unmodified browser page completes successfully.

Selected code runs in a fresh `node:vm` context with a timeout, no injected host objects/functions, disabled string/Wasm code generation, and no browser, network, Node process/require, storage, timers, or date/performance APIs. The page's loaders, caching, persistence, analytics, UI, and AI/API code are never run. Console methods are inert VM-local functions. Known credential/browser-code markers are rejected before evaluation. Only synthetic option values enter the VM; only JSON leaves it. `vm` is **not a security sandbox for hostile code**: this harness trusts the reviewed local selected production sections. No browser or screenshot validation is implied.

## Determinism and Rust handoff

`Math.random` is replaced with a non-writable seeded uint32 LCG before any catalog initialization. Generated champion stats therefore consume random draws **before** the forest's bootstrap samples and random feature selections, matching the selected production execution order. A single algorithm set is reused across profiles, as on the page. Prediction timing/cache fields are not captured.

Production selects features using `Array.sort(() => Math.random() - 0.5)`, which is engine-dependent even with a fixed seed. `manifest.json` records the generation Node/V8 versions and random call counts. Prefer that runtime when updating fixtures. Runtime version strings alone are informational and excluded from check comparison; changed forest subsets, stats, scores, or any other manifest values still fail. Cross-engine equivalence is not guaranteed.

**Rust should consume `forest.json`'s actual ordered `featureIndices`, not reproduce JS RNG or random-comparator sorting.** Every tree also includes ordered `bootstrapChampionNames` (including repeats). `predictAll` currently uses feature subsets across all champions, not bootstrap membership; the latter is exported for audit/completeness, not to change scoring.

All JSON uses sorted object keys, retained array order, numbers rounded to 12 decimal places, negative zero normalized to zero, and no wall-clock fields. Non-finite numbers or unexpected undefined values fail instead of becoming JSON nulls. Computations/ranking run at native JS precision **before** normalization. Compact JSON keeps the fixtures practical; use a JSON viewer for inspection.

- `catalog.json`: ordered `{name, data}` array. This array is authoritative for effective inline key order, duplicate overwrites, and stable-sort tie breaking. Never infer catalog order from alphabetically normalized score-map keys. Public image URLs are inert data, never fetched.
- `forest.json`: 10 generated trees with actual subsets and bootstrap names.
- `questions.json`: effective questions from the production successful-load projection. Its length is computed (currently 12), not copied from outdated 10-question comments. Failed-network fallback/UI filtering is not exercised.
- `profile-*.json`: eight synthetic profiles (four valid 12-answer questionnaires spanning all difficulty/playstyle choices; empty, unknown, no-preference/ADC, and legacy-10 psychological edge profiles). Each contains answers, actual converted features, all champion scores/raw scores/details (and RF uncertainty), ordered aggregation, top10, relevant champion names, precision@1/3/10, recall@10, F1@10, and MRR.
- `manifest.json`: provenance, counts, seed, runtime, extraction markers/hashes, helper repair, and capability checks.

Aggregated/top10 records omit only duplicated `details`; those are available at `scores[algorithm][championName].details`. Aggregation records retain every other production field. Evaluation uses the production answer-based relevance function with `mlResults.top10`, as the display/report path does; these are internal heuristic metrics, **not independent ground truth or a quality claim**.

## Intentional behavior preserved

- Questions JSON has 12 items, but conversion/relevance still read psychological fields from IDs **6–10**. Thus resource/damage-type answers become psychological values; IDs 11–12 do not affect scoring. No feature_mapping-driven correction is introduced.
- `Bot` remains `Bot`, even though the catalog uses `ADC`.
- Explicit `No Preference` range becomes `no preference`, whereas a missing range becomes `no_preference`.
- Playstyle defaults, unknown option handling, original `/ 3.33` derived-feature conversion, and missing champion `derived_stats`/`herotype`/title behavior remain unchanged. In particular, missing derived stats contribute zero through the production fallbacks; no enrichment from other databases occurs.
- The current aggregator's `average` is the 40/30/30 weighted mean, not an arithmetic mean. Top10 uses the current max-three-per-role first pass and production fallback, not the older top-five/max-two description.

Tests cover these behaviors, actual question counts, section hashes/boundaries, catalog/forest/export sizes, score bounds/details, aggregation/diversity, metrics bounds, VM capability absence, fresh-run determinism, and drift detection. They validate the scoped scoring harness only; they do not validate browser loaders, UI progression, API calls, localStorage, or the future Rust implementation.
