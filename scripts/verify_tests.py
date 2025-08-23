#!/usr/bin/env python3
"""
Simple test verification script for the LoL Champion Recommender
Verifies that the test suite is properly set up and fixtures are working
"""

import sys
import os
from pathlib import Path

# Add the parent directory to the path
sys.path.insert(0, str(Path(__file__).parent.parent))

def test_fixtures_import():
    """Test that fixtures can be imported"""
    try:
        from tests.fixtures import (
            get_test_champions, get_test_questions, get_test_session,
            get_response_pattern, get_expected_recommendation
        )
        print("✓ Test fixtures imported successfully")
        return True
    except ImportError as e:
        print(f"✗ Failed to import test fixtures: {e}")
        return False

def test_fixture_data():
    """Test that fixture data is valid"""
    try:
        from tests.fixtures import get_test_champions, get_test_questions
        
        champions = get_test_champions()
        if len(champions) >= 5:
            print(f"✓ Test champions loaded: {len(champions)} champions")
        else:
            print(f"✗ Insufficient test champions: {len(champions)}")
            return False
        
        questions = get_test_questions()
        if len(questions) >= 9:
            print(f"✓ Test questions loaded: {len(questions)} questions")
        else:
            print(f"✗ Insufficient test questions: {len(questions)}")
            return False
        
        return True
    except Exception as e:
        print(f"✗ Failed to load fixture data: {e}")
        return False

def test_models_import():
    """Test that models can be imported"""
    try:
        from models.champion import Champion, Ability
        from models.question import Question
        from models.user_response import UserSession, UserResponse
        print("✓ Models imported successfully")
        return True
    except ImportError as e:
        print(f"✗ Failed to import models: {e}")
        return False

def test_services_import():
    """Test that services can be imported"""
    try:
        from services.champion_service import ChampionService
        from services.question_service import QuestionService
        from services.recommendation_engine import RecommendationEngine
        print("✓ Services imported successfully")
        return True
    except ImportError as e:
        print(f"✗ Failed to import services: {e}")
        return False

def test_app_import():
    """Test that the main app can be imported"""
    try:
        from app import app
        print("✓ Flask app imported successfully")
        return True
    except ImportError as e:
        print(f"✗ Failed to import Flask app: {e}")
        return False

def test_basic_functionality():
    """Test basic functionality with fixtures"""
    try:
        from tests.fixtures import get_test_champions, get_response_pattern
        
        # Test champion data
        champions = get_test_champions()
        malphite = next((c for c in champions if c.name == "Malphite"), None)
        if malphite:
            print(f"✓ Found test champion: {malphite.name} ({malphite.role})")
        else:
            print("✗ Could not find Malphite in test champions")
            return False
        
        # Test response patterns
        tank_pattern = get_response_pattern('tank_easy')
        if tank_pattern.get('1') == 'Tank':
            print("✓ Response patterns working correctly")
        else:
            print("✗ Response patterns not working correctly")
            return False
        
        return True
    except Exception as e:
        print(f"✗ Basic functionality test failed: {e}")
        return False

def test_directory_structure():
    """Test that required directories exist"""
    required_dirs = [
        'tests',
        'tests/fixtures',
        'docs',
        'models',
        'services',
        'scripts'
    ]
    
    all_exist = True
    for dir_path in required_dirs:
        if os.path.exists(dir_path):
            print(f"✓ Directory exists: {dir_path}")
        else:
            print(f"✗ Directory missing: {dir_path}")
            all_exist = False
    
    return all_exist

def test_documentation_files():
    """Test that documentation files exist"""
    required_docs = [
        'docs/README.md',
        'docs/api_documentation.md',
        'docs/setup_guide.md',
        'docs/deployment_guide.md',
        'docs/user_guide.md',
        'docs/troubleshooting_guide.md'
    ]
    
    all_exist = True
    for doc_path in required_docs:
        if os.path.exists(doc_path):
            print(f"✓ Documentation exists: {doc_path}")
        else:
            print(f"✗ Documentation missing: {doc_path}")
            all_exist = False
    
    return all_exist

def test_fixture_files():
    """Test that fixture files exist"""
    required_fixtures = [
        'tests/fixtures/__init__.py',
        'tests/fixtures/champion_fixtures.py',
        'tests/fixtures/question_fixtures.py',
        'tests/fixtures/session_fixtures.py',
        'tests/fixtures/response_fixtures.py'
    ]
    
    all_exist = True
    for fixture_path in required_fixtures:
        if os.path.exists(fixture_path):
            print(f"✓ Fixture file exists: {fixture_path}")
        else:
            print(f"✗ Fixture file missing: {fixture_path}")
            all_exist = False
    
    return all_exist

def main():
    """Run all verification tests"""
    print("LoL Champion Recommender - Test Suite Verification")
    print("=" * 60)
    
    tests = [
        ("Directory Structure", test_directory_structure),
        ("Documentation Files", test_documentation_files),
        ("Fixture Files", test_fixture_files),
        ("Models Import", test_models_import),
        ("Services Import", test_services_import),
        ("App Import", test_app_import),
        ("Fixtures Import", test_fixtures_import),
        ("Fixture Data", test_fixture_data),
        ("Basic Functionality", test_basic_functionality)
    ]
    
    results = []
    for test_name, test_func in tests:
        print(f"\n{test_name}:")
        print("-" * 30)
        try:
            success = test_func()
            results.append((test_name, success))
        except Exception as e:
            print(f"✗ Test failed with exception: {e}")
            results.append((test_name, False))
    
    # Print summary
    print("\n" + "=" * 60)
    print("VERIFICATION SUMMARY")
    print("=" * 60)
    
    passed = sum(1 for _, success in results if success)
    total = len(results)
    
    for test_name, success in results:
        status = "PASSED" if success else "FAILED"
        print(f"{test_name:<25} {status}")
    
    print("-" * 60)
    print(f"Total: {total}, Passed: {passed}, Failed: {total - passed}")
    
    if passed == total:
        print("\n🎉 All verification tests passed!")
        print("The test suite is properly set up and ready to use.")
        return True
    else:
        print(f"\n❌ {total - passed} verification test(s) failed.")
        print("Please fix the issues before running the full test suite.")
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)