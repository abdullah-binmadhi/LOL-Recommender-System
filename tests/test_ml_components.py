import unittest
import numpy as np
import tempfile
import shutil
import os
from ml.feature_processor import FeatureProcessor
from ml.model_trainer import ModelTrainer
from services.recommendation_engine import RecommendationEngine
from services.champion_service import ChampionService
from services.question_service import QuestionService

class TestFeatureProcessor(unittest.TestCase):
    
    def setUp(self):
        self.processor = FeatureProcessor()
    
    def test_process_user_responses(self):
        """Test processing user responses into features"""
        responses = {
            '1': 'Medium (4-6)',
            '2': 'Fighter',
            '3': 'Top',
            '4': 'High Damage Output',
            '5': 'Melee'
        }
        
        features = self.processor.process_user_responses(responses)
        
        # Should return numpy array
        self.assertIsInstance(features, np.ndarray)
        self.assertEqual(len(features.shape), 2)  # Should be 2D
        self.assertGreater(features.shape[1], 0)  # Should have features
    
    def test_encode_champion_features(self):
        """Test encoding champion features"""
        champion_service = ChampionService()
        champions = champion_service.get_all_champions()
        
        if champions:
            champion = champions[0]
            features = self.processor.encode_champion_features(champion)
            
            self.assertIsInstance(features, np.ndarray)
            self.assertGreater(len(features), 0)
    
    def test_create_training_data(self):
        """Test creating training data"""
        X, y, champion_names = self.processor.create_training_data()
        
        self.assertIsInstance(X, np.ndarray)
        self.assertIsInstance(y, np.ndarray)
        self.assertIsInstance(champion_names, list)
        
        self.assertEqual(len(X), len(y))
        self.assertGreater(len(champion_names), 0)
        self.assertGreater(X.shape[1], 0)  # Should have features
    
    def test_explain_recommendation(self):
        """Test recommendation explanation generation"""
        champion_service = ChampionService()
        champions = champion_service.get_all_champions()
        
        if champions:
            responses = {
                '1': 'Medium (4-6)',
                '2': champions[0].role,  # Match champion's role
                '3': 'Top',
                '4': 'High Damage Output',
                '5': 'Melee'
            }
            
            explanations = self.processor.explain_recommendation(responses, champions[0])
            
            self.assertIsInstance(explanations, list)
            self.assertGreater(len(explanations), 0)

class TestModelTrainer(unittest.TestCase):
    
    def setUp(self):
        self.temp_dir = tempfile.mkdtemp()
        self.trainer = ModelTrainer(self.temp_dir)
    
    def tearDown(self):
        shutil.rmtree(self.temp_dir)
    
    def test_prepare_training_data(self):
        """Test training data preparation"""
        X, y, champion_names = self.trainer.prepare_training_data()
        
        self.assertIsInstance(X, np.ndarray)
        self.assertIsInstance(y, np.ndarray)
        self.assertIsInstance(champion_names, list)
        
        self.assertEqual(len(X), len(y))
        self.assertGreater(len(champion_names), 0)
        self.assertGreater(X.shape[1], 0)
    
    def test_train_models(self):
        """Test model training"""
        X, y, champion_names = self.trainer.prepare_training_data()
        
        # Use smaller dataset for testing
        if len(X) > 100:
            X = X[:100]
            y = y[:100]
        
        scores = self.trainer.train_models(X, y)
        
        self.assertIsInstance(scores, dict)
        self.assertGreater(len(scores), 0)
        
        # Check that all models have reasonable scores
        for model_name, score in scores.items():
            self.assertGreaterEqual(score, 0.0)
            self.assertLessEqual(score, 1.0)
    
    def test_save_and_load_models(self):
        """Test saving and loading models"""
        X, y, champion_names = self.trainer.prepare_training_data()
        
        # Use smaller dataset for testing
        if len(X) > 50:
            X = X[:50]
            y = y[:50]
        
        # Train models
        self.trainer.train_models(X, y)
        
        # Save models
        self.trainer.save_models()
        
        # Check that files were created
        expected_files = [
            'random_forest_model.pkl',
            'decision_tree_model.pkl', 
            'knn_model.pkl',
            'scaler.pkl',
            'metadata.pkl'
        ]
        
        for filename in expected_files:
            filepath = os.path.join(self.temp_dir, filename)
            self.assertTrue(os.path.exists(filepath), f"File {filename} not found")
        
        # Test loading
        new_trainer = ModelTrainer(self.temp_dir)
        success = new_trainer.load_models()
        self.assertTrue(success)
        self.assertGreater(len(new_trainer.trained_models), 0)

