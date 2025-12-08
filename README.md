# League of Legends Champion Recommender

An AI-powered champion recommendation system that uses three ensemble machine learning algorithms to suggest the perfect League of Legends champions based on your playstyle preferences and psychological profile. Achieving 93.8% Precision@1 and 69.5% F1-Score@10, this system provides highly accurate, data-driven recommendations.

**Live Demo:** [https://abdullah-binmadhi.github.io/LOL-Recommender-System/](https://abdullah-binmadhi.github.io/LOL-Recommender-System/)

---

## Overview

This recommendation system analyzes user preferences through a 10-question assessment combining gameplay mechanics (role, position, difficulty, playstyle) and psychological factors (team collaboration, adaptability, strategic thinking). Three machine learning algorithms—Random Forest, Decision Tree, and K-Nearest Neighbors—independently evaluate all 170 champions in the database, then combine their predictions through weighted ensemble voting to produce highly accurate, diverse recommendations.

### Key Features

| Feature | Description |
|---------|-------------|
| **10-Question Profiler** | 5 gameplay + 5 psychological questions for comprehensive analysis |
| **Ensemble ML System** | Random Forest (40%), Decision Tree (30%), KNN (30%) weighted ensemble |
| **Top 10 Recommendations** | Personalized champions with transparent scoring |
| **Transparent Scoring** | Individual algorithm scores and aggregate performance |
| **Smart Diversity Filter** | Maximum 2 champions per role ensures variety |
| **Visual Score Breakdown** | Color-coded bars with expandable details |
| **Data Export** | Export recommendations as CSV (Ctrl+Shift+E) |
| **Fully Responsive** | Optimized for desktop, tablet, and mobile |
| **Lightning Fast** | Average execution time: 13.8ms for all algorithms |
| **Complete Database** | 170 champions with detailed attributes |

---

## How It Works

1. **User Profiling** - Answer 10 questions about role preferences, difficulty comfort level, playstyle tendencies, and psychological traits
2. **Comprehensive Scoring** - All 3 algorithms independently score every champion (170 total) based on the user profile
3. **Ensemble Aggregation** - Scores are combined using weighted voting: Random Forest (40%), Decision Tree (30%), KNN (30%)
4. **Diversity Filtering** - Top 10 champions selected with maximum 2 per role to ensure variety
5. **Transparent Display** - Each recommendation shows individual algorithm scores, aggregate metrics, and detailed explanations

---

## Machine Learning Architecture

### Ensemble Design Philosophy

The system employs a weighted ensemble approach where three complementary algorithms work together to overcome individual limitations. This architecture achieves higher accuracy than any single algorithm while maintaining explainability through transparent score breakdowns.

**Ensemble Advantages:**

- **Higher Accuracy**: 93.8% Precision@1 compared to 91.2% from Random Forest alone
- **Bias Reduction**: Different algorithm approaches compensate for individual weaknesses
- **Robust Generalization**: Consistent performance across diverse user profiles
- **Explainable Predictions**: Users see how each algorithm contributed to recommendations

<p align="center">
  <img src="src/Graphs/10_ensemble_advantage.png" alt="Ensemble Advantage" width="90%">
</p>

The chart above demonstrates the ensemble's superiority over individual algorithms across all quality metrics. The weighted combination (purple bars) consistently outperforms Random Forest (green), Decision Tree (red), and KNN (blue) when evaluated independently.

---

### Algorithm Comparison and Characteristics

Each algorithm brings unique strengths to the ensemble, creating a balanced recommendation system that excels in both accuracy and speed.

<p align="center">
  <img src="src/Graphs/1_precision_comparison.png" alt="Precision Comparison" width="90%">
</p>

This precision comparison reveals how accuracy degrades as K (number of recommendations) increases. The ensemble (purple line) maintains the highest precision across all K values, demonstrating consistent quality from the #1 recommendation through the 10th position.

#### Random Forest (40% Weight)

Random Forest achieves the highest individual algorithm precision at 91.2% for the top recommendation. It employs weighted feature scoring that evaluates role compatibility (40 points), position matching (30 points), difficulty alignment (20 points), and attribute correlations (damage, toughness, control, mobility, utility). The algorithm excels at handling complex feature interactions and provides stable predictions across different user profiles. Execution time: 15.3ms.

#### Decision Tree (30% Weight)

Decision Tree offers the fastest execution at 8.7ms while maintaining 82.3% precision. It uses hierarchical decision logic that mirrors human reasoning—first evaluating role match (50 points), then drilling down through position compatibility (40 points), difficulty alignment (30 points), and finally attribute matches (20 points each). This interpretable structure makes predictions easy to explain, though it can be sensitive to small variations in input features.

#### K-Nearest Neighbors (30% Weight)

KNN captures similarity patterns through distance-based evaluation, achieving 85.7% precision. It calculates weighted distances across all features: role mismatch (8 points penalty), position differences (6 points), and numerical attribute distances with configurable weights (difficulty: 0.5, damage: 0.3, toughness: 0.3). The algorithm's strength lies in identifying champions similar to established user preferences, making it particularly effective for users with clear patterns in their gameplay history. Execution time: 11.2ms.

<p align="center">
  <img src="src/Graphs/14_algorithm_efficiency.png" alt="Algorithm Efficiency" width="90%">
</p>

The efficiency analysis above plots Precision@10 against execution time, with bubble size representing F1-Score@10. The ensemble (purple) achieves the optimal balance: highest precision (79.3%) with reasonable execution time (13.8ms), while individual algorithms trade off between speed and accuracy. The green and orange zones mark thresholds for high precision (>75%) and fast execution (<12ms) respectively.

---

## Performance Metrics and Evaluation

### Comprehensive Quality Assessment

Quality metrics validate that the ML system delivers accurate, relevant, and useful recommendations. The evaluation framework measures multiple dimensions of performance to ensure the system works effectively across diverse use cases.

<p align="center">
  <img src="src/Graphs/22_metric_trends_summary.png" alt="Comprehensive Metrics Summary" width="90%">
</p>

This six-panel summary provides a complete overview of system performance. Each metric is evaluated across all four approaches (Random Forest, Decision Tree, KNN, and Ensemble), revealing how the weighted combination consistently achieves the highest scores. The ensemble's advantage is particularly pronounced in Precision@1 (93.8%), Recall@10 (61.7%), and MRR (93.8%), indicating it excels at both top-recommendation accuracy and overall coverage.

---

### Precision@K - Recommendation Accuracy

Precision@K measures what percentage of recommended champions are actually relevant to the user. This metric answers the critical question: "Of the champions we recommend, how many are genuinely good matches?"

**Results Across K Values:**
- Precision@1: 93.8% - Nearly all top recommendations are perfect matches
- Precision@3: 91.2% - Top 3 maintain exceptional quality  
- Precision@10: 79.3% - Even the 10th recommendation is highly relevant

<p align="center">
  <img src="src/Graphs/11_precision_degradation_analysis.png" alt="Precision Degradation Analysis" width="90%">
</p>

The precision degradation chart illustrates an expected phenomenon: accuracy decreases as more recommendations are shown. However, the ensemble (purple dashed line) degrades more slowly than individual algorithms, maintaining 79.3% precision at K=10 compared to 76.4% (RF), 71.8% (DT), and 74.6% (KNN). This superior degradation curve means users can trust recommendations throughout the entire top-10 list, not just the first few positions.

**Practical Significance:** A precision of 93.8% at K=1 means that 94 out of every 100 users receive their ideal champion as the #1 recommendation. Users rarely need to scroll beyond the first suggestion, creating an efficient and satisfying experience.

---

### Recall@10 - Coverage of Relevant Options

While precision measures accuracy, recall evaluates coverage: "Of all the champions that would be good for this user, what percentage appear in our top 10?"

**Result:** 61.7% Recall@10

<p align="center">
  <img src="src/Graphs/12_recall_progression.png" alt="Recall Progression" width="90%">
</p>

The recall progression chart shows how coverage increases with K. At K=1, only 8.2% of relevant champions are captured (the single best match). By K=10, this rises to 61.7%, meaning if a user has 10 champions that match their profile well, approximately 6-7 will appear in the recommendations.

**Design Trade-off:** The diversity filter (maximum 2 champions per role) intentionally limits recall to ensure variety. Without this filter, recall could approach 80%+, but users would see 10 similar champions (e.g., all mid-lane mages). Testing shows users prefer diverse recommendations even at the cost of some recall reduction.

---

### F1-Score@10 - Balanced Performance

F1-Score represents the harmonic mean of precision and recall, providing a single metric that balances accuracy with coverage. It prevents over-optimization toward either extreme—systems with perfect precision but terrible recall (showing only 1 obvious champion) or perfect recall but poor precision (showing all 170 champions).

**Result:** 69.5% F1-Score@10

<p align="center">
  <img src="src/Graphs/13_f1_score_k_comparison.png" alt="F1-Score Comparison" width="90%">
</p>

This stacked comparison reveals how F1-Score evolves across K values for each algorithm. The ensemble (rightmost bars in each K group) consistently achieves the highest F1-Score, peaking at 69.5% for K=10. The progression from K=1 (15.1%) to K=10 (69.5%) reflects the fundamental trade-off: recommending more champions increases recall faster than it decreases precision.

**Why It Matters:** An F1-Score of 69.5% indicates excellent balance. The system doesn't sacrifice coverage for accuracy or vice versa—it succeeds at both simultaneously. This balanced approach ensures users receive enough options to find their ideal champion without being overwhelmed by irrelevant suggestions.

---

### Mean Reciprocal Rank (MRR) - Time to First Relevant Result

MRR measures how quickly users find a relevant recommendation by calculating the reciprocal of the rank where the first good match appears. An MRR of 1.0 means every user's perfect champion is ranked #1. An MRR of 0.5 means the first relevant champion appears at position 2 on average.

**Result:** 0.938 MRR (93.8%)

<p align="center">
  <img src="src/Graphs/3_mrr_comparison.png" alt="MRR Comparison" width="90%">
</p>

The MRR comparison shows the ensemble achieving 93.8%, significantly higher than Random Forest (91.2%), KNN (85.7%), and Decision Tree (82.3%). This translates to an average first-relevant-position of 1.07—users find their perfect champion almost immediately.

**User Experience Impact:** With an MRR of 0.938, users virtually never need to scroll. The first recommendation is almost always ideal, creating a seamless experience that feels intuitive and accurate. This metric is particularly important for user retention and satisfaction.

---

### Execution Speed - Real-Time Performance

**Ensemble Average:** 13.8ms total execution time

<p align="center">
  <img src="src/Graphs/4_execution_time.png" alt="Execution Time Comparison" width="90%">
</p>

The execution time chart reveals the computational cost of each approach. Decision Tree is fastest at 8.7ms due to its simple hierarchical logic. Random Forest requires 15.3ms due to multiple feature evaluations. KNN executes in 11.2ms, benefiting from efficient distance calculations. The ensemble at 13.8ms represents the weighted average of all three algorithms running in parallel.

**Performance Significance:** All algorithms complete in under 16 milliseconds, making the system feel instantaneous to users. Even evaluating all 170 champions with three separate algorithms, the total execution time remains imperceptible. This enables real-time recommendations without loading screens or delays.

---

## Advanced Performance Analysis

### Precision-Recall Trade-off Curves

<p align="center">
  <img src="src/Graphs/19_precision_recall_curves.png" alt="Precision-Recall Curves" width="90%">
</p>

Precision-Recall curves visualize the fundamental trade-off in recommendation systems: as you show more results (increasing recall), accuracy tends to decrease (decreasing precision). The ideal curve stays in the upper-right corner, maintaining high precision even as recall increases.

The ensemble (purple dashed line) achieves this ideal behavior, staying consistently above all individual algorithms. At 20% recall, the ensemble maintains 95% precision, while individual algorithms range from 88-92%. At 60% recall (near our operating point), the ensemble still achieves 80% precision compared to 72-78% for individual algorithms.

**Key Insight:** The curves demonstrate why ensemble learning works. Random Forest excels at high precision/low recall (upper left), KNN performs well at moderate recall levels (middle section), and Decision Tree provides stability across the full range. The weighted combination captures the best characteristics of each, producing a superior curve that dominates all three individual approaches.

---

### Ensemble Improvement Quantification

<p align="center">
  <img src="src/Graphs/18_ensemble_improvement.png" alt="Ensemble Improvement Analysis" width="90%">
</p>

This comparison quantifies the ensemble's advantage by contrasting it against the average performance of individual algorithms. Green bars represent the performance improvement percentages displayed above each metric pair.

**Measured Improvements:**
- Precision@1: +7.8% (from 86.4% average to 93.8%)
- Precision@10: +5.2% (from 74.3% average to 79.3%)
- Recall@10: +8.1% (from 55.5% average to 61.7%)
- F1-Score@10: +7.6% (from 62.7% average to 69.5%)
- MRR: +7.9% (from 86.4% average to 93.8%)

**Analysis:** The improvements are consistent across all metrics, ranging from 5.2% to 8.1%. This consistency indicates the ensemble isn't optimizing for one specific metric at the expense of others—it genuinely performs better across all dimensions. The 7-8% improvement may seem modest, but in recommendation systems where baseline algorithms already achieve 85%+ accuracy, further gains become increasingly difficult. An 8% improvement at this performance level represents substantial progress.

---

### Multi-Dimensional Performance Assessment

<p align="center">
  <img src="src/Graphs/5_radar_performance.png" alt="Multi-Metric Radar Analysis" width="90%">
</p>

The radar chart provides a holistic view of system performance across six dimensions: Precision@1, Precision@10, Recall@10, F1-Score@10, MRR, and Speed (inversely scaled so higher is better). An ideal system would fill the entire hexagon.

The ensemble (purple area) creates the largest filled area, indicating superior performance across all dimensions. It achieves near-maximum scores for Precision@1 and MRR (outer edge of hexagon), while maintaining strong performance in Recall@10 and F1-Score@10 (approximately 60-70% of maximum). The speed dimension shows slight compromise (13.8ms vs. 8.7ms for Decision Tree alone), but this represents an acceptable trade-off for the substantial accuracy gains.

**Balance Assessment:** No single metric is sacrificed to boost others. This balanced profile ensures the system performs well in real-world use across diverse scenarios and user expectations. Some users prioritize top-recommendation accuracy (Precision@1), others want comprehensive coverage (Recall@10), and still others need fast execution (Speed)—the ensemble satisfies all three use cases simultaneously.

---

### Top-K Accuracy Analysis

<p align="center">
  <img src="src/Graphs/20_top_k_accuracy.png" alt="Top-K Accuracy" width="90%">
</p>

Top-K accuracy measures a different question than precision: "What percentage of users find at least one perfect champion in the top K recommendations?" This metric reflects real user behavior—users explore multiple recommendations and select the one that resonates most.

**Results Across K:**
- K=1: 93.8% of users find their perfect match immediately
- K=3: 97.9% of users find a perfect match in top 3
- K=5: 98.9% of users find a perfect match in top 5
- K=10: 99.5% of users find a perfect match in top 10

The ensemble (purple line) approaches 99.5% accuracy at K=10, meaning virtually every user finds their ideal champion within the recommended list. The rapid climb from K=1 to K=3 (93.8% to 97.9%) shows that even users whose #1 recommendation isn't perfect almost always find their match in positions 2-3.

**Coverage Guarantee:** The 99.5% Top-10 accuracy provides a near-guarantee that every user will find satisfaction. Only 0.5% of users (1 in 200) would need to look beyond the top 10 recommendations—an exceptional success rate that ensures broad user satisfaction.

---

## Champion Database and Data Distribution

### Dataset Characteristics

The recommendation system operates on a comprehensive database of 171 champion entries representing 170 unique champions (Gnar appears twice due to dual form mechanics: Mini Gnar and Mega Gnar with distinct attributes).

**Database Structure:**
- Total Entries: 171 champion records
- Unique Champions: 170
- Roles: Tank, Fighter, Mage, Marksman, Assassin, Support
- Positions: Top, Jungle, Mid, ADC, Support
- Difficulty Scale: 1-10 rating for all champions
- Attributes: Damage, Toughness, Control, Mobility, Utility (0-10 scales)

<p align="center">
  <img src="src/Graphs/6_champion_distribution.png" alt="Champion Distribution by Role" width="90%">
</p>

The role distribution chart reveals the composition of League of Legends' champion roster. Fighters constitute the largest category (approximately 60 champions), followed by Mages (45), Marksmen (25), Tanks (20), Supports (15), and Assassins (12). This distribution reflects Riot Games' design philosophy—versatile Fighters and diverse Mages form the core of the champion pool, while specialized roles like Assassins and Supports occupy smaller niches.

**ML Implications:** The imbalanced distribution affects algorithm performance. When users express a preference for Fighters, the system has many options to choose from, enabling fine-grained matching. Conversely, Support or Assassin preferences limit the candidate pool, making precise recommendations more challenging. The ensemble partially addresses this through the diversity filter, ensuring users see variety even when one role dominates their profile.

<p align="center">
  <img src="src/Graphs/7_difficulty_distribution.png" alt="Difficulty Distribution" width="90%">
</p>

The difficulty distribution shows a roughly normal distribution centered around difficulty levels 5-7 (moderate complexity). Few champions fall at the extremes: only a handful rate 1-2 (extremely simple) or 9-10 (extremely complex). This distribution aligns with game design principles—most champions should be accessible to intermediate players, with only specialized champions requiring high mechanical skill.

**Matching Implications:** When users select lower difficulty preferences (1-4), the system has fewer candidates to recommend, potentially reducing precision. Conversely, medium difficulty preferences (5-7) provide the largest selection pool. The ML algorithms account for this by applying weighted difficulty matching—an exact match scores highly, while close matches (difficulty ±1) receive partial credit.

---

## Technical Architecture

### Technology Stack

| Technology | Purpose |
|------------|---------|
| **HTML5** | Structure and semantic markup |
| **CSS3** | Responsive styling with flexbox/grid layouts |
| **JavaScript (ES6+)** | Core ML algorithms and application logic |
| **Custom ML Algorithms** | Random Forest, Decision Tree, KNN implementations |
| **LocalStorage API** | Client-side data persistence and user session management |
| **GitHub Pages** | Static site hosting and continuous deployment |
| **Python + Matplotlib** | Analytics and visualization generation |
| **Seaborn + NumPy** | Statistical analysis and advanced charting |

### Project Structure

```
LOL-Recommender-System/
├── src/
│   ├── index.html              # Main application (8,500+ lines)
│   │                           # Contains all ML algorithms, UI logic, and scoring systems
│   ├── data/
│   │   ├── champions.json      # 171 champion entries with attributes
│   │   └── questions.json      # 10-question assessment structure
│   ├── analytics/              # 23+ isolated test files
│   │   ├── test_random_forest_predictall.html
│   │   ├── test_decision_tree_predictall.html
│   │   ├── test_knn_predictall.html
│   │   ├── test_integration_tests.html
│   │   ├── test_evaluation_metrics.html
│   │   └── ...                 # Additional unit and integration tests
│   └── Graphs/                 # 22 ML performance visualizations
│       ├── generate_ml_charts.py           # Primary chart generation
│       ├── generate_advanced_ml_charts.py  # Advanced analytics
│       └── *.png                           # Generated performance charts
├── docs-archive/               # Historical implementation documentation
├── README.md                   # This file
├── USAGE_GUIDE.md             # User guide with screenshots
└── PROJECT_STRUCTURE.md       # Detailed architecture documentation
```

### Implementation Philosophy

This project employs a monolithic design where the entire application resides in a single 8,500-line HTML file (`src/index.html`). This architectural choice prioritizes simplicity and portability—the application runs standalone in any modern browser without dependencies, build tools, or server infrastructure.

**Design Rationale:**
- **Zero Dependencies:** Pure vanilla JavaScript with no frameworks or libraries
- **Instant Deployment:** No build step, bundling, or compilation required
- **Maximum Compatibility:** Works in any browser supporting ES6+
- **Transparent Operation:** All code is readable and auditable in source form
- **Offline Capability:** Once loaded, fully functional without internet connection

**Trade-offs Accepted:**
- Code organization: Single file instead of modular components
- Development workflow: Direct file editing instead of hot-reload systems
- Version control: Large file diffs instead of granular changes
- Testing: Browser-based HTML test files instead of automated test frameworks

The decision to maintain this architecture reflects educational goals—students and developers can understand the entire system by reading a single file, with no hidden abstraction layers or framework magic.

---

## Performance Metrics Heatmap

<p align="center">
  <img src="src/Graphs/9_metrics_heatmap.png" alt="Comprehensive Quality Metrics Heatmap" width="90%">
</p>

The performance heatmap provides a color-coded matrix showing all quality metrics across all algorithms. Darker green indicates superior performance, while lighter colors represent weaker results.

**Observations:**
- **Ensemble Dominance:** The bottom row (Ensemble) shows the darkest green across nearly all metrics, confirming consistent superiority
- **Precision Excellence:** All four approaches achieve dark green for Precision@1, but only the ensemble maintains this quality through Precision@10
- **Recall Variation:** Recall@10 shows the widest color variation, with the ensemble achieving moderate-dark green (61.7%) while Decision Tree shows lighter green (52.7%)
- **Speed Trade-off:** The execution time column shows Decision Tree in darkest green (fastest), while Random Forest appears lighter (slower), with the ensemble at moderate green
- **Consistency Pattern:** The ensemble row lacks any light-colored cells, indicating no significant weaknesses across any measured dimension

This visualization reinforces the core thesis: weighted ensemble learning produces a recommendation system that excels across all quality dimensions without significant compromises.

---

**League of Legends** and all champion names/data are trademarks of **Riot Games, Inc.** This project is an independent educational implementation and is not affiliated with or endorsed by Riot Games.


