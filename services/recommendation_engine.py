import numpy as np
import pickle
import os
from typing import List, Dict, Tuple, Optional
import joblib
from models.champion import Champion, ChampionRecommendation
from services.champion_service import ChampionService
from ml.feature_processor import FeatureProcessor
from ml.model_trainer import ModelTrainer
from utils.cache_manager import cached, timed, cache_key_for_user_responses, cache_manager

class RecommendationEngine:
    """Main recommendation engine using trained ML models"""
    
    def __init__(self, model_folder: str = "models/trained"):
        self.model_folder = model_folder
        self.champion_service = ChampionService()
        self.feature_processor = FeatureProcessor()
        
        # Model components
        self.models = {}
        self.scaler = None
        self.champion_names = []
        self.best_model_name = 'random_forest'  # Default
        
        # Load trained models
        self.load_models()
    
    def load_models(self) -> bool:
        """Load trained models and metadata"""
        try:
            # Load metadata
            metadata_path = os.path.join(self.model_folder, "metadata.pkl")
            if not os.path.exists(metadata_path):
                print("No trained models found. Training new models...")
                self._train_models()
                return True
            
            with open(metadata_path, 'rb') as f:
                metadata = pickle.load(f)
            
            self.champion_names = metadata['champion_names']
            
            # Load scaler
            scaler_path = os.path.join(self.model_folder, "scaler.pkl")
            if os.path.exists(scaler_path):
                self.scaler = joblib.load(scaler_path)
            
            # Load models
            model_names = ['random_forest', 'decision_tree', 'knn']
            for model_name in model_names:
                model_path = os.path.join(self.model_folder, f"{model_name}_model.pkl")
                if os.path.exists(model_path):
                    self.models[model_name] = joblib.load(model_path)
            
            print(f"Loaded {len(self.models)} trained models")
            return True
            
        except Exception as e:
            print(f"Error loading models: {e}")
            print("Training new models...")
            self._train_models()
            return True
    
    def _train_models(self) -> None:
        """Train models if they don't exist"""
        trainer = ModelTrainer(self.model_folder)
        self.best_model_name = trainer.train_and_save_all()
        
        # Reload after training
        self.load_models()
    
    @cached(ttl=1800, namespace="ml_predictions", key_func=lambda self, responses, model=None: f"{cache_key_for_user_responses(responses)}_{model or self.best_model_name}")
    @timed("ml_prediction")
    def predict_champion(self, user_responses: Dict[str, str], 
                        model_name: Optional[str] = None) -> ChampionRecommendation:
        """Predict the best champion for user responses"""
        if not self.models:
            raise RuntimeError("No trained models available")
        
        # Use specified model or best model
        model_name = model_name or self.best_model_name
        if model_name not in self.models:
            model_name = list(self.models.keys())[0]  # Fallback to first available
        
        model = self.models[model_name]
        
        # Process user responses into features
        user_features = self.feature_processor.process_user_responses(user_responses)
        
        # Scale features if using KNN
        if model_name == 'knn' and self.scaler:
            user_features = self.scaler.transform(user_features)
        
        # Get prediction and probabilities
        prediction = model.predict(user_features)[0]
        probabilities = model.predict_proba(user_features)[0]
        
        # Get champion
        champion_name = self.champion_names[prediction]
        champion = self.champion_service.get_champion_by_name(champion_name)
        
        if not champion:
            # Fallback to first champion if not found
            champion = self.champion_service.get_all_champions()[0]
        
        # Calculate confidence score
        confidence_score = probabilities[prediction]
        
        # Generate explanation
        explanation = self._generate_explanation(user_responses, champion, model_name)
        match_reasons = self.feature_processor.explain_recommendation(user_responses, champion)
        
        return ChampionRecommendation(
            champion=champion,
            confidence_score=confidence_score,
            explanation=explanation,
            match_reasons=match_reasons
        )
    
    @cached(ttl=1800, namespace="ml_alternatives", key_func=lambda self, responses, exclude=None, num=3: f"{cache_key_for_user_responses(responses)}_alt_{len(exclude or [])}")
    @timed("ml_alternatives")
    def get_alternative_recommendations(self, user_responses: Dict[str, str], 
                                     exclude: List[str] = None,
                                     num_alternatives: int = 3) -> List[ChampionRecommendation]:
        """Get alternative champion recommendations"""
        if not self.models:
            return []
        
        exclude = exclude or []
        alternatives = []
        
        # Use ensemble approach - get predictions from all models
        model_predictions = {}
        user_features = self.feature_processor.process_user_responses(user_responses)
        
        for model_name, model in self.models.items():
            features = user_features
            if model_name == 'knn' and self.scaler:
                features = self.scaler.transform(features)
            
            probabilities = model.predict_proba(features)[0]
            model_predictions[model_name] = probabilities
        
        # Ensemble predictions (average probabilities)
        ensemble_probs = np.mean(list(model_predictions.values()), axis=0)
        
        # Get top predictions excluding already recommended
        top_indices = np.argsort(ensemble_probs)[::-1]
        
        count = 0
        for idx in top_indices:
            if count >= num_alternatives:
                break
                
            champion_name = self.champion_names[idx]
            if champion_name in exclude:
                continue
            
            champion = self.champion_service.get_champion_by_name(champion_name)
            if not champion:
                continue
            
            confidence_score = ensemble_probs[idx]
            explanation = self._generate_explanation(user_responses, champion, "ensemble")
            match_reasons = self.feature_processor.explain_recommendation(user_responses, champion)
            
            alternatives.append(ChampionRecommendation(
                champion=champion,
                confidence_score=confidence_score,
                explanation=explanation,
                match_reasons=match_reasons
            ))
            
            count += 1
        
        return alternatives
    
    def _generate_explanation(self, user_responses: Dict[str, str], 
                            champion: Champion, model_name: str) -> str:
        """Generate explanation for the recommendation"""
        explanations = []
        
        # Model-specific explanation
        model_explanations = {
            'random_forest': f"Random Forest analysis of {len(self.champion_names)} champions",
            'decision_tree': f"Decision tree analysis based on your preferences",
            'knn': f"Similarity matching with champion database",
            'ensemble': f"Combined analysis from multiple AI models"
        }
        
        base_explanation = model_explanations.get(model_name, "AI analysis")
        
        # Add champion-specific details
        role_match = any(resp for resp in user_responses.values() if resp == champion.role)
        if role_match:
            explanations.append(f"Perfect role alignment with {champion.role}")
        
        # Difficulty match
        for resp in user_responses.values():
            if "Easy" in resp and champion.difficulty <= 3:
                explanations.append("Matches your preference for easier champions")
                break
            elif "Hard" in resp and champion.difficulty >= 7:
                explanations.append("Matches your preference for challenging champions")
                break
        
        if explanations:
            return f"{base_explanation} shows {champion.name} is ideal because: {', '.join(explanations)}"
        else:
            return f"{base_explanation} identified {champion.name} as your best match based on overall compatibility"
    
    def get_model_info(self) -> Dict[str, any]:
        """Get information about loaded models"""
        return {
            'available_models': list(self.models.keys()),
            'best_model': self.best_model_name,
            'num_champions': len(self.champion_names),
            'models_loaded': len(self.models) > 0
        }
    
    def predict_with_all_models(self, user_responses: Dict[str, str]) -> Dict[str, ChampionRecommendation]:
        """Get predictions from all available models"""
        predictions = {}
        
        for model_name in self.models.keys():
            try:
                prediction = self.predict_champion(user_responses, model_name)
                predictions[model_name] = prediction
            except Exception as e:
                print(f"Error with model {model_name}: {e}")
        
        return predictions
    
    def get_champion_similarity_scores(self, user_responses: Dict[str, str]) -> List[Tuple[Champion, float]]:
        """Get similarity scores for all champions"""
        if not self.models:
            return []
        
        user_features = self.feature_processor.process_user_responses(user_responses)
        champions = self.champion_service.get_all_champions()
        
        similarities = []
        
        for champion in champions:
            champion_features = self.feature_processor.encode_champion_features(champion)
            similarity = self.feature_processor.calculate_similarity_score(
                user_features, champion_features
            )
            similarities.append((champion, similarity))
        
        # Sort by similarity score
        similarities.sort(key=lambda x: x[1], reverse=True)
        
        return similarities