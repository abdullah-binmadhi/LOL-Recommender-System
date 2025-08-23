#!/usr/bin/env python3
"""
Simple test script to verify CSV file and add a test user
"""

import os
from user_data_manager import UserDataManager
from datetime import datetime

def test_csv_file():
    """Test the CSV file functionality"""
    print("🧪 Testing CSV File Functionality")
    print("=" * 40)
    
    # Initialize manager
    manager = UserDataManager()
    
    # Show current status
    print(f"📁 CSV file location: {manager.get_csv_path()}")
    print(f"📊 Current user count: {manager.get_user_count()}")
    print(f"✅ CSV file exists: {os.path.exists(manager.csv_file)}")
    
    # Add a test user
    test_user = {
        'full_name': 'Test User',
        'email': 'test@example.com',
        'phone': '+1111111111',
        'age': 20,
        'gender': 'Other',
        'lol_experience': 'Beginner',
        'session_id': f'test_session_{datetime.now().timestamp()}'
    }
    
    print(f"\n➕ Adding test user: {test_user['full_name']}")
    manager.add_user(test_user)
    
    print(f"📊 New user count: {manager.get_user_count()}")
    print("\n✅ Test completed successfully!")
    print(f"📂 You can now open the CSV file: {manager.get_csv_path()}")

if __name__ == "__main__":
    test_csv_file()