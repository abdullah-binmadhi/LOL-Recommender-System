# Unified ML Recommendations - Implementation Plan

## Current System vs. Desired System

### Current System ❌
```
User completes questionnaire
         ↓
┌────────────────────────────────────┐
│  Run 3 ML Algorithms               │
│  - Random Forest → Champion A (85%)│
│  - Decision Tree → Champion B (78%)│
│  - KNN → Champion C (82%)          │
└────────────────────────────────────┘
         ↓
Display 3 different champions
(one from each algorithm)
```

### Desired System ✅
```
User completes questionnaire
         ↓
┌────────────────────────────────────┐
│  Run 3 ML Algorithms               │
│  Each scores ALL champions         │
│  - Random Forest → Scores for all  │
│  - Decision Tree → Scores for all  │
│  - KNN → Scores for all            │
└────────────────────────────────────┘
         ↓
┌────────────────────────────────────┐
│  Aggregate Scores                  │
│  Combine scores from all 3         │
│  algorithms for each champion      │
└────────────────────────────────────┘
         ↓
┌────────────────────────────────────┐
│  Select Top 5 Unique Champions     │
│  Based on aggregated scores        │
└────────────────────────────────────┘
         ↓
Display 5 champions, each showing:
- Champion name
- Random Forest score (%)
- Decision Tree score (%)
- KNN score (%)
- Average/aggregate score (%)
```

## Required Changes

### 1. Modify ML Algorithm Classes

**Current**: Each algorithm returns ONE champion
```javascript
predict(features) {
    // ... calculations ...
    return {
        champion: "Ahri",
        confidence: 85,
        algorithm: "Random Forest"
    };
}
```

**New**: Each algorithm returns scores for ALL champions
```javascript
predictAll(features) {
    const scores = {};
    for (const [name, champion] of Object.entries(allChampions)) {
        scores[name] = this.calculateScore(features, champion);
    }
    return scores; // { "Ahri": 85.3, "Lux": 78.2, ... }
}
```

### 2. Create Aggregation Function

```javascript
function aggregateMLScores(rfScores, dtScores, knnScores) {
    const aggregated = {};
    
    for (const championName of Object.keys(allChampions)) {
        aggregated[championName] = {
            randomForest: rfScores[championName] || 0,
            decisionTree: dtScores[championName] || 0,
            knn: knnScores[championName] || 0,
            average: (rfScores[championName] + dtScores[championName] + knnScores[championName]) / 3
        };
    }
    
    return aggregated;
}
```

### 3. Select Top 5 Champions

```javascript
function getTop5Champions(aggregatedScores) {
    const champions = Object.entries(aggregatedScores)
        .map(([name, scores]) => ({
            name,
            ...scores
        }))
        .sort((a, b) => b.average - a.average)
        .slice(0, 5);
    
    return champions;
}
```

### 4. Update Display Logic

**Current**: Shows 3 separate champion cards
**New**: Shows 5 champion cards, each with 3 ML scores

```html
<div class="champion-card">
    <h3>Ahri</h3>
    <div class="ml-scores">
        <div class="score-item">
            <span>Random Forest:</span>
            <span class="score">85.3%</span>
            <div class="score-bar" style="width: 85.3%"></div>
        </div>
        <div class="score-item">
            <span>Decision Tree:</span>
            <span class="score">78.2%</span>
            <div class="score-bar" style="width: 78.2%"></div>
        </div>
        <div class="score-item">
            <span>KNN:</span>
            <span class="score">82.1%</span>
            <div class="score-bar" style="width: 82.1%"></div>
        </div>
        <div class="score-item aggregate">
            <span>Average:</span>
            <span class="score">81.9%</span>
        </div>
    </div>
</div>
```

## Implementation Steps

### Phase 1: Modify ML Algorithms
1. Add `predictAll()` method to SimpleRandomForest
2. Add `predictAll()` method to SimpleDecisionTree
3. Add `predictAll()` method to SimpleKNN
4. Ensure scores are normalized to 0-100 range
5. Keep existing `predict()` for backward compatibility

### Phase 2: Create Aggregation System
1. Create `aggregateMLScores()` function
2. Create `getTop5Champions()` function
3. Add scoring transparency (show calculation details)
4. Implement score normalization

### Phase 3: Update UI
1. Modify champion display cards
2. Add ML score visualization (bars/charts)
3. Add expandable calculation details
4. Update CSS for new layout
5. Ensure responsive design

### Phase 4: Update Evaluation Metrics
1. Modify EvaluationMetrics class to work with new system
2. Update `getRecommendedChampions()` to return top 5
3. Recalculate Precision@K and MRR with 5 recommendations

### Phase 5: Testing
1. Test with various user preferences
2. Verify score accuracy
3. Ensure no duplicate champions
4. Test visual display
5. Validate calculations

## Benefits

✅ **More Options**: Users get 5 champions instead of 3
✅ **Transparency**: Users see how each algorithm scored each champion
✅ **Better Decisions**: Users can choose based on multiple ML opinions
✅ **Accuracy**: All calculations are visible and verifiable
✅ **Diversity**: More variety in recommendations

## Challenges

⚠️ **Performance**: Calculating scores for all champions (168) x 3 algorithms
⚠️ **Complexity**: More complex UI to show all scores
⚠️ **Testing**: Need to verify accuracy of all calculations
⚠️ **Backward Compatibility**: Existing code may depend on current structure

## Estimated Effort

- **Phase 1**: 2-3 hours (ML algorithm modifications)
- **Phase 2**: 1-2 hours (Aggregation system)
- **Phase 3**: 3-4 hours (UI updates)
- **Phase 4**: 1 hour (Evaluation metrics)
- **Phase 5**: 2 hours (Testing)

**Total**: 9-12 hours of development

## Next Steps

1. Review and approve this plan
2. Create detailed design document
3. Implement Phase 1 (ML algorithms)
4. Test Phase 1 before proceeding
5. Continue with remaining phases

---

**Status**: 📋 Planning Phase  
**Priority**: High  
**Complexity**: High  
**Impact**: High (Major system change)

Would you like me to proceed with implementation?
