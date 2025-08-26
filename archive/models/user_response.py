from dataclasses import dataclass
from typing import List, Any, Optional
from datetime import datetime

@dataclass
class UserResponse:
    question_id: int
    answer: Any
    timestamp: datetime
    
    def __post_init__(self):
        """Validate user response data after initialization"""
        self.validate()
    
    def validate(self) -> bool:
        """Validate user response data"""
        if not isinstance(self.question_id, int) or self.question_id <= 0:
            raise ValueError(f"Question ID must be a positive integer, got: {self.question_id}")
        
        if self.answer is None:
            raise ValueError("Answer cannot be None")
        
        if not isinstance(self.timestamp, datetime):
            raise ValueError(f"Timestamp must be a datetime object, got: {type(self.timestamp)}")
        
        return True

@dataclass
class UserSession:
    session_id: str
    responses: List[UserResponse]
    current_question: int
    completed: bool = False
    created_at: Optional[datetime] = None
    
    def __post_init__(self):
        """Initialize session data after creation"""
        if self.created_at is None:
            self.created_at = datetime.now()
        self.validate()
    
    def validate(self) -> bool:
        """Validate user session data"""
        if not self.session_id or not isinstance(self.session_id, str):
            raise ValueError(f"Session ID must be a non-empty string, got: {self.session_id}")
        
        if not isinstance(self.responses, list):
            raise ValueError("Responses must be a list")
        
        # Validate all responses
        for response in self.responses:
            if not isinstance(response, UserResponse):
                raise ValueError("All responses must be UserResponse objects")
            response.validate()
        
        if not isinstance(self.current_question, int) or self.current_question < 0:
            raise ValueError(f"Current question must be a non-negative integer, got: {self.current_question}")
        
        if not isinstance(self.completed, bool):
            raise ValueError(f"Completed must be a boolean, got: {type(self.completed)}")
        
        return True
    
    def add_response(self, question_id: int, answer: Any) -> None:
        """Add a response to the session"""
        response = UserResponse(
            question_id=question_id,
            answer=answer,
            timestamp=datetime.now()
        )
        self.responses.append(response)
    
    def get_response_by_question_id(self, question_id: int) -> Optional[UserResponse]:
        """Get a response by question ID"""
        for response in self.responses:
            if response.question_id == question_id:
                return response
        return None
    
    def has_answered_question(self, question_id: int) -> bool:
        """Check if a question has been answered"""
        return self.get_response_by_question_id(question_id) is not None
    
    def get_completion_percentage(self, total_questions: int) -> float:
        """Get completion percentage of the questionnaire"""
        if total_questions == 0:
            return 0.0
        return len(self.responses) / total_questions * 100