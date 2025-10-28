#!/usr/bin/env python3
"""
Champion Recommender User Data Manager
This script creates and manages CSV files for user data storage in the PYTHON_TESTING folder.
"""

import csv
import json
import os
from datetime import datetime
from pathlib import Path

# Import the evaluation metrics module
from evaluation_metrics import EvaluationMetrics

class UserDataManager:
    def __init__(self, csv_file='champion_recommender_users.csv', json_backup='user_data_backup.json'):
        # Ensure we're working in the PYTHON_TESTING directory
        self.base_dir = Path(__file__).parent
        self.csv_file = self.base_dir / csv_file
        self.json_backup = self.base_dir / json_backup
        self.users_data = self.load_data()
    
    def load_data(self):
        """Load existing user data from JSON backup file"""
        if self.json_backup.exists():
            with open(self.json_backup, 'r') as f:
                return json.load(f)
        return []
    
    def save_data(self):
        """Save user data to JSON backup file"""
        with open(self.json_backup, 'w') as f:
            json.dump(self.users_data, f, indent=2)
    
    def add_user(self, user_data):
        """Add a new user to the database and update CSV"""
        user_data['id'] = len(self.users_data) + 1
        user_data['registration_timestamp'] = datetime.now().isoformat()
        self.users_data.append(user_data)
        self.save_data()
        self.create_csv_file()
        print(f"User {user_data.get('full_name', 'Unknown')} added successfully!")
        print(f"CSV file updated: {self.csv_file}")
    
    def create_csv_file(self):
        """Create/Update CSV file from user data"""
        if not self.users_data:
            print("No user data available to create CSV file.")
            return
        
        # Define CSV headers
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
        
        # Write CSV file
        with open(self.csv_file, 'w', newline='', encoding='utf-8') as csvfile:
            writer = csv.DictWriter(csvfile, fieldnames=headers)
            writer.writeheader()
            
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
            
            for user in self.users_data:
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
        
        print(f"CSV file '{self.csv_file}' created/updated successfully with {len(self.users_data)} users!")
    
    def update_user_results(self, session_id, results_data):
        """Update user with recommendation results"""
        for user in self.users_data:
            if user.get('session_id') == session_id:
                user.update(results_data)
                user['completion_timestamp'] = datetime.now().isoformat()
                self.save_data()
                self.create_csv_file()
                print(f"Results updated for user with session ID: {session_id}")
                return True
        print(f"User with session ID {session_id} not found.")
        return False
    
    def get_user_count(self):
        """Get total number of registered users"""
        return len(self.users_data)
    
    def get_csv_path(self):
        """Get the full path to the CSV file"""
        return str(self.csv_file.absolute())
    
    def calculate_evaluation_metrics(self):
        """Calculate and display evaluation metrics for all users"""
        try:
            # Import here to avoid circular imports
            from evaluation_metrics import evaluate_model_performance
            results = evaluate_model_performance()
            return results
        except Exception as e:
            print(f"Error calculating evaluation metrics: {e}")
            return {}

# Example usage and test functions
def add_sample_user():
    """Add a sample user for testing"""
    manager = UserDataManager()
    
    sample_user = {
        'full_name': 'John Doe',
        'email': 'john.doe@example.com',
        'phone': '+1234567890',
        'age': 25,
        'gender': 'Male',
        'lol_experience': 'Intermediate',
        'session_id': f'session_{datetime.now().timestamp()}'
    }
    
    manager.add_user(sample_user)
    return manager

def main():
    """Main function to demonstrate usage"""
    print("Champion Recommender User Data Manager")
    print("=" * 50)
    print(f"Working directory: {Path(__file__).parent.absolute()}")
    
    manager = UserDataManager()
    
    while True:
        print("\nOptions:")
        print("1. Add sample user")
        print("2. View user count")
        print("3. Create/Update CSV file")
        print("4. Show CSV file path")
        print("5. Calculate evaluation metrics")
        print("6. Exit")
        
        choice = input("\nEnter your choice (1-6): ").strip()
        
        if choice == '1':
            manager = add_sample_user()
        elif choice == '2':
            count = manager.get_user_count()
            print(f"Total registered users: {count}")
        elif choice == '3':
            manager.create_csv_file()
        elif choice == '4':
            print(f"CSV file location: {manager.get_csv_path()}")
        elif choice == '5':
            manager.calculate_evaluation_metrics()
        elif choice == '6':
            print("Goodbye!")
            break
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()