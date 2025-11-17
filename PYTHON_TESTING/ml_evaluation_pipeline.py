#!/usr/bin/env python3
"""
ML Model Evaluation Pipeline for Champion Recommender System

This script implements a comprehensive evaluation system that:
1. Loads trained ML models (Random Forest, Decision Tree, KNN)
2. Creates a unified recommendation system
3. Implements Precision@K and MRR metrics
4. Builds an evaluation pipeline
5. Compares all three models
6. Integrates evaluation into the full system workflow
"""

import pickle
import json
import pandas as pd
import numpy as np
from pathlib import Path
from typing import List, Dict, Tuple, Any, Optional
from dataclasses import dataclass
import warnings
warnings.filterwarnings('ignore')


@dataclass
class ModelEvaluationResult:
    """Data class to store evaluation results for a single model"""
    model_name: str
    precision_at_1: float
    precision_at_3: float
    precision_at_5: float
    mrr: float
    num_users: int


class MLModelLoader:
    """Handles loading of trained ML models"""
    
    def __init__(self, models_dir: str = "../archive/models/trained"):
        self.models_dir = Path(models_dir)
        self.models = {}
        self.scaler = None
        self.metadata = None
        
    def load_all_models(self) -> Dict[str, Any]:
        """Load all trained ML models"""
        model_files = {
            'random_forest': 'random_forest_model.pkl',
            'decision_tree': 'decision_tree_model.pkl',
            'knn': 'knn_model.pkl'
        }
        
        for model_name, filename in model_files.items():
            model_path = self.models_dir / filename
            if model_path.exists():
                with open(model_path, 'rb') as f:
                    self.models[model_name] = pickle.load(f)
                print(f"✓ Loaded {model_name} model")
            else:
                print(f"✗ Model file not found: {model_path}")
        
        # Load scaler
        scaler_path = self.models_dir / 'scaler.pkl'
        if scaler_path.exists():
            with open(scaler_path, 'rb') as f:
                self.scaler = pickle.load(f)
            print("✓ Loaded scaler")
        
        # Load metadata
        metadata_path = self.models_dir / 'metadata.pkl'
        if metadata_path.exists():
            with open(metadata_path, 'rb') as f:
                self.metadata = pickle.load(f)
            print("✓ Loaded metadata")
        
        return self.models


class UnifiedRecommendationSystem:
    """Unified system for generating champion recommendations"""
    
    def __init__(self, model, scaler, champion_names: List[str], metadata: Optional[Dict] = None):
        self.model = model
        self.scaler = scaler
        self.champion_names = champion_names
        self.metadata = metadata or {}
        
    def preprocess_user_input(self, user_answers: Dict[str, Any]) -> np.ndarray:
        """
        Preprocess user questionnaire answers into model input features
        
        Args:
            user_answers: Dictionary of user responses
            
        Returns:
            Preprocessed feature array
        """
        # Extract and encode features based on your questionnaire
        # This is a placeholder - adjust based on your actual feature engineering
        features = []
        
        # Example feature extraction (adjust to match your actual features)
        # Role encoding
        role_map = {'Mage': 0, 'Fighter': 1, 'Assassin': 2, 'Tank': 3, 'Support': 4, 'Marksman': 5, 'No Preference': 6}
        role = user_answers.get('role', 'No Preference')
        features.append(role_map.get(role, 6))
        
        # Difficulty (numeric)
        difficulty = user_answers.get('difficulty', 5)
        if isinstance(difficulty, str):
            if '1-3' in difficulty:
                difficulty = 2
            elif '4-6' in difficulty:
                difficulty = 5
            elif '7-8' in difficulty:
                difficulty = 7.5
            elif '9-10' in difficulty:
                difficulty = 9.5
        features.append(float(difficulty))
        
        # Add other features as needed
        # Playstyle, preferences, etc.
        
        # Convert to numpy array and reshape
        features_array = np.array(features).reshape(1, -1)
        
        # Scale features if scaler is available
        if self.scaler:
            features_array = self.scaler.transform(features_array)
        
        return features_array
    
    def generate_recommendations(self, user_input: np.ndarray, k: int = 5) -> Tuple[List[str], List[float]]:
        """
        Generate ranked champion recommendations
        
        Args:
            user_input: Preprocessed user features
            k: Number of top recommendations to return
            
        Returns:
            Tuple of (champion names, confidence scores)
        """
        # Get prediction probabilities for all champions
        if hasattr(self.model, 'predict_proba'):
            probabilities = self.model.predict_proba(user_input)[0]
        elif hasattr(self.model, 'decision_function'):
            probabilities = self.model.decision_function(user_input)[0]
        else:
            # Fallback: use predict and create dummy scores
            prediction = self.model.predict(user_input)
            probabilities = np.zeros(len(self.champion_names))
            if len(prediction) > 0:
                probabilities[0] = 1.0
        
        # Sort champions by probability/score (descending)
        sorted_indices = np.argsort(probabilities)[::-1]
        
        # Get top K champions
        top_k_indices = sorted_indices[:k]
        top_k_champions = [self.champion_names[i] for i in top_k_indices]
        top_k_scores = [probabilities[i] for i in top_k_indices]
        
        return top_k_champions, top_k_scores
    
    def get_full_ranking(self, user_input: np.ndarray) -> List[Tuple[str, float]]:
        """
        Get complete ranking of all champions
        
        Args:
            user_input: Preprocessed user features
            
        Returns:
            List of (champion_name, score) tuples sorted by score
        """
        if hasattr(self.model, 'predict_proba'):
            probabilities = self.model.predict_proba(user_input)[0]
        elif hasattr(self.model, 'decision_function'):
            probabilities = self.model.decision_function(user_input)[0]
        else:
            probabilities = np.random.rand(len(self.champion_names))
        
        # Create list of (champion, score) tuples
        champion_scores = list(zip(self.champion_names, probabilities))
        
        # Sort by score descending
        champion_scores.sort(key=lambda x: x[1], reverse=True)
        
        return champion_scores


