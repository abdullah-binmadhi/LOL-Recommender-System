#!/usr/bin/env python3
"""
Simple CSV writer that can be called from command line
This bypasses server issues and writes directly to CSV
"""

import csv
import json
import sys
from datetime import datetime
from pathlib import Path

def write_user_to_csv(user_data_json):
    """Write user data directly to CSV file"""
    
    # Parse the JSON data
    try:
        user_data = json.loads(user_data_json)
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON: {e}")
        return False
    
    csv_file = Path('champion_recommender_users.csv')
    
    # Define headers
    headers = [
        'ID', 'Full Name', 'Email', 'Phone', 'Age', 'Gender', 
        'LoL Experience', 'Registration Date', 'Session ID',
        'Recommended Champion', 'Winning Algorithm', 'Confidence Score',
        'Random Forest Champion', 'Random Forest Confidence',
        'Decision Tree Champion', 'Decision Tree Confidence',
        'KNN Champion', 'KNN Confidence', 'Consensus Level',
        'User Answers', 'Completion Date'
    ]
    
    # Read existing data
    existing_data = []
    if csv_file.exists():
        with open(csv_file, 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.DictReader(csvfile)
            existing_data = list(reader)
    
    # Create new user row
    new_user = {
        'ID': len(existing_data) + 1,
        'Full Name': user_data.get('fullName', ''),
        'Email': user_data.get('email', ''),
        'Phone': user_data.get('phone', ''),
        'Age': user_data.get('age', ''),
        'Gender': user_data.get('gender', ''),
        'LoL Experience': user_data.get('experience', ''),
        'Registration Date': datetime.now().isoformat() + 'Z',
        'Session ID': user_data.get('sessionId', f'session_{datetime.now().timestamp()}'),
        'Recommended Champion': user_data.get('recommendedChampion', ''),
        'Winning Algorithm': user_data.get('winningAlgorithm', ''),
        'Confidence Score': user_data.get('confidence', ''),
        'Random Forest Champion': user_data.get('randomForestChampion', ''),
        'Random Forest Confidence': user_data.get('randomForestConfidence', ''),
        'Decision Tree Champion': user_data.get('decisionTreeChampion', ''),
        'Decision Tree Confidence': user_data.get('decisionTreeConfidence', ''),
        'KNN Champion': user_data.get('knnChampion', ''),
        'KNN Confidence': user_data.get('knnConfidence', ''),
        'Consensus Level': user_data.get('consensusLevel', ''),
        'User Answers': user_data.get('userAnswers', ''),
        'Completion Date': user_data.get('completionDate', '')
    }
    
    # Add to existing data
    existing_data.append(new_user)
    
    # Write to CSV
    with open(csv_file, 'w', newline='', encoding='utf-8') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=headers)
        writer.writeheader()
        writer.writerows(existing_data)
    
    print(f"✅ User '{new_user['Full Name']}' added to CSV successfully!")
    print(f"📊 Total users: {len(existing_data)}")
    print(f"📁 CSV file: {csv_file.absolute()}")
    return True

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python simple_csv_writer.py '<json_data>'")
        print("Example: python simple_csv_writer.py '{\"fullName\":\"John Doe\",\"email\":\"john@example.com\",\"age\":25}'")
        sys.exit(1)
    
    # Change to script directory
    script_dir = Path(__file__).parent
    import os
    os.chdir(script_dir)
    
    success = write_user_to_csv(sys.argv[1])
    sys.exit(0 if success else 1)