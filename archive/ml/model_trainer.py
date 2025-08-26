import numpy as np
import pandas as pd
import pickle
import os
from typing import List, Dict, Tuple, Any, Optional
from sklearn.ensemble import RandomForestClassifier
from sklearn.tree import DecisionTreeClassifier
from sklearn.neighbors import KNeighborsClassifier
from sklearn.model_selection import train_test_split, cross_val_score, GridSearchCV
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix
from sklearn.preprocessing import StandardScaler
import joblib
from ml.feature_processor import FeatureProcessor
from services.champion_service import ChampionService

class ModelTrainer:
    """Train and manage multiple ML models for champion recommendation"""
    
    def __init__(self, model_folder: str = "models/trained"):
        self.model_folder = model_folder
        self.feature_processor = FeatureProcessor()
        self.champion_service = ChampionService()
        
        # Initialize models
        self.models = {
            'random_forest': RandomForestClassifier(
                n_estimators=100,
                max_depth=10,
                min_samples_split=5,
                min_samples_leaf=2,
                random_state=42
            ),
            'decision_tree': DecisionTreeClassifier(
                max_depth=15,
                min_samples_split=10,
                min_samples_leaf=5,
                random_state=42
            ),
            'knn': KNeighborsClassifier(
                n_neighbors=5,
                weights='distance',
                metric='euclidean'
            )
        }
        
        self.trained_models = {}
        self.model_scores = {}
        self.champion_names = []
        self.scaler = StandardScaler()
        
        # Ensure model directory exists
        os.makedirs(self.model_folder, exist_ok=True)
    
    def prepare_training_data(self) -> Tuple[np.ndarray, np.ndarray, List[str]]:
        """Prepare training data from champions and synthetic user preferences"""
        print("Preparing training data...")
        
        X, y, champion_names = self.feature_processor.create_training_data()
        self.champion_names = champion_names
        
        print(f"Generated {len(X)} training samples for {len(champion_names)} champions")
        print(f"Feature vector size: {X.shape[1]}")
        
        return X, y, champion_names
    
    def train_models(self, X: np.ndarray, y: np.ndarray) -> Dict[str, float]:
        """Train all models and return their scores"""
        print("Training models...")
        
        # Split data
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42, stratify=y
        )
        
        # Scale features for KNN
        X_train_scaled = self.scaler.fit_transform(X_train)
        X_test_scaled = self.scaler.transform(X_test)
        
        scores = {}
        
        for model_name, model in self.models.items():
            print(f"\nTraining {model_name}...")
            
            # Use scaled data for KNN, original for tree-based models
            if model_name == 'knn':
                model.fit(X_train_scaled, y_train)
                y_pred = model.predict(X_test_scaled)
            else:
                model.fit(X_train, y_train)
                y_pred = model.predict(X_test)
            
            # Calculate accuracy
            accuracy = accuracy_score(y_test, y_pred)
            scores[model_name] = accuracy
            
            # Cross-validation score (use 3-fold to avoid issues with many classes)
            try:
                if model_name == 'knn':
                    cv_scores = cross_val_score(model, X_train_scaled, y_train, cv=3)
                else:
                    cv_scores = cross_val_score(model, X_train, y_train, cv=3)
            except ValueError:
                # If still issues with CV, skip it
                cv_scores = np.array([accuracy])
            
            print(f"{model_name} - Accuracy: {accuracy:.3f}, CV Score: {cv_scores.mean():.3f} (+/- {cv_scores.std() * 2:.3f})")
            
            # Store trained model
            self.trained_models[model_name] = model
        
        self.model_scores = scores
        return scores
    
    def hyperparameter_tuning(self, X: np.ndarray, y: np.ndarray) -> None:
        """Perform hyperparameter tuning for the best performing models"""
        print("\nPerforming hyperparameter tuning...")
        
        # Split data
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42, stratify=y
        )
        
        # Scale for KNN
        X_train_scaled = self.scaler.fit_transform(X_train)
        
        # Parameter grids
        param_grids = {
            'random_forest': {
                'n_estimators': [50, 100, 200],
                'max_depth': [10, 15, 20],
                'min_samples_split': [2, 5, 10]
            },
            'decision_tree': {
                'max_depth': [10, 15, 20, 25],
                'min_samples_split': [5, 10, 20],
                'min_samples_leaf': [2, 5, 10]
            },
            'knn': {
                'n_neighbors': [3, 5, 7, 9],
                'weights': ['uniform', 'distance'],
                'metric': ['euclidean', 'manhattan']
            }
        }
        
        # Tune each model
        for model_name in ['random_forest', 'decision_tree']:  # Skip KNN for speed
            if model_name in self.model_scores and self.model_scores[model_name] > 0.7:
                print(f"Tuning {model_name}...")
                
                grid_search = GridSearchCV(
                    self.models[model_name],
                    param_grids[model_name],
                    cv=2,  # Reduced CV folds
                    scoring='accuracy',
                    n_jobs=1  # Avoid parallel issues
                )
                
                grid_search.fit(X_train, y_train)
                
                print(f"Best parameters for {model_name}: {grid_search.best_params_}")
                print(f"Best CV score: {grid_search.best_score_:.3f}")
                
                # Update model with best parameters
                self.trained_models[model_name] = grid_search.best_estimator_
    
    def evaluate_models(self, X: np.ndarray, y: np.ndarray) -> Dict[str, Dict]:
        """Evaluate all trained models"""
        print("\nEvaluating models...")
        
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42, stratify=y
        )
        
        X_test_scaled = self.scaler.transform(X_test)
        
        evaluation_results = {}
        
        for model_name, model in self.trained_models.items():
            print(f"\nEvaluating {model_name}...")
            
            # Make predictions
            if model_name == 'knn':
                y_pred = model.predict(X_test_scaled)
                y_proba = model.predict_proba(X_test_scaled)
            else:
                y_pred = model.predict(X_test)
                y_proba = model.predict_proba(X_test)
            
            # Calculate metrics
            accuracy = accuracy_score(y_test, y_pred)
            
            # Top-k accuracy (how often the correct champion is in top 3 predictions)
            top_3_accuracy = self._calculate_top_k_accuracy(y_test, y_proba, k=3)
            top_5_accuracy = self._calculate_top_k_accuracy(y_test, y_proba, k=5)
            
            evaluation_results[model_name] = {
                'accuracy': accuracy,
                'top_3_accuracy': top_3_accuracy,
                'top_5_accuracy': top_5_accuracy,
                'model': model
            }
            
            print(f"Accuracy: {accuracy:.3f}")
            print(f"Top-3 Accuracy: {top_3_accuracy:.3f}")
            print(f"Top-5 Accuracy: {top_5_accuracy:.3f}")
        
        return evaluation_results
    
    def _calculate_top_k_accuracy(self, y_true: np.ndarray, y_proba: np.ndarray, k: int) -> float:
        """Calculate top-k accuracy"""
        top_k_predictions = np.argsort(y_proba, axis=1)[:, -k:]
        correct = 0
        
        for i, true_label in enumerate(y_true):
            if true_label in top_k_predictions[i]:
                correct += 1
        
        return correct / len(y_true)
    
    def select_best_model(self, evaluation_results: Dict[str, Dict]) -> str:
        """Select the best performing model"""
        best_model = None
        best_score = 0
        
        # Weight different metrics
        for model_name, results in evaluation_results.items():
            # Composite score: 50% accuracy + 30% top-3 + 20% top-5
            composite_score = (
                0.5 * results['accuracy'] +
                0.3 * results['top_3_accuracy'] +
                0.2 * results['top_5_accuracy']
            )
            
            print(f"{model_name} composite score: {composite_score:.3f}")
            
            if composite_score > best_score:
                best_score = composite_score
                best_model = model_name
        
        print(f"\nBest model: {best_model} (score: {best_score:.3f})")
        return best_model
    
    def save_models(self) -> None:
        """Save all trained models and metadata"""
        print("Saving models...")
        
        # Save each model
        for model_name, model in self.trained_models.items():
            model_path = os.path.join(self.model_folder, f"{model_name}_model.pkl")
            joblib.dump(model, model_path)
            print(f"Saved {model_name} to {model_path}")
        
        # Save scaler
        scaler_path = os.path.join(self.model_folder, "scaler.pkl")
        joblib.dump(self.scaler, scaler_path)
        
        # Save metadata
        metadata = {
            'champion_names': self.champion_names,
            'model_scores': self.model_scores,
            'feature_names': self.feature_processor.get_feature_names(),
            'num_champions': len(self.champion_names),
            'num_features': len(self.feature_processor.get_feature_names())
        }
        
        metadata_path = os.path.join(self.model_folder, "metadata.pkl")
        with open(metadata_path, 'wb') as f:
            pickle.dump(metadata, f)
        
        print(f"Saved metadata to {metadata_path}")
    
    def load_models(self) -> bool:
        """Load trained models and metadata"""
        try:
            # Load metadata
            metadata_path = os.path.join(self.model_folder, "metadata.pkl")
            with open(metadata_path, 'rb') as f:
                metadata = pickle.load(f)
            
            self.champion_names = metadata['champion_names']
            self.model_scores = metadata['model_scores']
            
            # Load scaler
            scaler_path = os.path.join(self.model_folder, "scaler.pkl")
            self.scaler = joblib.load(scaler_path)
            
            # Load models
            for model_name in self.models.keys():
                model_path = os.path.join(self.model_folder, f"{model_name}_model.pkl")
                if os.path.exists(model_path):
                    self.trained_models[model_name] = joblib.load(model_path)
            
            print(f"Loaded {len(self.trained_models)} models")
            return True
            
        except Exception as e:
            print(f"Error loading models: {e}")
            return False
    
    def train_and_save_all(self) -> str:
        """Complete training pipeline"""
        print("Starting complete training pipeline...")
        
        # Prepare data
        X, y, champion_names = self.prepare_training_data()
        
        # Train models
        scores = self.train_models(X, y)
        
        # Hyperparameter tuning for best models
        self.hyperparameter_tuning(X, y)
        
        # Evaluate models
        evaluation_results = self.evaluate_models(X, y)
        
        # Select best model
        best_model = self.select_best_model(evaluation_results)
        
        # Save everything
        self.save_models()
        
        print(f"\nTraining complete! Best model: {best_model}")
        return best_model

def main():
    """Train models if run directly"""
    trainer = ModelTrainer()
    best_model = trainer.train_and_save_all()
    print(f"Training completed. Best model: {best_model}")

if __name__ == "__main__":
    main()