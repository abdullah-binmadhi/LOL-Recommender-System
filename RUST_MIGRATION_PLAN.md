# Rust Migration Plan

**Status:** in progress
**Last verified:** 2026-10-21
**Scope:** migrate the website's first-party application logic to Rust while preserving today's UI, questionnaire, effective champion catalog, and recommendation behavior.

## Objective and non-goals

The migration changes implementation language, not product behavior. Keep the current visual design, question text/order/options, effective inline champion data, answer conversions, scoring rules, result ordering, storage compatibility, and reachable user journey unchanged until parity is accepted.

Keep these as separate post-migration work: website bug fixes and UX redesign, questionnaire improvements, champion updates/data quality, and genuinely trained ML models. Python analysis/report tooling is outside the website migration and can remain unchanged.

Target architecture:

- A Cargo workspace with `crates/recommender-core` for data contracts, conversion, scoring, aggregation, and evaluation.
- `crates/recommender-web` with Leptos for the browser UI and browser-facing behavior.
- Trunk to build and serve the Leptos application as WebAssembly.
- Serde for JSON and compatibility with existing local records.
- HTML and CSS remain web technologies; generated JavaScript/Wasm glue is expected, but handwritten JavaScript application logic is not a target runtime dependency.
- Static hosting on GitHub Pages initially; no backend is required.

## Current repository status

The repository now contains the Rust workspace and an isolated Leptos/Trunk preview alongside the current production site. The migration is incomplete: the existing site remains the default, and its main UI, storage, exports, and analytics have not been ported.

| Phase | Status | Evidence / remaining work |
|---|---|---|
| 0 — Freeze the migration contract | **Complete** | The production-reference harness, 12 JSON fixtures, eight synthetic profiles, source inventory, known-issues list, desktop/mobile screenshots, and browser-observed interaction/network report are recorded here and under `tests/fixtures/migration/screenshots/`. Reference, native Rust, Wasm, and browser checks pass. |
| 1 — Build and deployment skeleton | **Local browser gate passes; Pages publication pending** | The root workspace contains `recommender-core` and `recommender-web`; pinned Leptos 0.8.22 and Trunk 0.22.0 build successfully. A project-subpath local browser smoke test confirms CSS/Wasm loading and Rust recommendations. The configured Pages workflow still needs to publish and verify the preview; the default root entry remains unchanged. |
| 2 — Data contracts and feature conversion | **Implemented and parity-tested** | Rust contracts load the unchanged question JSON and the extracted ordered 162-champion production catalog. Typed answer conversion matches all eight reference profiles; storage fields and unknown values have Serde coverage. Production UI state remains JavaScript until Phase 4. |
| 3 — Recommendation engine | **Scoring parity implemented against the captured reference** | Rust ports all three scorers, aggregation, ranking/diversity, explanations, and metrics. Native and Wasm parity checks pass for all eight profiles. Preserve these tests while moving code into the eventual workspace crates. |
| 4 — Main website UI | **Not started** | Screens, navigation, and DOM state are still in `src/index.html`. |
| 5 — Storage, exports, integrations | **Not started** | Browser storage, CSV export, and existing integrations remain JavaScript/browser-owned. |
| 6 — Analytics and remaining pages | **Not started** | The active dashboard remains HTML/JavaScript and loads Chart.js from a CDN. |
| 7 — Validate, cut over, retire old runtime | **Not started** | There is no Leptos/Trunk cutover, generated-site-only deployment, measured release baseline, or tested rollback. |

The published root website remains the default and is not redirected to the Rust preview. In the current working tree, the recommendation call is wired through the Rust/Wasm adapter while the old JavaScript scorer remains the Phase 0 differential oracle; these root-page working-tree changes are not part of the preview-only Pages publication. The Leptos preview itself runs its questionnaire and scoring in Rust.

## Phase 0 — Freeze the migration contract

### Phase 0 checklist and exit gate

- [x] Inventory the current entry points, reachable screens, controls, storage, export, analytics, and statically discoverable external requests.
- [x] Separate active behavior from disconnected/legacy helpers; list known product/security issues separately from migration tasks.
- [x] Capture current effective questions and champion/scoring inputs in reproducible fixtures.
- [x] Repair only the missing `FeatureExtractor` binding inside the reference harness; do not change production feature conversion or fabricate champion stats.
- [x] Verify JavaScript reference drift, native Rust parity, and Wasm parity.
- [x] Capture reference screenshots at desktop and mobile sizes, using synthetic registration values and no real user data.
- [x] Run a browser-observed pass for navigation, filtering, modals, storage/export behavior, and actual network requests. No real registration or dormant AI feature was invoked.

