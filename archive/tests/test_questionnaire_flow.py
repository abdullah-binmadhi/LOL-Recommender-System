import unittest
import json
from app import app
from services.question_service import QuestionService
from services.champion_service import ChampionService

class TestQuestionnaireFlow(unittest.TestCase):
    
    def setUp(self):
        """Set up test client and services"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
        self.question_service = QuestionService()
        self.champion_service = ChampionService()
    
    def test_complete_questionnaire_flow(self):
        """Test complete questionnaire flow from start to finish"""
        # Start questionnaire
        response = self.client.get('/start')
        self.assertEqual(response.status_code, 302)  # Redirect to questionnaire
        
        # Get all questions
        questions = self.question_service.get_all_questions()
        total_questions = len(questions)
        
        # Answer each question
        for i, question in enumerate(questions, 1):
            # Get questionnaire page
            response = self.client.get('/questionnaire')
            self.assertEqual(response.status_code, 200)
            
            # Submit answer
            answer_data = {
                'answer': question.options[0],  # Select first option
                'action': 'next'
            }
            response = self.client.post('/answer', data=answer_data)
            
            if i < total_questions:
                # Should redirect to next question
                self.assertEqual(response.status_code, 302)
            else:
                # Should redirect to recommendation
                self.assertEqual(response.status_code, 302)
        
        # Check recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
    
    def test_navigation_back_and_forth(self):
        """Test navigation between questions"""
        # Start questionnaire
        self.client.get('/start')
        
        # Answer first question
        questions = self.question_service.get_all_questions()
        first_answer = questions[0].options[0]
        
        response = self.client.post('/answer', data={
            'answer': first_answer,
            'action': 'next'
        })
        self.assertEqual(response.status_code, 302)
        
        # Go to second question and then back
        response = self.client.post('/answer', data={
            'action': 'previous'
        })
        self.assertEqual(response.status_code, 302)
        
        # Check we're back at first question
        response = self.client.get('/questionnaire')
        self.assertIn(questions[0].text.encode(), response.data)
    
    def test_session_persistence(self):
        """Test that session data persists correctly"""
        # Start questionnaire
        self.client.get('/start')
        
        # Answer first question
        questions = self.question_service.get_all_questions()
        first_answer = questions[0].options[1]  # Select second option
        
        self.client.post('/answer', data={
            'answer': first_answer,
            'action': 'next'
        })
        
        # Go back to first question
        self.client.post('/answer', data={'action': 'previous'})
        
        # Check that previous answer is still selected
        response = self.client.get('/questionnaire')
        # The template should show the previous answer as checked
        self.assertIn(first_answer.encode(), response.data)
    
    def test_session_status_endpoint(self):
        """Test session status API endpoint"""
        # No session initially
        response = self.client.get('/session-status')
        data = json.loads(response.data)
        self.assertEqual(data['status'], 'no_session')
        
        # Start questionnaire
        self.client.get('/start')
        
        # Check session status
        response = self.client.get('/session-status')
        data = json.loads(response.data)
        self.assertEqual(data['status'], 'active')
        self.assertEqual(data['current_question'], 1)
        self.assertGreater(data['total_questions'], 0)
    
    def test_invalid_session_handling(self):
        """Test handling of invalid or expired sessions"""
        # Try to access questionnaire without session
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 302)  # Redirect to home
        
        # Try to submit answer without session
        response = self.client.post('/answer', data={'answer': 'test'})
        self.assertEqual(response.status_code, 302)  # Redirect to home
    
    def test_answer_validation(self):
        """Test answer validation"""
        # Start questionnaire
        self.client.get('/start')
        
        # Try to submit empty answer
        response = self.client.post('/answer', data={'action': 'next'})
        self.assertEqual(response.status_code, 302)  # Redirect back to question
        
        # Try to submit invalid answer
        response = self.client.post('/answer', data={
            'answer': 'invalid_option',
            'action': 'next'
        })
        self.assertEqual(response.status_code, 302)  # Redirect back to question
    
    def test_retake_functionality(self):
        """Test questionnaire retake functionality"""
        # Start and partially complete questionnaire
        self.client.get('/start')
        questions = self.question_service.get_all_questions()
        
        self.client.post('/answer', data={
            'answer': questions[0].options[0],
            'action': 'next'
        })
        
        # Retake questionnaire
        response = self.client.get('/retake')
        self.assertEqual(response.status_code, 302)  # Redirect to home
        
        # Start again - should be at first question
        self.client.get('/start')
        response = self.client.get('/questionnaire')
        self.assertIn(questions[0].text.encode(), response.data)
    
    def test_error_handling(self):
        """Test error handling in questionnaire flow"""
        # Start questionnaire
        self.client.get('/start')
        
        # Test with malformed data
        response = self.client.post('/answer', data={
            'answer': None,
            'action': 'invalid_action'
        })
        self.assertEqual(response.status_code, 302)  # Should redirect gracefully

if __name__ == '__main__':
    unittest.main()