#!/usr/bin/env python3
"""
Test script for evaluation metrics
"""

import json
from pathlib import Path
from evaluation_metrics import EvaluationMetrics, evaluate_model_performance

def create_sample_data():
    """Create sample user data for testing"""
    # Sample champion data
    sample_champions = [
        {'name': 'Ahri', 'role': 'Mage', 'difficulty': 6, 'damage': 8, 'toughness': 3},
        {'name': 'Garen', 'role': 'Fighter', 'difficulty': 5, 'damage': 7, 'toughness': 7},
        {'name': 'Thresh', 'role': 'Support', 'difficulty': 7, 'damage': 5, 'toughness': 6},
        {'name': 'Jinx', 'role': 'Marksman', 'difficulty': 6, 'damage': 9, 'toughness': 3},
        {'name': 'Malphite', 'role': 'Tank', 'difficulty': 4, 'damage': 6, 'toughness': 8},
        {'name': 'Yasuo', 'role': 'Fighter', 'difficulty': 8, 'damage': 9, 'toughness': 4},
        {'name': 'Lux', 'role': 'Mage', 'difficulty': 5, 'damage': 8, 'toughness': 3},
        {'name': 'Braum', 'role': 'Support', 'difficulty': 3, 'damage': 3, 'toughness': 9},
        {'name': 'Zed', 'role': 'Assassin', 'difficulty': 9, 'damage': 9, 'toughness': 3},
        {'name': 'Leona', 'role': 'Support', 'difficulty': 5, 'damage': 5, 'toughness': 9}
    ]
    
    # Sample user data
    sample_users = [
        {
            'id': 1,
            'full_name': 'Alice Johnson',
            'role': 'Mage',
            'difficulty': '6',
            'damage': '8',
            'toughness': '3',
            'recommended_champion': 'Ahri',
            'random_forest_champion': 'Lux',
            'decision_tree_champion': 'Ahri',
            'knn_champion': 'Zed',
            'confidence_score': '0.85',
            'random_forest_confidence': '0.78',
            'decision_tree_confidence': '0.92',
            'knn_confidence': '0.71'
        },
        {
            'id': 2,
            'full_name': 'Bob Smith',
            'role': 'Fighter',
            'difficulty': '7',
            'damage': '8',
            'toughness': '5',
            'recommended_champion': 'Yasuo',
            'random_forest_champion': 'Garen',
            'decision_tree_champion': 'Yasuo',
            'knn_champion': 'Jinx',
            'confidence_score': '0.88',
            'random_forest_confidence': '0.72',
            'decision_tree_confidence': '0.91',
            'knn_confidence': '0.69'
        },
        {
            'id': 3,
            'full_name': 'Carol Williams',
            'role': 'Support',
            'difficulty': '4',
            'damage': '4',
            'toughness': '8',
            'recommended_champion': 'Thresh',
            'random_forest_champion': 'Braum',
            'decision_tree_champion': 'Leona',
            'knn_champion': 'Thresh',
            'confidence_score': '0.82',
            'random_forest_confidence': '0.75',
            'decision_tree_confidence': '0.79',
            'knn_confidence': '0.88'
        }
    ]
    
    # Save sample data
    base_dir = Path(__file__).parent
    json_file = base_dir / 'user_data_backup.json'
    
    with open(json_file, 'w') as f:
        json.dump(sample_users, f, indent=2)
    
    print(f"Sample data created with {len(sample_users)} users")
    return sample_users, sample_champions

def test_precision_at_k():
    """Test Precision@K metric"""
    print("Testing Precision@K metric...")
    
    # Test case 1
    recommended = ['Ahri', 'Garen', 'Thresh', 'Jinx', 'Malphite']
    relevant = {'Ahri', 'Lux', 'Yasuo'}  # Ahri is relevant
    
    for k in [1, 3, 5]:
        precision = EvaluationMetrics.precision_at_k(recommended, relevant, k)
        print(f"  Precision@{k}: {precision:.3f}")
    
    print()

def test_mean_reciprocal_rank():
    """Test Mean Reciprocal Rank metric"""
    print("Testing Mean Reciprocal Rank metric...")
    
    # Test case 1: Relevant item at position 1
    recommended = ['Ahri', 'Garen', 'Thresh', 'Jinx', 'Malphite']
    relevant = {'Ahri'}
    mrr = EvaluationMetrics.mean_reciprocal_rank(recommended, relevant)
    print(f"  MRR (relevant at position 1): {mrr:.3f}")
    
    # Test case 2: Relevant item at position 3
    relevant = {'Thresh'}
    mrr = EvaluationMetrics.mean_reciprocal_rank(recommended, relevant)
    print(f"  MRR (relevant at position 3): {mrr:.3f}")
    
    # Test case 3: No relevant items
    relevant = {'Zed'}
    mrr = EvaluationMetrics.mean_reciprocal_rank(recommended, relevant)
    print(f"  MRR (no relevant items): {mrr:.3f}")
    
    print()

def test_user_relevance():
    """Test user relevance calculation"""
    print("Testing user relevance calculation...")
    
    # Sample user data
    user_data = {
        'role': 'Mage',
        'difficulty': '6',
        'damage': '8',
        'toughness': '3'
    }
    
    # Sample champions
    sample_champions = [
        {'name': 'Ahri', 'role': 'Mage', 'difficulty': 6, 'damage': 8, 'toughness': 3},
        {'name': 'Garen', 'role': 'Fighter', 'difficulty': 5, 'damage': 7, 'toughness': 7},
        {'name': 'Lux', 'role': 'Mage', 'difficulty': 5, 'damage': 8, 'toughness': 3},
        {'name': 'Zed', 'role': 'Assassin', 'difficulty': 9, 'damage': 9, 'toughness': 3}
    ]
    
    relevant_champions = EvaluationMetrics.calculate_user_relevance(user_data, sample_champions)
    print(f"  Relevant champions for Mage player: {relevant_champions}")
    print()

def test_full_evaluation():
    """Test full evaluation"""
    print("Testing full evaluation...")
    
    # Create sample data
    create_sample_data()
    
    # Run evaluation
    results = evaluate_model_performance()
    
    print("Evaluation complete!")
    print()

def main():
    """Main test function"""
    print("Evaluation Metrics Test Suite")
    print("=" * 40)
    
    test_precision_at_k()
    test_mean_reciprocal_rank()
    test_user_relevance()
    test_full_evaluation()
    
    print("All tests completed!")

if __name__ == "__main__":
    main()