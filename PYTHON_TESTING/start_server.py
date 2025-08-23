#!/usr/bin/env python3
"""
Simple startup script for the Champion Recommender Data Server
"""

import subprocess
import sys
import os
from pathlib import Path

def check_dependencies():
    """Check if required packages are installed"""
    try:
        import flask
        import flask_cors
        print("✅ All dependencies are installed")
        return True
    except ImportError as e:
        print(f"❌ Missing dependency: {e}")
        print("Please install dependencies with: pip install -r requirements.txt")
        return False

def main():
    print("🚀 Champion Recommender Data Server Startup")
    print("=" * 50)
    
    # Change to the script directory
    script_dir = Path(__file__).parent
    os.chdir(script_dir)
    
    print(f"📁 Working directory: {script_dir.absolute()}")
    
    # Check dependencies
    if not check_dependencies():
        sys.exit(1)
    
    # Show CSV file location
    try:
        from user_data_manager import UserDataManager
        manager = UserDataManager()
        print(f"📊 CSV file will be saved to: {manager.get_csv_path()}")
    except Exception as e:
        print(f"⚠️ Could not initialize data manager: {e}")
    
    print("\n🌐 Starting Flask server...")
    print("Press Ctrl+C to stop the server")
    print("-" * 50)
    
    # Start the server
    try:
        subprocess.run([sys.executable, "data_server.py"])
    except KeyboardInterrupt:
        print("\n👋 Server stopped by user")
    except Exception as e:
        print(f"❌ Error starting server: {e}")

if __name__ == "__main__":
    main()