# Design Document: Unified ML Recommendations System

## Overview

This document provides the technical design for transforming the recommendation system from showing 3 champions (one per ML algorithm) to showing 5 unique champions, each evaluated by all 3 ML algorithms with transparent scoring.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    USER COMPLETES QUESTIONNAIRE                  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    FEATURE EXTRACTION                            │
│  - Extract user preferences (role, difficulty, playstyle, etc.) │
│  - Map to numerical features                                     │
│  - Create feature vector                                         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    ML ALGORITHM SCORING                          │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Random Forest.predictAll(features)                       │  │
│  │  → { "Ahri": 85.3, "Lux": 78.2, "Zed": 72.1, ... }      │  │
│  └───────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Decision Tree.predictAll(features)                       │  │
│  │  → { "Ahri": 78.2, "Lux": 84.1, "Zed": 69.5, ... }      │  │
│  └───────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  KNN.predictAll(features)                                 │  │
│  │  → { "Ahri": 82.1, "Lux": 77.8, "Zed": 75.3, ... }      │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    SCORE AGGREGATION                             │
│  Combine scores for each champion:                              │
│  {                                                               │
│    "Ahri": {                                                     │
│      randomForest: 85.3, decisionTree: 78.2, knn: 82.1,        │
│      average: 81.9, weighted: 82.5                              │
│    },                                                            │
│    "Lux": { ... },                                              │
│    ...                                                           │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    TOP 5 SELECTION                               │
│  - Sort by aggregate score (descending)                          │
│  - Select top 5 unique champions                                 │
│  - Apply diversity filters                                       │
│  - Ensure no duplicates                                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    DISPLAY RESULTS                               │
│  For each of 5 champions, show:                                 │
│  - Champion name, image, role                                    │
│  - Random Forest score (with bar)                                │
│  - Decision Tree score (with bar)                                │
│  - KNN score (with bar)                                          │
│  - Average/aggregate score                                       │
│  - Expandable calculation details                                │
└─────────────────────────────────────────────────────────────────┘
```

## Data Structures

### 1. Champion Score Object
```javascript
{
  championName: string,
  randomForest: number,      // 0-100
  decisionTree: number,       // 0-100
  knn: number,                // 0-100
  average: number,            // 0-100
  weighted: number,           // 0-100 (optional)
  details: {
    randomForest: ScoreDetails,
    decisionTree: ScoreDetails,
    knn: ScoreDetails
  }
}
```

### 2. Score Details Object
```javascript
{
  rawScore: number,
  normalizedScore: number,
  contributingFactors: [
    { factor: string, weight: number, contribution: number },
    ...
  ],
  matchedCriteria: string[],
  penalties: string[]
}
```

### 3. ML Results Object (New)
```javascript
mlResults = {
  scores: {
    randomForest: { "Ahri": 85.3, "Lux": 78.2, ... },
    decisionTree: { "Ahri": 78.2, "Lux": 84.1, ... },
    knn: { "Ahri": 82.1, "Lux": 77.8, ... }
  },
  aggregated: {
    "Ahri": { randomForest: 85.3, decisionTree: 78.2, ... },
    "Lux": { randomForest: 78.2, decisionTree: 84.1, ... },
    ...
  },
  top5: [
    { championName: "Ahri", randomForest: 85.3, ... },
    { championName: "Lux", randomForest: 78.2, ... },
    ...
  ]
}
```

## Component Design

### 1. ML Algorithm Classes (Modified)

Each algorithm class needs a new `predictAll()` method:

```javascript
class SimpleRandomForest {
  // Existing method (keep for backward compatibility)
  predict(features) { ... }
  
  // NEW: Score all champions
  predictAll(features) {
    const scores = {};
    
    for (const [name, champion] of Object.entries(allChampions)) {
      scores[name] = this.calculateChampionScore(features, champion);
    }
    
    return scores;
  }
  
  // NEW: Calculate score for a single champion
  calculateChampionScore(features, champion) {
    let score = 0;
    const details = {
      contributingFactors: [],
      matchedCriteria: [],
      penalties: []
    };
    
    // Role matching
    if (features.role === champion.role || features.role === 'No Preference') {
      score += 40;
      details.contributingFactors.push({
        factor: 'Role Match',
        weight: 40,
        contribution: 40
      });
    }
    
    // ... more scoring logic ...
    
    // Normalize to 0-100
    const normalizedScore = this.normalizeScore(score);
    
    return {
      score: normalizedScore,
      details: details
    };
  }
  
  normalizeScore(rawScore) {
    // Normalize to 0-100 range
    const min = 0;
    const max = 200; // Adjust based on max possible score
    return Math.max(0, Math.min(100, ((rawScore - min) / (max - min)) * 100));
  }
}
```

### 2. Score Aggregation Module

```javascript
class ScoreAggregator {
  /**
   * Aggregate scores from all ML algorithms
   */
  static aggregateScores(rfScores, dtScores, knnScores) {
    const aggregated = {};
    
    for (const championName of Object.keys(allChampions)) {
      const rf = rfScores[championName]?.score || 0;
      const dt = dtScores[championName]?.score || 0;
      const knn = knnScores[championName]?.score || 0;
      
      aggregated[championName] = {
        championName: championName,
        randomForest: rf,
        decisionTree: dt,
        knn: knn,
        average: (rf + dt + knn) / 3,
        weighted: this.calculateWeightedScore(rf, dt, knn),
        details: {
          randomForest: rfScores[championName]?.details,
          decisionTree: dtScores[championName]?.details,
          knn: knnScores[championName]?.details
        }
      };
    }
    
    return aggregated;
  }
  
  /**
   * Calculate weighted score (optional - can weight algorithms differently)
   */
  static calculateWeightedScore(rf, dt, knn) {
    // Example: Random Forest 40%, Decision Tree 30%, KNN 30%
    return (rf * 0.4) + (dt * 0.3) + (knn * 0.3);
  }
  
  /**
   * Select top 5 champions
   */
  static selectTop5(aggregatedScores, diversityFilter = true) {
    let champions = Object.values(aggregatedScores);
    
    // Sort by average score (or weighted score)
    champions.sort((a, b) => b.average - a.average);
    
    // Apply diversity filter if enabled
    if (diversityFilter) {
      champions = this.applyDiversityFilter(champions);
    }
    
    // Return top 5
    return champions.slice(0, 5);
  }
  
  /**
   * Apply diversity filter to avoid too many similar champions
   */
  static applyDiversityFilter(champions) {
    const selected = [];
    const roleCount = {};
    
    for (const champion of champions) {
      const championData = allChampions[champion.championName];
      const role = championData.role;
      
      // Limit to max 2 champions per role in top 5
      if ((roleCount[role] || 0) < 2) {
        selected.push(champion);
        roleCount[role] = (roleCount[role] || 0) + 1;
      }
      
      if (selected.length >= 5) break;
    }
    
    // If we don't have 5 yet, add remaining highest scores
    if (selected.length < 5) {
      for (const champion of champions) {
        if (!selected.includes(champion)) {
          selected.push(champion);
          if (selected.length >= 5) break;
        }
      }
    }
    
    return selected;
  }
}
```

### 3. Main Recommendation Flow

```javascript
function generateRecommendations() {
  // Extract features from user answers
  const userFeatures = extractFeatures(answers);
  
  // Run all 3 algorithms to score ALL champions
  const rfScores = algorithms['random-forest'].predictAll(userFeatures);
  const dtScores = algorithms['decision-tree'].predictAll(userFeatures);
  const knnScores = algorithms['knn'].predictAll(userFeatures);
  
  // Aggregate scores
  const aggregated = ScoreAggregator.aggregateScores(rfScores, dtScores, knnScores);
  
  // Select top 5
  const top5 = ScoreAggregator.selectTop5(aggregated);
  
  // Store results
  mlResults = {
    scores: {
      randomForest: rfScores,
      decisionTree: dtScores,
      knn: knnScores
    },
    aggregated: aggregated,
    top5: top5
  };
  
  // Display results
  displayUnifiedRecommendations(top5);
}
```

## UI Design

### Champion Card Layout

```html
<div class="unified-recommendation-card">
  <div class="champion-header">
    <img src="champion-image.jpg" class="champion-image-large">
    <div class="champion-info">
      <h3 class="champion-name">Ahri</h3>
      <span class="champion-role">Mage</span>
      <span class="champion-tier">Tier A</span>
    </div>
    <div class="aggregate-score">
      <div class="score-number">81.9%</div>
      <div class="score-label">Overall Match</div>
    </div>
  </div>
  
  <div class="ml-scores-section">
    <h4>ML Algorithm Scores</h4>
    
    <!-- Random Forest Score -->
    <div class="score-row">
      <div class="score-label-col">
        <span class="algorithm-icon">🌳</span>
        <span class="algorithm-name">Random Forest</span>
      </div>
      <div class="score-bar-col">
        <div class="score-bar-container">
          <div class="score-bar rf-bar" style="width: 85.3%"></div>
        </div>
      </div>
      <div class="score-value-col">85.3%</div>
      <button class="details-btn" onclick="showDetails('rf', 'Ahri')">
        Details
      </button>
    </div>
    
    <!-- Decision Tree Score -->
    <div class="score-row">
      <div class="score-label-col">
        <span class="algorithm-icon">🎯</span>
        <span class="algorithm-name">Decision Tree</span>
      </div>
      <div class="score-bar-col">
        <div class="score-bar-container">
          <div class="score-bar dt-bar" style="width: 78.2%"></div>
        </div>
      </div>
      <div class="score-value-col">78.2%</div>
      <button class="details-btn" onclick="showDetails('dt', 'Ahri')">
        Details
      </button>
    </div>
    
    <!-- KNN Score -->
    <div class="score-row">
      <div class="score-label-col">
        <span class="algorithm-icon">⚡</span>
        <span class="algorithm-name">K-Nearest Neighbors</span>
      </div>
      <div class="score-bar-col">
        <div class="score-bar-container">
          <div class="score-bar knn-bar" style="width: 82.1%"></div>
        </div>
      </div>
      <div class="score-value-col">82.1%</div>
      <button class="details-btn" onclick="showDetails('knn', 'Ahri')">
        Details
      </button>
    </div>
  </div>
  
  <!-- Expandable Details Section -->
  <div class="calculation-details" id="details-Ahri" style="display: none;">
    <!-- Details populated dynamically -->
  </div>
</div>
```

### CSS Styles (New)

```css
.unified-recommendation-card {
  background: white;
  border-radius: 15px;
  padding: 25px;
  margin: 20px 0;
  box-shadow: 0 5px 15px rgba(0,0,0,0.1);
}

.champion-header {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 25px;
  padding-bottom: 20px;
  border-bottom: 2px solid #e9ecef;
}

.aggregate-score {
  margin-left: auto;
  text-align: center;
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
  padding: 15px 25px;
  border-radius: 10px;
}

.aggregate-score .score-number {
  font-size: 2rem;
  font-weight: bold;
}

.ml-scores-section {
  margin: 20px 0;
}

.score-row {
  display: grid;
  grid-template-columns: 200px 1fr 80px 80px;
  align-items: center;
  gap: 15px;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.score-bar-container {
  height: 24px;
  background: #e9ecef;
  border-radius: 12px;
  overflow: hidden;
}

.score-bar {
  height: 100%;
  border-radius: 12px;
  transition: width 0.5s ease;
}

.rf-bar { background: linear-gradient(90deg, #28a745, #20c997); }
.dt-bar { background: linear-gradient(90deg, #007bff, #17a2b8); }
.knn-bar { background: linear-gradient(90deg, #ffc107, #fd7e14); }

.score-value-col {
  font-size: 1.2rem;
  font-weight: bold;
  color: #495057;
}

.details-btn {
  padding: 6px 12px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 0.85rem;
}

.details-btn:hover {
  background: #5a6268;
}
```


## Calculation Details Display

### Details Modal/Expandable Section

```javascript
function showCalculationDetails(algorithm, championName) {
  const champion = mlResults.top5.find(c => c.championName === championName);
  const details = champion.details[algorithm];
  
  let html = `
    <div class="calculation-details-content">
      <h4>${algorithm} Calculation for ${championName}</h4>
      
      <div class="score-summary">
        <div class="detail-item">
          <span class="label">Raw Score:</span>
          <span class="value">${details.rawScore.toFixed(2)}</span>
        </div>
        <div class="detail-item">
          <span class="label">Normalized Score:</span>
          <span class="value">${details.normalizedScore.toFixed(1)}%</span>
        </div>
      </div>
      
      <h5>Contributing Factors</h5>
      <table class="factors-table">
        <thead>
          <tr>
            <th>Factor</th>
            <th>Weight</th>
            <th>Contribution</th>
          </tr>
        </thead>
        <tbody>
  `;
  
  details.contributingFactors.forEach(factor => {
    html += `
      <tr>
        <td>${factor.factor}</td>
        <td>${factor.weight}</td>
        <td>${factor.contribution.toFixed(2)}</td>
      </tr>
    `;
  });
  
  html += `
        </tbody>
      </table>
      
      <h5>Matched Criteria</h5>
      <ul class="criteria-list">
  `;
  
  details.matchedCriteria.forEach(criterion => {
    html += `<li class="matched">✓ ${criterion}</li>`;
  });
  
  html += `</ul>`;
  
  if (details.penalties.length > 0) {
    html += `
      <h5>Penalties Applied</h5>
      <ul class="criteria-list">
    `;
    details.penalties.forEach(penalty => {
      html += `<li class="penalty">✗ ${penalty}</li>`;
    });
    html += `</ul>`;
  }
  
  html += `</div>`;
  
  // Display in modal or expandable section
  document.getElementById(`details-${championName}`).innerHTML = html;
  document.getElementById(`details-${championName}`).style.display = 'block';
}
```

## Performance Optimization

### 1. Caching Strategy

```javascript
// Cache champion scores to avoid recalculation
const scoreCache = new Map();

function getCachedScores(algorithm, features) {
  const cacheKey = `${algorithm}-${JSON.stringify(features)}`;
  
  if (scoreCache.has(cacheKey)) {
    return scoreCache.get(cacheKey);
  }
  
  const scores = algorithm.predictAll(features);
  scoreCache.set(cacheKey, scores);
  
  return scores;
}
```

### 2. Lazy Loading Details

```javascript
// Only calculate detailed breakdown when user clicks "Details"
function loadDetailsOnDemand(algorithm, championName) {
  if (!detailsCache.has(`${algorithm}-${championName}`)) {
    const details = calculateDetailedBreakdown(algorithm, championName);
    detailsCache.set(`${algorithm}-${championName}`, details);
  }
  
  return detailsCache.get(`${algorithm}-${championName}`);
}
```

### 3. Parallel Processing

```javascript
// Run all 3 algorithms in parallel (if using Web Workers)
async function generateRecommendationsParallel() {
  const userFeatures = extractFeatures(answers);
  
  const [rfScores, dtScores, knnScores] = await Promise.all([
    algorithms['random-forest'].predictAll(userFeatures),
    algorithms['decision-tree'].predictAll(userFeatures),
    algorithms['knn'].predictAll(userFeatures)
  ]);
  
  // Continue with aggregation...
}
```

## Algorithm Scoring Logic

### Random Forest Scoring

```javascript
calculateChampionScore(features, champion) {
  let score = 0;
  const details = {
    contributingFactors: [],
    matchedCriteria: [],
    penalties: []
  };
  
  // 1. Role Match (40 points max)
  if (features.role === champion.role || features.role === 'No Preference') {
    score += 40;
    details.contributingFactors.push({
      factor: 'Role Match',
      weight: 40,
      contribution: 40
    });
    details.matchedCriteria.push(`Role: ${champion.role}`);
  } else {
    details.penalties.push(`Role mismatch: wanted ${features.role}, got ${champion.role}`);
  }
  
  // 2. Position Match (30 points max)
  const championPositions = champion.positions || [];
  if (features.position === 'No Preference' || 
      championPositions.includes(features.position)) {
    score += 30;
    details.contributingFactors.push({
      factor: 'Position Match',
      weight: 30,
      contribution: 30
    });
  }
  
  // 3. Difficulty Match (20 points max)
  const diffDelta = Math.abs(features.difficulty - champion.difficulty);
  const diffScore = Math.max(0, 20 - (diffDelta * 2));
  score += diffScore;
  details.contributingFactors.push({
    factor: 'Difficulty Match',
    weight: 20,
    contribution: diffScore
  });
  
  // 4. Damage Match (15 points max)
  const damageDelta = Math.abs(features.damage - champion.damage);
  const damageScore = Math.max(0, 15 - (damageDelta * 1.5));
  score += damageScore;
  details.contributingFactors.push({
    factor: 'Damage Match',
    weight: 15,
    contribution: damageScore
  });
  
  // 5. Toughness Match (15 points max)
  const toughnessDelta = Math.abs(features.toughness - champion.toughness);
  const toughnessScore = Math.max(0, 15 - (toughnessDelta * 1.5));
  score += toughnessScore;
  details.contributingFactors.push({
    factor: 'Toughness Match',
    weight: 15,
    contribution: toughnessScore
  });
  
  // 6. Playstyle Bonuses (20 points max)
  let playstyleScore = 0;
  if (features.playstyle === 'aggressive' && champion.damage >= 8) {
    playstyleScore += 20;
    details.matchedCriteria.push('High damage for aggressive playstyle');
  } else if (features.playstyle === 'defensive' && champion.toughness >= 7) {
    playstyleScore += 20;
    details.matchedCriteria.push('High toughness for defensive playstyle');
  } else if (features.playstyle === 'supportive' && champion.utility >= 7) {
    playstyleScore += 20;
    details.matchedCriteria.push('High utility for supportive playstyle');
  } else if (features.playstyle === 'balanced') {
    playstyleScore += 10;
  }
  score += playstyleScore;
  details.contributingFactors.push({
    factor: 'Playstyle Bonus',
    weight: 20,
    contribution: playstyleScore
  });
  
  // 7. Psychological Preferences (30 points max)
  let psychScore = 0;
  
  // Pressure response
  if (features.pressure_response === 'Stay calm and strategic' && champion.toughness >= 6) {
    psychScore += 10;
    details.matchedCriteria.push('Calm playstyle matches champion toughness');
  }
  // ... more psychological matching ...
  
  score += psychScore;
  details.contributingFactors.push({
    factor: 'Psychological Match',
    weight: 30,
    contribution: psychScore
  });
  
  // Total possible: 40+30+20+15+15+20+30 = 170 points
  // Normalize to 0-100
  const normalizedScore = (score / 170) * 100;
  
  return {
    score: Math.min(100, Math.max(0, normalizedScore)),
    rawScore: score,
    details: details
  };
}
```

### Decision Tree Scoring

```javascript
calculateChampionScore(features, champion) {
  let score = 0;
  const details = {
    contributingFactors: [],
    matchedCriteria: [],
    penalties: []
  };
  
  // Decision tree uses hierarchical scoring
  // Level 1: Role (50 points)
  if (features.role === champion.role || features.role === 'No Preference') {
    score += 50;
    details.matchedCriteria.push(`Role: ${champion.role}`);
    
    // Level 2: Position (40 points) - only if role matches
    const championPositions = champion.positions || [];
    if (features.position === 'No Preference' || 
        championPositions.includes(features.position)) {
      score += 40;
      details.matchedCriteria.push(`Position: ${features.position}`);
      
      // Level 3: Difficulty (30 points) - only if position matches
      if (Math.abs(features.difficulty - champion.difficulty) <= 2) {
        score += 30;
        details.matchedCriteria.push('Difficulty within range');
        
        // Level 4: Attributes (20 points each)
        if (Math.abs(features.damage - champion.damage) <= 2) {
          score += 20;
          details.matchedCriteria.push('Damage matches');
        }
        if (Math.abs(features.toughness - champion.toughness) <= 2) {
          score += 20;
          details.matchedCriteria.push('Toughness matches');
        }
      }
    }
  } else {
    score -= 30;
    details.penalties.push(`Role mismatch: wanted ${features.role}`);
  }
  
  // Psychological bonuses (25 points each)
  // ... similar to Random Forest ...
  
  // Total possible: 50+40+30+20+20+25+25 = 210 points
  // Normalize to 0-100
  const normalizedScore = ((score + 30) / 240) * 100; // +30 to offset penalty
  
  return {
    score: Math.min(100, Math.max(0, normalizedScore)),
    rawScore: score,
    details: details
  };
}
```

### KNN Scoring

```javascript
calculateChampionScore(features, champion) {
  let distance = 0;
  const details = {
    contributingFactors: [],
    matchedCriteria: [],
    penalties: []
  };
  
  // KNN uses distance calculation (lower is better)
  
  // Role distance (8 points penalty if mismatch)
  if (features.role !== 'No Preference' && features.role !== champion.role) {
    distance += 8;
    details.penalties.push(`Role mismatch: ${champion.role}`);
  } else {
    details.matchedCriteria.push(`Role: ${champion.role}`);
  }
  
  // Position distance (6 points penalty)
  const championPositions = champion.positions || [];
  if (features.position !== 'No Preference' && 
      !championPositions.includes(features.position)) {
    distance += 6;
    details.penalties.push('Position mismatch');
  }
  
  // Numerical distances
  const diffDist = Math.abs(features.difficulty - champion.difficulty) * 0.5;
  const damageDist = Math.abs(features.damage - champion.damage) * 0.3;
  const toughnessDist = Math.abs(features.toughness - champion.toughness) * 0.3;
  
  distance += diffDist + damageDist + toughnessDist;
  
  details.contributingFactors.push(
    { factor: 'Difficulty Distance', weight: 0.5, contribution: diffDist },
    { factor: 'Damage Distance', weight: 0.3, contribution: damageDist },
    { factor: 'Toughness Distance', weight: 0.3, contribution: toughnessDist }
  );
  
  // Psychological distances (2 points each)
  // ... similar logic ...
  
  // Convert distance to score (inverse relationship)
  // Max distance ~30, convert to 0-100 score
  const normalizedScore = Math.max(0, 100 - (distance * 3.33));
  
  return {
    score: normalizedScore,
    rawScore: distance,
    details: details
  };
}
```

## Integration Points

### 1. Update showResults() Function

```javascript
function showResults() {
  // Extract features
  const userFeatures = extractFeatures(answers);
  
  // Run all algorithms
  const rfScores = algorithms['random-forest'].predictAll(userFeatures);
  const dtScores = algorithms['decision-tree'].predictAll(userFeatures);
  const knnScores = algorithms['knn'].predictAll(userFeatures);
  
  // Aggregate
  const aggregated = ScoreAggregator.aggregateScores(rfScores, dtScores, knnScores);
  
  // Select top 5
  const top5 = ScoreAggregator.selectTop5(aggregated);
  
  // Store globally
  mlResults = {
    scores: { randomForest: rfScores, decisionTree: dtScores, knn: knnScores },
    aggregated: aggregated,
    top5: top5
  };
  
  // Display
  displayUnifiedRecommendations();
}
```

### 2. Update Evaluation Metrics

```javascript
// Modify getRecommendedChampions to return top 5
EvaluationMetrics.getRecommendedChampions = function(mlResults) {
  // Return top 5 champion names in order
  return mlResults.top5.map(c => c.championName);
};
```

### 3. Update Analytics Tracking

```javascript
function trackRecommendations(top5) {
  // Track each of the 5 recommendations
  top5.forEach((champion, index) => {
    // Analytics code here
    console.log(`Recommendation ${index + 1}: ${champion.championName}`);
    console.log(`  RF: ${champion.randomForest}%`);
    console.log(`  DT: ${champion.decisionTree}%`);
    console.log(`  KNN: ${champion.knn}%`);
    console.log(`  Avg: ${champion.average}%`);
  });
}
```

## Testing Strategy

### 1. Unit Tests

```javascript
// Test score normalization
function testScoreNormalization() {
  const rf = new SimpleRandomForest();
  const score = rf.normalizeScore(85);
  assert(score >= 0 && score <= 100, 'Score should be 0-100');
}

// Test aggregation
function testAggregation() {
  const rfScores = { "Ahri": 85, "Lux": 78 };
  const dtScores = { "Ahri": 78, "Lux": 84 };
  const knnScores = { "Ahri": 82, "Lux": 77 };
  
  const aggregated = ScoreAggregator.aggregateScores(rfScores, dtScores, knnScores);
  
  assert(aggregated["Ahri"].average === 81.67, 'Average should be correct');
}

// Test top 5 selection
function testTop5Selection() {
  const aggregated = { /* ... */ };
  const top5 = ScoreAggregator.selectTop5(aggregated);
  
  assert(top5.length === 5, 'Should return exactly 5 champions');
  assert(new Set(top5.map(c => c.championName)).size === 5, 'All should be unique');
}
```

### 2. Integration Tests

```javascript
// Test full flow
function testFullRecommendationFlow() {
  const testAnswers = {
    role: 'Mage',
    difficulty: 'Medium (4-6)',
    playstyle: 'High Damage Output'
  };
  
  answers = testAnswers;
  showResults();
  
  assert(mlResults.top5.length === 5, 'Should have 5 recommendations');
  assert(mlResults.top5[0].randomForest > 0, 'Should have RF score');
  assert(mlResults.top5[0].decisionTree > 0, 'Should have DT score');
  assert(mlResults.top5[0].knn > 0, 'Should have KNN score');
}
```

### 3. Visual Tests

- Test with different screen sizes
- Verify score bars display correctly
- Check expandable details work
- Ensure colors and styling are consistent

## Migration Strategy

### Phase 1: Add New Methods (Backward Compatible)
- Add `predictAll()` to each algorithm class
- Keep existing `predict()` methods
- Test new methods independently

### Phase 2: Create Aggregation Module
- Implement `ScoreAggregator` class
- Test aggregation logic
- Verify top 5 selection

### Phase 3: Update UI
- Create new champion card layout
- Add CSS styles
- Implement details display

### Phase 4: Switch Over
- Update `showResults()` to use new system
- Update evaluation metrics
- Remove old code (optional)

### Phase 5: Deploy
- Test thoroughly
- Deploy to production
- Monitor for issues

## Rollback Plan

If issues arise:
1. Keep old `predict()` methods as fallback
2. Add feature flag to switch between old/new systems
3. Can revert to showing 3 champions if needed

```javascript
const USE_UNIFIED_RECOMMENDATIONS = true; // Feature flag

function showResults() {
  if (USE_UNIFIED_RECOMMENDATIONS) {
    showUnifiedRecommendations();
  } else {
    showLegacyRecommendations();
  }
}
```

---

## Summary

This design provides:
- ✅ Clear architecture for unified recommendations
- ✅ Detailed data structures
- ✅ Complete algorithm scoring logic
- ✅ UI mockups and CSS
- ✅ Performance optimizations
- ✅ Testing strategy
- ✅ Migration plan
- ✅ Rollback capability

**Ready for implementation!**
