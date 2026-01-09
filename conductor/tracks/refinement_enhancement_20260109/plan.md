# Track Plan: Refinement and Enhancement

## Phase 1: Data Analysis & Feature Engineering
**Goal:** Audit the current dataset and introduce new derived features to improve recommendation granularity.

- [x] Task: Audit `champions.json` for data completeness and relevance.
    - [ ] Sub-task: Write script to check for missing values or outdated stats.
    - [ ] Sub-task: Identify key missing features (e.g., damage type, power spikes).
- [x] Task: Implement `FeatureExtractor` class in JavaScript.
    - [ ] Sub-task: Write tests for `FeatureExtractor` (TDD).
    - [ ] Sub-task: Implement logic to calculate derived stats (Burst, Sustain, CC).
- [x] Task: Update `champions.json` with new derived features.
    - [ ] Sub-task: Run extraction script to populate new fields.
    - [ ] Sub-task: Verify data integrity of the updated JSON.
- [ ] Task: Conductor - User Manual Verification 'Data Analysis & Feature Engineering' (Protocol in workflow.md)

## Phase 2: Algorithm Refinement
**Goal:** Optimize the ensemble weighting and individual algorithm parameters based on the new features.

- [ ] Task: Create a validation test suite for the ensemble model.
    - [ ] Sub-task: Define a set of "ground truth" user profiles and expected recommendations.
    - [ ] Sub-task: Write automated tests to measure current accuracy.
- [ ] Task: Refine K-Nearest Neighbors (KNN) distance metric.
    - [ ] Sub-task: Write tests for weighted distance calculation.
    - [ ] Sub-task: Update KNN to use new features (Burst, Sustain, etc.) with appropriate weights.
- [ ] Task: Optimize Ensemble Voting Weights.
    - [ ] Sub-task: Create a utility to test different weight combinations (e.g., RF: 0.3, DT: 0.3, KNN: 0.4).
    - [ ] Sub-task: Select and hardcode the optimal weights based on validation results.
- [ ] Task: Conductor - User Manual Verification 'Algorithm Refinement' (Protocol in workflow.md)

## Phase 3: UI Integration & Visualization
**Goal:** Surface the new data and improved logic to the user through the interface.

- [ ] Task: Update Recommendation Cards to display new features.
    - [ ] Sub-task: Write component tests for the updated card display.
    - [ ] Sub-task: Update HTML/CSS to show "Damage Type" and "Power Spike" badges.
- [ ] Task: enhance "Why this champion?" explanation.
    - [ ] Sub-task: Update the explanation generation logic to reference specific matching features (e.g., "Matches your preference for High Sustain").
- [ ] Task: Conductor - User Manual Verification 'UI Integration & Visualization' (Protocol in workflow.md)
