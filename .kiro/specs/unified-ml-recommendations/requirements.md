# Requirements Document: Unified ML Recommendations System

## Introduction

This feature transforms the recommendation system from showing 3 champions (one per ML algorithm) to showing 5 unique champions, each evaluated by all 3 ML algorithms with transparent scoring.

## Glossary

- **ML Algorithm**: Machine Learning algorithm (Random Forest, Decision Tree, KNN)
- **Champion Score**: Numerical confidence/probability score from an ML algorithm for a specific champion
- **Unified Recommendation**: A single champion evaluated by all 3 ML algorithms
- **Top-5 List**: The 5 highest-scoring unique champions across all algorithms

## Requirements

### Requirement 1: Unified Scoring System

**User Story:** As a user, I want to see 5 recommended champions instead of 3, so that I have more options to choose from.

#### Acceptance Criteria

1. THE System SHALL generate exactly 5 unique champion recommendations
2. THE System SHALL ensure no champion appears more than once in the top-5 list
3. THE System SHALL rank champions by aggregated score across all 3 ML algorithms
4. THE System SHALL display champions in descending order of total score
5. THE System SHALL maintain diversity in recommendations

### Requirement 2: Multi-Algorithm Evaluation

**User Story:** As a user, I want to see how each ML algorithm scored each recommended champion, so that I understand why each champion was recommended.

#### Acceptance Criteria

1. WHEN a champion is recommended, THE System SHALL display the Random Forest score
2. WHEN a champion is recommended, THE System SHALL display the Decision Tree score
3. WHEN a champion is recommended, THE System SHALL display the KNN score
4. THE System SHALL display all scores as percentages with one decimal place
5. THE System SHALL ensure all scores are between 0.0% and 100.0%

### Requirement 3: Transparent ML Calculations

**User Story:** As a user, I want to see accurate ML calculations for each champion, so that I can trust the recommendations.

#### Acceptance Criteria

1. THE System SHALL calculate actual scores from each ML algorithm for all champions
2. THE System SHALL NOT use simulated or random scores
3. THE System SHALL display the calculation methodology for each algorithm
4. THE System SHALL show which factors contributed to each score
5. THE System SHALL ensure mathematical accuracy in all calculations

### Requirement 4: Visual Presentation

**User Story:** As a user, I want to see a clear comparison of ML scores for each champion, so that I can easily understand the recommendations.

#### Acceptance Criteria

1. THE System SHALL display each champion with all 3 ML scores side-by-side
2. THE System SHALL use visual indicators (bars, colors) to represent score magnitudes
3. THE System SHALL highlight the highest-scoring algorithm for each champion
4. THE System SHALL show an aggregate/average score for each champion
5. THE System SHALL maintain consistent styling with the existing design

### Requirement 5: Algorithm Details

**User Story:** As a developer/user, I want to understand how each algorithm calculated its scores, so that I can verify the recommendation logic.

#### Acceptance Criteria

1. THE System SHALL provide expandable details for each algorithm's calculation
2. THE System SHALL show which features were weighted most heavily
3. THE System SHALL display intermediate calculation steps
4. THE System SHALL explain why a champion scored high or low
5. THE System SHALL make all calculations auditable and verifiable