class TestRecommendationEngine(unittest.TestCase):
    
    def setUp(self):
        self.temp_dir = tempfile.mkdtemp()
        # Create a simple trained model for testing
        self._create_test_models()
        self.engine = RecommendationEngine(self.temp_dir)
    
    def tearDown(self):
        shutil.rmtree(self.temp_dir)
    
    def _create_test_models(self):
        """Create simple test models"""
        trainer = ModelTrainer(self.temp_dir)
        X, y, champion_names = trainer.prepare_training_data()
        
        # Use smaller dataset for testing
        if len(X) > 100:
            X = X[:100]
            y = y[:100]
        
        trainer.train_models(X, y)
        trainer.save_models()
    
    def test_predict_champion(self):
        """Test champion prediction"""
        responses = {
            '1': 'Medium (4-6)',
            '2': 'Fighter',
            '3': 'Top',
            '4': 'High Damage Output',
            '5': 'Melee'
        }
        
        recommendation = self.engine.predict_champion(responses)
        
        self.assertIsNotNone(recommendation)
        self.assertIsNotNone(recommendation.champion)
        self.assertGreaterEqual(recommendation.confidence_score, 0.0)
        self.assertLessEqual(recommendation.confidence_score, 1.0)
        self.assertIsInstance(recommendation.explanation, str)
        self.assertIsInstance(recommendation.match_reasons, list)
    
    def test_get_alternative_recommendations(self):
        """Test getting alternative recommendations"""
        responses = {
            '1': 'Medium (4-6)',
            '2': 'Fighter',
            '3': 'Top',
            '4': 'High Damage Output',
            '5': 'Melee'
        }
        
        alternatives = self.engine.get_alternative_recommendations(responses, num_alternatives=3)
        
        self.assertIsInstance(alternatives, list)
        self.assertLessEqual(len(alternatives), 3)
        
        for alt in alternatives:
            self.assertIsNotNone(alt.champion)
            self.assertGreaterEqual(alt.confidence_score, 0.0)
            self.assertLessEqual(alt.confidence_score, 1.0)
    
    def test_model_info(self):
        """Test getting model information"""
        info = self.engine.get_model_info()
        
        self.assertIsInstance(info, dict)
        self.assertIn('available_models', info)
        self.assertIn('best_model', info)
        self.assertIn('num_champions', info)
        self.assertIn('models_loaded', info)
        
        self.assertTrue(info['models_loaded'])
        self.assertGreater(len(info['available_models']), 0)
    
    def test_predict_with_all_models(self):
        """Test predictions from all models"""
        responses = {
            '1': 'Medium (4-6)',
            '2': 'Fighter',
            '3': 'Top',
            '4': 'High Damage Output',
            '5': 'Melee'
        }
        
        predictions = self.engine.predict_with_all_models(responses)
        
        self.assertIsInstance(predictions, dict)
        self.assertGreater(len(predictions), 0)
        
        for model_name, prediction in predictions.items():
            self.assertIsNotNone(prediction.champion)
            self.assertIsInstance(prediction.explanation, str)

if __name__ == '__main__':
    # Run tests with reduced verbosity for ML components
    unittest.main(verbosity=1)