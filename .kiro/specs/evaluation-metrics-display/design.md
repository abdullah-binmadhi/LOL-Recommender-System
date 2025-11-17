# Design Document

## Overview

This design implements evaluation metrics display on the detailed champion analysis page. The solution adds a JavaScript implementation of Precision@K and MRR calculations, integrates them into the existing `showDetailedAnalysis()` function, and displays the results in a visually appealing metrics section.

## Architecture

### Component Structure

```
src/index.html
├── JavaScript Functions
│   ├── EvaluationMetrics class (new)
│   │   ├── precisionAtK()
│   │   ├── meanReciprocalRank()
│   │   ├── calculateUserRelevance()
│   │   └── getRecommendedChampions()
│   ├── showDetailedAnalysis() (modified)
│   │   └── calls displayEvaluationMetrics()
│   └── displayEvaluationMetrics() (new)
└── CSS Styles (new)
    └── .evaluation-metrics-section
```

### Data Flow

1. User completes questionnaire → `runAllAlgorithms()` → `displayResults()`
2. `displayResults()` → `showDetailedAnalysis()`
3. `showDetailedAnalysis()` collects:
   - User answers from `answers` object
   - ML results from `mlResults` object
   - Champion data from `allChampions` object
4. `EvaluationMetrics.calculateUserRelevance()` determines relevant champions
5. `EvaluationMetrics.getRecommendedChampions()` extracts recommendations
6. Calculate Precision@1, @3, @5 and MRR
7. `displayEvaluationMetrics()` renders metrics section
8. Metrics section inserted into `#analysis-content` div

## Components and Interfaces

### EvaluationMetrics Class

```javascript
class EvaluationMetrics {
    static precisionAtK(recommendedChampions, relevantChampions, k) {
        // Returns: float (0.0 to 1.0)
        // Calculates proportion of relevant champions in top K
    }
    
    static meanReciprocalRank(recommendedChampions, relevantChampions) {
        // Returns: float (0.0 to 1.0)
        // Calculates 1/rank of first relevant champion
    }
    
    static calculateUserRelevance(userAnswers, allChampions) {
        // Returns: Set<string> - champion names
        // Determines relevant champions based on user preferences
        // Matches: role, difficulty (±2), damage (±2), toughness (±2)
    }
    
    static getRecommendedChampions(mlResults) {
        // Returns: Array<string> - champion names sorted by confidence
        // Extracts all recommended champions from ML results
    }
}
```

### displayEvaluationMetrics Function

```javascript
function displayEvaluationMetrics(metrics, relevantCount, recommendedCount) {
    // Parameters:
    //   metrics: { p1, p3, p5, mrr }
    //   relevantCount: number of relevant champions
    //   recommendedCount: number of recommended champions
    // Returns: HTML string
    // Creates styled metrics section with:
    //   - Header with icon
    //   - Metric cards (4 cards: P@1, P@3, P@5, MRR)
    //   - Performance interpretation
    //   - Tooltips/descriptions
}
```

### Integration Point

Modify `showDetailedAnalysis()` function:
- After generating champion match analysis
- Before final tips section
- Insert metrics section into content string

## Data Models

### Metrics Object

```javascript
{
    precision_at_1: float,      // 0.0 to 1.0
    precision_at_3: float,      // 0.0 to 1.0
    precision_at_5: float,      // 0.0 to 1.0
    mean_reciprocal_rank: float // 0.0 to 1.0
}
```

### Relevant Champions Determination

Champions are considered relevant if they match:
- **Role**: Exact match OR user selected "No Preference"
- **Difficulty**: Within ±2 points of user preference
- **Damage**: Within ±2 points of user preference (if specified)
- **Toughness**: Within ±2 points of user preference (if specified)

