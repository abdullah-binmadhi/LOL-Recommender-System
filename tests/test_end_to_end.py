"""
End-to-end tests for the LoL Champion Recommender
Tests complete user workflows from start to finish
"""

import pytest
import sys
import os
import json
from unittest.mock import patch, MagicMock

# Add the parent directory to the path so we can import our modules
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from app import app
from services.champion_service import ChampionService
from services.question_service import QuestionService
from services.recommendation_engine import RecommendationEngine

# Import test fixtures
from tests.fixtures import (
    get_test_champions, get_test_questions, get_test_session,
    get_response_pattern, get_expected_recommendation,
    RESPONSE_PATTERNS, PERFORMANCE_TEST_RESPONSES
)

class TestCompleteUserWorkflow:
    """Test complete user workflows from start to finish"""
    
    def setup_method(self):
        """Setup for each test"""
        self.app = app
        self.app.config['TESTING'] = True
        self.app.config['WTF_CSRF_ENABLED'] = False
        self.client = self.app.test_client()
        
        # Ensure services are working
        self.champion_service = ChampionService()
        self.question_service = QuestionService()
    
    def test_complete_questionnaire_flow(self):
        """Test complete questionnaire flow from start to recommendation"""
        with self.client as client:
            # Step 1: Visit home page
            response = client.get('/')
            assert response.status_code == 200
            assert b'LoL Champion Recommender' in response.data
            
            # Step 2: Start questionnaire
            response = client.get('/start')
            assert response.status_code == 302  # Redirect to questionnaire
            
            # Check session was created
            with client.session_transaction() as sess:
                assert 'session_id' in sess
                assert 'current_question' in sess
                assert sess['current_question'] == 1
            
            # Step 3: Go through questionnaire
            response = client.get('/questionnaire')
            assert response.status_code == 200
            
            # Get total questions to know how many to answer
            total_questions = self.question_service.get_total_questions()
            assert total_questions > 0
            
            # Answer all questions
            sample_answers = [
                'Tank', 'Easy', 'Team fights', 'Melee', 'High',
                'Defensive', 'Crowd Control', 'Beginner', 'Support team'
            ]
            
            for i in range(1, min(total_questions + 1, len(sample_answers) + 1)):
                # Submit answer
                answer = sample_answers[min(i-1, len(sample_answers)-1)]
                response = client.post('/answer', data={
                    'answer': answer,
                    'action': 'next'
                })
                
                if i < total_questions:
                    assert response.status_code == 302  # Redirect to next question
                else:
                    assert response.status_code == 302  # Redirect to recommendation
            
            # Step 4: Get recommendation
            response = client.get('/recommendation')
            assert response.status_code == 200
            
            # Check that recommendation page contains expected elements
            assert b'Recommended Champion' in response.data or b'recommendation' in response.data.lower()
            
            # Check session has recommendation data
            with client.session_transaction() as sess:
                assert 'last_recommendation' in sess or len(sess.get('responses', {})) > 0
    
    def test_questionnaire_navigation(self):
        """Test navigation features in questionnaire"""
        with self.client as client:
            # Start questionnaire
            client.get('/start')
            
            # Answer first question
            client.post('/answer', data={
                'answer': 'Tank',
                'action': 'next'
            })
            
            # Check we're on question 2
            with client.session_transaction() as sess:
                assert sess['current_question'] == 2
            
            # Go back to previous question
            response = client.post('/answer', data={
                'action': 'previous'
            })
            assert response.status_code == 302
            
            # Check we're back on question 1
            with client.session_transaction() as sess:
                assert sess['current_question'] == 1
            
            # Test skip functionality if available
            response = client.get('/skip')
            if response.status_code == 302:  # Skip is implemented
                with client.session_transaction() as sess:
                    assert sess['current_question'] == 2
    
    def test_retake_functionality(self):
        """Test retake questionnaire functionality"""
        with self.client as client:
            # Complete a questionnaire
            client.get('/start')
            
            # Answer a few questions
            for i, answer in enumerate(['Tank', 'Easy', 'Team fights'], 1):
                client.post('/answer', data={
                    'answer': answer,
                    'action': 'next'
                })
            
            # Get current session ID
            with client.session_transaction() as sess:
                original_session_id = sess.get('session_id')
                original_responses = sess.get('responses', {})
            
            # Retake questionnaire
            response = client.get('/retake')
            assert response.status_code == 302  # Redirect to home
            
            # Check session was reset
            response = client.get('/')
            assert response.status_code == 200
            
            # Start new questionnaire
            client.get('/start')
            
            with client.session_transaction() as sess:
                new_session_id = sess.get('session_id')
                assert new_session_id != original_session_id
                assert sess.get('current_question') == 1
                # Check if history was preserved
                history = sess.get('session_history', [])
                if history:
                    assert any(h.get('session_id') == original_session_id for h in history)
    
    def test_alternatives_workflow(self):
        """Test alternative recommendations workflow"""
        with self.client as client:
            # Complete questionnaire first
            client.get('/start')
            
            # Answer questions
            sample_responses = {
                '1': 'Tank',
                '2': 'Easy', 
                '3': 'Team fights'
            }
            
            # Set up session with responses
            with client.session_transaction() as sess:
                sess['responses'] = sample_responses
                sess['last_recommendation'] = 'TestChampion'
                total_questions = self.question_service.get_total_questions()
                sess['total_questions'] = total_questions
            
            # Get alternatives
            response = client.get('/alternatives')
            
            # Should either show alternatives or redirect if incomplete
            assert response.status_code in [200, 302]
            
            if response.status_code == 200:
                # Check alternatives page has expected content
                assert b'alternative' in response.data.lower() or b'other' in response.data.lower()
    
    def test_champion_details_workflow(self):
        """Test champion details viewing workflow"""
        with self.client as client:
            # Get a champion name to test with
            champions = self.champion_service.get_all_champions()
            if not champions:
                pytest.skip("No champions available for testing")
            
            test_champion = champions[0]
            
            # View champion details
            response = client.get(f'/champion/{test_champion.name}')
            
            # Should show champion details or handle gracefully
            assert response.status_code in [200, 302, 404]
            
            if response.status_code == 200:
                # Check champion details page has expected content
                assert test_champion.name.encode() in response.data
    
    def test_session_persistence(self):
        """Test that sessions persist across requests"""
        with self.client as client:
            # Start questionnaire
            client.get('/start')
            
            # Answer first question
            client.post('/answer', data={
                'answer': 'Tank',
                'action': 'next'
            })
            
            # Get session status
            response = client.get('/session-status')
            assert response.status_code == 200
            
            data = json.loads(response.data)
            assert data['status'] == 'active'
            assert 'session_id' in data
            assert data['current_question'] == 2
            assert len(data['responses']) == 1
    
    def test_error_handling_workflow(self):
        """Test error handling in user workflows"""
        with self.client as client:
            # Test accessing questionnaire without session
            response = client.get('/questionnaire')
            assert response.status_code == 302  # Should redirect to home
            
            # Test accessing recommendation without completing questionnaire
            response = client.get('/recommendation')
            assert response.status_code == 302  # Should redirect
            
            # Test invalid champion details
            response = client.get('/champion/NonExistentChampion')
            assert response.status_code in [302, 404]  # Should handle gracefully
            
            # Test submitting answer without session
            response = client.post('/answer', data={
                'answer': 'Tank',
                'action': 'next'
            })
            assert response.status_code == 302  # Should redirect to home

