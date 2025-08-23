import unittest
from app import app
from services.champion_details_service import ChampionDetailsService

class TestChampionDetails(unittest.TestCase):
    
    def setUp(self):
        """Set up test client"""
        self.app = app
        self.app.config['TESTING'] = True
        self.client = self.app.test_client()
        self.details_service = ChampionDetailsService()
    
    def test_champion_details_service_initialization(self):
        """Test champion details service initialization"""
        service = ChampionDetailsService()
        self.assertIsNotNone(service)
        self.assertIsNotNone(service.champion_service)
    
    def test_get_champion_details(self):
        """Test getting champion details"""
        # Get details for a known champion
        details = self.details_service.get_champion_details("Aatrox")
        
        if details:  # Only test if champion exists
            self.assertIn('basic_info', details)
            self.assertIn('abilities', details)
            self.assertIn('playstyle_tips', details)
            self.assertIn('build_recommendations', details)
            self.assertIn('matchup_info', details)
            self.assertIn('learning_resources', details)
            self.assertIn('statistics', details)
            self.assertIn('lore_snippet', details)
    
    def test_champion_details_route(self):
        """Test champion details route"""
        # Test with a valid champion name
        response = self.client.get('/champion/Aatrox')
        
        # Should either show details or redirect if champion not found
        self.assertIn(response.status_code, [200, 302])
        
        if response.status_code == 200:
            # Check for key elements in the response
            self.assertIn(b'Champion Details', response.data)
            self.assertIn(b'Abilities', response.data)
            self.assertIn(b'Tips & Strategy', response.data)
    
    def test_champion_details_route_invalid_champion(self):
        """Test champion details route with invalid champion"""
        response = self.client.get('/champion/NonexistentChampion')
        self.assertEqual(response.status_code, 302)  # Should redirect
    
    def test_basic_info_structure(self):
        """Test basic info structure"""
        # Get a champion from the service
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                basic_info = details['basic_info']
                self.assertIn('name', basic_info)
                self.assertIn('title', basic_info)
                self.assertIn('role', basic_info)
                self.assertIn('difficulty', basic_info)
                self.assertIn('resource_type', basic_info)
                self.assertIn('damage_type', basic_info)
    
    def test_abilities_info_structure(self):
        """Test abilities info structure"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                abilities = details['abilities']
                self.assertIn('passive', abilities)
                self.assertIn('q_ability', abilities)
                self.assertIn('w_ability', abilities)
                self.assertIn('e_ability', abilities)
                self.assertIn('ultimate', abilities)
                self.assertIn('ability_tips', abilities)
    
    def test_playstyle_tips_generation(self):
        """Test playstyle tips generation"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                tips = details['playstyle_tips']
                self.assertIsInstance(tips, list)
                self.assertLessEqual(len(tips), 6)  # Should limit to 6 tips
    
    def test_build_recommendations_structure(self):
        """Test build recommendations structure"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                builds = details['build_recommendations']
                self.assertIn('starter_items', builds)
                self.assertIn('core_items', builds)
                self.assertIn('situational_items', builds)
                self.assertIn('boots_options', builds)
    
    def test_matchup_info_structure(self):
        """Test matchup info structure"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                matchups = details['matchup_info']
                self.assertIn('strong_against', matchups)
                self.assertIn('weak_against', matchups)
                self.assertIn('synergizes_with', matchups)
    
    def test_learning_resources_structure(self):
        """Test learning resources structure"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                resources = details['learning_resources']
                self.assertIn('guides', resources)
                self.assertIn('videos', resources)
                self.assertIn('pro_builds', resources)
                self.assertIn('community', resources)
    
    def test_statistics_calculation(self):
        """Test statistics calculation"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                stats = details['statistics']
                self.assertIn('difficulty_rating', stats)
                self.assertIn('popularity_tier', stats)
                self.assertIn('skill_floor', stats)
                self.assertIn('skill_ceiling', stats)
                
                # Check that ratings are within valid ranges
                self.assertGreaterEqual(stats['difficulty_rating'], 1)
                self.assertLessEqual(stats['difficulty_rating'], 10)
    
    def test_lore_snippet_generation(self):
        """Test lore snippet generation"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                lore = details['lore_snippet']
                self.assertIsInstance(lore, str)
                self.assertGreater(len(lore), 0)
                self.assertIn(champion.name, lore)
    
    def test_champion_details_template_elements(self):
        """Test champion details template elements"""
        # Complete questionnaire to get a champion
        self._complete_questionnaire()
        
        # Get recommendation
        response = self.client.get('/recommendation')
        self.assertEqual(response.status_code, 200)
        
        # Check for champion details link
        self.assertIn(b'VIEW DETAILS', response.data)
    
    def test_champion_details_accessibility(self):
        """Test champion details page accessibility"""
        response = self.client.get('/champion/Aatrox')
        
        if response.status_code == 200:
            # Check for accessibility features
            self.assertIn(b'role="tab"', response.data)
            self.assertIn(b'role="tabpanel"', response.data)
            self.assertIn(b'aria-', response.data)
    
    def test_champion_details_navigation(self):
        """Test navigation elements in champion details"""
        response = self.client.get('/champion/Aatrox')
        
        if response.status_code == 200:
            # Check for navigation elements
            self.assertIn(b'Back to Recommendation', response.data)
            self.assertIn(b'View Alternatives', response.data)
            self.assertIn(b'New Search', response.data)
    
    def test_champion_details_tabs(self):
        """Test tab structure in champion details"""
        response = self.client.get('/champion/Aatrox')
        
        if response.status_code == 200:
            # Check for all tabs
            self.assertIn(b'Abilities', response.data)
            self.assertIn(b'Tips & Strategy', response.data)
            self.assertIn(b'Builds', response.data)
            self.assertIn(b'Matchups', response.data)
            self.assertIn(b'Resources', response.data)
    
    def test_external_links_structure(self):
        """Test external links structure"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            champion = champions[0]
            details = self.details_service.get_champion_details(champion.name)
            
            if details:
                resources = details['learning_resources']
                for resource_type, resource_list in resources.items():
                    for resource in resource_list:
                        self.assertIn('title', resource)
                        self.assertIn('url', resource)
                        self.assertIn('description', resource)
    
    def test_role_based_recommendations(self):
        """Test that recommendations are role-appropriate"""
        champions = self.details_service.champion_service.get_all_champions()
        if champions:
            # Test different roles
            roles_to_test = ['Tank', 'Fighter', 'Mage', 'Marksman', 'Support', 'Assassin']
            
            for champion in champions[:5]:  # Test first 5 champions
                if champion.role in roles_to_test:
                    details = self.details_service.get_champion_details(champion.name)
                    
                    if details:
                        # Check that build recommendations are role-appropriate
                        builds = details['build_recommendations']
                        self.assertIsInstance(builds['core_items'], list)
                        self.assertGreater(len(builds['core_items']), 0)
    
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