**Phase 0 is complete.** The screenshot set covers nine states at 1440×900 and 390×844. The browser report records navigation, role-filtered options, the 10-result Rust scoring flow, modal open/close behavior, isolated localStorage, the user-facing CSV download, analytics rendering, and observed requests. See `tests/fixtures/migration/screenshots/README.md` and `browser-observations.json`.

### Baseline user journey and reachable surfaces

| Surface | Current behavior to preserve |
|---|---|
| Root entry (`index.html`) | Intro page links to the recommender (`src/index.html`) and analytics dashboard (`analytics.html`). |
| Recommender start (`src/index.html`, `#start-screen`) | Existing hero, champion images, feature panels, “Find My Champion”, and Analytics navigation. Preserve markup appearance and layout, not the marketing claims as quality evidence. |
| Registration (`#user-registration`) | Name, email, age, and experience are required; gender and phone are optional. Registration appends a record to localStorage before advancing. This includes personal data and must remain local unless a separate product decision changes it. |
| Questionnaire (`#questions-container`) | Effective JSON contains 12 questions in IDs 1–12 order. Cards expose option selection, progress, Previous/Next, disabled-next state, loading/error/retry states, and a final Rust scoring action. Role-dependent filters affect Q3–5 and Q7; Q1 also has logic based on answer 10. Preserve even quirky filtering/mapping behavior during this migration. |
| Recommendation/detailed analysis (`#detailed-analysis`) | The intermediate multi-algorithm page is skipped. Current output is up to 10 ranked champions, with three algorithm scores, the field named `average`, weighted/result details, evaluation metric cards, explanations, champion-specific/role fallback tips, and Try Again. |
| Metric and champion modals | Metric cards open a metric explanation modal; recommendation detail actions open champion score/calculation details. Escape closes the metric/champion modal. Detailed analysis also requests matchup counters for the top recommendation. |
| Analytics (`analytics.html` → `src/analytics/analytics.html`) | Root dashboard entry redirects to the dashboard page. It renders charts for hero type, melee/ranged, resource type, difficulty, and release year, with a back-to-home link. This is a separate active page, not an isolated test fixture. |

Legacy analytics/test HTML files are not individual user-facing application flows. `migration_files/` and `docs-archive/` are not runtime sources for the migration; keep the latter read-only. The Python analysis/report scripts are outside the website scope.

### Questionnaire contract

The effective question sequence is read from `src/data/questions.json` (the successful production projection), not inferred from stale “10 question” comments:

| ID | Current question | Declared mapping |
|---:|---|---|
| 1 | What is your preferred difficulty? | `difficulty` |
| 2 | What is your preferred role? | `role` |
| 3 | What is your preferred position? | `position` |
| 4 | What playstyle do you prefer? | `playstyle` |
| 5 | Do you prefer range or melee? | `attack_range` |
| 6 | What is your preferred champion resource? | `resource_type` |
| 7 | What is your preferred damage type? | `damage_type` |
| 8 | How do you usually respond under pressure in a game? | `pressure_response` |
| 9 | What type of character aesthetic appeals to you most? | `aesthetic_preference` |
| 10 | When cooperating with others, how do you prefer to contribute to the team? | `team_contribution` |
| 11 | Which character identity do you usually prefer? | `character_identity` |
| 12 | When faced with a difficult challenge, what best describes your approach? | `problem_solving` |

The production scorer's current mapping is not the declared mapping: IDs 6–10 supply the five psychological feature values (pressure, aesthetic, team contribution, identity, problem-solving), while IDs 11–12 do not affect scoring. Preserve that behavior for parity; do not “correct” it based on `feature_mapping` in this migration.

### Effective catalog and scoring contract

