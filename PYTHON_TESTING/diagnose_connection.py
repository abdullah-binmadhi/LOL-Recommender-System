#!/usr/bin/env python3
"""
Diagnostic script to check server connection and CSV file status
"""

try:
    import requests
except ImportError:
    print("❌ requests module not installed. Installing...")
    import subprocess
    subprocess.check_call(['pip', 'install', 'requests'])
    import requests
import os
import csv
from pathlib import Path

def check_server_connection():
    """Check if the server is running"""
    try:
        response = requests.get('http://localhost:5001/', timeout=5)
        if response.status_code == 200:
            data = response.json()
            print("✅ Server is running!")
            print(f"📊 Total users on server: {data.get('total_users', 0)}")
            return True
        else:
            print(f"❌ Server responded with status: {response.status_code}")
            return False
    except requests.exceptions.ConnectionError:
        print("❌ Server is not running or not accessible")
        return False
    except Exception as e:
        print(f"❌ Error connecting to server: {e}")
        return False

def check_csv_file():
    """Check CSV file status"""
    csv_path = Path('champion_recommender_users.csv')
    
    if csv_path.exists():
        print(f"✅ CSV file exists: {csv_path.absolute()}")
        
        # Count rows
        with open(csv_path, 'r', newline='', encoding='utf-8') as csvfile:
            reader = csv.DictReader(csvfile)
            rows = list(reader)
            print(f"📊 Users in CSV file: {len(rows)}")
            
            if rows:
                print("👥 Users in CSV:")
                for i, row in enumerate(rows, 1):
                    print(f"  {i}. {row.get('Full Name', 'Unknown')} ({row.get('Email', 'No email')})")
            else:
                print("📝 CSV file is empty (only headers)")
    else:
        print(f"❌ CSV file not found: {csv_path.absolute()}")

def test_server_registration():
    """Test server registration with sample data"""
    if not check_server_connection():
        print("⚠️ Cannot test registration - server not running")
        return
    
    test_data = {
        'fullName': 'Test User',
        'email': 'test@example.com',
        'phone': '+1234567890',
        'age': 25,
        'gender': 'Other',
        'experience': 'Beginner',
        'sessionId': 'test_session_123'
    }
    
    try:
        response = requests.post(
            'http://localhost:5001/api/register_user',
            json=test_data,
            timeout=5
        )
        
        if response.status_code == 200:
            result = response.json()
            if result.get('success'):
                print("✅ Test registration successful!")
                print(f"📁 CSV updated at: {result.get('csv_path')}")
            else:
                print(f"❌ Registration failed: {result.get('error')}")
        else:
            print(f"❌ Server error: {response.status_code}")
            
    except Exception as e:
        print(f"❌ Registration test failed: {e}")

def main():
    print("🔍 Champion Recommender Diagnostics")
    print("=" * 40)
    
    # We're already in PYTHON_TESTING directory
    print(f"📁 Working directory: {os.getcwd()}")
    
    print("\n1. Checking server connection...")
    server_running = check_server_connection()
    
    print("\n2. Checking CSV file...")
    check_csv_file()
    
    if server_running:
        print("\n3. Testing server registration...")
        test_server_registration()
        
        print("\n4. Checking CSV file after test...")
        check_csv_file()
    
    print("\n" + "=" * 40)
    if not server_running:
        print("💡 To start the server, run: python data_server.py")
    print("📂 CSV file location: champion_recommender_users.csv")

if __name__ == "__main__":
    main()