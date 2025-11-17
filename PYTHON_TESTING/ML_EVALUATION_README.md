# ML Model Evaluation Pipeline

## Overview

This evaluation pipeline implements a comprehensive system for testing and comparing three machine learning models (Random Forest, Decision Tree, and K-Nearest Neighbors) for the Champion Recommender System.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   ML EVALUATION PIPELINE                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  1. LOAD TRAINED MODELS                                      │
│     ├── Random Forest Model (.pkl)                           │
│     ├── Decision Tree Model (.pkl)                           │
│     ├── KNN Model (.pkl)                                     │
│     ├── Scaler (.pkl)                                        │
│     └── Metadata (.pkl)                                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  2. UNIFIED RECOMMENDATION SYSTEM                            │
│     ├── Preprocess user questionnaire input                 │
│     ├── Generate ranked champion recommendations            │
│     ├── Output Top-K champions with confidence scores        │
│     └── Provide full ranking of all champions               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  3. EVALUATION METRICS                                       │
│     ├── Precision@K (K=1, 3, 5)                             │
│     │   └── Measures relevant champions in top-K            │
│     └── Mean Reciprocal Rank (MRR)                          │
│         └── Measures position of first relevant champion    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  4. EVALUATION PIPELINE                                      │
│     ├── Load test dataset with ground truth                 │
│     ├── For each user:                                       │
│     │   ├── Preprocess questionnaire answers                │
│     │   ├── Generate recommendations from each model         │
│     │   ├── Calculate Precision@K                           │
│     │   └── Calculate Reciprocal Rank                       │
│     ├── Average metrics across all users                    │
│     └── Return final scores                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  5. MODEL COMPARISON                                         │
│     ├── Compare Random Forest vs Decision Tree vs KNN       │
│     ├── Display results in formatted table                  │
│     ├── Identify best performing model                      │
│     └── Provide production recommendation                   │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. MLModelLoader
Loads trained ML models from pickle files.

**Methods:**
- `load_all_models()`: Loads Random Forest, Decision Tree, KNN, scaler, and metadata

### 2. UnifiedRecommendationSystem
Generates champion recommendations from any ML model.

**Methods:**
- `preprocess_user_input(user_answers)`: Converts questionnaire to model features
- `generate_recommendations(user_input, k)`: Returns top-K champions
- `get_full_ranking(user_input)`: Returns all champions ranked by score

### 3. RankingMetrics
Implements evaluation metrics.

**Methods:**
- `precision_at_k(recommended, relevant, k)`: Calculates Precision@K
- `reciprocal_rank(recommended, relevant)`: Calculates Reciprocal Rank
- `mean_reciprocal_rank(reciprocal_ranks)`: Averages RR across users

### 4. EvaluationPipeline
Complete evaluation workflow.

**Methods:**
- `load_test_dataset(test_file)`: Loads test data (JSON/CSV)
- `extract_ground_truth(user_data)`: Gets relevant champions for user
- `evaluate_single_model(model_name, test_data)`: Evaluates one model
- `compare_all_models(test_data)`: Evaluates all three models
- `print_comparison_table(results)`: Displays formatted results

## Usage

### Basic Usage

```python
from ml_evaluation_pipeline import main

# Run complete evaluation
main()
```

### Advanced Usage

```python
from ml_evaluation_pipeline import (
    MLModelLoader,
    UnifiedRecommendationSystem,
    EvaluationPipeline
)

# 1. Load models
loader = MLModelLoader()
models = loader.load_all_models()

# 2. Load champion names
with open("../src/data/champions.json", 'r') as f:
    champions = json.load(f)
    champion_names = list(champions.keys())

# 3. Create pipeline
pipeline = EvaluationPipeline(
    models, 
    loader.scaler, 
    champion_names, 
    loader.metadata
)

# 4. Load test data
test_data = pipeline.load_test_dataset("user_data_backup.json")

# 5. Evaluate all models
results = pipeline.compare_all_models(test_data)

# 6. Display results
pipeline.print_comparison_table(results)
```

### Single Model Evaluation

```python
# Evaluate just Random Forest
result = pipeline.evaluate_single_model('random_forest', test_data)
print(f"Precision@5: {result.precision_at_5:.4f}")
print(f"MRR: {result.mrr:.4f}")
```

### Generate Recommendations

```python
# Create recommendation system for a specific model
rec_system = UnifiedRecommendationSystem(
    models['random_forest'],
    loader.scaler,
    champion_names
)

# User answers
user_answers = {
    'role': 'Mage',
    'difficulty': 'Medium (4-6)',
    'playstyle': 'High Damage Output'
}

# Preprocess
features = rec_system.preprocess_user_input(user_answers)

# Get top 5 recommendations
top_5, scores = rec_system.generate_recommendations(features, k=5)
print(f"Top 5 Champions: {top_5}")
print(f"Confidence Scores: {scores}")
```

## Metrics Explained

### Precision@K