Mapping user answers to numeric values:
- Difficulty: "Easy (1-3)" → 2, "Medium (4-6)" → 5, "Hard (7-8)" → 7.5, "Very Hard (9-10)" → 9.5
- Playstyle to damage/toughness:
  - "High Damage Output" → damage: 8, toughness: 3
  - "Tanky and Durable" → damage: 4, toughness: 8
  - "Support Team" → damage: 3, toughness: 5
  - "Balanced/Hybrid" → damage: 5, toughness: 5

## Error Handling

### Missing Data
- If `answers` object is incomplete: Use default values (role: "No Preference", difficulty: 5)
- If `mlResults` is empty: Display message "Metrics unavailable"
- If no relevant champions found: Display "0 relevant champions" with explanation

### Calculation Errors
- Wrap all metric calculations in try-catch blocks
- If calculation fails: Display "N/A" for that metric
- Log errors to console for debugging

### Display Errors
- If metrics section fails to render: Continue showing rest of analysis
- Graceful degradation: Page remains functional without metrics

## Testing Strategy

### Unit Tests (Manual)
1. Test `precisionAtK()` with known inputs:
   - Recommended: ["Ahri", "Lux", "Zed"], Relevant: {"Ahri", "Lux"}, K=3 → 0.667
   - Recommended: ["Garen"], Relevant: {"Ahri"}, K=1 → 0.0
   - Recommended: [], Relevant: {"Ahri"}, K=3 → 0.0

2. Test `meanReciprocalRank()` with known inputs:
   - Recommended: ["Ahri", "Lux"], Relevant: {"Ahri"} → 1.0
   - Recommended: ["Zed", "Ahri"], Relevant: {"Ahri"} → 0.5
   - Recommended: ["Zed", "Lux"], Relevant: {"Ahri"} → 0.0

3. Test `calculateUserRelevance()`:
   - User wants "Mage", difficulty 5 → Should include Ahri, Lux, etc.
   - User wants "No Preference", difficulty 2 → Should include easy champions

### Integration Tests
1. Complete questionnaire with specific preferences
2. Verify metrics section appears on detailed analysis page
3. Verify metrics values are reasonable (0.0 to 1.0)
4. Verify visual styling matches existing page design

### Visual Tests
1. Check metrics section positioning
2. Verify color coding (green/yellow/red)
3. Test responsive design on mobile
4. Verify tooltips/descriptions are readable

## CSS Styling

### New Styles

```css
.evaluation-metrics-section {
    background: linear-gradient(135deg, #e8f5e9 0%, #c8e6c9 100%);
    border-radius: 15px;
    padding: 30px;
    margin: 30px 0;
}

.metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 20px;
}

.metric-card {
    background: white;
    border-radius: 10px;
    padding: 20px;
    text-align: center;
    box-shadow: 0 3px 10px rgba(0,0,0,0.1);
}

.metric-value {
    font-size: 2rem;
    font-weight: bold;
    margin: 10px 0;
}

.metric-value.excellent { color: #2e7d32; }
.metric-value.good { color: #f57c00; }
.metric-value.poor { color: #c62828; }

.metric-label {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 5px;
}

.metric-description {
    font-size: 0.8rem;
    color: #999;
    margin-top: 10px;
}

.performance-interpretation {
    background: white;
    border-radius: 10px;
    padding: 20px;
    margin-top: 20px;
    border-left: 4px solid #2e7d32;
}
```

## Implementation Notes

### Performance Considerations
- Metrics calculation is O(n) where n = number of champions (~50)
- Calculation happens once per session, minimal performance impact
- No external API calls required

### Browser Compatibility
- Uses ES6 classes and Set data structure
- Compatible with modern browsers (Chrome, Firefox, Safari, Edge)
- No polyfills required for target audience

### Maintainability
- Metrics logic isolated in EvaluationMetrics class
- Easy to add new metrics in the future
- Clear separation between calculation and display logic

### Future Enhancements
- Add more metrics (NDCG, MAP)
- Store metrics history in localStorage
- Compare metrics across sessions
- Export metrics with CSV data
