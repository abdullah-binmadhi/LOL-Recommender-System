# Phase 0 browser baseline

Captured on 2026-10-21 with Chromium 145.0.7632.6. The current working tree was served over local HTTP under `/LOL-Recommender-System/` to exercise the repository-subpath paths. Source commit: `d0e8c674132e7b5034b80f0aa00d56c9c63234f1`; working tree was dirty.

## Screenshots

There are nine states at each viewport (1440×900 desktop and 390×844 mobile): root entry, recommender start, registration, question 1, Mage-filtered question 3, recommendations, metric modal, champion modal, and analytics dashboard. The matching PNGs are named `desktop-01-...` and `mobile-01-...` through `-09-...`.

The screenshots are viewport captures, not full-page captures. Five of five hero champion images and ten of ten recommendation images loaded at both viewport sizes; the champion modal image loaded too. The analytics page rendered all five charts.

## Observed interactions and storage

- The root links navigate to the recommender and analytics dashboard; the analytics redirect reaches `src/analytics/analytics.html`.
- A synthetic registration advances to the questionnaire and creates one `championRecommenderUsers` entry in the isolated context.
- Selecting Mage filters question 3's position options to Mid and Support. The full 12-question flow completes and the Rust/Wasm browser scorer renders 10 recommendations.
- Metric and champion modals open and close with Escape.
- `Ctrl+Shift+E` downloads `champion_recommender_users.csv` from the synthetic-only record. The download was observed but not retained. The hidden admin prompt was not used.
- The browser context was non-persistent and closed after each viewport run. Its storage was discarded; no existing browser records were read, changed, or exported.

## Network observations

`browser-observations.json` records requests/responses by method, host, path, resource type, and stage; query strings and bodies are intentionally omitted.

- Local GETs served the root/recommender/analytics pages, question and champion data, counter data, the Rust scoring adapter, and `public/wasm/lol_scoring_core.wasm`.
- External GETs loaded Data Dragon champion imagery and Chart.js from jsDelivr.
- The page attempted one Vercel Web Vitals POST during load. The capture harness intercepted it before transmission; no body was sent or recorded.
- No request to Google Generative Language/Gemini was attempted. The capture harness blocks that host as a safety guard.
- There were no uncaught browser page errors. Any blocked telemetry resource console message is expected from the harness interception.

The registration values were synthetic, and no answers or registration data were submitted to a remote service. Reproduce the legacy-site baseline with `python scripts/migration/capture_browser_baseline.py` (requires the optional Playwright Python package and Chromium). Verify the local or deployed Rust preview with `python scripts/migration/verify_rust_preview.py` or `python scripts/migration/verify_rust_preview.py --url https://abdullah-binmadhi.github.io/LOL-Recommender-System/rust-preview/`.

## Published preview smoke test

