from dataclasses import dataclass
from typing import List, Any, Optional
from enum import Enum

class QuestionType(Enum):
    MULTIPLE_CHOICE = "multiple_choice"

@dataclass
class Question:
    id: int
    text: str
    question_type: str
    options: List[str]
    weight: float
    feature_mapping: str
    
    def __post_init__(self):
        """Validate question data after initialization"""
        self.validate()
    
    def validate(self) -> bool:
        """Validate question data"""
        if not isinstance(self.id, int) or self.id <= 0:
            raise ValueError(f"Question ID must be a positive integer, got: {self.id}")
        
        if not self.text or not isinstance(self.text, str):
            raise ValueError(f"Question text must be a non-empty string, got: {self.text}")
        
        if self.question_type not in [qt.value for qt in QuestionType]:
            raise ValueError(f"Invalid question type: {self.question_type}")
        
        # All questions must be multiple choice
        if not self.options or len(self.options) < 2:
            raise ValueError("All questions must have at least 2 options")
        
        if not isinstance(self.weight, (int, float)) or self.weight <= 0 or self.weight > 1:
            raise ValueError(f"Weight must be a number between 0 and 1, got: {self.weight}")
        
        if not self.feature_mapping or not isinstance(self.feature_mapping, str):
            raise ValueError(f"Feature mapping must be a non-empty string, got: {self.feature_mapping}")
        
        return True
    
    def is_valid_answer(self, answer: Any) -> bool:
        """Check if an answer is valid for this question"""
        # All questions are multiple choice, so answer must be in options
        return answer in self.options