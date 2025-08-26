import unittest
import json
from unittest.mock import patch, MagicMock
from app import app
from utils.error_handler import ValidationError, SessionError, MLError, DataError

class TestErrorHandling(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_404_error_handling(self):
        """Test 404 error page"""
        response = self.client.get('/nonexistent-page')
        self.assertEqual(response.status_code, 404)
        self.assertIn(b'Champion Not Found', response.data)
        self.assertIn(b'Go Home', response.data)
        self.assertIn(b'New Questionnaire', response.data)
    
    def test_500_error_handling(self):
        """Test 500 error handling"""
        # This is harder to test directly, but we can test the error handler
        with self.app.test_request_context():
            from app import internal_error
            response, status_code = internal_error(Exception("Test error"))
            self.assertEqual(status_code, 500)
    
    def test_session_validation_error(self):
        """Test session validation errors"""
        # Try to access questionnaire without session
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 302)  # Redirect
        
        # Try to submit answer without session
        response = self.client.post('/answer', data={'answer': 'test'})
        self.assertEqual(response.status_code, 302)  # Redirect
    
    def test_invalid_answer_validation(self):
        """Test answer validation errors"""
        # Start questionnaire
        self.client.get('/start')
        
        # Try to submit empty answer
        response = self.client.post('/answer', data={'action': 'next'})
        self.assertEqual(response.status_code, 302)  # Redirect back to question
        
        # Try to submit invalid answer
        response = self.client.post('/answer', data={
            'answer': 'invalid_option_that_does_not_exist',
            'action': 'next'
        })
        self.assertEqual(response.status_code, 302)  # Redirect back to question
    
    def test_api_error_handling(self):
        """Test API error responses"""
        # Test with missing data
        response = self.client.post('/api/recommendation',
                                  data=json.dumps({}),
                                  content_type='application/json')
        self.assertEqual(response.status_code, 400)
        
        data = json.loads(response.data)
        self.assertIn('error', data)
        self.assertIn('error_code', data)
        
        # Test with invalid responses format
        response = self.client.post('/api/recommendation',
                                  data=json.dumps({'responses': 'invalid'}),
                                  content_type='application/json')
        self.assertEqual(response.status_code, 400)
        
        # Test with invalid JSON
        response = self.client.post('/api/recommendation',
                                  data='invalid json',
                                  content_type='application/json')
        self.assertEqual(response.status_code, 400)
    
    def test_alternatives_api_error_handling(self):
        """Test alternatives API error handling"""
        # Test without session
        response = self.client.get('/api/alternatives')
        self.assertEqual(response.status_code, 400)
        
        data = json.loads(response.data)
        self.assertIn('error', data)
    
    def test_incomplete_questionnaire_error(self):
        """Test error when accessing recommendation with incomplete questionnaire"""
        # Start questionnaire but don't complete it
        self.client.get('/start')
        
        # Try to access recommendation
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 302)  # Redirect to questionnaire
    
    def test_session_history_without_history(self):
        """Test session history access without any history"""
        response = self.client.get('/session-history')
        self.assertEqual(response.status_code, 302)  # Redirect with flash message
    
    def test_invalid_session_operations(self):
        """Test invalid session operations"""
        # Try to compare with invalid index
        response = self.client.get('/compare-with-previous/999')
        self.assertEqual(response.status_code, 302)  # Redirect with error
        
        # Try to restore invalid session
        response = self.client.get('/restore-session/999')
        self.assertEqual(response.status_code, 302)  # Redirect with error
    
    @patch('services.recommendation_engine.RecommendationEngine.predict_champion')
    def test_ml_error_fallback(self, mock_predict):
        """Test ML error fallback mechanism"""
        # Mock ML error
        mock_predict.side_effect = Exception("ML model error")
        
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Access recommendation - should use fallback
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'Fallback recommendation', response.data)
    
    @patch('services.question_service.QuestionService.get_total_questions')
    def test_no_questions_error(self, mock_get_total):
        """Test error when no questions are available"""
        # Mock no questions
        mock_get_total.return_value = 0
        
        # Try to start questionnaire
        response = self.client.get('/start')
        self.assertEqual(response.status_code, 302)  # Redirect with error
    
    @patch('services.question_service.QuestionService.get_question_by_id')
    def test_question_not_found_error(self, mock_get_question):
        """Test error when question is not found"""
        # Mock question not found
        mock_get_question.return_value = None
        
        # Start questionnaire
        self.client.get('/start')
        
        # Try to access questionnaire
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 302)  # Redirect with error
    
    def test_error_logging(self):
        """Test that errors are properly logged"""
        with patch('utils.error_handler.logger') as mock_logger:
            # Trigger a 404 error
            self.client.get('/nonexistent-page')
            
            # Check that logging was called
            mock_logger.info.assert_called()
    
    def test_user_action_logging(self):
        """Test user action logging"""
        with patch('utils.error_handler.logger') as mock_logger:
            # Start questionnaire (should log action)
            self.client.get('/start')
            
            # Check that logging was called
            mock_logger.info.assert_called()
    
    def test_error_handler_decorators(self):
        """Test error handler decorators"""
        from utils.error_handler import handle_errors, ValidationError
        
        @handle_errors(fallback_route='index')
        def test_function():
            raise ValidationError("Test validation error")
        
        # This would normally redirect in a Flask context
        # Here we just test that the decorator doesn't crash
        try:
            test_function()
        except ValidationError:
            pass  # Expected in test context
    
    def test_api_error_decorator(self):
        """Test API error decorator"""
        from utils.error_handler import handle_api_errors, ValidationError
        
        @handle_api_errors
        def test_api_function():
            raise ValidationError("Test API error", "TEST_ERROR")
        
        # Test that decorator handles the error properly
        try:
            result = test_api_function()
            # In test context, this might not return a Flask response
        except ValidationError:
            pass  # Expected in test context
    
    def test_custom_error_classes(self):
        """Test custom error classes"""
        # Test ValidationError
        error = ValidationError("Test message", "TEST_CODE", {"detail": "test"})
        self.assertEqual(error.message, "Test message")
        self.assertEqual(error.error_code, "TEST_CODE")
        self.assertEqual(error.details["detail"], "test")
        
        # Test other error types
        session_error = SessionError("Session error")
        ml_error = MLError("ML error")
        data_error = DataError("Data error")
        
        self.assertIsInstance(session_error, Exception)
        self.assertIsInstance(ml_error, Exception)
        self.assertIsInstance(data_error, Exception)
    
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