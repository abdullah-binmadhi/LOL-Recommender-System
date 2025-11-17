# Implementation Tasks: Unified ML Recommendations

## Phase 1: Modify ML Algorithm Classes

- [ ] 1. Update SimpleRandomForest class
  - [ ] 1.1 Add predictAll() method
    - Accept features parameter
    - Loop through all champions in allChampions
    - Calculate score for each champion
    - Return object with champion names as keys, scores as values
    - _Requirements: 2.1, 3.1_
  
  - [ ] 1.2 Add calculateChampionScore() method
    - Calculate role match score (40 points)
    - Calculate position match score (30 points)
    - Calculate difficulty match score (20 points)
    - Calculate damage match score (15 points)
    - Calculate toughness match score (15 points)
    - Calculate playstyle bonus (20 points)
    - Calculate psychological match (30 points)
    - Track contributing factors for transparency
    - Track matched criteria
    - Track penalties applied
    - Return score object with details
    - _Requirements: 2.1, 3.1, 3.3, 3.4_
  
  - [ ] 1.3 Add normalizeScore() method
    - Accept raw score
    - Normalize to 0-100 range
    - Ensure min is 0, max is 100
    - Return normalized score
    - _Requirements: 2.5, 3.2_
  
  - [ ] 1.4 Keep existing predict() method for backward compatibility
    - Do not modify existing predict() method
    - _Requirements: N/A (compatibility)_

- [ ] 2. Update SimpleDecisionTree class
  - [ ] 2.1 Add predictAll() method
    - Same structure as RandomForest.predictAll()
    - _Requirements: 2.2, 3.1_
  
  - [ ] 2.2 Add calculateChampionScore() method
    - Use hierarchical scoring logic
    - Level 1: Role match (50 points)
    - Level 2: Position match (40 points, only if role matches)
    - Level 3: Difficulty match (30 points, only if position matches)
    - Level 4: Attribute matches (20 points each)
    - Add psychological bonuses (25 points each)
    - Apply penalties for mismatches
    - Track details for transparency
    - _Requirements: 2.2, 3.1, 3.3, 3.4_
  
  - [ ] 2.3 Add normalizeScore() method
    - Normalize to 0-100 range
    - Account for negative penalties
    - _Requirements: 2.5, 3.2_
  
  - [ ] 2.4 Keep existing predict() method
    - _Requirements: N/A (compatibility)_

- [ ] 3. Update SimpleKNN class
  - [ ] 3.1 Add predictAll() method
    - Same structure as other algorithms
    - _Requirements: 2.3, 3.1_
  
  - [ ] 3.2 Add calculateChampionScore() method
    - Calculate distance-based scoring
    - Role distance (8 points penalty)
    - Position distance (6 points penalty)
    - Numerical attribute distances (weighted)
    - Psychological distances
    - Convert distance to score (inverse)
    - Track details
    - _Requirements: 2.3, 3.1, 3.3, 3.4_
  
  - [ ] 3.3 Add normalizeScore() method
    - Convert distance to 0-100 score
    - Lower distance = higher score
    - _Requirements: 2.5, 3.2_
  
  - [ ] 3.4 Keep existing predict() method
    - _Requirements: N/A (compatibility)_

## Phase 2: Create Score Aggregation System

- [ ] 4. Create ScoreAggregator class
  - [ ] 4.1 Implement aggregateScores() static method
    - Accept rfScores, dtScores, knnScores parameters
    - Create aggregated object for all champions
    - For each champion, store RF, DT, KNN scores
    - Calculate average score
    - Calculate weighted score (optional)
    - Store details from each algorithm
    - Return aggregated object
    - _Requirements: 1.3, 2.1, 2.2, 2.3_
  
  - [ ] 4.2 Implement calculateWeightedScore() static method
    - Accept RF, DT, KNN scores
    - Apply weights (e.g., RF: 40%, DT: 30%, KNN: 30%)
    - Return weighted score
    - _Requirements: 1.3_
  
  - [ ] 4.3 Implement selectTop5() static method
    - Accept aggregated scores object
    - Convert to array
    - Sort by average score (descending)
    - Apply diversity filter if enabled
    - Select top 5 champions
    - Ensure all are unique
    - Return array of 5 champion objects
    - _Requirements: 1.1, 1.2, 1.5_
  
  - [ ] 4.4 Implement applyDiversityFilter() static method
    - Accept champions array
    - Limit to max 2 champions per role
    - Maintain score-based ranking
    - Fill remaining slots with highest scores
    - Return filtered array
    - _Requirements: 1.5_

