#!/usr/bin/env python3
"""
Test script to verify the new questions and CSV functionality
"""

import csv
import json
import os
from datetime import datetime
from pathlib import Path

def test_csv_structure():
    """Test that the CSV file has the correct structure with new columns"""
    # Define the expected headers
    expected_headers = [
        'ID', 'Full Name', 'Email', 'Phone', 'Age', 'Gender', 
        'LoL Experience', 'Registration Date', 'Session ID',
        'Recommended Champion', 'Winning Algorithm', 'Confidence Score',
        'Random Forest Champion', 'Random Forest Confidence',
        'Decision Tree Champion', 'Decision Tree Confidence',
        'KNN Champion', 'KNN Confidence', 'Consensus Level',
        'User Answers', 'Completion Date',
        'Pressure Response', 'Aesthetic Preference', 'Team Contribution',
        'Character Identity', 'Problem Solving Approach'
    ]
    
    # Check if CSV file exists
    csv_file = Path(__file__).parent / 'champion_recommender_users.csv'
    
    if not csv_file.exists():
        print("CSV file does not exist yet. Creating a test entry...")
        create_test_entry()
        return
    
    # Read the CSV file
    with open(csv_file, 'r', newline='', encoding='utf-8') as csvfile:
        reader = csv.reader(csvfile)
        headers = next(reader)
        
        print("Current CSV headers:")
        for i, header in enumerate(headers):
            print(f"  {i+1}. {header}")
        
        print("\nExpected CSV headers:")
        for i, header in enumerate(expected_headers):
            print(f"  {i+1}. {header}")
        
        # Check if all expected headers are present
        missing_headers = [h for h in expected_headers if h not in headers]
        if missing_headers:
            print(f"\n❌ Missing headers: {missing_headers}")
        else:
            print("\n✅ All expected headers are present!")

def create_test_entry():
    """Create a test entry in the CSV file"""
    csv_file = Path(__file__).parent / 'champion_recommender_users.csv'
    
    # Sample data with new fields
    sample_data = {
        'ID': 1,
        'Full Name': 'Test User',
        'Email': 'test@example.com',
        'Phone': '123-456-7890',
        'Age': 25,
        'Gender': 'Male',
        'LoL Experience': 'Intermediate',
        'Registration Date': datetime.now().isoformat(),
        'Session ID': f'session_{datetime.now().timestamp()}',
        'Recommended Champion': 'Yasuo',
        'Winning Algorithm': 'Random Forest',
        'Confidence Score': 85.5,
        'Random Forest Champion': 'Yasuo',
        'Random Forest Confidence': 85.5,
        'Decision Tree Champion': 'Zed',
        'Decision Tree Confidence': 78.2,
        'KNN Champion': 'Yasuo',
        'KNN Confidence': 82.1,
        'Consensus Level': 2,
        'User Answers': '{"1": "Hard (7-8)", "2": "Assassin", "3": "Mid", "4": "High Damage Output", "5": "Melee", "6": "Get aggressive and take risks", "7": "Dark and edgy", "8": "Take charge and lead", "9": "Male", "10": "Jump in and adapt on the fly"}',
        'Completion Date': datetime.now().isoformat(),
        'Pressure Response': 'Get aggressive and take risks',
        'Aesthetic Preference': 'Dark and edgy',
        'Team Contribution': 'Take charge and lead',
        'Character Identity': 'Male',
        'Problem Solving Approach': 'Jump in and adapt on the fly'
    }
    
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
        'Character Identity', 'Problem Solving Approach'
    ]
    
    # Write CSV file
    with open(csv_file, 'w', newline='', encoding='utf-8') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=headers)
        writer.writeheader()
        writer.writerow(sample_data)
    
    print(f"✅ Created test CSV file with sample data: {csv_file}")

def test_questions_json():
    """Test that the questions.json file has the correct structure"""
    questions_file = Path(__file__).parent.parent / 'src' / 'data' / 'questions.json'
    
    if not questions_file.exists():
        print("❌ questions.json file not found!")
        return
    
    with open(questions_file, 'r') as f:
        data = json.load(f)
    
    questions = data.get('questions', [])
    print(f"Found {len(questions)} questions in questions.json")
    
    # Check that we have 10 questions now
    if len(questions) == 10:
        print("✅ Correct number of questions (10)")
    else:
        print(f"❌ Expected 10 questions, found {len(questions)}")
    
    # Check that the new questions are present
    question_ids = [q['id'] for q in questions]
    expected_ids = list(range(1, 11))
    
    missing_ids = [i for i in expected_ids if i not in question_ids]
    if missing_ids:
        print(f"❌ Missing question IDs: {missing_ids}")
    else:
        print("✅ All expected question IDs are present")
    
    # Print the new questions
    print("\nNew psychological and behavioral questions:")
    for question in questions:
        if question['id'] > 5:
            print(f"  {question['id']}. {question['text']}")
            if 'dimension' in question:
                print(f"     Dimension: {question['dimension']}")

if __name__ == "__main__":
    print("Testing LOL Recommender System updates...")
    print("=" * 50)
    
    test_questions_json()
    print()
    test_csv_structure()
    
    print("\n✅ Testing complete!")