- The authoritative catalog is the effective inline `allChampions` object in `src/index.html`, after JavaScript duplicate-key overwrite behavior. The captured effective order contains **162 champions**. Do not replace it with the enriched `src/data/champions.json` or `migration_files/data/champions.json` catalog during the language migration.
- Random Forest: 10 trees; fixture data records each tree's actual ordered feature subset and bootstrap sample. Production uses the subsets across all champions for `predictAll`; bootstrap membership does not filter the scored catalog. The JS random comparator is engine-dependent, so fixtures provide the actual choices rather than asking Rust to mimic JavaScript RNG.
- Decision Tree: preserve current scoring, details, and order.
- KNN: preserve current distance/normalization and explanation behavior.
- Aggregation: RF 40%, DT 30%, KNN 30%. The weighted mean is named `average`; `weighted` is currently the same calculation. Preserve the names and values.
- Selection: stable descending order and top 10. Diversity's first pass allows up to three champions per role, then fills any remaining places without a role cap. Do not substitute older top-five/max-two descriptions.
- Evaluation/relevance is the current user-answer heuristic, partly based on model outputs; it is not independent ground truth and does not support accuracy claims.
- Preserve `Bot` versus catalog `ADC`, feature defaults, range casing/default differences, missing derived-stat behavior, and the current handling of unknown/historical values until migration acceptance.

### Storage and export contract

| Key / mechanism | Current serialized shape and behavior |
|---|---|
| `championRecommenderUsers` | JSON array in localStorage. Reachable registration appends objects with `fullName`, `email`, `phone`, `age`, `gender`, `experience`, `registrationDate`, and `sessionId`. Optional/defaulted values include `Not provided` phone and `Not specified` gender. Older/disconnected save helpers expect optional result properties such as `recommendedChampion`, `winningAlgorithm`, `confidence`, per-algorithm champion/confidence fields, `consensusLevel`, stringified `userAnswers`, `completionDate`, and psychological answer fields. Treat existing records as heterogeneous; do not overwrite or migrate them destructively. |
| `recentChampions` | JSON array of champion names, updated after results and trimmed to the latest 15. |
| Ctrl+Shift+E | Reachable CSV export from `championRecommenderUsers`, named `champion_recommender_users.csv`. The active `exportToCSV()` header has 26 columns, including registration/PII fields, optional recommendation fields, serialized answers, and completion time. Preserve quoting/order/field names for compatibility, while tests should include commas, quotes, and Unicode when the exporter is ported. |
| Ctrl+Shift+A | Hidden admin prompt that can invoke the same CSV export. |
| “Save” result button | Calls `saveRecommendation()`, which displays stored user records; it does not persist the current recommendation in the reachable path. Do not silently activate disconnected result-saving helpers during migration. |

There are duplicate/older export and save helpers in the monolithic page. They are inventory material, not permission to make them reachable. The actual CSV format and every historical object variant should be fixture-tested before cutover.

### Analytics and external request inventory

Static source inspection and the Chromium 145.0.7632.6 browser pass on 2026-10-21 identify these request surfaces. The capture ran against a local static server using the repository Pages subpath and a fresh, non-persistent browser context:

- Two question loaders run at page startup. The active quiz loader fetches local `data/questions.json` with path fallbacks, including the published GitHub Pages URL, then projects `data.questions` into `questionsDatabase`. A second loader also runs, stores a separate `questions` value, expects a different data shape, and is not used by the active quiz.
- The page sends a Vercel Web Vitals request on load.
- Champion imagery uses public Data Dragon URLs; an image fallback can use `via.placeholder.com`.
- Detailed analysis fetches `data/champion_counters.json` for the top recommendation.
- Dormant Gemini report/metric explanation functions contain POST requests to Google Generative Language. No current reachable button/call path was found for those features. Do not invoke, copy, or activate them as part of migration. A hard-coded Gemini credential exists in the current source; treat it as exposed and revoke/rotate it. Never include credentials in Rust, Wasm, Tauri bundles, fixtures, or migration documentation.
- The analytics dashboard loads Chart.js from jsDelivr and tries local/repository/raw-GitHub champion JSON paths.
- No backend persistence is part of the normal registration path; the visible record store is browser localStorage.

During the observed pass, local GETs loaded the HTML, both question-loader paths, catalog/counter data, the Rust scoring adapter, and the scoring Wasm. External GETs loaded Data Dragon images and Chart.js from jsDelivr. The app attempted one Vercel Web Vitals POST on page load; the harness intercepted it before transmission. No Gemini request was attempted, and no request body or query string was recorded. The dashboard rendered five charts and the browser reported no uncaught page errors. The Phase 5 gate must still verify that the Rust application makes no accidental extra network requests and sends no user answers/PII to an external service.

