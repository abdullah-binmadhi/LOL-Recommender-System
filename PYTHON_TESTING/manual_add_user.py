#!/usr/bin/env python3
"""
Manual script to add a user directly to CSV file
Use this if the server connection isn't working
"""

import csv
import json
from datetime import datetime

def add_user_manually():
    """Add a user manually to the CSV file"""
    print("📝 Manual User Registration")
    print("=" * 30)
    
    # Get user input
    full_name = input("Enter your full name: ")
    email = input("Enter your email: ")
    phone = input("Enter your phone (optional): ")
    age = input("Enter your age: ")
    gender = input("Enter your gender: ")
    experience = input("Enter your LoL experience (Beginner/Intermediate/Advanced/Expert): ")
    
    # Create user data
    user_data = {
        'ID': '',  # Will be set when we read existing data
        'Full Name': full_name,
        'Email': email,
        'Phone': phone,
        'Age': age,
        'Gender': gender,
        'LoL Experience': experience,
        'Registration Date': datetime.now().isoformat() + 'Z',
        'Session ID': f'manual_session_{datetime.now().timestamp()}',
        'Recommended Champion': '',
        'Winning Algorithm': '',
        'Confidence Score': '',
        'Random Forest Champion': '',
        'Random Forest Confidence': '',
        'Decision Tree Champion': '',
        'Decision Tree Confidence': '',
        'KNN Champion': '',
        'KNN Confidence': '',
        'Consensus Level': '',
        'User Answers': '',
        'Completion Date': ''
    }
    
    # Read existing data
    existing_data = []
    try:
        with open('champion_recommender_users.csv', 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.DictReader(csvfile)
            existing_data = list(reader)
    except FileNotFoundError:
        print("CSV file not found!")
        return
    
    # Set ID
    user_data['ID'] = len(existing_data) + 1
    
    # Add new user
    existing_data.append(user_data)
    
    # Write back to CSV
    fieldnames = [
        'ID', 'Full Name', 'Email', 'Phone', 'Age', 'Gender', 
        'LoL Experience', 'Registration Date', 'Session ID',
        'Recommended Champion', 'Winning Algorithm', 'Confidence Score',
        'Random Forest Champion', 'Random Forest Confidence',
        'Decision Tree Champion', 'Decision Tree Confidence',
        'KNN Champion', 'KNN Confidence', 'Consensus Level',
        'User Answers', 'Completion Date'
    ]
    
    with open('champion_recommender_users.csv', 'w', newline='', encoding='utf-8') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(existing_data)
    
    print(f"\n✅ User '{full_name}' added successfully!")
    print(f"📊 Total users in CSV: {len(existing_data)}")
    print(f"📁 CSV file: champion_recommender_users.csv")

if __name__ == "__main__":
    import os
    os.chdir('PYTHON_TESTING')
    add_user_manually()