## Phase 3: Update Main Recommendation Flow

- [ ] 5. Modify generateRecommendations() function
  - [ ] 5.1 Extract user features
    - Use existing extractFeatures() function
    - _Requirements: N/A (existing)_
  
  - [ ] 5.2 Run all 3 algorithms with predictAll()
    - Call algorithms['random-forest'].predictAll(features)
    - Call algorithms['decision-tree'].predictAll(features)
    - Call algorithms['knn'].predictAll(features)
    - Store results in rfScores, dtScores, knnScores
    - _Requirements: 2.1, 2.2, 2.3_
  
  - [ ] 5.3 Aggregate scores
    - Call ScoreAggregator.aggregateScores()
    - Pass all 3 score objects
    - Store result in aggregated variable
    - _Requirements: 1.3_
  
  - [ ] 5.4 Select top 5 champions
    - Call ScoreAggregator.selectTop5()
    - Pass aggregated scores
    - Store result in top5 variable
    - _Requirements: 1.1, 1.2_
  
  - [ ] 5.5 Update mlResults global object
    - Store scores object (RF, DT, KNN)
    - Store aggregated object
    - Store top5 array
    - _Requirements: 2.1, 2.2, 2.3_
  
  - [ ] 5.6 Call display function
    - Call displayUnifiedRecommendations(top5)
    - _Requirements: 4.1_

## Phase 4: Create UI Components

- [ ] 6. Add CSS styles for unified recommendations
  - [ ] 6.1 Add .unified-recommendation-card styles
    - Background, border-radius, padding, margin
    - Box shadow
    - _Requirements: 4.2_
  
  - [ ] 6.2 Add .champion-header styles
    - Flexbox layout
    - Alignment and spacing
    - Border bottom
    - _Requirements: 4.1_
  
  - [ ] 6.3 Add .aggregate-score styles
    - Gradient background
    - Text styling
    - Positioning
    - _Requirements: 4.4_
  
  - [ ] 6.4 Add .ml-scores-section styles
    - Margin and padding
    - _Requirements: 4.1_
  
  - [ ] 6.5 Add .score-row styles
    - Grid layout (4 columns)
    - Alignment
    - Border bottom
    - _Requirements: 4.1_
  
  - [ ] 6.6 Add .score-bar-container and .score-bar styles
    - Container: height, background, border-radius
    - Bar: height, gradient, transition
    - Different colors for RF, DT, KNN
    - _Requirements: 4.2, 4.3_
  
  - [ ] 6.7 Add .details-btn styles
    - Button styling
    - Hover effects
    - _Requirements: 5.1_
  
  - [ ] 6.8 Add .calculation-details styles
    - Expandable section styling
    - Table styles for factors
    - List styles for criteria
    - _Requirements: 5.2, 5.3, 5.4_

- [ ] 7. Create displayUnifiedRecommendations() function
  - [ ] 7.1 Clear existing results container
    - Get results container element
    - Clear innerHTML
    - _Requirements: 4.1_
  
  - [ ] 7.2 Loop through top 5 champions
    - For each champion in mlResults.top5
    - _Requirements: 1.1_
  
  - [ ] 7.3 Create champion card HTML for each
    - Champion header with image, name, role
    - Aggregate score display
    - ML scores section with 3 score rows
    - Each row: algorithm name, score bar, percentage, details button
    - Expandable details section (initially hidden)
    - _Requirements: 4.1, 4.2, 4.3, 4.4_
  
  - [ ] 7.4 Append cards to container
    - Add each card to results container
    - _Requirements: 4.1_
  
  - [ ] 7.5 Animate score bars
    - Use setTimeout to animate bars after render
    - _Requirements: 4.2_

- [ ] 8. Create showCalculationDetails() function
  - [ ] 8.1 Accept algorithm and championName parameters
    - _Requirements: 5.1_
  
  - [ ] 8.2 Retrieve champion details
    - Find champion in mlResults.top5
    - Get details for specified algorithm
    - _Requirements: 5.2_
  
  - [ ] 8.3 Build details HTML
    - Score summary (raw and normalized)
    - Contributing factors table
    - Matched criteria list
    - Penalties list (if any)
    - _Requirements: 5.2, 5.3, 5.4, 5.5_
  
  - [ ] 8.4 Display details
    - Insert HTML into details section
    - Show details section
    - _Requirements: 5.1_
  
  - [ ] 8.5 Add close button
    - Allow user to collapse details
    - _Requirements: 5.1_