### Current reference harness and reproducible fixtures

The source of truth for scoring parity is the current working-tree `src/index.html`, with the narrowly repaired `FeatureExtractor` binding documented in `scripts/migration/README.md`:

- `scripts/migration/reference.mjs` selects guarded production source sections, evaluates them in a fresh Node VM, and emits deterministic normalized JSON. It does not execute the whole page or browser code.
- The harness uses `src/analytics/FeatureExtractor.js` only to supply the otherwise missing `FeatureExtractor` binding. Production code is not rewritten, and derived champion attributes are not manufactured. Node's `vm` is not a security sandbox for hostile source.
- `tests/fixtures/migration/` has 12 JSON artifacts: effective catalog, question projection, actual forest choices, manifest/provenance, and eight profiles (`mage-mid`, `marksman-bot`, `tank-top`, `support-team`, `empty-defaults`, `unknown-values`, `no-preference-adc`, `legacy-psychological-10`). Profile fixtures include converted features, every champion's scores/details, aggregation, top10, relevant names, and metrics.
- Reference output normalizes object-key order and rounds finite floats to 12 decimals; parity uses absolute tolerance `1e-9` for numeric values and exact structure/string/array order.
- Fixture updates are reviewed changes. Run `--check` by default; regenerate only after examining source drift and intended behavior.

Commands verified on 2026-10-21:

```sh
node scripts/migration/reference.mjs --check
node --test scripts/migration/reference.test.mjs
cargo fmt --all
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo build --release --locked --target wasm32-unknown-unknown --package recommender-core
node --test scripts/migration/wasm.test.mjs
node scripts/build-wasm.mjs
trunk build --release
```

All listed scoring/build checks passed: 12 reference fixtures matched; reference harness 8/8 tests; Rust 30 unit + 4 parity/integration tests; clippy and formatting passed; core Wasm build and adapter 4/4 tests passed. Trunk 0.22.0 generated the preview with CSS, JS glue, and Wasm URLs under `/LOL-Recommender-System/rust-preview/`. The local browser validation and Phase 0 browser baseline passed. Actual GitHub Pages publication and remote preview verification remain the Phase 1 exit gate. The Wasm test's incorrect adapter import extension was fixed from `.mjss` to `.mjs` as a harness-only correction.

### Captured screenshot and browser baseline

Captured on 2026-10-21 with Chromium 145.0.7632.6 from the local working tree at `d0e8c674132e7b5034b80f0aa00d56c9c63234f1` (working tree was dirty), served under the repository subpath to exercise relative URLs.

- Desktop 1440×900 and mobile 390×844; nine states at each size: root entry, recommender start, registration form, first question, Mage-filtered position question, recommendations, metric modal, champion modal, and analytics dashboard.
- Five of five hero images and ten of ten recommendation images loaded at both sizes. The champion modal image loaded successfully.
- Each run used a fresh, non-persistent browser context. A synthetic registration was stored only in that context; `championRecommenderUsers` and `recentChampions` were verified, then discarded when the context closed.
- The reachable `Ctrl+Shift+E` CSV download was observed using the synthetic record; the downloaded file was not retained. The hidden admin prompt and dormant AI paths were not used.
- Request hosts, methods, paths, status codes, and failures (without query strings or bodies) are recorded in `browser-observations.json`. The Vercel Web Vitals POST was blocked before transmission; no Gemini request was attempted.

Artifacts: `tests/fixtures/migration/screenshots/`. Reproduce with `python scripts/migration/capture_browser_baseline.py` after installing the optional Playwright Python package and Chromium. Compare these against the Rust preview before any redesign or visual cleanup.

### Known issues — separate from migration tasks

These are recorded to prevent the language migration from disguising product changes. Do not fix them during parity work unless a separate decision explicitly expands scope.

