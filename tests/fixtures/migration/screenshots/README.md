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
