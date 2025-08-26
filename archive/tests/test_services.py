import unittest
import os
import json
import tempfile
import shutil
from services.question_service import QuestionService
from services.champion_service import ChampionService
from services.data_validator import DataValidator

class TestQuestionService(unittest.TestCase):
    
    def setUp(self):
        """Set up test environment"""
        self.test_dir = tempfile.mkdtemp()
        self.questions_file = os.path.join(self.test_dir, "questions.json")
        
        # Create test questions data
        test_questions = {
            "questions": [
                {
                    "id": 1,
                    "text": "Test question 1?",
                    "type": "multiple_choice",
                    "options": ["A", "B", "C"],
                    "weight": 0.8,
                    "feature_mapping": "test1"
                },
                {
                    "id": 2,
                    "text": "Test question 2?",
                    "type": "multiple_choice",
                    "options": ["X", "Y", "Z"],
                    "weight": 0.6,
                    "feature_mapping": "test2"
                }
            ]
        }
        
        with open(self.questions_file, 'w') as f:
            json.dump(test_questions, f)
    
    def tearDown(self):
        """Clean up test environment"""
        shutil.rmtree(self.test_dir)
    
    def test_load_questions(self):
        """Test loading questions from file"""
        service = QuestionService(self.test_dir)
        questions = service.get_all_questions()
        
        self.assertEqual(len(questions), 2)
        self.assertEqual(questions[0].id, 1)
        self.assertEqual(questions[0].text, "Test question 1?")
        self.assertEqual(questions[1].id, 2)
    
    def test_get_question_by_id(self):
        """Test getting specific question by ID"""
        service = QuestionService(self.test_dir)
        
        question = service.get_question_by_id(1)
        self.assertIsNotNone(question)
        self.assertEqual(question.text, "Test question 1?")
        
        question = service.get_question_by_id(999)
        self.assertIsNone(question)
    
    def test_validate_answer(self):
        """Test answer validation"""
        service = QuestionService(self.test_dir)
        
        # Valid answers
        self.assertTrue(service.validate_answer(1, "A"))
        self.assertTrue(service.validate_answer(2, "X"))
        
        # Invalid answers
        self.assertFalse(service.validate_answer(1, "D"))
        self.assertFalse(service.validate_answer(999, "A"))
    
    def test_questions_validation(self):
        """Test questions file validation"""
        service = QuestionService(self.test_dir)
        self.assertTrue(service.validate_questions_file())

class TestChampionService(unittest.TestCase):
    
    def setUp(self):
        """Set up test environment"""
        self.test_dir = tempfile.mkdtemp()
        self.champions_file = os.path.join(self.test_dir, "champions.json")
        
        # Create test champions data
        test_champions = {
            "champions": [
                {
                    "id": "testchamp1",
                    "name": "Test Champion 1",
                    "title": "The Test",
                    "role": "Fighter",
                    "difficulty": 5,
                    "tags": ["Fighter"],
                    "attributes": {"attack": 7, "defense": 5, "magic": 3, "difficulty": 5},
                    "range_type": "Melee",
                    "position": "Top"
                },
                {
                    "id": "testchamp2",
                    "name": "Test Champion 2",
                    "title": "The Tester",
                    "role": "Mage",
                    "difficulty": 7,
                    "tags": ["Mage"],
                    "attributes": {"attack": 3, "defense": 4, "magic": 9, "difficulty": 7},
                    "range_type": "Ranged",
                    "position": "Mid"
                }
            ]
        }
        
        with open(self.champions_file, 'w') as f:
            json.dump(test_champions, f)
    
    def tearDown(self):
        """Clean up test environment"""
        shutil.rmtree(self.test_dir)
    
    def test_load_champions(self):
        """Test loading champions from file"""
        service = ChampionService(self.test_dir)
        # Mock the JSON file exists
        service.json_file = self.champions_file
        
        champions = service.get_all_champions()
        
        self.assertEqual(len(champions), 2)
        self.assertEqual(champions[0].name, "Test Champion 1")
        self.assertEqual(champions[0].role, "Fighter")
        self.assertEqual(champions[1].name, "Test Champion 2")
    
    def test_get_champion_by_name(self):
        """Test getting champion by name"""
        service = ChampionService(self.test_dir)
        service.json_file = self.champions_file
        
        champion = service.get_champion_by_name("Test Champion 1")
        self.assertIsNotNone(champion)
        self.assertEqual(champion.role, "Fighter")
        
        champion = service.get_champion_by_name("Nonexistent Champion")
        self.assertIsNone(champion)
    
    def test_get_champions_by_role(self):
        """Test filtering champions by role"""
        service = ChampionService(self.test_dir)
        service.json_file = self.champions_file
        
        fighters = service.get_champions_by_role("Fighter")
        self.assertEqual(len(fighters), 1)
        self.assertEqual(fighters[0].name, "Test Champion 1")
        
        mages = service.get_champions_by_role("Mage")
        self.assertEqual(len(mages), 1)
        self.assertEqual(mages[0].name, "Test Champion 2")

class TestDataValidator(unittest.TestCase):
    
    def setUp(self):
        """Set up test environment"""
        self.test_dir = tempfile.mkdtemp()
        
        # Create test questions
        questions_file = os.path.join(self.test_dir, "questions.json")
        test_questions = {
            "questions": [
                {
                    "id": 1,
                    "text": "Test question?",
                    "type": "multiple_choice",
                    "options": ["A", "B"],
                    "weight": 0.8,
                    "feature_mapping": "test"
                }
            ]
        }
        with open(questions_file, 'w') as f:
            json.dump(test_questions, f)
        
        # Create test champions
        champions_file = os.path.join(self.test_dir, "champions.json")
        test_champions = {
            "champions": [
                {
                    "id": "testchamp",
                    "name": "Test Champion",
                    "title": "The Test",
                    "role": "Fighter",
                    "difficulty": 5,
                    "tags": ["Fighter"],
                    "attributes": {"attack": 5, "defense": 5, "magic": 5, "difficulty": 5},
                    "range_type": "Melee",
                    "position": "Top"
                }
            ]
        }
        with open(champions_file, 'w') as f:
            json.dump(test_champions, f)
    
    def tearDown(self):
        """Clean up test environment"""
        shutil.rmtree(self.test_dir)
    
    def test_validate_all_data(self):
        """Test complete data validation"""
        validator = DataValidator(self.test_dir)
        # Mock the champion service to use our test file
        validator.champion_service.json_file = os.path.join(self.test_dir, "champions.json")
        
        results = validator.validate_all_data()
        
        self.assertTrue(results["questions"]["valid"])
        self.assertTrue(results["champions"]["valid"])
        self.assertTrue(results["overall"]["valid"])
        self.assertEqual(results["questions"]["count"], 1)
        self.assertEqual(results["champions"]["count"], 1)

if __name__ == '__main__':
    unittest.main()