# Analytics & Historical Reference Files

> [!NOTE]
> Per Phase 6 of [RUST_MIGRATION_PLAN.md](../../RUST_MIGRATION_PLAN.md), the active analytics dashboard has been ported to the Rust/Wasm application (`recommender-web` with `recommender-core::analytics`). All five charts (Hero Types, Melee vs Ranged, Resource Types, Difficulty Distribution, and Release Chronology) are rendered using native Rust SVG components without external CDN or Chart.js dependencies.

## Historical HTML Reference Files

The HTML files in this directory are retained as historical reference only until parity acceptance:

- **analytics.html** - Legacy Chart.js analytics dashboard (superseded by Rust/Wasm SVG dashboard in `recommender-web`).
- **debug-analytics.html**, **simple-analytics.html**, **test-analytics.html** - Legacy debugging variants.
- **test_*.html** & **test-data-*.html** - Legacy algorithm test pages (superseded by automated Rust tests in `crates/recommender-core/tests/`).

## Automated Rust Test Coverage

The algorithm verification previously done via standalone HTML test files is now implemented as native Rust tests in `crates/recommender-core/`:

- `crates/recommender-core/tests/analytics.rs`: Verifies exact champion counts, hero types, combat ranges, difficulty tiers, deterministic resource assignment, and release chronology.
- `crates/recommender-core/tests/contracts.rs`: Verifies JSON contract fidelity and storage compatibility.
- `crates/recommender-core/tests/evaluation.rs`: Verifies evaluation metric calculation.
- `crates/recommender-core/tests/parity.rs`: Verifies cross-engine parity and deterministic recommendations.

## Data Disclosures

- **Hero Type, Range Type, Difficulty**: Aggregated directly from `src/data/champions.json` (171 champions).
- **Release Chronology**: Historical LoL release statistics from 2009 launch to 2024.
- **Resource Types**: The original dataset does not catalog champion resource types; in the Rust implementation, resource distribution is deterministically assigned and explicitly labeled as an illustrative visualization dataset.
