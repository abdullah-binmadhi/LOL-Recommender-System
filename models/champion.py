from dataclasses import dataclass
from typing import List, Dict, Optional

@dataclass
class Ability:
    name: str
    description: str
    ability_type: str  # passive, Q, W, E, R
    
    def validate(self) -> bool:
        """Validate ability data"""
        if not self.name or not isinstance(self.name, str):
            raise ValueError(f"Ability name must be a non-empty string, got: {self.name}")
        
        if not self.description or not isinstance(self.description, str):
            raise ValueError(f"Ability description must be a non-empty string")
        
        valid_types = ['passive', 'Q', 'W', 'E', 'R']
        if self.ability_type not in valid_types:
            raise ValueError(f"Ability type must be one of {valid_types}, got: {self.ability_type}")
        
        return True

@dataclass
class Champion:
    id: str
    name: str
    title: str
    role: str
    difficulty: int
    tags: List[str]
    image_url: Optional[str] = None
    abilities: Optional[List[Ability]] = None
    attributes: Optional[Dict[str, float]] = None
    
    def __post_init__(self):
        """Validate champion data after initialization"""
        self.validate()
        
        # Set default values
        if self.abilities is None:
            self.abilities = []
        if self.attributes is None:
            self.attributes = {}
    
    def validate(self) -> bool:
        """Validate champion data"""
        if not self.id or not isinstance(self.id, str):
            raise ValueError(f"Champion ID must be a non-empty string, got: {self.id}")
        
        if not self.name or not isinstance(self.name, str):
            raise ValueError(f"Champion name must be a non-empty string, got: {self.name}")
        
        if not self.title or not isinstance(self.title, str):
            raise ValueError(f"Champion title must be a non-empty string, got: {self.title}")
        
        valid_roles = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
        if self.role not in valid_roles:
            raise ValueError(f"Champion role must be one of {valid_roles}, got: {self.role}")
        
        if not isinstance(self.difficulty, int) or self.difficulty < 1 or self.difficulty > 10:
            raise ValueError(f"Difficulty must be an integer between 1 and 10, got: {self.difficulty}")
        
        if not isinstance(self.tags, list) or len(self.tags) == 0:
            raise ValueError(f"Tags must be a non-empty list, got: {self.tags}")
        
        # Validate abilities if provided
        if self.abilities:
            for ability in self.abilities:
                if not isinstance(ability, Ability):
                    raise ValueError("All abilities must be Ability objects")
                ability.validate()
        
        return True

@dataclass
class ChampionRecommendation:
    champion: Champion
    confidence_score: float
    explanation: str
    match_reasons: List[str]
    
    def __post_init__(self):
        """Validate recommendation data after initialization"""
        self.validate()
    
    def validate(self) -> bool:
        """Validate recommendation data"""
        if not isinstance(self.champion, Champion):
            raise ValueError("Champion must be a Champion object")
        
        if not isinstance(self.confidence_score, (int, float)) or self.confidence_score < 0 or self.confidence_score > 1:
            raise ValueError(f"Confidence score must be between 0 and 1, got: {self.confidence_score}")
        
        if not self.explanation or not isinstance(self.explanation, str):
            raise ValueError("Explanation must be a non-empty string")
        
        if not isinstance(self.match_reasons, list):
            raise ValueError("Match reasons must be a list")
        
        return True