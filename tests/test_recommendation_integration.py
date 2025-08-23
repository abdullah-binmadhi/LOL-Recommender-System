import unittest
import json
from app import app
from services.recommendation_engine import RecommendationEngine

class TestRecommendationIntegration(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
        self.recommendation_engine = RecommendationEngine()
    
    def test_complete_ml_recommendation_flow(self):
        """Test complete flow with ML recommendations"""
        # Start questionnaire
        response = self.client.get('/start')
        self.assertEqual(response.status_code, 302)
        
        # Answer all questions
        test_responses = {
            'answer': 'Medium (4-6)',
            'action': 'next'
        }
        
        # Answer question 1
        response = self.client.post('/answer', data=test_responses)
        self.assertEqual(response.status_code, 302)
        
        # Answer question 2
        test_responses['answer'] = 'Fighter'
        response = self.client.post('/answer', data=test_responses)
        self.assertEqual(response.status_code, 302)
        
        # Answer question 3
        test_responses['answer'] = 'Top'
        response = self.client.post('/answer', data=test_responses)
        self.assertEqual(response.status_code, 302)
        
        # Answer question 4
        test_responses['answer'] = 'High Damage Output'
        response = self.client.post('/answer', data=test_responses)
        self.assertEqual(response.status_code, 302)
        
        # Answer question 5 (final)
        test_responses['answer'] = 'Melee'
        response = self.client.post('/answer', data=test_responses)
        self.assertEqual(response.status_code, 302)
        
        # Check recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Should contain ML-generated content
        self.assertIn(b'AI Analysis Details', response.data)
    
    def test_alternatives_page(self):
        """Test alternatives page functionality"""
        # Complete questionnaire first
        self._complete_questionnaire()
        
        # Get alternatives
        response = self.client.get('/alternatives')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'Alternative Champions', response.data)
    
    def test_model_comparison_page(self):
        """Test model comparison page"""
        # Complete questionnaire first
        self._complete_questionnaire()
        
        # Get model comparison
        response = self.client.get('/compare-models')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'AI Model Comparison', response.data)
    
    def test_api_recommendation_endpoint(self):
        """Test API recommendation endpoint"""
        test_data = {
            'responses': {
                '1': 'Medium (4-6)',
                '2': 'Fighter',
                '3': 'Top',
                '4': 'High Damage Output',
                '5': 'Melee'
            },
            'include_alternatives': True
        }
        
        response = self.client.post('/api/recommendation',
                                  data=json.dumps(test_data),
                                  content_type='application/json')
        
        self.assertEqual(response.status_code, 200)
        
        data = json.loads(response.data)
        self.assertIn('champion', data)
        self.assertIn('confidence', data)
        self.assertIn('explanation', data)
        self.assertIn('alternatives', data)
        
        # Check champion data structure
        champion = data['champion']
        self.assertIn('name', champion)
        self.assertIn('role', champion)
        self.assertIn('difficulty', champion)
    
    def test_api_recommendation_invalid_data(self):
        """Test API with invalid data"""
        # Test with missing responses
        response = self.client.post('/api/recommendation',
                                  data=json.dumps({}),
                                  content_type='application/json')
        self.assertEqual(response.status_code, 400)
        
        # Test with invalid JSON
        response = self.client.post('/api/recommendation',
                                  data='invalid json',
                                  content_type='application/json')
        self.assertEqual(response.status_code, 400)
    
    def test_recommendation_engine_direct(self):
        """Test recommendation engine directly"""
        responses = {
            '1': 'Medium (4-6)',
            '2': 'Fighter',
            '3': 'Top',
            '4': 'High Damage Output',
            '5': 'Melee'
        }
        
        # Test prediction
        recommendation = self.recommendation_engine.predict_champion(responses)
        
        self.assertIsNotNone(recommendation)
        self.assertIsNotNone(recommendation.champion)
        self.assertGreaterEqual(recommendation.confidence_score, 0.0)
        self.assertLessEqual(recommendation.confidence_score, 1.0)
        self.assertIsInstance(recommendation.explanation, str)
        self.assertIsInstance(recommendation.match_reasons, list)
        
        # Test alternatives
        alternatives = self.recommendation_engine.get_alternative_recommendations(
            responses, exclude=[recommendation.champion.name], num_alternatives=2
        )
        
        self.assertIsInstance(alternatives, list)
        self.assertLessEqual(len(alternatives), 2)
        
        # Test model info
        model_info = self.recommendation_engine.get_model_info()
        self.assertIsInstance(model_info, dict)
        self.assertIn('available_models', model_info)
        self.assertIn('models_loaded', model_info)
    
    def test_error_handling_in_recommendation(self):
        """Test error handling in recommendation flow"""
        # Try to access recommendation without session
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 302)  # Redirect to home
        
        # Try to access alternatives without session
        response = self.client.get('/alternatives')
        self.assertEqual(response.status_code, 302)  # Redirect to home
        
        # Try to access model comparison without session
        response = self.client.get('/compare-models')
        self.assertEqual(response.status_code, 302)  # Redirect to home
    
    def _complete_questionnaire(self):
        """Helper method to complete questionnaire"""
        # Start questionnaire
        self.client.get('/start')
        
        # Answer all questions
        answers = [
            'Medium (4-6)',
            'Fighter', 
            'Top',
            'High Damage Output',
            'Melee'
        ]
        
        for answer in answers:
            self.client.post('/answer', data={
                'answer': answer,
                'action': 'next'
            })

if __name__ == '__main__':
    unittest.main()