1. The app and several docs describe a 10-question flow, while the effective questionnaire has 12 entries.
2. The current answer IDs do not align with the declared mappings: psychological features read IDs 6–10 and ignore 11–12.
3. The question loader has duplicate implementations and different expected JSON shapes; keep fallback/UI behavior visible during browser verification.
4. `Bot` is not aliased to champion `ADC`; derived champion stats are missing for some catalog entries and use current production fallbacks.
5. The current marketing accuracy claims are not backed by an independent held-out ground truth; the displayed evaluation is a heuristic.
6. Registration collects name/email/age and optional phone, storing them in localStorage. Browser-local storage is not a secure or synchronized database.
7. Some result persistence, AI-report, and duplicate CSV helpers are disconnected or stale. The reachable Save button is not a “save this recommendation” action despite its label.
8. A Gemini API credential is hard-coded in the existing page. Treat it as compromised: revoke/rotate outside this migration and never bundle a replacement secret client-side.
9. Analytics uses a CDN dependency and multiple data-path fallbacks; actual network behavior and mobile chart usability need baseline verification.

## Phase 1 — Establish the Rust build and deployment skeleton

**Purpose:** prove the target stack builds and can be previewed before porting the main UI. Keep the current site as the default/live implementation.

**Current status:** the workspace, root lockfile, Leptos preview, Trunk configuration, CI commands, Pages build step, and repository-subpath output are implemented. Native tests, lint, Wasm build/parity, and the Trunk release build pass. Chromium loaded the local preview under `/LOL-Recommender-System/rust-preview/`: CSS returned 200, the Trunk Wasm asset returned 200, all 12 questions rendered, and the Rust UI produced 10 recommendations without writing localStorage. Actual GitHub Pages publication and remote verification remain outstanding; do not mark the phase complete until they pass.

- Create a Cargo workspace and commit the root lockfile. Move/organize scoring into `crates/recommender-core`; add `crates/recommender-web` for the Leptos UI.
- Pin mutually compatible Leptos, Trunk, Serde, and browser-target dependencies; add the `wasm32-unknown-unknown` target and Trunk configuration.
- Create a minimal `web/index.html` and static asset/CSS structure. Do not redesign; demonstrate loading a simple Leptos page and extracting existing CSS as a later UI task.
- Add repeatable development, release, format, lint, native test, and Wasm build commands/CI.
- Build with the GitHub Pages project subpath (for example `/LOL-Recommender-System/`) and verify asset/Wasm URLs against a preview. Do not switch the default entry or overwrite the current deployment.
- Align `.github/copilot-instructions.md` with the staged migration so it no longer mandates the old monolithic-JS target.

**Validation:** native core tests run; Leptos app compiles and loads in a browser; styles/assets/Wasm resolve from a repository-subpath preview.
**Exit gate:** a working Rust preview without changing the live application.

## Phase 2 — Port data contracts and feature conversion

**Purpose:** make Rust accept the same inputs and produce the same feature values.

**Current status:** implemented in `recommender-core`. `QuestionnaireDefinition::current()` reads the canonical question JSON, `EffectiveCatalog::current()` reads the ordered extracted 162-champion catalog, and typed answer conversion is covered against all eight synthetic profile fixtures. Serde contracts cover scores/results and heterogeneous historical localStorage fields. Production questionnaire UI loading/rendering remains unchanged until Phase 4.

- Define Rust types for champion catalog entries, questions, answers, score/details, and results.
- Extract the effective inline 162-champion catalog with overwrite and order semantics intact. Keep ordered catalog and tree choices explicit; do not infer order from sorted JSON keys.
- Load the existing question JSON without editing it. Keep questions UI-owned by JS until Phase 4, then move their loading/rendering to Rust.
- Port difficulty/playstyle/range conversions and derived feature calculations, including source defaults and unknown/historical values.
- Preserve storage field names via Serde and tolerate older/historical records without crashing or rewriting them.
- Preserve the current answer mapping—even where incorrect—so any later correction is separately measurable.

**Exit gate:** Rust produces equivalent feature values for every reference profile and the source effective catalog/question data are runtime inputs, not test-only fixture assumptions.

## Phase 3 — Port the recommendation engine

**Purpose:** make recommendation behavior independently testable and maintainable in `recommender-core`.

- Preserve all-champion scoring for Random Forest, Decision Tree, and KNN; normalization; contributing factors; detailed explanations; aggregation; stable top-10 diversity/fallback behavior; relevance rules; and evaluation metrics.
- Keep one narrow public contract: `Answers + Catalog + Scoring configuration → Recommendation results + Explanations`.
- Compare per-champion score values and explanation structure, not only winners. Use documented floating-point tolerance and preserve array/tie ordering.
- Feed Random Forest the actual fixture feature subsets. A Rust seeded RNG will not reproduce JavaScript's `Math.random()`/engine-dependent sort choices.