class RankingMetrics:
    """Implements ranking evaluation metrics"""
    
    @staticmethod
    def precision_at_k(recommended: List[str], relevant: set, k: int) -> float:
        """
        Calculate Precision@K
        
        Args:
            recommended: Ranked list of recommended champions
            relevant: Set of ground truth relevant champions
            k: Number of top recommendations to consider
            
        Returns:
            Precision@K score (0.0 to 1.0)
        """
        if not recommended or k <= 0 or not relevant:
            return 0.0
        
        # Get top K recommendations
        top_k = recommended[:k]
        
        # Count relevant items in top K
        relevant_in_top_k = sum(1 for champ in top_k if champ in relevant)
        
        # Calculate precision
        return relevant_in_top_k / k
    
    @staticmethod
    def reciprocal_rank(recommended: List[str], relevant: set) -> float:
        """
        Calculate Reciprocal Rank for a single query
        
        Args:
            recommended: Ranked list of recommended champions
            relevant: Set of ground truth relevant champions
            
        Returns:
            Reciprocal rank (1/position of first relevant item, or 0)
        """
        if not recommended or not relevant:
            return 0.0
        
        # Find position of first relevant champion (1-indexed)
        for position, champion in enumerate(recommended, start=1):
            if champion in relevant:
                return 1.0 / position
        
        # No relevant champion found
        return 0.0
    
    @staticmethod
    def mean_reciprocal_rank(reciprocal_ranks: List[float]) -> float:
        """
        Calculate Mean Reciprocal Rank across multiple queries
        
        Args:
            reciprocal_ranks: List of reciprocal rank scores
            
        Returns:
            Mean of all reciprocal ranks
        """
        if not reciprocal_ranks:
            return 0.0
        return sum(reciprocal_ranks) / len(reciprocal_ranks)


