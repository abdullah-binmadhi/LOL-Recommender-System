#!/usr/bin/env python3
"""
Script to update existing CSV files with evaluation metrics
"""

import csv
import json
from pathlib import Path
from evaluation_metrics import EvaluationMetrics

def update_csv_with_metrics():
    """Update the existing CSV file with evaluation metrics"""
    # Define paths
    base_dir = Path(__file__).parent
    csv_file = base_dir / 'champion_recommender_users.csv'
    json_file = base_dir / 'user_data_backup.json'
    
    # Check if files exist
    if not csv_file.exists():
        print(f"CSV file not found: {csv_file}")
        return False
        
    if not json_file.exists():
        print(f"JSON file not found: {json_file}")
        return False
    
    # Load user data from JSON
    try:
        with open(json_file, 'r') as f:
            users_data = json.load(f)
    except Exception as e:
        print(f"Error loading JSON data: {e}")
        return False
    
    # Sample champions for relevance calculation
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
    
    # Define updated CSV headers with metrics
    headers = [
        'ID', 'Full Name', 'Email', 'Phone', 'Age', 'Gender', 
        'LoL Experience', 'Registration Date', 'Session ID',
        'Recommended Champion', 'Winning Algorithm', 'Confidence Score',
        'Random Forest Champion', 'Random Forest Confidence',
        'Decision Tree Champion', 'Decision Tree Confidence',
        'KNN Champion', 'KNN Confidence', 'Consensus Level',
        'User Answers', 'Completion Date',
        'Pressure Response', 'Aesthetic Preference', 'Team Contribution',
        'Character Identity', 'Problem Solving Approach',
        'Precision@1', 'Precision@3', 'Precision@5', 'MRR'
    ]
    
    # Read existing CSV data
    existing_rows = []
    try:
        with open(csv_file, 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.reader(csvfile)
            existing_rows = list(reader)
    except Exception as e:
        print(f"Error reading CSV file: {e}")
        return False
    
    # Check if metrics columns already exist
    if len(existing_rows) > 0 and len(existing_rows[0]) == len(headers):
        print("CSV file already has metrics columns")
        return True
    
    # Create backup of original CSV
    backup_file = csv_file.with_suffix('.csv.backup')
    try:
        with open(csv_file, 'r', newline='', encoding='utf-8') as original, \
             open(backup_file, 'w', newline='', encoding='utf-8') as backup:
            backup.write(original.read())
        print(f"Backup created: {backup_file}")
    except Exception as e:
        print(f"Error creating backup: {e}")
    
    # Create updated CSV with metrics
    try:
        with open(csv_file, 'w', newline='', encoding='utf-8') as csvfile:
            writer = csv.DictWriter(csvfile, fieldnames=headers)
            writer.writeheader()
            
            # Process each user row
            for i, user in enumerate(users_data):
                # Calculate evaluation metrics for this user
                recommended_champions = EvaluationMetrics.get_recommended_champions(user)
                relevant_champions = EvaluationMetrics.calculate_user_relevance(user, sample_champions)
                
                precision_at_1 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 1)
                precision_at_3 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 3)
                precision_at_5 = EvaluationMetrics.precision_at_k(recommended_champions, relevant_champions, 5)
                mrr = EvaluationMetrics.mean_reciprocal_rank(recommended_champions, relevant_champions)
                
                row = {
                    'ID': user.get('id', ''),
                    'Full Name': user.get('full_name', ''),
                    'Email': user.get('email', ''),
                    'Phone': user.get('phone', ''),
                    'Age': user.get('age', ''),
                    'Gender': user.get('gender', ''),
                    'LoL Experience': user.get('lol_experience', ''),
                    'Registration Date': user.get('registration_timestamp', ''),
                    'Session ID': user.get('session_id', ''),
                    'Recommended Champion': user.get('recommended_champion', ''),
                    'Winning Algorithm': user.get('winning_algorithm', ''),
                    'Confidence Score': user.get('confidence_score', ''),
                    'Random Forest Champion': user.get('random_forest_champion', ''),
                    'Random Forest Confidence': user.get('random_forest_confidence', ''),
                    'Decision Tree Champion': user.get('decision_tree_champion', ''),
                    'Decision Tree Confidence': user.get('decision_tree_confidence', ''),
                    'KNN Champion': user.get('knn_champion', ''),
                    'KNN Confidence': user.get('knn_confidence', ''),
                    'Consensus Level': user.get('consensus_level', ''),
                    'User Answers': user.get('user_answers', ''),
                    'Completion Date': user.get('completion_timestamp', ''),
                    'Pressure Response': user.get('pressure_response', ''),
                    'Aesthetic Preference': user.get('aesthetic_preference', ''),
                    'Team Contribution': user.get('team_contribution', ''),
                    'Character Identity': user.get('character_identity', ''),
                    'Problem Solving Approach': user.get('problem_solving', ''),
                    'Precision@1': f"{precision_at_1:.3f}",
                    'Precision@3': f"{precision_at_3:.3f}",
                    'Precision@5': f"{precision_at_5:.3f}",
                    'MRR': f"{mrr:.3f}"
                }
                writer.writerow(row)
        
        print(f"CSV file updated successfully with evaluation metrics: {csv_file}")
        return True
        
    except Exception as e:
        print(f"Error updating CSV file: {e}")
        return False

def main():
    """Main function"""
    print("Updating CSV file with evaluation metrics...")
    success = update_csv_with_metrics()
    if success:
        print("Update completed successfully!")
    else:
        print("Update failed!")

if __name__ == "__main__":
    main()