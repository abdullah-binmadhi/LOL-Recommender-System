import unittest
import json
from app import app

class TestRetakeAndAlternatives(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_retake_with_session_history(self):
        """Test retake functionality with session history"""
        # Complete first questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        
        # Get recommendation to establish session
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Retake questionnaire
        response = self.client.get('/retake')
        self.assertEqual(response.status_code, 302)  # Redirect to home
        
        # Check that session history was created
        response = self.client.get('/session-history')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'Session 1', response.data)
    
    def test_session_history_page(self):
        """Test session history page functionality"""
        # Complete and retake questionnaire to create history
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        self.client.get('/recommendation')
        self.client.get('/retake')
        
        # Access session history
        response = self.client.get('/session-history')
        self.assertEqual(response.status_code, 200)
        
        # Check for history elements
        self.assertIn(b'Session History', response.data)
        self.assertIn(b'Session 1', response.data)
        self.assertIn(b'Compare with Current', response.data)
        self.assertIn(b'Restore Session', response.data)
    
    def test_session_comparison(self):
        """Test session comparison functionality"""
        # Complete first questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        self.client.get('/recommendation')
        self.client.get('/retake')
        
        # Complete second questionnaire with different answers
        self._complete_questionnaire(['Easy (1-3)', 'Mage', 'Mid', 'Support Team', 'Ranged'])
        self.client.get('/recommendation')
        
        # Compare sessions
        response = self.client.get('/compare-with-previous/0')
        self.assertEqual(response.status_code, 200)
        
        # Check comparison elements
        self.assertIn(b'Session Comparison', response.data)
        self.assertIn(b'Current Session', response.data)
        self.assertIn(b'Previous Session', response.data)
        self.assertIn(b'Comparison Overview', response.data)
    
    def test_restore_session(self):
        """Test session restoration functionality"""
        # Complete first questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        self.client.get('/recommendation')
        self.client.get('/retake')
        
        # Restore previous session
        response = self.client.get('/restore-session/0')
        self.assertEqual(response.status_code, 302)  # Redirect to recommendation
        
        # Check that session was restored
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
    
    def test_alternatives_with_filters(self):
        """Test alternatives page with filtering"""
        # Complete questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        
        # Get alternatives with role filter
        response = self.client.get('/alternatives?role=Fighter&num=6')
        self.assertEqual(response.status_code, 200)
        
        # Check for filter elements
        self.assertIn(b'Filter Alternatives', response.data)
        self.assertIn(b'All Roles', response.data)
        self.assertIn(b'Any Difficulty', response.data)
        self.assertIn(b'Apply Filters', response.data)
    
    def test_alternatives_api_endpoint(self):
        """Test alternatives API endpoint"""
        # Complete questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        
        # Get alternatives via API
        response = self.client.get('/api/alternatives?num=3&role=Fighter')
        self.assertEqual(response.status_code, 200)
        
        data = json.loads(response.data)
        self.assertIn('alternatives', data)
        self.assertIn('total', data)
        self.assertIn('filters_applied', data)
        
        # Check data structure
        if data['alternatives']:
            alt = data['alternatives'][0]
            self.assertIn('champion', alt)
            self.assertIn('confidence', alt)
            self.assertIn('explanation', alt)
    
    def test_enhanced_alternatives_display(self):
        """Test enhanced alternatives display elements"""
        # Complete questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        
        # Get alternatives page
        response = self.client.get('/alternatives')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced display elements
        self.assertIn(b'champion-card', response.data)
        self.assertIn(b'Match', response.data)  # Confidence percentage
        self.assertIn(b'Why this champion', response.data)
        self.assertIn(b'Session History', response.data)
    
    def test_session_history_without_history(self):
        """Test session history page when no history exists"""
        response = self.client.get('/session-history')
        self.assertEqual(response.status_code, 302)  # Redirect to home with flash message
    
    def test_invalid_session_operations(self):
        """Test invalid session operations"""
        # Try to compare without sessions
        response = self.client.get('/compare-with-previous/0')
        self.assertEqual(response.status_code, 302)  # Redirect with error
        
        # Try to restore invalid session
        response = self.client.get('/restore-session/999')
        self.assertEqual(response.status_code, 302)  # Redirect with error
        
        # Try alternatives API without session
        response = self.client.get('/api/alternatives')
        self.assertEqual(response.status_code, 400)  # Bad request
    
    def test_multiple_session_history(self):
        """Test multiple sessions in history"""
        # Complete multiple questionnaires
        for i in range(3):
            answers = [
                ['Easy (1-3)', 'Mage', 'Mid', 'Support Team', 'Ranged'],
                ['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'],
                ['Hard (7-8)', 'Assassin', 'Jungle', 'High Damage Output', 'Melee']
            ]
            
            self._complete_questionnaire(answers[i])
            self.client.get('/recommendation')
            if i < 2:  # Don't retake after last session
                self.client.get('/retake')
        
        # Check session history
        response = self.client.get('/session-history')
        self.assertEqual(response.status_code, 200)
        
        # Should have multiple sessions
        self.assertIn(b'Session 1', response.data)
        self.assertIn(b'Session 2', response.data)
    
    def test_alternatives_filtering_functionality(self):
        """Test alternatives filtering with different parameters"""
        # Complete questionnaire
        self._complete_questionnaire(['Medium (4-6)', 'Fighter', 'Top', 'High Damage Output', 'Melee'])
        
        # Test different filter combinations
        filter_tests = [
            {'role': 'Mage'},
            {'difficulty': '5'},
            {'num': '9'},
            {'role': 'Fighter', 'difficulty': '7', 'num': '6'}
        ]
        
        for filters in filter_tests:
            query_string = '&'.join([f"{k}={v}" for k, v in filters.items()])
            response = self.client.get(f'/alternatives?{query_string}')
            self.assertEqual(response.status_code, 200)
    
    def _complete_questionnaire(self, answers):
        """Helper method to complete questionnaire with specific answers"""
        # Start questionnaire
        self.client.get('/start')
        
        # Answer all questions
        for answer in answers:
            self.client.post('/answer', data={
                'answer': answer,
                'action': 'next'
            })

if __name__ == '__main__':
    unittest.main()