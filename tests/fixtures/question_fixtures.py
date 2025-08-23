"""
Question test fixtures
Provides consistent question data for testing
"""

from typing import List, Dict, Any
from models.question import Question

# Sample question data for testing
SAMPLE_QUESTIONS_DATA = [
    {
        "id": 1,
        "text": "What role do you prefer to play?",
        "question_type": "multiple_choice",
        "options": ["Tank", "Fighter", "Assassin", "Mage", "Marksman", "Support"],
        "feature_mapping": "role_preference",
        "weight": 0.9
    },
    {
        "id": 2,
        "text": "What difficulty level are you comfortable with?",
        "question_type": "multiple_choice",
        "options": ["Easy", "Moderate", "Hard", "Expert"],
        "feature_mapping": "difficulty_preference",
        "weight": 0.8
    },
    {
        "id": 3,
        "text": "What type of gameplay do you enjoy most?",
        "question_type": "multiple_choice",
        "options": ["Team fights", "Solo plays", "Supporting allies", "Farming", "Roaming"],
        "feature_mapping": "playstyle_preference",
        "weight": 0.7
    },
    {
        "id": 4,
        "text": "Do you prefer melee or ranged champions?",
        "question_type": "multiple_choice",
        "options": ["Melee", "Ranged", "No preference"],
        "feature_mapping": "range_preference",
        "weight": 0.6
    },
    {
        "id": 5,
        "text": "How important is high damage output to you?",
        "question_type": "multiple_choice",
        "options": ["Very Low", "Low", "Medium", "High", "Very High"],
        "feature_mapping": "damage_preference",
        "weight": 0.7
    },
    {
        "id": 6,
        "text": "Do you prefer defensive or offensive playstyles?",
        "question_type": "multiple_choice",
        "options": ["Defensive", "Offensive", "Balanced"],
        "feature_mapping": "style_preference",
        "weight": 0.6
    },
    {
        "id": 7,
        "text": "What type of abilities do you prefer?",
        "question_type": "multiple_choice",
        "options": ["Crowd Control", "Burst Damage", "Sustained Damage", "Healing/Shielding", "Mobility"],
        "feature_mapping": "ability_preference",
        "weight": 0.8
    },
    {
        "id": 8,
        "text": "How experienced are you with League of Legends?",
        "question_type": "multiple_choice",
        "options": ["Beginner", "Intermediate", "Advanced", "Expert"],
        "feature_mapping": "experience_level",
        "weight": 0.5
    },
    {
        "id": 9,
        "text": "What motivates you most when playing?",
        "question_type": "multiple_choice",
        "options": ["Winning games", "Learning new champions", "Having fun", "Improving skills", "Support team"],
        "feature_mapping": "motivation",
        "weight": 0.4
    }
]

def create_test_question(question_data: Dict[str, Any]) -> Question:
    """Create a Question object from test data"""
    return Question(
        id=question_data["id"],
        text=question_data["text"],
        question_type=question_data["question_type"],
        options=question_data["options"],
        weight=question_data.get("weight", 1.0),
        feature_mapping=question_data.get("feature_mapping", f"feature_{question_data['id']}")
    )

def get_test_questions() -> List[Question]:
    """Get all test questions as Question objects"""
    return [create_test_question(data) for data in SAMPLE_QUESTIONS_DATA]

def get_test_question_by_id(question_id: int) -> Question:
    """Get a specific test question by ID"""
    for question_data in SAMPLE_QUESTIONS_DATA:
        if question_data["id"] == question_id:
            return create_test_question(question_data)
    raise ValueError(f"Test question with ID {question_id} not found")

def get_test_questions_by_category(category: str) -> List[Question]:
    """Get test questions filtered by category"""
    questions = []
    for question_data in SAMPLE_QUESTIONS_DATA:
        if question_data["category"] == category:
            questions.append(create_test_question(question_data))
    return questions

def get_total_test_questions() -> int:
    """Get total number of test questions"""
    return len(SAMPLE_QUESTIONS_DATA)

# Question categories for testing
QUESTION_CATEGORIES = [
    "role_preference",
    "difficulty_preference", 
    "playstyle_preference",
    "range_preference",
    "damage_preference",
    "style_preference",
    "ability_preference",
    "experience_level",
    "motivation"
]

# Valid answers for each question type
VALID_ANSWERS = {
    1: ["Tank", "Fighter", "Assassin", "Mage", "Marksman", "Support"],
    2: ["Easy", "Moderate", "Hard", "Expert"],
    3: ["Team fights", "Solo plays", "Supporting allies", "Farming", "Roaming"],
    4: ["Melee", "Ranged", "No preference"],
    5: ["Very Low", "Low", "Medium", "High", "Very High"],
    6: ["Defensive", "Offensive", "Balanced"],
    7: ["Crowd Control", "Burst Damage", "Sustained Damage", "Healing/Shielding", "Mobility"],
    8: ["Beginner", "Intermediate", "Advanced", "Expert"],
    9: ["Winning games", "Learning new champions", "Having fun", "Improving skills", "Support team"]
}

# Invalid answers for testing validation
INVALID_ANSWERS = [
    "",  # Empty string
    None,  # None value
    "Invalid Option",  # Non-existent option
    123,  # Wrong type
    [],  # Wrong type
    {}   # Wrong type
]

# Question type mappings
QUESTION_TYPES = {
    "multiple_choice": [1, 2, 3, 4, 6, 7, 8, 9],
    "scale": [5]
}

# Required vs optional questions
REQUIRED_QUESTIONS = [1, 2, 3, 4, 5, 6, 7, 8, 9]
OPTIONAL_QUESTIONS = []

# Question weights for ML model
QUESTION_WEIGHTS = {
    1: 0.9,  # Role preference - highest weight
    2: 0.8,  # Difficulty preference
    3: 0.7,  # Playstyle preference
    4: 0.6,  # Range preference
    5: 0.7,  # Damage preference
    6: 0.6,  # Style preference
    7: 0.8,  # Ability preference
    8: 0.5,  # Experience level
    9: 0.4   # Motivation - lowest weight
}