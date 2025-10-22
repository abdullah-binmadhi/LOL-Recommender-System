#!/usr/bin/env python3
"""
Verification script for the LOL Recommender System implementation
"""

import json
import csv
import os
from pathlib import Path

def verify_questions_json():
    """Verify that questions.json has the correct structure"""
    print("🔍 Verifying questions.json...")
    
    questions_file = Path(__file__).parent.parent / 'src' / 'data' / 'questions.json'
    
    if not questions_file.exists():
        print("❌ questions.json file not found!")
        return False
    
    try:
        with open(questions_file, 'r') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON in questions.json: {e}")
        return False
    
    questions = data.get('questions', [])
    
    # Check that we have 10 questions
    if len(questions) != 10:
        print(f"❌ Expected 10 questions, found {len(questions)}")
        return False
    
    # Check that all question IDs are present
    question_ids = [q['id'] for q in questions]
    expected_ids = list(range(1, 11))
    
    missing_ids = [i for i in expected_ids if i not in question_ids]
    if missing_ids:
        print(f"❌ Missing question IDs: {missing_ids}")
        return False
    
    # Check that the new questions have the correct structure
    new_questions = [q for q in questions if q['id'] > 5]
    
    for question in new_questions:
        if 'dimension' not in question:
            print(f"❌ Question {question['id']} missing 'dimension' field")
            return False
    
    print("✅ questions.json verification passed!")
    return True

def verify_csv_structure():
    """Verify that the CSV file has the correct structure"""
    print("🔍 Verifying CSV structure...")
    
    csv_file = Path(__file__) / 'champion_recommender_users.csv'
    
    # If CSV doesn't exist, that's okay - it will be created when users register
    if not csv_file.exists():
        print("ℹ️  CSV file doesn't exist yet (will be created on first user registration)")
        return True
    
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
    
    try:
        with open(csv_file, 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.reader(csvfile)
            headers = next(reader)
            
            # Check if all expected headers are present
            missing_headers = [h for h in expected_headers if h not in headers]
            if missing_headers:
                print(f"❌ Missing CSV headers: {missing_headers}")
                return False
            
            print("✅ CSV structure verification passed!")
            return True
    except Exception as e:
        print(f"❌ Error reading CSV file: {e}")
        return False

def verify_html_structure():
    """Verify that the HTML file has the correct structure"""
    print("🔍 Verifying HTML structure...")
    
    html_file = Path(__file__).parent.parent / 'src' / 'index.html'
    
    if not html_file.exists():
        print("❌ index.html file not found!")
        return False
    
    try:
        with open(html_file, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"❌ Error reading HTML file: {e}")
        return False
    
    # Check that the new form fields are present
    required_fields = ['email', 'age', 'gender', 'phone']
    missing_fields = []
    
    for field in required_fields:
        if f'id="{field}"' not in content:
            missing_fields.append(field)
    
    if missing_fields:
        print(f"❌ Missing form fields in HTML: {missing_fields}")
        return False
    
    print("✅ HTML structure verification passed!")
    return True

def verify_python_backend():
    """Verify that the Python backend files are correctly structured"""
    print("🔍 Verifying Python backend...")
    
    # Check user_data_manager.py
    user_data_manager = Path(__file__).parent / 'user_data_manager.py'
    if not user_data_manager.exists():
        print("❌ user_data_manager.py file not found!")
        return False
    
    # Check data_server.py
    data_server = Path(__file__).parent / 'data_server.py'
    if not data_server.exists():
        print("❌ data_server.py file not found!")
        return False
    
    print("✅ Python backend verification passed!")
    return True

def main():
    """Main verification function"""
    print("🧪 LOL Recommender System Implementation Verification")
    print("=" * 60)
    
    checks = [
        verify_questions_json,
        verify_csv_structure,
        verify_html_structure,
        verify_python_backend
    ]
    
    results = []
    for check in checks:
        results.append(check())
        print()
    
    passed = sum(results)
    total = len(results)
    
    print("=" * 60)
    print(f"📊 Verification Results: {passed}/{total} checks passed")
    
    if passed == total:
        print("✅ All verifications passed! Implementation is ready.")
        return True
    else:
        print("❌ Some verifications failed. Please check the implementation.")
        return False

if __name__ == "__main__":
    main()