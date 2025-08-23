import json
import os
from typing import List, Optional
from models.question import Question
from utils.cache_manager import cached, timed

class QuestionService:
    def __init__(self, data_folder: str = "data"):
        self.data_folder = data_folder
        self.questions_file = os.path.join(data_folder, "questions.json")
        self._questions = None
    
    @cached(ttl=7200, namespace="questions")  # Cache for 2 hours
    @timed("load_questions")
    def _load_questions(self) -> List[Question]:
        """Load questions from JSON file"""
        if not os.path.exists(self.questions_file):
            raise FileNotFoundError(f"Questions file not found: {self.questions_file}")
        
        try:
            with open(self.questions_file, 'r', encoding='utf-8') as file:
                data = json.load(file)
            
            questions = []
            for q_data in data.get('questions', []):
                question = Question(
                    id=q_data['id'],
                    text=q_data['text'],
                    question_type=q_data['type'],
                    options=q_data['options'],
                    weight=q_data['weight'],
                    feature_mapping=q_data['feature_mapping']
                )
                questions.append(question)
            
            return questions
            
        except json.JSONDecodeError as e:
            raise ValueError(f"Invalid JSON in questions file: {e}")
        except KeyError as e:
            raise ValueError(f"Missing required field in questions data: {e}")
        except Exception as e:
            raise RuntimeError(f"Error loading questions: {e}")
    
    def get_all_questions(self) -> List[Question]:
        """Get all questions, loading them if not already loaded"""
        if self._questions is None:
            self._questions = self._load_questions()
        return self._questions
    
    def get_question_by_id(self, question_id: int) -> Optional[Question]:
        """Get a specific question by ID"""
        questions = self.get_all_questions()
        for question in questions:
            if question.id == question_id:
                return question
        return None
    
    def get_total_questions(self) -> int:
        """Get total number of questions"""
        return len(self.get_all_questions())
    
    def validate_questions_file(self) -> bool:
        """Validate that the questions file exists and is properly formatted"""
        try:
            questions = self._load_questions()
            
            # Check for duplicate IDs
            question_ids = [q.id for q in questions]
            if len(question_ids) != len(set(question_ids)):
                raise ValueError("Duplicate question IDs found")
            
            # Check that IDs are sequential starting from 1
            expected_ids = list(range(1, len(questions) + 1))
            if sorted(question_ids) != expected_ids:
                raise ValueError("Question IDs must be sequential starting from 1")
            
            return True
            
        except Exception as e:
            print(f"Questions validation failed: {e}")
            return False
    
    def validate_answer(self, question_id: int, answer: str) -> bool:
        """Validate if an answer is valid for a specific question"""
        question = self.get_question_by_id(question_id)
        if question is None:
            return False
        return question.is_valid_answer(answer)