class EvaluationPipeline:
    """Complete evaluation pipeline for ML models"""
    
    def __init__(self, models: Dict[str, Any], scaler, champion_names: List[str], metadata: Optional[Dict] = None):
        self.models = models
        self.scaler = scaler
        self.champion_names = champion_names
        self.metadata = metadata
        self.metrics = RankingMetrics()
        
    def load_test_dataset(self, test_file: str) -> List[Dict[str, Any]]:
        """
        Load test dataset with ground truth labels
        
        Args:
            test_file: Path to test data file (JSON or CSV)
            
        Returns:
            List of test user data dictionaries
        """
        test_path = Path(test_file)
        
        if not test_path.exists():
            print(f"Test file not found: {test_file}")
            return []
        
        if test_path.suffix == '.json':
            with open(test_path, 'r') as f:
                return json.load(f)
        elif test_path.suffix == '.csv':
            df = pd.read_csv(test_path)
            return df.to_dict('records')
        else:
            print(f"Unsupported file format: {test_path.suffix}")
            return []
    
    def extract_ground_truth(self, user_data: Dict[str, Any]) -> set:
        """
        Extract ground truth relevant champions for a user
        
        Args:
            user_data: User test data
            
        Returns:
            Set of relevant champion names
        """
        # This depends on how your test data is structured
        # Option 1: Explicit ground truth field
        if 'ground_truth' in user_data:
            truth = user_data['ground_truth']
            if isinstance(truth, list):
                return set(truth)
            elif isinstance(truth, str):
                return {truth}
        
        # Option 2: Use actual champion they selected
        if 'selected_champion' in user_data:
            return {user_data['selected_champion']}
        
        # Option 3: Calculate based on preferences (fallback)
        return self._calculate_relevant_champions(user_data)
    
    def _calculate_relevant_champions(self, user_data: Dict[str, Any]) -> set:
        """
        Calculate relevant champions based on user preferences
        (Fallback when no ground truth is available)
        """
        # Load champion data
        champion_data_path = Path("../src/data/champions.json")
        if not champion_data_path.exists():
            return set()
        
        with open(champion_data_path, 'r') as f:
            all_champions = json.load(f)
        
        relevant = set()
        user_role = user_data.get('role', 'No Preference')
        
        # Simple relevance: match role
        for champ_name, champ_data in all_champions.items():
            if user_role == 'No Preference' or champ_data.get('role') == user_role:
                relevant.add(champ_name)
        
        return relevant
    
    def evaluate_single_model(self, model_name: str, test_data: List[Dict[str, Any]], 
                             k_values: List[int] = [1, 3, 5]) -> ModelEvaluationResult:
        """
        Evaluate a single model on test dataset
        
        Args:
            model_name: Name of the model to evaluate
            test_data: List of test user data
            k_values: List of K values for Precision@K
            
        Returns:
            ModelEvaluationResult object
        """
        model = self.models.get(model_name)
        if not model:
            print(f"Model {model_name} not found")
            return None
        
        # Create recommendation system for this model
        rec_system = UnifiedRecommendationSystem(model, self.scaler, self.champion_names, self.metadata)
        
        # Initialize metric accumulators
        precision_scores = {k: [] for k in k_values}
        reciprocal_ranks = []
        
        # Evaluate each user
        for user_data in test_data:
            try:
                # Preprocess user input
                user_features = rec_system.preprocess_user_input(user_data)
                
                # Generate recommendations
                full_ranking = rec_system.get_full_ranking(user_features)
                recommended_champions = [champ for champ, _ in full_ranking]
                
                # Get ground truth
                relevant_champions = self.extract_ground_truth(user_data)
                
                if not relevant_champions:
                    continue
                
                # Calculate Precision@K for each K
                for k in k_values:
                    p_at_k = self.metrics.precision_at_k(recommended_champions, relevant_champions, k)
                    precision_scores[k].append(p_at_k)
                
                # Calculate Reciprocal Rank
                rr = self.metrics.reciprocal_rank(recommended_champions, relevant_champions)
                reciprocal_ranks.append(rr)
                
            except Exception as e:
                print(f"Error evaluating user: {e}")
                continue
        
        # Calculate average metrics
        avg_p1 = np.mean(precision_scores[1]) if precision_scores[1] else 0.0
        avg_p3 = np.mean(precision_scores[3]) if precision_scores[3] else 0.0
        avg_p5 = np.mean(precision_scores[5]) if precision_scores[5] else 0.0
        avg_mrr = self.metrics.mean_reciprocal_rank(reciprocal_ranks)
        
        return ModelEvaluationResult(
            model_name=model_name,
            precision_at_1=avg_p1,
            precision_at_3=avg_p3,
            precision_at_5=avg_p5,
            mrr=avg_mrr,
            num_users=len(reciprocal_ranks)
        )
    
    def compare_all_models(self, test_data: List[Dict[str, Any]]) -> List[ModelEvaluationResult]:
        """
        Compare all three models on the same test dataset
        
        Args:
            test_data: List of test user data
            
        Returns:
            List of ModelEvaluationResult objects
        """
        results = []
        
        print("\n" + "="*70)
        print("EVALUATING ALL MODELS")
        print("="*70)
        
        for model_name in ['random_forest', 'decision_tree', 'knn']:
            print(f"\nEvaluating {model_name.replace('_', ' ').title()}...")
            result = self.evaluate_single_model(model_name, test_data)
            if result:
                results.append(result)
                print(f"  ✓ Completed: {result.num_users} users evaluated")
        
        return results
    
    def print_comparison_table(self, results: List[ModelEvaluationResult]):
        """
        Print a formatted comparison table of all models
        
        Args:
            results: List of evaluation results
        """
        print("\n" + "="*70)
        print("MODEL COMPARISON RESULTS")
        print("="*70)
        print(f"{'Model':<20} {'P@1':<10} {'P@3':<10} {'P@5':<10} {'MRR':<10} {'Users':<10}")
        print("-"*70)
        
        for result in results:
            print(f"{result.model_name.replace('_', ' ').title():<20} "
                  f"{result.precision_at_1:<10.4f} "
                  f"{result.precision_at_3:<10.4f} "
                  f"{result.precision_at_5:<10.4f} "
                  f"{result.mrr:<10.4f} "
                  f"{result.num_users:<10}")
        
        print("="*70)
        
        # Determine best model
        if results:
            best_by_p5 = max(results, key=lambda x: x.precision_at_5)
            best_by_mrr = max(results, key=lambda x: x.mrr)
            
            print(f"\n🏆 Best Model by Precision@5: {best_by_p5.model_name.replace('_', ' ').title()} "
                  f"({best_by_p5.precision_at_5:.4f})")
            print(f"🏆 Best Model by MRR: {best_by_mrr.model_name.replace('_', ' ').title()} "
                  f"({best_by_mrr.mrr:.4f})")
            
            # Overall recommendation
            if best_by_p5.model_name == best_by_mrr.model_name:
                print(f"\n✨ RECOMMENDED MODEL FOR PRODUCTION: {best_by_p5.model_name.replace('_', ' ').title()}")
            else:
                print(f"\n💡 Consider: {best_by_p5.model_name.replace('_', ' ').title()} for overall accuracy, "
                      f"{best_by_mrr.model_name.replace('_', ' ').title()} for early relevance")


