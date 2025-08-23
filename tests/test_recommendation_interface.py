import unittest
from app import app
from services.recommendation_engine import RecommendationEngine

class TestRecommendationInterface(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_recommendation_page_content(self):
        """Test that recommendation page contains enhanced elements"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced UI elements
        self.assertIn(b'champion-name', response.data)
        self.assertIn(b'copy-btn', response.data)
        self.assertIn(b'AI Analysis Details', response.data)
        self.assertIn(b'Combat Statistics', response.data)
        self.assertIn(b'Strategic Analysis', response.data)
        self.assertIn(b'VIEW ALTERNATIVES', response.data)
        self.assertIn(b'COMPARE MODELS', response.data)
        self.assertIn(b'keyboard-shortcuts', response.data)
    
    def test_alternatives_page_content(self):
        """Test alternatives page enhanced content"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get alternatives page
        response = self.client.get('/alternatives')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced elements
        self.assertIn(b'Alternative Champions', response.data)
        self.assertIn(b'champion-card', response.data)
        self.assertIn(b'Match', response.data)  # Confidence percentage
    
    def test_model_comparison_page_content(self):
        """Test model comparison page enhanced content"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get model comparison page
        response = self.client.get('/compare-models')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced elements
        self.assertIn(b'AI Model Comparison', response.data)
        self.assertIn(b'System Information', response.data)
        self.assertIn(b'About the AI Models', response.data)
        self.assertIn(b'Random Forest', response.data)
        self.assertIn(b'Decision Tree', response.data)
        self.assertIn(b'K-Nearest Neighbors', response.data)
    
    def test_enhanced_champion_display(self):
        """Test enhanced champion display elements"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced champion display
        self.assertIn(b'champion-image-container', response.data)
        self.assertIn(b'confidence-indicator', response.data)
        self.assertIn(b'champion-stat-card', response.data)
        self.assertIn(b'attribute-card', response.data)
    
    def test_interactive_elements(self):
        """Test interactive elements are present"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for interactive elements
        self.assertIn(b'data-bs-toggle="tooltip"', response.data)
        self.assertIn(b'data-copy=', response.data)
        self.assertIn(b'keyboard-shortcuts', response.data)
    
    def test_progress_bars_and_animations(self):
        """Test that progress bars and animations are included"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for progress bars and animation classes
        self.assertIn(b'progress-bar', response.data)
        self.assertIn(b'progress-bar-animated', response.data)
        self.assertIn(b'champion-border-animation', response.data)
    
    def test_responsive_design_elements(self):
        """Test responsive design elements"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for responsive classes
        self.assertIn(b'col-md-', response.data)
        self.assertIn(b'col-sm-', response.data)
        self.assertIn(b'justify-content-center', response.data)
    
    def test_accessibility_features(self):
        """Test accessibility features"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for accessibility attributes
        self.assertIn(b'aria-valuenow', response.data)
        self.assertIn(b'aria-valuemin', response.data)
        self.assertIn(b'aria-valuemax', response.data)
        self.assertIn(b'role="progressbar"', response.data)
    
    def test_javascript_inclusion(self):
        """Test that JavaScript files are included"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for JavaScript inclusion
        self.assertIn(b'recommendation.js', response.data)
    
    def test_enhanced_styling(self):
        """Test enhanced styling elements"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for enhanced styling
        self.assertIn(b'var(--accent-gold)', response.data)
        self.assertIn(b'linear-gradient', response.data)
        self.assertIn(b'backdrop-filter', response.data)
    
    def test_model_info_display(self):
        """Test model information display"""
        # Complete questionnaire
        self._complete_questionnaire()
        
        # Get recommendation page
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for model info elements
        self.assertIn(b'AI Models', response.data)
        self.assertIn(b'Best Model', response.data)
        self.assertIn(b'Champions Analyzed', response.data)
        self.assertIn(b'Confidence', response.data)
    
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