**Exit gate:** all reference profiles match features, per-champion results, ranking/order, details, and metrics. Current scoring work meets this against captured fixtures; retain that gate during workspace reorganization.

## Phase 4 — Port the main website UI

**Purpose:** replace the current DOM/global-script interaction layer in Rust without redesigning the user journey.

- Port the existing landing/start, registration, questionnaire and conditional option filtering, progress/navigation, loading/error states, recommendation cards, algorithm details, metrics, champion/metric modals, tips, counters, restart controls, and keyboard behavior.
- Extract and reuse the current CSS; compare to Phase 0 desktop/mobile screenshots rather than recreating styles.
- Use one explicit application state model rather than loose globals while keeping externally visible behavior unchanged.

**Validation:** browser interaction checks for back/next/restart/filter/modal/keyboard behavior and desktop/mobile visual comparisons.
**Exit gate:** the full primary flow uses Rust UI/state and no longer calls the old JS application logic.

## Phase 5 — Port browser storage, exports, and integrations

**Purpose:** preserve browser-facing behavior and existing user records.

- Implement localStorage access from Rust and read/write both existing keys without destructive migrations.
- Port active CSV export, Ctrl+Shift+E, hidden admin access if retained as agreed, and browser downloads. Preserve field names/order and validate historical records.
- Inventory each external integration and port only agreed reachable behavior. Do not activate disconnected result-saving code, dormant Gemini features, or previously uncalled server-save paths as a side effect.
- Security exception: do not copy the current exposed Gemini credential into Rust, Wasm, JavaScript glue, or a native package. Credential-dependent functionality requires a separate secure configuration decision; browser bundles cannot protect secrets.

**Validation:** synthetic historical records load; current records are not erased/overwritten unexpectedly; CSV/download/clipboard behavior is verified; no unintended external requests occur.
**Exit gate:** agreed browser behavior works without handwritten JavaScript application logic.

## Phase 6 — Migrate analytics and remaining website pages

**Purpose:** cover agreed active website pages, not just the quiz.

- Port the active analytics dashboard and preserve its current calculations/data source.
- Render charts with Rust-generated SVG or a compatible Rust plotting approach; avoid custom JavaScript/Chart.js for the new dashboard if preserving the Rust-first objective.
- Label illustrative/random data honestly. Replace copied JS algorithm-test pages with `recommender-core` tests; retain historical HTML tests as reference only until parity acceptance.

**Exit gate:** every agreed active website page uses the Rust implementation.

## Phase 7 — Validate, cut over, and retire the old runtime

- Run core, browser, storage compatibility, and release checks. Compare initial download size, startup time, recommendation latency, and mobile responsiveness against the baseline.
- Configure CI to build the Rust/Wasm artifact and deploy only generated site files. Add a working rollback path.
- Switch the default entry to the Rust application only after acceptance; remove obsolete runtime code only after the rollback window/acceptance decision.
- Update `README.md` and `USAGE_GUIDE.md`; leave `docs-archive/` unchanged.

**Exit gate:** agreed user flows work in Rust, recommendation parity is accepted, existing records remain compatible, deployment and rollback are tested, and known product issues remain separately tracked.

## Tauri decision — future optional track, not current scope

The current product scope is exclusively web-based. Leptos + Trunk supplies the browser UI/build and GitHub Pages hosts static output; no Tauri implementation or equivalent native packaging is needed now.

There is no `src-tauri/`, `tauri.conf.json`, Tauri dependency, or native application setup. Revisit Tauri only after the website migration if a separate desktop/mobile application is explicitly chosen. It would complement, not replace, the web app; reuse `recommender-core` and do not duplicate scoring. Any future native track would separately cover platform targets, icons/bundling/signing, permissions/capabilities, updates, and native testing. Treat all frontend/Wasm contents as public and never package API secrets.

Official references: [Tauri v2 overview](https://v2.tauri.app/start/), [frontend configuration](https://v2.tauri.app/start/frontend/), [calling Rust](https://v2.tauri.app/develop/calling-rust/), and [capabilities/security](https://v2.tauri.app/security/capabilities/).