def main():
    """Main execution function"""
    print("="*70)
    print("ML MODEL EVALUATION PIPELINE")
    print("Champion Recommender System")
    print("="*70)
    
    # Step 1: Load trained models
    print("\n[Step 1] Loading trained ML models...")
    loader = MLModelLoader()
    models = loader.load_all_models()
    
    if not models:
        print("❌ No models loaded. Please train models first.")
        return
    
    # Step 2: Load champion names
    print("\n[Step 2] Loading champion data...")
    champion_data_path = Path("../src/data/champions.json")
    if champion_data_path.exists():
        with open(champion_data_path, 'r') as f:
            champions_dict = json.load(f)
            champion_names = list(champions_dict.keys())
        print(f"✓ Loaded {len(champion_names)} champions")
    else:
        print("❌ Champion data not found")
        return
    
    # Step 3: Create evaluation pipeline
    print("\n[Step 3] Initializing evaluation pipeline...")
    pipeline = EvaluationPipeline(models, loader.scaler, champion_names, loader.metadata)
    print("✓ Pipeline ready")
    
    # Step 4: Load test dataset
    print("\n[Step 4] Loading test dataset...")
    test_data = pipeline.load_test_dataset("user_data_backup.json")
    if not test_data:
        print("❌ No test data loaded")
        return
    print(f"✓ Loaded {len(test_data)} test users")
    
    # Step 5: Compare all models
    print("\n[Step 5] Running evaluation...")
    results = pipeline.compare_all_models(test_data)
    
    # Step 6: Display results
    print("\n[Step 6] Results:")
    pipeline.print_comparison_table(results)
    
    print("\n" + "="*70)
    print("EVALUATION COMPLETE")
    print("="*70)


if __name__ == "__main__":
    main()
