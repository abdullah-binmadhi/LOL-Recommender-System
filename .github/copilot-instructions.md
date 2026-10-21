# LoL Champion Recommender — Agent Instructions

## Project goal

Move first-party website application logic to Rust while preserving the current product behavior. The active migration roadmap and parity contract are in [`RUST_MIGRATION_PLAN.md`](../RUST_MIGRATION_PLAN.md); consult it before changing questions, catalog data, scoring, storage, or deployment.

The current website remains the production/default entry until Phase 7 acceptance. The Rust Leptos/Trunk application is an isolated preview at `/LOL-Recommender-System/rust-preview/`. Do not redirect the root entry or remove the legacy implementation during Phases 0–6.

## Architecture

- `crates/recommender-core/`: Rust data contracts, production catalog/question inputs, feature conversion, scoring, aggregation, and evaluation. Native and `wasm32-unknown-unknown` builds share this crate.
- `crates/recommender-web/`: Leptos browser UI. Website application logic should be Rust; generated Wasm JavaScript glue is expected.
- `web/`: Trunk entry HTML and CSS for the Rust preview.
- `src/index.html`: current production UI and Phase 0 differential reference. Keep the root website as the default through Phase 6; do not include local production-page edits in a preview-only Pages deployment. The isolated Leptos preview uses `recommender-core` directly.
- `src/data/questions.json`: canonical unchanged 12-question source during migration.
- `crates/recommender-core/data/catalog.json`: extracted effective 162-champion inline catalog, preserving source order and duplicate-key overwrite behavior. Do not substitute the enriched external champion JSON.
- `tests/fixtures/migration/`: captured reference outputs and eight synthetic profiles. Fixture changes require an explicit parity review.
- `scripts/migration/`: JavaScript reference harness used only for differential tests, not the target application runtime.
- Python analysis/report tooling is outside the website migration. `docs-archive/` is read-only.

## Parity requirements

Preserve the 12-question ordering and option text, existing feature conversion (including Q6–10 supplying psychological features and Q11–12 being unused), 40/30/30 algorithm weights, stable ranking, top-10 selection, three-per-role first pass and uncapped fallback, current explanations, evaluation heuristic, and serialized localStorage field names. Do not bundle the enriched 171-record champion JSON, repair existing product bugs, redesign the UI, improve the questionnaire, update champion data, or claim trained ML models as part of this migration.

Never copy the exposed Gemini credential into Rust, Wasm, generated JavaScript, Tauri packages, tests, or documentation. Do not activate the dormant AI/report paths. The credential must be revoked or rotated outside this code migration.

## Work and validation

- Keep changes scoped to the current phase and update `RUST_MIGRATION_PLAN.md` with verified status only.
- Run `cargo fmt --all`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` for Rust changes; run reference/Wasm parity checks when scoring, data, or the legacy adapter changes.
- Use `cargo run -p recommender-core --bin lol-scoring-core` for the Rust CLI. Build the Rust preview with `trunk build --release`; its Trunk public URL is repository-subpath-aware.
- Preserve the existing root-site deployment and its Wasm asset path (`public/wasm/lol_scoring_core.wasm`) until cutover is explicitly accepted.
- Treat screenshots, browser storage, and synthetic registration data as user data: use isolated browser storage and avoid real personal information.