**Formula:** `Precision@K = (Relevant items in top-K) / K`

**Example:**
- Top 5 recommendations: [Ahri, Lux, Zed, Yasuo, Garen]
- Relevant champions: {Ahri, Lux, Annie}
- Relevant in top-5: Ahri, Lux (2 champions)
- Precision@5 = 2/5 = 0.4 (40%)

**Interpretation:**
- 1.0 (100%): Perfect - all top-K are relevant
- 0.7-0.9: Excellent
- 0.4-0.6: Good
- <0.4: Needs improvement

### Mean Reciprocal Rank (MRR)

**Formula:** `MRR = Average(1 / rank_of_first_relevant)`

**Example:**
- User 1: First relevant at position 1 → RR = 1/1 = 1.0
- User 2: First relevant at position 3 → RR = 1/3 = 0.333
- User 3: First relevant at position 2 → RR = 1/2 = 0.5
- MRR = (1.0 + 0.333 + 0.5) / 3 = 0.611

**Interpretation:**
- 1.0: Perfect - first recommendation always relevant
- 0.7-0.9: Excellent early ranking
- 0.4-0.6: Good early ranking
- <0.4: Poor early ranking

## Test Data Format

### JSON Format

```json
[
  {
    "id": "user_001",
    "role": "Mage",
    "difficulty": "Medium (4-6)",
    "playstyle": "High Damage Output",
    "ground_truth": ["Ahri", "Lux", "Syndra"]
  },
  {
    "id": "user_002",
    "role": "Fighter",
    "difficulty": "Easy (1-3)",
    "ground_truth": ["Garen", "Darius"]
  }
]
```

### CSV Format

```csv
id,role,difficulty,playstyle,ground_truth
user_001,Mage,Medium (4-6),High Damage Output,"Ahri,Lux,Syndra"
user_002,Fighter,Easy (1-3),Tanky and Durable,"Garen,Darius"
```

## Output Example

```
======================================================================
MODEL COMPARISON RESULTS
======================================================================
Model                P@1        P@3        P@5        MRR        Users     
----------------------------------------------------------------------
Random Forest        0.8500     0.7333     0.6400     0.8750     50        
Decision Tree        0.7800     0.6667     0.5800     0.8100     50        
Knn                  0.8200     0.7000     0.6200     0.8500     50        
======================================================================

🏆 Best Model by Precision@5: Random Forest (0.6400)
🏆 Best Model by MRR: Random Forest (0.8750)

✨ RECOMMENDED MODEL FOR PRODUCTION: Random Forest
```

## Integration with Website

### During Testing/Validation
```python
# Run evaluation pipeline
results = pipeline.compare_all_models(test_data)

# Select best model
best_model = max(results, key=lambda x: x.precision_at_5)
print(f"Use {best_model.model_name} for production")
```

### During Live Prediction
```python
# Website collects user answers
user_answers = get_user_questionnaire_responses()

# Preprocess
features = rec_system.preprocess_user_input(user_answers)

# Generate recommendations (no evaluation)
recommendations, scores = rec_system.generate_recommendations(features, k=5)

# Display to user
display_recommendations(recommendations, scores)
```

## Requirements

```bash
pip install numpy pandas scikit-learn
```

## File Structure

```
PYTHON_TESTING/
├── ml_evaluation_pipeline.py      # Main evaluation pipeline
├── ML_EVALUATION_README.md        # This file
├── user_data_backup.json          # Test dataset
└── requirements.txt               # Python dependencies

archive/models/trained/
├── random_forest_model.pkl        # Trained Random Forest
├── decision_tree_model.pkl        # Trained Decision Tree
├── knn_model.pkl                  # Trained KNN
├── scaler.pkl                     # Feature scaler
└── metadata.pkl                   # Model metadata
```

## Troubleshooting

### Models not found
```bash
# Check if model files exist
ls -la ../archive/models/trained/

# If missing, train models first
python train_models.py
```

### Test data not found
```bash
# Check if test data exists
ls -la user_data_backup.json

# Create sample test data
python create_test_dataset.py
```

### Import errors
```bash
# Install dependencies
pip install -r requirements.txt
```

## Best Practices

1. **Always use test data separate from training data**
2. **Run evaluation after any model changes**
3. **Compare all three models before production**
4. **Monitor metrics over time**
5. **Update test dataset regularly**
6. **Document model performance**

## Next Steps

1. ✅ Run initial evaluation: `python ml_evaluation_pipeline.py`
2. ✅ Review comparison table
3. ✅ Select best model for production
4. ✅ Integrate selected model into website
5. ✅ Monitor live performance
6. ✅ Re-evaluate periodically

## Notes

- Evaluation runs on **test data only**, not during live predictions
- Metrics measure **ranking quality**, not just accuracy
- **Ground truth** is required for proper evaluation
- **MRR** is especially important for recommendation systems
- **Precision@5** shows overall top-5 quality

---

**For questions or issues, refer to the main project documentation.**
