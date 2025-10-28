# Evaluation Metrics for Champion Recommender System

This document explains the implementation and usage of Precision@K and Mean Reciprocal Rank (MRR) evaluation metrics for the League of Legends Champion Recommender System.

## Overview

Two key evaluation metrics have been implemented to assess the performance of the champion recommendation algorithms:

1. **Precision@K** - Evaluates the overall relevance of top-ranked champions for user preferences
2. **Mean Reciprocal Rank (MRR)** - Rewards models that recommend relevant champions early for faster adoption

## Metrics Implementation

### Precision@K

Precision@K measures the proportion of recommended champions in the top-K results that are actually relevant to the user.

**Formula:**
```
Precision@K = (Number of relevant champions in top-K recommendations) / K
```

**Usage:**
- Higher values indicate better overall relevance of top-ranked champions
- Common values: Precision@1, Precision@3, Precision@5, Precision@10

### Mean Reciprocal Rank (MRR)

MRR measures how early the first relevant champion appears in the ranking.

**Formula:**
```
MRR = 1 / (Rank of first relevant champion)
```

**Usage:**
- Higher values indicate that relevant champions are recommended earlier
- Values range from 0 (no relevant champions) to 1 (first recommendation is relevant)

## Files

- `evaluation_metrics.py` - Core implementation of Precision@K and MRR metrics
- `test_evaluation_metrics.py` - Test suite for the evaluation metrics
- `update_csv_with_metrics.py` - Script to update existing CSV files with metrics
- `user_data_manager.py` - Updated to include metrics in CSV output

## Usage Examples

### Calculate Metrics for a Single User

```python
from evaluation_metrics import EvaluationMetrics

# Example user recommendations (ranked by confidence)
recommended_champions = ['Ahri', 'Lux', 'Yasuo', 'Garen', 'Jinx']

# Relevant champions for this user (based on preferences)
relevant_champions = {'Ahri', 'Lux', 'Thresh'}

# Calculate Precision@K
p_at_1 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 1)
p_at_3 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 3)
p_at_5 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 5)

# Calculate MRR
mrr = EvaluationMetrics.mean_reciprocal_rank(recommended_champions, relevant_champions)

print(f"Precision@1: {p_at_1:.3f}")
print(f"Precision@3: {p_at_3:.3f}")
print(f"Precision@5: {p_at_5:.3f}")
print(f"MRR: {mrr:.3f}")
```

### Run Full Evaluation

```bash
cd PYTHON_TESTING
python3 test_evaluation_metrics.py
```

### Update Existing CSV with Metrics

```bash
cd PYTHON_TESTING
python3 update_csv_with_metrics.py
```

## Interpretation

### Precision@K Interpretation
- **> 0.7**: Excellent model performance - high relevance in top-K recommendations
- **0.5-0.7**: Good model performance - reasonable relevance in top-K recommendations
- **< 0.5**: Model needs improvement - low relevance in top-K recommendations

### MRR Interpretation
- **> 0.6**: Strong early recommendation capability - relevant champions recommended early
- **0.4-0.6**: Moderate early recommendation capability
- **< 0.4**: Weak early recommendation capability - relevant champions recommended late

## Integration with Website

The evaluation metrics are automatically calculated and stored in the CSV file with the following columns:
- `Precision@1`
- `Precision@3`
- `Precision@5`
- `MRR`

These metrics can be used to:
1. Compare the performance of different ML algorithms (Random Forest, Decision Tree, KNN)
2. Track improvements as recommendation algorithms are refined
3. Provide performance feedback to users
4. Optimize the recommendation engine based on real usage data

## Future Improvements

1. **Ground Truth Data**: Currently, relevant champions are determined algorithmically based on user preferences. In the future, this could be improved with actual user feedback on recommended champions.

2. **Additional Metrics**: Consider implementing other metrics like Normalized Discounted Cumulative Gain (NDCG) for more comprehensive evaluation.

3. **Real-time Evaluation**: Integrate real-time metric calculation into the web application to provide immediate feedback on recommendation quality.

4. **A/B Testing**: Use these metrics to compare different versions of the recommendation engine in controlled experiments.