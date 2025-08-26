#!/usr/bin/env python3
"""Simple test script to verify the Flask app works"""

import sys
import os
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from app import app

def test_routes():
    """Test that all routes are accessible"""
    with app.test_client() as client:
        # Test home page
        response = client.get('/')
        print(f"Home page: {response.status_code}")
        assert response.status_code == 200
        
        # Test start questionnaire
        response = client.get('/start')
        print(f"Start questionnaire: {response.status_code} (should redirect)")
        assert response.status_code == 302
        
        # Test questionnaire with session
        with client.session_transaction() as sess:
            sess['session_id'] = 'test-session'
            sess['current_question'] = 1
            sess['responses'] = {}
        
        response = client.get('/questionnaire')
        print(f"Questionnaire page: {response.status_code}")
        assert response.status_code == 200
        
        print("✅ All basic routes working!")

if __name__ == '__main__':
    test_routes()
    print("🎉 Web interface is ready!")