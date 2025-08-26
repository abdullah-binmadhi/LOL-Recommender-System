import unittest
from app import app

class TestAccessibility(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
    
    def test_html_lang_attribute(self):
        """Test that HTML has lang attribute"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'<html lang="en">', response.data)
    
    def test_skip_links_present(self):
        """Test that skip links are present"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'Skip to main content', response.data)
        self.assertIn(b'Skip to navigation', response.data)
    
    def test_main_content_landmark(self):
        """Test that main content has proper landmark"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'<main', response.data)
        self.assertIn(b'id="main-content"', response.data)
        self.assertIn(b'role="main"', response.data)
    
    def test_navigation_landmark(self):
        """Test that navigation has proper landmark"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'role="navigation"', response.data)
        self.assertIn(b'aria-label="Main navigation"', response.data)
    
    def test_meta_description_present(self):
        """Test that meta description is present"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'<meta name="description"', response.data)
        self.assertIn(b'AI-powered League of Legends', response.data)
    
    def test_page_title_present(self):
        """Test that page has proper title"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b'<title>', response.data)
        self.assertIn(b'LoL Champion Recommender', response.data)
    
    def test_form_accessibility_questionnaire(self):
        """Test form accessibility in questionnaire"""
        # Start questionnaire
        self.client.get('/start')
        
        # Get questionnaire page
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 200)
        
        # Check for proper form structure
        self.assertIn(b'role="form"', response.data)
        self.assertIn(b'<fieldset>', response.data)
        self.assertIn(b'<legend', response.data)
        self.assertIn(b'role="radiogroup"', response.data)
        self.assertIn(b'aria-required="true"', response.data)
    
    def test_radio_button_accessibility(self):
        """Test radio button accessibility"""
        # Start questionnaire
        self.client.get('/start')
        
        # Get questionnaire page
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 200)
        
        # Check for proper radio button structure
        self.assertIn(b'role="radio"', response.data)
        self.assertIn(b'aria-checked=', response.data)
        self.assertIn(b'aria-describedby=', response.data)
    
    def test_progress_bar_accessibility(self):
        """Test progress bar accessibility"""
        # Start questionnaire
        self.client.get('/start')
        
        # Get questionnaire page
        response = self.client.get('/questionnaire')
        self.assertEqual(response.status_code, 200)
        
        # Check for progress bar accessibility
        self.assertIn(b'role="progressbar"', response.data)
        self.assertIn(b'aria-valuenow=', response.data)
        self.assertIn(b'aria-valuemin=', response.data)
        self.assertIn(b'aria-valuemax=', response.data)
    
    def test_button_accessibility(self):
        """Test button accessibility"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for proper button labels
        self.assertIn(b'aria-label=', response.data)
    
    def test_image_alt_attributes(self):
        """Test that images have alt attributes"""
        # Complete questionnaire to get to recommendation
        self._complete_questionnaire()
        
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for aria-hidden on decorative icons
        self.assertIn(b'aria-hidden="true"', response.data)
    
    def test_heading_hierarchy(self):
        """Test proper heading hierarchy"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Should have h1 as main heading
        self.assertIn(b'<h1', response.data)
    
    def test_color_contrast_toggle(self):
        """Test that color contrast toggle is present"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for accessibility JavaScript
        self.assertIn(b'accessibility.js', response.data)
    
    def test_live_region_present(self):
        """Test that live region for screen readers is present"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for live region
        self.assertIn(b'aria-live="polite"', response.data)
        self.assertIn(b'id="live-region"', response.data)
    
    def test_flash_message_accessibility(self):
        """Test flash message accessibility"""
        # Trigger a flash message
        response = self.client.get('/questionnaire')  # Should redirect and flash
        self.assertEqual(response.status_code, 302)
        
        # Follow redirect
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
    
    def test_breadcrumb_navigation(self):
        """Test breadcrumb navigation accessibility"""
        # Complete questionnaire to get breadcrumbs
        self._complete_questionnaire()
        
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for breadcrumb accessibility
        self.assertIn(b'aria-label="Breadcrumb"', response.data)
        self.assertIn(b'aria-current="page"', response.data)
    
    def test_footer_accessibility(self):
        """Test footer accessibility"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for footer landmark
        self.assertIn(b'role="contentinfo"', response.data)
    
    def test_keyboard_help_present(self):
        """Test that keyboard help is available"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for keyboard help
        self.assertIn(b'Keyboard Help', response.data)
        self.assertIn(b'Accessibility Help', response.data)
    
    def test_theme_color_meta(self):
        """Test theme color meta tags for mobile"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for theme color
        self.assertIn(b'name="theme-color"', response.data)
        self.assertIn(b'#c89b3c', response.data)
    
    def test_preload_resources(self):
        """Test that critical resources are preloaded"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for preload links
        self.assertIn(b'rel="preload"', response.data)
    
    def test_social_media_meta_tags(self):
        """Test social media meta tags"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check for Open Graph tags
        self.assertIn(b'property="og:type"', response.data)
        self.assertIn(b'property="og:title"', response.data)
        
        # Check for Twitter tags
        self.assertIn(b'property="twitter:card"', response.data)
    
    def test_error_page_accessibility(self):
        """Test error page accessibility"""
        response = self.client.get('/nonexistent-page')
        self.assertEqual(response.status_code, 404)
        
        # Check that error page has proper structure
        self.assertIn(b'<main', response.data)
        self.assertIn(b'role="main"', response.data)
    
    def test_form_validation_accessibility(self):
        """Test form validation accessibility"""
        # Start questionnaire
        self.client.get('/start')
        
        # Try to submit without answer
        response = self.client.post('/answer', data={'action': 'next'})
        self.assertEqual(response.status_code, 302)  # Should redirect back
    
    def test_alternative_text_for_icons(self):
        """Test that decorative icons have aria-hidden"""
        response = self.client.get('/')
        self.assertEqual(response.status_code, 200)
        
        # Check that Font Awesome icons are marked as decorative
        self.assertIn(b'aria-hidden="true"', response.data)
    
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