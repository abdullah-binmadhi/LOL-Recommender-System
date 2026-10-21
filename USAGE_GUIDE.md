# LoL Champion Recommender System — Usage Guide

## Overview
This guide explains how to use and operate the League of Legends Champion Recommender System, now completely powered by Rust and WebAssembly with Leptos.

## System Requirements
- A modern web browser with WebAssembly support (Chrome, Firefox, Safari, or Edge)
- For local builds: Stable Rust 1.80+ and Trunk (`cargo install trunk --version 0.22.0 --locked`)

## Running Locally

### Option 1: Live Development Server
Start Trunk dev server with hot reload:
```bash
trunk serve
```
Open `http://127.0.0.1:8081/` in your browser.

### Option 2: Build & Static Server
Build release assets and serve with any HTTP server:
```bash
trunk build --release
python3 -m http.server 8080
```
Open `http://127.0.0.1:8080/` (or `http://127.0.0.1:8080/rust-preview/`).

---

## Features & User Flow

### 1. Registration
The quick setup screen captures participant details for recommendation context:
- **Full Name** (required)
- **Email Address** (required)
- **Age** (required, 13–120)
- **Experience Level** (Beginner, Intermediate, Advanced, Expert)
- **Summoner Name** (optional)
- **Preferred Role / Region** (optional)

Data is saved locally in `localStorage` under `championRecommenderUsers`.

### 2. Interactive Questionnaire
- 12 comprehensive questions covering mechanical playstyle, game tempo, combat distance, and psychological decision-making.
- Smart role-based question filtering dynamically restricts position and damage options based on your selected primary class.
- Full keyboard accessibility: navigate options with `Tab` and select with `Enter` or `Space`.

### 3. Detailed ML Analysis & Recommendations
- Ranked Top 10 champions calculated via ensemble weighting across K-Nearest Neighbors, Decision Trees, and Random Forests.
- Detailed metrics card showing Precision@1, Precision@3, Precision@10, Recall@10, F1-Score@10, and Mean Reciprocal Rank.
- Click any champion card to open the Champion Deep-Dive Modal with abilities, matchups, and individual algorithm contributions.
- Click any metric card to inspect mathematical definitions and confidence breakdowns.

### 4. Champion Analytics Dashboard
- Navigate to the Analytics Dashboard from the landing screen or via `#analytics` hash.
- Pure Rust SVG visualizations (no external JavaScript libraries):
  - **Hero Types**: Character archetype distribution across 171 champions.
  - **Combat Distance**: Melee vs. Ranged combat ratio.
  - **Resource Types**: Deterministically partitioned with explicit data disclosure.
  - **Difficulty Curve**: Low, Medium, and High mechanical complexity tiers.
  - **Release Chronology**: Historical LoL champion launches from 2009 to 2024.

### 5. Shortcuts & Exports
- **CSV Data Export (`Ctrl+Shift+E`)**: Downloads `champion_recommender_users.csv` with 26-column user demographic and quiz records.
- **Enhanced Admin Panel (`Ctrl+Shift+A`)**: Opens confirmation dialog for administrative record inspection.
- **Save Table (`Save` button)**: Inspects saved user records and exports table CSV (`champion_recommendations.csv`).
- **Modal Close (`Escape`)**: Instantly closes any active dialog modal.
