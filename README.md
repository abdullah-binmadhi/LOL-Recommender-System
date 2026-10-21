# League of Legends Champion Recommender (Rust & WebAssembly)

An AI-powered champion recommendation system that uses three ensemble machine learning algorithms (KNN, Decision Tree, and Random Forest) to suggest the perfect League of Legends champions based on your playstyle preferences and psychological profile. Achieving 93.8% Precision@1 and 69.5% F1-Score@10, this system provides highly accurate, deterministic, data-driven recommendations.

**Live Demo:** [https://abdullah-binmadhi.github.io/LOL-Recommender-System/](https://abdullah-binmadhi.github.io/LOL-Recommender-System/)

---

## Architecture & Migration Status

The system has completed full migration to Rust and WebAssembly, fulfilling all 7 phases defined in [`RUST_MIGRATION_PLAN.md`](RUST_MIGRATION_PLAN.md).

- **`crates/recommender-core`**: Pure offline Rust library containing:
  - Strongly typed contracts and questionnaire evaluation.
  - Three parity-verified machine learning engines (K-Nearest Neighbors, Decision Tree, and Random Forest ensemble).
  - Evaluation metric engine (Precision@K, Recall@K, F1@K, Mean Reciprocal Rank).
  - Analytics calculation engine for all 171 champions.
  - 26-column and 8-column CSV exporters.
- **`crates/recommender-web`**: Leptos WebAssembly application:
  - Reactive questionnaire interface with role-based question filtering.
  - Detailed ML analysis with top 10 ranked recommendations and algorithm score breakdowns.
  - Interactive champion details modals and evaluation metric explanation dialogs.
  - Pure Rust SVG Champion Analytics Dashboard (Zero external Chart.js / CDN dependencies).
  - Browser local storage persistence (`championRecommenderUsers` and `recentChampions`).
  - Keyboard shortcuts: `Ctrl+Shift+E` (CSV export), `Ctrl+Shift+A` (Admin prompt), and `Escape` (Close modal).
- **`web/`**: HTML/CSS web assets and styles for Trunk packaging.
- **`rust-preview/`**: Release WebAssembly distribution.

---

## Quick Start & Local Development

### Prerequisites
- Current stable Rust toolchain (`rustup default stable`)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Trunk bundler: `cargo install trunk --version 0.22.0 --locked`

### Run Locally with Trunk
From the repository root:
```sh
trunk serve
```
Open `http://127.0.0.1:8081/` in your browser.

### Build Production Release
```sh
trunk build --release
```
Compiles optimized WebAssembly and static site assets directly into `rust-preview/`.

### Run Test Suites
```sh
# Run all workspace unit and integration tests (53 tests)
cargo test --workspace

# Lint with Clippy (enforcing 0 warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Verify formatting
cargo fmt --all -- --check
```

---

## Offline CLI Scoring Engine

You can run deterministic scoring directly from the command line:

```sh
printf '%s' '{"1":"Medium (4-6)","2":"Mage","3":"Mid","4":"High Damage Output"}' | cargo run --locked --quiet -p recommender-core --bin lol-scoring-core
```

Input is a JSON object mapping question IDs (`"1"`–`"12"`) to answer strings. The CLI outputs a complete JSON result with individual algorithm scores, ensemble weights, ranked top 10 recommendations, and evaluation metrics.
