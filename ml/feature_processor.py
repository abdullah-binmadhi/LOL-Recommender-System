import numpy as np
import pandas as pd
from typing import List, Dict, Tuple, Any
from sklearn.preprocessing import LabelEncoder, StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer
from models.user_response import UserResponse
from models.champion import Champion
from services.question_service import QuestionService
from services.champion_service import ChampionService

class FeatureProcessor:
    """Process user responses and champion data into ML-ready features"""
    
    def __init__(self):
        self.question_service = QuestionService()
        self.champion_service = ChampionService()
        self.label_encoders = {}
        self.scaler = StandardScaler()
        self.feature_names = []
        self.is_fitted = False
        
        # Define feature mappings from questions to champion attributes
        self.feature_mappings = {
            'difficulty': 'difficulty',
            'role': 'role', 
            'position': 'position',
            'playstyle': 'herotype',
            'attack_range': 'range_type'
        }
        
        # Define difficulty mappings
        self.difficulty_mapping = {
            'Easy (1-3)': 2,
            'Medium (4-6)': 5,
            'Hard (7-8)': 7.5,
            'Very Hard (9-10)': 9.5
        }
        
        # Define playstyle mappings
        self.playstyle_mapping = {
            'High Damage Output': 'damage',
            'Tanky and Durable': 'toughness',
            'Support Team': 'utility',
            'Balanced/Hybrid': 'balanced'
        }
    
    def process_user_responses(self, responses: Dict[str, str]) -> np.ndarray:
        """Convert user responses to feature vector"""
        features = {}
        
        # Get questions for mapping
        questions = self.question_service.get_all_questions()
        question_map = {q.id: q for q in questions}
        
        for question_id_str, answer in responses.items():
            question_id = int(question_id_str)
            question = question_map.get(question_id)
            
            if not question:
                continue
                
            feature_name = question.feature_mapping
            
            # Process different types of responses
            if feature_name == 'difficulty':
                features['preferred_difficulty'] = self.difficulty_mapping.get(answer, 5)
            elif feature_name == 'role':
                features['preferred_role'] = answer
            elif feature_name == 'position':
                features['preferred_position'] = answer
            elif feature_name == 'playstyle':
                features['preferred_playstyle'] = self.playstyle_mapping.get(answer, 'balanced')
            elif feature_name == 'attack_range':
                features['preferred_range'] = answer.lower() if answer != 'No Preference' else 'any'
        
        return self._encode_user_features(features)
    
    def _encode_user_features(self, features: Dict[str, Any]) -> np.ndarray:
        """Encode user features into numerical format"""
        encoded_features = []
        
        # Numerical features
        encoded_features.append(features.get('preferred_difficulty', 5))
        
        # Categorical features - one-hot encode
        roles = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
        for role in roles:
            encoded_features.append(1 if features.get('preferred_role') == role else 0)
        
        positions = ['Top', 'Jungle', 'Mid', 'Bot', 'Support']
        for position in positions:
            encoded_features.append(1 if features.get('preferred_position') == position else 0)
        
        playstyles = ['damage', 'toughness', 'utility', 'balanced']
        for playstyle in playstyles:
            encoded_features.append(1 if features.get('preferred_playstyle') == playstyle else 0)
        
        ranges = ['ranged', 'melee', 'any']
        for range_type in ranges:
            encoded_features.append(1 if features.get('preferred_range') == range_type else 0)
        
        return np.array(encoded_features).reshape(1, -1)
    
    def encode_champion_features(self, champion: Champion) -> np.ndarray:
        """Encode champion features into numerical format"""
        features = []
        
        # Numerical features
        features.append(champion.difficulty)
        
        # Champion attributes
        if hasattr(champion, 'attributes') and champion.attributes:
            features.extend([
                champion.attributes.get('damage', 5),
                champion.attributes.get('toughness', 5),
                champion.attributes.get('control', 5),
                champion.attributes.get('mobility', 5),
                champion.attributes.get('utility', 5)
            ])
        else:
            features.extend([5, 5, 5, 5, 5])  # Default values
        
        # Role one-hot encoding
        roles = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
        for role in roles:
            features.append(1 if champion.role == role else 0)
        
        # Position one-hot encoding (if available)
        positions = ['Top', 'Jungle', 'Mid', 'Bot', 'Support']
        champion_position = getattr(champion, 'position', 'Any')
        for position in positions:
            features.append(1 if champion_position == position else 0)
        
        # Range type one-hot encoding
        ranges = ['Ranged', 'Melee']
        champion_range = getattr(champion, 'range_type', 'Melee')
        for range_type in ranges:
            features.append(1 if champion_range == range_type else 0)
        
        return np.array(features)
    
    def create_training_data(self) -> Tuple[np.ndarray, np.ndarray, List[str]]:
        """Create training dataset from champions and synthetic user preferences"""
        champions = self.champion_service.get_all_champions()
        
        X = []  # Features (user preferences)
        y = []  # Labels (champion indices)
        champion_names = []
        
        # Generate synthetic training data based on champion characteristics
        for i, champion in enumerate(champions):
            champion_names.append(champion.name)
            
            # Generate multiple synthetic user profiles that would prefer this champion
            for _ in range(10):  # 10 synthetic users per champion for better training
                synthetic_user = self._generate_synthetic_user_for_champion(champion)
                user_features = self._encode_user_features(synthetic_user)
                
                X.append(user_features.flatten())
                y.append(i)  # Champion index as label
        
        return np.array(X), np.array(y), champion_names
    
    def _generate_synthetic_user_for_champion(self, champion: Champion) -> Dict[str, Any]:
        """Generate synthetic user preferences that would match this champion"""
        synthetic_user = {}
        
        # Base preferences on champion characteristics
        synthetic_user['preferred_difficulty'] = champion.difficulty + np.random.normal(0, 1)
        synthetic_user['preferred_difficulty'] = max(1, min(10, synthetic_user['preferred_difficulty']))
        
        # Role preference (80% chance to prefer champion's role)
        if np.random.random() < 0.8:
            synthetic_user['preferred_role'] = champion.role
        else:
            roles = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
            synthetic_user['preferred_role'] = np.random.choice(roles)
        
        # Position preference
        champion_position = getattr(champion, 'position', 'Any')
        if champion_position != 'Any' and np.random.random() < 0.7:
            synthetic_user['preferred_position'] = champion_position
        else:
            positions = ['Top', 'Jungle', 'Mid', 'Bot', 'Support']
            synthetic_user['preferred_position'] = np.random.choice(positions)
        
        # Playstyle based on champion attributes
        if hasattr(champion, 'attributes') and champion.attributes:
            attrs = champion.attributes
            max_attr = max(attrs.get('damage', 5), attrs.get('toughness', 5), 
                          attrs.get('utility', 5))
            
            if max_attr == attrs.get('damage', 5):
                synthetic_user['preferred_playstyle'] = 'damage'
            elif max_attr == attrs.get('toughness', 5):
                synthetic_user['preferred_playstyle'] = 'toughness'
            elif max_attr == attrs.get('utility', 5):
                synthetic_user['preferred_playstyle'] = 'utility'
            else:
                synthetic_user['preferred_playstyle'] = 'balanced'
        else:
            synthetic_user['preferred_playstyle'] = 'balanced'
        
        # Range preference
        champion_range = getattr(champion, 'range_type', 'Melee').lower()
        if np.random.random() < 0.8:
            synthetic_user['preferred_range'] = champion_range
        else:
            synthetic_user['preferred_range'] = 'any'
        
        return synthetic_user
    
    def calculate_similarity_score(self, user_features: np.ndarray, 
                                 champion_features: np.ndarray) -> float:
        """Calculate similarity score between user preferences and champion"""
        # Normalize features
        user_norm = user_features / (np.linalg.norm(user_features) + 1e-8)
        champion_norm = champion_features / (np.linalg.norm(champion_features) + 1e-8)
        
        # Calculate cosine similarity
        similarity = np.dot(user_norm.flatten(), champion_norm.flatten())
        
        # Convert to percentage (0-100)
        return max(0, min(100, (similarity + 1) * 50))
    
    def get_feature_names(self) -> List[str]:
        """Get names of all features"""
        if not self.feature_names:
            self.feature_names = [
                'preferred_difficulty',
                # Role one-hot
                'role_Tank', 'role_Fighter', 'role_Assassin', 
                'role_Mage', 'role_Marksman', 'role_Support',
                # Position one-hot
                'position_Top', 'position_Jungle', 'position_Mid', 
                'position_Bot', 'position_Support',
                # Playstyle one-hot
                'playstyle_damage', 'playstyle_toughness', 
                'playstyle_utility', 'playstyle_balanced',
                # Range one-hot
                'range_ranged', 'range_melee', 'range_any'
            ]
        return self.feature_names
    
    def explain_recommendation(self, user_responses: Dict[str, str], 
                             champion: Champion) -> List[str]:
        """Generate explanation for why this champion was recommended"""
        explanations = []
        
        # Get questions for mapping
        questions = self.question_service.get_all_questions()
        question_map = {q.id: q for q in questions}
        
        for question_id_str, answer in user_responses.items():
            question_id = int(question_id_str)
            question = question_map.get(question_id)
            
            if not question:
                continue
            
            feature_name = question.feature_mapping
            
            # Generate explanations based on matches
            if feature_name == 'role' and answer == champion.role:
                explanations.append(f"Perfect role match: You wanted {answer} and {champion.name} is a {champion.role}")
            
            elif feature_name == 'difficulty':
                user_diff = self.difficulty_mapping.get(answer, 5)
                if abs(user_diff - champion.difficulty) <= 2:
                    explanations.append(f"Good difficulty match: You prefer {answer} difficulty and {champion.name} has {champion.difficulty}/10 difficulty")
            
            elif feature_name == 'position':
                champion_pos = getattr(champion, 'position', 'Any')
                if answer == champion_pos:
                    explanations.append(f"Position match: You prefer {answer} and {champion.name} excels in {champion_pos}")
            
            elif feature_name == 'attack_range':
                champion_range = getattr(champion, 'range_type', 'Melee')
                if answer.lower() in champion_range.lower() or answer == 'No Preference':
                    explanations.append(f"Range preference: You wanted {answer} and {champion.name} is {champion_range}")
        
        if not explanations:
            explanations.append(f"{champion.name} is a well-rounded champion that matches your overall preferences")
        
        return explanations