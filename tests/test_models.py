import unittest
from datetime import datetime
from models.question import Question, QuestionType
from models.champion import Champion, Ability, ChampionRecommendation
from models.user_response import UserResponse, UserSession

class TestQuestion(unittest.TestCase):
    
    def test_valid_question_creation(self):
        """Test creating a valid question"""
        question = Question(
            id=1,
            text="What is your preferred role?",
            question_type="multiple_choice",
            options=["Tank", "Fighter", "Mage"],
            weight=0.8,
            feature_mapping="role"
        )
        self.assertEqual(question.id, 1)
        self.assertEqual(question.text, "What is your preferred role?")
    
    def test_invalid_question_id(self):
        """Test question with invalid ID"""
        with self.assertRaises(ValueError):
            Question(
                id=0,
                text="Test question",
                question_type="multiple_choice",
                options=["A", "B"],
                weight=0.5,
                feature_mapping="test"
            )
    
    def test_question_requires_options(self):
        """Test that all questions require options"""
        with self.assertRaises(ValueError):
            Question(
                id=1,
                text="Test question",
                question_type="multiple_choice",
                options=[],  # Empty options should fail
                weight=0.5,
                feature_mapping="test"
            )
    
    def test_valid_answer_checking(self):
        """Test answer validation"""
        question = Question(
            id=1,
            text="Choose one",
            question_type="multiple_choice",
            options=["A", "B", "C"],
            weight=0.5,
            feature_mapping="test"
        )
        
        self.assertTrue(question.is_valid_answer("A"))
        self.assertFalse(question.is_valid_answer("D"))

class TestChampion(unittest.TestCase):
    
    def test_valid_champion_creation(self):
        """Test creating a valid champion"""
        champion = Champion(
            id="aatrox",
            name="Aatrox",
            title="The Darkin Blade",
            role="Fighter",
            difficulty=4,
            tags=["Fighter", "Tank"]
        )
        self.assertEqual(champion.name, "Aatrox")
        self.assertEqual(champion.role, "Fighter")
    
    def test_invalid_champion_role(self):
        """Test champion with invalid role"""
        with self.assertRaises(ValueError):
            Champion(
                id="test",
                name="Test",
                title="Test Champion",
                role="InvalidRole",
                difficulty=5,
                tags=["Test"]
            )
    
    def test_invalid_difficulty(self):
        """Test champion with invalid difficulty"""
        with self.assertRaises(ValueError):
            Champion(
                id="test",
                name="Test",
                title="Test Champion",
                role="Fighter",
                difficulty=15,  # Invalid: > 10
                tags=["Test"]
            )

class TestUserResponse(unittest.TestCase):
    
    def test_valid_user_response(self):
        """Test creating a valid user response"""
        response = UserResponse(
            question_id=1,
            answer="Fighter",
            timestamp=datetime.now()
        )
        self.assertEqual(response.question_id, 1)
        self.assertEqual(response.answer, "Fighter")
    
    def test_invalid_question_id(self):
        """Test user response with invalid question ID"""
        with self.assertRaises(ValueError):
            UserResponse(
                question_id=0,
                answer="Test",
                timestamp=datetime.now()
            )

class TestUserSession(unittest.TestCase):
    
    def test_valid_session_creation(self):
        """Test creating a valid user session"""
        session = UserSession(
            session_id="test-session",
            responses=[],
            current_question=1
        )
        self.assertEqual(session.session_id, "test-session")
        self.assertEqual(len(session.responses), 0)
    
    def test_add_response(self):
        """Test adding a response to session"""
        session = UserSession(
            session_id="test-session",
            responses=[],
            current_question=1
        )
        
        session.add_response(1, "Fighter")
        self.assertEqual(len(session.responses), 1)
        self.assertEqual(session.responses[0].answer, "Fighter")
    
    def test_completion_percentage(self):
        """Test completion percentage calculation"""
        session = UserSession(
            session_id="test-session",
            responses=[],
            current_question=1
        )
        
        # No responses
        self.assertEqual(session.get_completion_percentage(5), 0.0)
        
        # Add some responses
        session.add_response(1, "A")
        session.add_response(2, "B")
        self.assertEqual(session.get_completion_percentage(5), 40.0)

if __name__ == '__main__':
    unittest.main()