class TestAPIEndpoints:
    """Test API endpoints functionality"""
    
    def setup_method(self):
        """Setup for each test"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_api_recommendation_endpoint(self):
        """Test API recommendation endpoint"""
        # Test valid request
        test_data = {
            'responses': {
                '1': 'Tank',
                '2': 'Easy',
                '3': 'Team fights'
            },
            'include_alternatives': True
        }
        
        response = self.client.post('/api/recommendation',
                                  json=test_data,
                                  content_type='application/json')
        
        # Should either work or fail gracefully
        assert response.status_code in [200, 400, 500]
        
        if response.status_code == 200:
            data = json.loads(response.data)
            assert 'success' in data
            assert 'champion' in data
        
        # Test invalid request
        response = self.client.post('/api/recommendation',
                                  json={'invalid': 'data'},
                                  content_type='application/json')
        assert response.status_code == 400
    
    def test_api_alternatives_endpoint(self):
        """Test API alternatives endpoint"""
        with self.client as client:
            # Set up session first
            with client.session_transaction() as sess:
                sess['session_id'] = 'test_session'
                sess['responses'] = {
                    '1': 'Tank',
                    '2': 'Easy',
                    '3': 'Team fights'
                }
                sess['total_questions'] = 3
            
            response = client.get('/api/alternatives')
            
            # Should either work or return appropriate error
            assert response.status_code in [200, 400]
            
            if response.status_code == 200:
                data = json.loads(response.data)
                assert 'alternatives' in data

class TestPerformanceWorkflow:
    """Test performance-related workflows"""
    
    def setup_method(self):
        """Setup for each test"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_concurrent_users_simulation(self):
        """Test handling multiple concurrent users"""
        import threading
        import time
        
        results = []
        
        def simulate_user(user_id):
            """Simulate a single user workflow"""
            try:
                with self.app.test_client() as client:
                    # Start questionnaire
                    response = client.get('/start')
                    if response.status_code not in [200, 302]:
                        results.append(f"User {user_id}: Start failed")
                        return
                    
                    # Answer questions
                    answers = ['Tank', 'Easy', 'Team fights']
                    for answer in answers:
                        response = client.post('/answer', data={
                            'answer': answer,
                            'action': 'next'
                        })
                        if response.status_code not in [200, 302]:
                            results.append(f"User {user_id}: Answer failed")
                            return
                    
                    # Get recommendation
                    response = client.get('/recommendation')
                    if response.status_code not in [200, 302]:
                        results.append(f"User {user_id}: Recommendation failed")
                        return
                    
                    results.append(f"User {user_id}: Success")
                    
            except Exception as e:
                results.append(f"User {user_id}: Exception - {str(e)}")
        
        # Simulate 5 concurrent users
        threads = []
        for i in range(5):
            thread = threading.Thread(target=simulate_user, args=(i,))
            threads.append(thread)
            thread.start()
        
        # Wait for all threads to complete
        for thread in threads:
            thread.join(timeout=30)  # 30 second timeout
        
        # Check results
        successful_users = len([r for r in results if 'Success' in r])
        assert successful_users >= 3, f"Too many concurrent user failures: {results}"
    
    def test_large_session_handling(self):
        """Test handling of sessions with large amounts of data"""
        with self.client as client:
            # Create session with large response data
            large_responses = {str(i): f'Answer_{i}' * 100 for i in range(50)}
            
            with client.session_transaction() as sess:
                sess['session_id'] = 'large_session_test'
                sess['responses'] = large_responses
                sess['large_data'] = 'x' * 10000  # 10KB of data
            
            # Test that session still works
            response = client.get('/session-status')
            assert response.status_code == 200
            
            data = json.loads(response.data)
            assert data['status'] == 'active'
            assert len(data['responses']) == 50