After Pages deployment run [39570548610](https://github.com/abdullah-binmadhi/LOL-Recommender-System/actions/runs/39570548610), Chromium 145.0.7632.6 verified `https://abdullah-binmadhi.github.io/LOL-Recommender-System/rust-preview/` on 2026-10-21. The document, generated CSS, and generated Wasm all returned HTTP 200; all 12 questionnaire fields enabled Rust scoring, which returned 10 champions. The isolated preview did not write localStorage. See `rust-preview-published-observations.json` and `rust-preview-published.png`.

The deployed root entry remained unchanged: the Git blob IDs for both `index.html` and `src/index.html` are identical before and after the preview deployment.

## Phase 4 Rust preview baseline

Captured on 2026-10-21 with Chromium 145.0.7632.6 via `scripts/migration/verify_rust_preview.py`.

Seven screenshots at each viewport (desktop 1440×900 and mobile 390×844) cover:
- Start screen (`rust-phase4-{viewport}-start.png`)
- Registration screen (`rust-phase4-{viewport}-registration.png`)
- Question 1 (`rust-phase4-{viewport}-question-1.png`)
- Position question filtered by Mage role (`rust-phase4-{viewport}-filtered-question.png`)
- Top 10 recommendations and evaluation metrics (`rust-phase4-{viewport}-results.png`)
- Champion details modal (`rust-phase4-{viewport}-champion-modal.png`)
- Metric explanation modal (`rust-phase4-{viewport}-metric-modal.png`)

All checks passed with 0 page errors and 0 console errors; observations are saved in `rust-phase4-local-observations.json`.

## Phase 5 Rust storage, exports, and integrations baseline

Captured on 2026-10-21 with Chromium via `scripts/migration/verify_rust_preview.py`.

Eight screenshots at each viewport (desktop 1440×900 and mobile 390×844) cover:
- Start screen (`rust-phase5-{viewport}-start.png`)
- Registration screen (`rust-phase5-{viewport}-registration.png`)
- Question 1 (`rust-phase5-{viewport}-question-1.png`)
- Position question filtered by Mage role (`rust-phase5-{viewport}-filtered-question.png`)
- Top 10 recommendations and evaluation metrics (`rust-phase5-{viewport}-results.png`)
- Champion details modal (`rust-phase5-{viewport}-champion-modal.png`)
- Metric explanation modal (`rust-phase5-{viewport}-metric-modal.png`)
- Saved users modal (`rust-phase5-{viewport}-saved-users-modal.png`)

Verified storage and interaction behaviors:
- Registration appends 1 record to `championRecommenderUsers` in `localStorage` with required and optional fields.
- Recommendation results update `recentChampions` in `localStorage` (10 champions, unshifted and deduplicated).
- "Save" button displays saved users table modal.
- "Download CSV" inside the saved users modal downloads `champion_recommendations.csv`.
- `Ctrl+Shift+E` keyboard shortcut triggers `champion_recommender_users.csv` download with 26-column header and quoted fields.
- `Ctrl+Shift+A` keyboard shortcut opens the Enhanced Admin Panel confirm prompt.
- Zero console errors and zero page errors observed; observations are saved in `rust-phase5-local-observations.json`.


## Phase 6 Rust analytics and website pages baseline

Captured on 2026-10-21 with Chromium via `scripts/migration/verify_rust_preview.py`.

Nine screenshots at each viewport (desktop 1440×900 and mobile 390×844) cover:
- Start screen (`rust-phase6-{viewport}-start.png`)
- Analytics Dashboard with pure SVG charts (`rust-phase6-{viewport}-analytics.png`)
- Registration screen (`rust-phase6-{viewport}-registration.png`)
- Question 1 (`rust-phase6-{viewport}-question-1.png`)
- Position question filtered by Mage role (`rust-phase6-{viewport}-filtered-question.png`)
- Top 10 recommendations and evaluation metrics (`rust-phase6-{viewport}-results.png`)
- Champion details modal (`rust-phase6-{viewport}-champion-modal.png`)
- Metric explanation modal (`rust-phase6-{viewport}-metric-modal.png`)
- Saved users modal (`rust-phase6-{viewport}-saved-users-modal.png`)

Verified analytics and interaction behaviors:
- Native Rust SVG charts render cleanly with positive dimensions and zero external dependencies:
  - Hero Type Bar Chart (`#heroTypeSvg`)
  - Melee vs Ranged Donut Chart (`#rangeTypeSvg`)
  - Resource Types Bar Chart (`#resourceTypeSvg`) with explicit synthetic/illustrative data disclosure
  - Champion Difficulty Donut Chart (`#difficultySvg`)
  - Champions Released by Year Line Chart (`#releaseYearSvg`)
- Analytics stat ribbon displays all 171 champions across 6 hero classes, 2 combat types, 3 difficulty tiers, and 16 historical release years.
- "← Back to Home" button returns seamlessly to the landing screen.
- Direct URL hash routing (`#analytics`) and static redirect (`analytics.html`) route directly to the analytics dashboard.
- Zero external calls to Chart.js or jsDelivr; zero page errors; zero console errors. Observations recorded in `rust-phase6-local-observations.json`.
