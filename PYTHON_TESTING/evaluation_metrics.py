#!/usr/bin/env python3
"""
Evaluation Metrics for Champion Recommender System
Implements Precision@K and Mean Reciprocal Rank (MRR) metrics
"""

import json
import csv
from pathlib import Path
from typing import List, Set, Dict, Any, Tuple, Optional

class EvaluationMetrics:
    """Class to calculate evaluation metrics for champion recommendations"""
    
    @staticmethod
    def precision_at_k(recommended_champions: List[str], relevant_champions: Set[str], k: int) -> float:
        """
        Calculate Precision@K metric
        
        Args:
            recommended_champions: List of champions recommended by the model (ranked)
            relevant_champions: Set of champions that are actually relevant to the user
            k: Number of top recommendations to consider
            
        Returns:
            float: Precision@K score
        """
        if not recommended_champions or k <= 0:
            return 0.0
            
        # Consider only top K recommendations
        top_k_recommendations = recommended_champions[:k]
        
        # Count how many of the top K recommendations are relevant
        relevant_count = sum(1 for champ in top_k_recommendations if champ in relevant_champions)
        
        # Calculate precision
        precision = relevant_count / min(k, len(top_k_recommendations))
        return precision
    
    @staticmethod
    def mean_reciprocal_rank(recommended_champions: List[str], relevant_champions: Set[str]) -> float:
        """
        Calculate Mean Reciprocal Rank (MRR) metric
        
        Args:
            recommended_champions: List of champions recommended by the model (ranked)
            relevant_champions: Set of champions that are actually relevant to the user
            
        Returns:
            float: MRR score
        """
        if not recommended_champions:
            return 0.0
            
        # Find the rank of the first relevant champion
        for rank, champion in enumerate(recommended_champions, 1):
            if champion in relevant_champions:
                return 1.0 / rank
                
        # If no relevant champion is found, return 0
        return 0.0
    
    @staticmethod
    def calculate_user_relevance(user_data: Dict[str, Any], all_champions: List[Dict[str, Any]]) -> Set[str]:
        """
        Determine which champions are relevant to a user based on their preferences
        
        Args:
            user_data: Dictionary containing user responses and preferences
            all_champions: List of all champion data
            
        Returns:
            Set[str]: Set of champion names that are relevant to the user
        """
        relevant_champions = set()
        
        # Extract user preferences
        user_role = user_data.get('role', user_data.get('1', ''))  # Question 1: Role
        user_difficulty = user_data.get('difficulty', user_data.get('2', '5'))  # Question 2: Difficulty
        user_damage = user_data.get('damage', '5')
        user_toughness = user_data.get('toughness', '5')
        
        # Convert difficulty to numeric if it's a string range
        if isinstance(user_difficulty, str):
            # Extract numeric value from strings like "Easy (1-3)"
            if '1-3' in user_difficulty:
                user_difficulty = 2
            elif '4-6' in user_difficulty:
                user_difficulty = 5
            elif '7-8' in user_difficulty:
                user_difficulty = 7.5
            elif '9-10' in user_difficulty:
                user_difficulty = 9.5
            else:
                try:
                    user_difficulty = float(user_difficulty)
                except ValueError:
                    user_difficulty = 5
        
        try:
            user_difficulty = float(user_difficulty)
        except (ValueError, TypeError):
            user_difficulty = 5
            
        try:
            user_damage = float(user_damage)
        except (ValueError, TypeError):
            user_damage = 5
            
        try:
            user_toughness = float(user_toughness)
        except (ValueError, TypeError):
            user_toughness = 5
        
        # Determine relevant champions based on user preferences
        for champion in all_champions:
            # Check if champion matches user's role preference
            role_match = (user_role == 'No Preference' or 
                         user_role == 'No preference' or 
                         user_role == '' or 
                         user_role == champion.get('role', ''))
            
            # Check if champion matches difficulty preference (within 2 points)
            difficulty_match = abs(champion.get('difficulty', 5) - user_difficulty) <= 2
            
            # Check if champion matches damage preference (within 2 points)
            damage_match = abs(champion.get('damage', 5) - user_damage) <= 2
            
            # Check if champion matches toughness preference (within 2 points)
            toughness_match = abs(champion.get('toughness', 5) - user_toughness) <= 2
            
            # If champion matches key preferences, consider it relevant
            if role_match and difficulty_match and (damage_match or toughness_match):
                relevant_champions.add(champion.get('name', ''))
        
        return relevant_champions
    
    @staticmethod
    def get_recommended_champions(user_data: Dict[str, Any]) -> List[str]:
        """
        Extract recommended champions from user data
        
        Args:
            user_data: Dictionary containing user recommendation results
            
        Returns:
            List[str]: List of champion names recommended to the user (ranked by confidence)
        """
        recommendations = []
        
        # Collect all recommendations with their confidence scores
        champ_confidence_pairs = []
        
        # Main recommendation
        if user_data.get('recommended_champion'):
            confidence = user_data.get('confidence_score', 0)
            try:
                confidence = float(confidence)
            except (ValueError, TypeError):
                confidence = 0
            champ_confidence_pairs.append((user_data['recommended_champion'], confidence))
        
        # Random Forest recommendation
        if user_data.get('random_forest_champion'):
            confidence = user_data.get('random_forest_confidence', 0)
            try:
                confidence = float(confidence)
            except (ValueError, TypeError):
                confidence = 0
            champ_confidence_pairs.append((user_data['random_forest_champion'], confidence))
        
        # Decision Tree recommendation
        if user_data.get('decision_tree_champion'):
            confidence = user_data.get('decision_tree_confidence', 0)
            try:
                confidence = float(confidence)
            except (ValueError, TypeError):
                confidence = 0
            champ_confidence_pairs.append((user_data['decision_tree_champion'], confidence))
        
        # KNN recommendation
        if user_data.get('knn_champion'):
            confidence = user_data.get('knn_confidence', 0)
            try:
                confidence = float(confidence)
            except (ValueError, TypeError):
                confidence = 0
            champ_confidence_pairs.append((user_data['knn_champion'], confidence))
        
        # Sort by confidence score (descending)
        champ_confidence_pairs.sort(key=lambda x: x[1], reverse=True)
        
        # Extract just the champion names in order
        recommendations = [champ for champ, _ in champ_confidence_pairs]
        
        return recommendations