## Phase 5: Update Evaluation Metrics

- [ ] 9. Modify EvaluationMetrics class
  - [ ] 9.1 Update getRecommendedChampions() method
    - Accept mlResults parameter
    - Extract top 5 champion names from mlResults.top5
    - Return array of 5 champion names
    - Maintain order (highest score first)
    - _Requirements: 1.1_
  
  - [ ] 9.2 Update calculateUserRelevance() method (if needed)
    - Ensure it works with 5 recommendations
    - _Requirements: 1.4_
  
  - [ ] 9.3 Update displayEvaluationMetrics() function
    - Update text to reflect "Top 5" instead of "Top 3"
    - Adjust Precision@K calculations if needed
    - _Requirements: 1.1_

## Phase 6: Testing and Validation

- [ ] 10. Unit tests
  - [ ] 10.1 Test score normalization
    - Test with various raw scores
    - Verify all results are 0-100
    - _Requirements: 2.5, 3.2_
  
  - [ ] 10.2 Test aggregation logic
    - Test with sample scores
    - Verify average calculation
    - Verify weighted calculation
    - _Requirements: 1.3_
  
  - [ ] 10.3 Test top 5 selection
    - Verify exactly 5 champions returned
    - Verify all are unique
    - Verify correct ordering
    - _Requirements: 1.1, 1.2_
  
  - [ ] 10.4 Test diversity filter
    - Verify role distribution
    - Verify no more than 2 per role
    - _Requirements: 1.5_

- [ ] 11. Integration tests
  - [ ] 11.1 Test full recommendation flow
    - Test with various user preferences
    - Verify 5 champions returned
    - Verify all have 3 scores
    - _Requirements: 1.1, 2.1, 2.2, 2.3_
  
  - [ ] 11.2 Test with edge cases
    - Test with "No Preference" for all questions
    - Test with very specific preferences
    - Test with conflicting preferences
    - _Requirements: 1.1, 1.5_
  
  - [ ] 11.3 Test score accuracy
    - Manually verify scores for sample champions
    - Check calculation logic
    - Verify transparency of calculations
    - _Requirements: 3.1, 3.2, 3.3_

- [ ] 12. Visual tests
  - [ ] 12.1 Test champion cards display
    - Verify layout on desktop
    - Verify layout on tablet
    - Verify layout on mobile
    - _Requirements: 4.1, 4.2, 4.5_
  
  - [ ] 12.2 Test score bars
    - Verify bars display correctly
    - Verify colors are distinct
    - Verify animation works
    - _Requirements: 4.2, 4.3_
  
  - [ ] 12.3 Test expandable details
    - Click details button
    - Verify details display
    - Verify close functionality
    - _Requirements: 5.1, 5.2_
  
  - [ ] 12.4 Test responsive design
    - Test on various screen sizes
    - Verify no layout breaks
    - _Requirements: 4.5_

## Phase 7: Performance Optimization

- [ ] 13. Implement caching
  - [ ] 13.1 Add score caching
    - Cache algorithm scores
    - Use feature hash as key
    - _Requirements: Performance_
  
  - [ ] 13.2 Add details caching
    - Cache calculation details
    - Load on demand
    - _Requirements: Performance_

- [ ] 14. Optimize calculations
  - [ ] 14.1 Profile algorithm performance
    - Measure time for predictAll()
    - Identify bottlenecks
    - _Requirements: Performance_
  
  - [ ] 14.2 Optimize loops
    - Reduce unnecessary iterations
    - Use efficient data structures
    - _Requirements: Performance_

## Phase 8: Documentation and Deployment

- [ ] 15. Update documentation
  - [ ] 15.1 Update README
    - Document new recommendation system
    - Add examples
    - _Requirements: N/A_
  
  - [ ] 15.2 Add code comments
    - Comment new functions
    - Explain scoring logic
    - _Requirements: 3.5, 5.5_

- [ ] 16. Deploy to production
  - [ ] 16.1 Test on staging
    - Full end-to-end test
    - _Requirements: All_
  
  - [ ] 16.2 Deploy to production
    - Push to GitHub
    - Deploy to Vercel
    - _Requirements: All_
  
  - [ ] 16.3 Monitor for issues
    - Check error logs
    - Monitor user feedback
    - _Requirements: All_

---

**Total Tasks**: 16 main tasks, 60+ subtasks
**Estimated Time**: 9-12 hours
**Priority**: High
**Complexity**: High
