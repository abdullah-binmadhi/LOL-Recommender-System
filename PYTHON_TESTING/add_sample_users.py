#!/usr/bin/env python3
"""
Script to add sample users to the CSV file for testing
"""

import csv
import json
from datetime import datetime, timedelta
import random

def add_sample_users():
    """Add sample users to the CSV file"""
    
    # Sample user data
    sample_users = [
        {
            'ID': 4,
            'Full Name': 'Sarah Wilson',
            'Email': 'sarah.w@example.com',
            'Phone': '+1555123456',
            'Age': 24,
            'Gender': 'Female',
            'LoL Experience': 'Intermediate',
            'Registration Date': '2024-01-16T09:30:00.000Z',
            'Session ID': 'session_1705401000_jkl012',
            'Recommended Champion': 'Lux',
            'Winning Algorithm': 'Random Forest',
            'Confidence Score': 90.3,
            'Random Forest Champion': 'Lux',
            'Random Forest Confidence': 90.3,
            'Decision Tree Champion': 'Morgana',
            'Decision Tree Confidence': 86.7,
            'KNN Champion': 'Ahri',
            'KNN Confidence': 88.9,
            'Consensus Level': 1,
            'User Answers': '{"role":"Support","complexity":3,"difficulty":3,"damage":4,"toughness":3}',
            'Completion Date': '2024-01-16T09:35:00.000Z'
        },
        {
            'ID': 5,
            'Full Name': 'Alex Chen',
            'Email': 'alex.chen@example.com',
            'Phone': '',
            'Age': 30,
            'Gender': 'Non-binary',
            'LoL Experience': 'Expert',
            'Registration Date': '2024-01-16T14:20:00.000Z',
            'Session ID': 'session_1705418400_mno345',
            'Recommended Champion': 'Azir',
            'Winning Algorithm': 'KNN',
            'Confidence Score': 93.1,
            'Random Forest Champion': 'Orianna',
            'Random Forest Confidence': 89.5,
            'Decision Tree Champion': 'Syndra',
            'Decision Tree Confidence': 91.2,
            'KNN Champion': 'Azir',
            'KNN Confidence': 93.1,
            'Consensus Level': 1,
            'User Answers': '{"role":"Mid","complexity":5,"difficulty":5,"damage":5,"toughness":2}',
            'Completion Date': '2024-01-16T14:25:00.000Z'
        },
        {
            'ID': 6,
            'Full Name': 'Emma Davis',
            'Email': 'emma.davis@example.com',
            'Phone': '+1444987654',
            'Age': 19,
            'Gender': 'Female',
            'LoL Experience': 'Beginner',
            'Registration Date': '2024-01-17T16:45:00.000Z',
            'Session ID': 'session_1705513500_pqr678',
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
    ]
    
    # Read existing data
    existing_data = []
    try:
        with open('champion_recommender_users.csv', 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.DictReader(csvfile)
            existing_data = list(reader)
    except FileNotFoundError:
        print("CSV file not found, creating new one...")
    
    # Add new users
    all_data = existing_data + sample_users
    
    # Write updated data
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
        writer.writerows(all_data)
    
    print(f"✅ Added {len(sample_users)} sample users to CSV file")
    print(f"📊 Total users in CSV: {len(all_data)}")
    print(f"📁 File location: {os.path.abspath('champion_recommender_users.csv')}")

if __name__ == "__main__":
    import os
    print("Adding sample users to CSV file...")
    add_sample_users()
    print("Done!")