def evaluate_model_performance(user_data_file: str = 'user_data_backup.json', 
                              champion_data_file: Optional[str] = None) -> Dict[str, float]:
    """
    Evaluate the performance of recommendation models using Precision@K and MRR
    
    Args:
        user_data_file: Path to the JSON file containing user data
        champion_data_file: Path to champion data file (optional)
        
    Returns:
        Dict[str, float]: Dictionary containing evaluation metrics
    """
    # Load user data
    base_dir = Path(__file__).parent
    json_file = base_dir / user_data_file
    
    if not json_file.exists():
        print(f"User data file not found: {json_file}")
        return {}
    
    try:
        with open(json_file, 'r') as f:
            users_data = json.load(f)
    except Exception as e:
        print(f"Error loading user data: {e}")
        return {}
    
    # For now, we'll use a simplified champion representation
    # In a real implementation, you would load this from your champion database
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
    
    print("Champion Recommender System - Model Evaluation")
    print("=" * 50)
    print(f"Loaded data for {len(users_data)} users")
    print()
    
    # Initialize metric accumulators
    total_precision_at_1 = 0
    total_precision_at_3 = 0
    total_precision_at_5 = 0
    total_mrr = 0
    valid_users = 0
    
    # Calculate metrics for each user
    for i, user in enumerate(users_data):
        try:
            # Get recommended champions for this user
            recommended_champions = EvaluationMetrics.get_recommended_champions(user)
            
            # Determine relevant champions for this user
            relevant_champions = EvaluationMetrics.calculate_user_relevance(user, sample_champions)
            
            # Calculate metrics
            precision_at_1 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 1)
            precision_at_3 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 3)
            precision_at_5 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 5)
            mrr = EvaluationMetrics.mean_reciprocal_rank(recommended_champions, relevant_champions)
            
            total_precision_at_1 += precision_at_1
            total_precision_at_3 += precision_at_3
            total_precision_at_5 += precision_at_5
            total_mrr += mrr
            valid_users += 1
            
            # Print individual user results for first few users
            if i < 3:
                print(f"User {user.get('id', 'N/A')} Metrics:")
                print(f"  Recommended Champions: {recommended_champions[:3]}")
                print(f"  Relevant Champions: {list(relevant_champions)[:3]}")
                print(f"  Precision@1: {precision_at_1:.3f}")
                print(f"  Precision@3: {precision_at_3:.3f}")
                print(f"  Precision@5: {precision_at_5:.3f}")
                print(f"  MRR: {mrr:.3f}")
                print()
                
        except Exception as e:
            print(f"Error calculating metrics for user {user.get('id', 'unknown')}: {e}")
            continue
    
    # Calculate and display aggregate metrics
    if valid_users > 0:
        avg_precision_at_1 = total_precision_at_1 / valid_users
        avg_precision_at_3 = total_precision_at_3 / valid_users
        avg_precision_at_5 = total_precision_at_5 / valid_users
        avg_mrr = total_mrr / valid_users
        
        results = {
            'average_precision_at_1': avg_precision_at_1,
            'average_precision_at_3': avg_precision_at_3,
            'average_precision_at_5': avg_precision_at_5,
            'mean_reciprocal_rank': avg_mrr,
            'total_users_evaluated': valid_users
        }
        
        print("Aggregate Evaluation Metrics:")
        print("-" * 30)
        print(f"Average Precision@1: {avg_precision_at_1:.3f}")
        print(f"Average Precision@3: {avg_precision_at_3:.3f}")
        print(f"Average Precision@5: {avg_precision_at_5:.3f}")
        print(f"Mean Reciprocal Rank: {avg_mrr:.3f}")
        print(f"Evaluated {valid_users} users out of {len(users_data)} total")
        
        # Model performance interpretation
        print("\nPerformance Interpretation:")
        print("-" * 25)
        if avg_precision_at_5 > 0.7:
            print("Excellent model performance - high relevance in top-5 recommendations")
        elif avg_precision_at_5 > 0.5:
            print("Good model performance - reasonable relevance in top-5 recommendations")
        else:
            print("Model needs improvement - low relevance in top-5 recommendations")
            
        if avg_mrr > 0.6:
            print("Strong early recommendation capability - relevant champions recommended early")
        elif avg_mrr > 0.4:
            print("Moderate early recommendation capability")
        else:
            print("Weak early recommendation capability - relevant champions recommended late")
            
        return results
    else:
        print("No valid users found for evaluation")
        return {}

def main():
    """Main function to run evaluation"""
    evaluate_model_performance()

if __name__ == "__main__":
    main()