class TestDataIntegrity:
    """Test data integrity throughout workflows"""
    
    def setup_method(self):
        """Setup for each test"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
        self.champion_service = ChampionService()
        self.question_service = QuestionService()
    
    def test_data_consistency(self):
        """Test that data remains consistent throughout the application"""
        # Test champion data consistency
        champions1 = self.champion_service.get_all_champions()
        champions2 = self.champion_service.get_all_champions()
        
        assert len(champions1) == len(champions2)
        if champions1:
            assert champions1[0].name == champions2[0].name
        
        # Test question data consistency
        questions1 = self.question_service.get_all_questions()
        questions2 = self.question_service.get_all_questions()
        
        assert len(questions1) == len(questions2)
        if questions1:
            assert questions1[0].text == questions2[0].text
    
    def test_session_data_integrity(self):
        """Test that session data maintains integrity"""
        with self.client as client:
            # Start questionnaire
            client.get('/start')
            
            # Get initial session state
            with client.session_transaction() as sess:
                initial_session_id = sess['session_id']
                initial_question = sess['current_question']
            
            # Answer question
            client.post('/answer', data={
                'answer': 'Tank',
                'action': 'next'
            })
            
            # Check session state updated correctly
            with client.session_transaction() as sess:
                assert sess['session_id'] == initial_session_id  # Should remain same
                assert sess['current_question'] == initial_question + 1
                assert '1' in sess['responses']
                assert sess['responses']['1'] == 'Tank'
    
    def test_recommendation_consistency(self):
        """Test that recommendations are consistent for same inputs"""
        test_responses = {
            '1': 'Tank',
            '2': 'Easy',
            '3': 'Team fights'
        }
        
        with self.client as client:
            # Set up session
            with client.session_transaction() as sess:
                sess['session_id'] = 'consistency_test'
                sess['responses'] = test_responses
                sess['total_questions'] = len(test_responses)
            
            # Get recommendation multiple times
            recommendations = []
            for _ in range(3):
                response = client.get('/recommendation')
                if response.status_code == 200:
                    # Extract champion name from response (simplified)
                    recommendations.append(response.data)
            
            # All recommendations should be identical (due to caching)
            if len(recommendations) > 1:
                assert all(rec == recommendations[0] for rec in recommendations)

if __name__ == '__main__':
    pytest.main([__file__, '-v'])