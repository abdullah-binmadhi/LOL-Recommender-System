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
            'User Answers', 'Completion Date'
        ]
        
        # Write CSV file
        with open(self.csv_file, 'w', newline='', encoding='utf-8') as csvfile:
            writer = csv.DictWriter(csvfile, fieldnames=headers)
            writer.writeheader()
            
            for user in self.users_data:
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
                    'Completion Date': user.get('completion_timestamp', '')
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
        print("5. Exit")
        
        choice = input("\nEnter your choice (1-5): ").strip()
        
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
            print("Goodbye!")
            break
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()