"""
Pytest configuration and shared fixtures for the LoL Champion Recommender test suite
"""

import pytest
import sys
import os
import tempfile
import shutil
from unittest.mock import Mock, patch, MagicMock

# Add the parent directory to the path so we can import our modules
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from app import app
from services.champion_service import ChampionService
from services.question_service import QuestionService
from services.recommendation_engine import RecommendationEngine

# Import test fixtures
from tests.fixtures import (
    get_test_champions, get_test_questions, get_test_session,
    get_response_pattern, get_expected_recommendation,
    SAMPLE_CHAMPIONS_DATA, SAMPLE_QUESTIONS_DATA, TEST_SESSIONS
)

@pytest.fixture(scope="session")
def test_app():
    """Create a test Flask application"""
    app.config['TESTING'] = True
    app.config['WTF_CSRF_ENABLED'] = False
    app.config['SECRET_KEY'] = 'test-secret-key'
    app.config['CACHE_TYPE'] = 'simple'
    app.config['CACHE_DEFAULT_TTL'] = 300
    
    with app.app_context():
        yield app

@pytest.fixture(scope="function")
def client(test_app):
    """Create a test client"""
    return test_app.test_client()

@pytest.fixture(scope="function")
def app_context(test_app):
    """Create an application context"""
    with test_app.app_context():
        yield test_app

@pytest.fixture(scope="session")
def temp_dir():
    """Create a temporary directory for test files"""
    temp_dir = tempfile.mkdtemp()
    yield temp_dir
    shutil.rmtree(temp_dir)

@pytest.fixture(scope="function")
def mock_champion_service():
    """Mock ChampionService with test data"""
    service = Mock(spec=ChampionService)
    
    # Mock methods with test data
    service.get_all_champions.return_value = get_test_champions()
    service.get_champion_by_name.side_effect = lambda name: next(
        (c for c in get_test_champions() if c.name.lower() == name.lower()), None
    )
    service.get_champions_by_role.side_effect = lambda role: [
        c for c in get_test_champions() if c.role.lower() == role.lower()
    ]
    service.get_champions_by_difficulty.side_effect = lambda min_d, max_d: [
        c for c in get_test_champions() if min_d <= c.difficulty <= max_d
    ]
    
    return service

@pytest.fixture(scope="function")
def mock_question_service():
    """Mock QuestionService with test data"""
    service = Mock(spec=QuestionService)
    
    # Mock methods with test data
    service.get_all_questions.return_value = get_test_questions()
    service.get_question_by_id.side_effect = lambda qid: next(
        (q for q in get_test_questions() if q.id == qid), None
    )
    service.get_total_questions.return_value = len(get_test_questions())
    service.validate_answer.return_value = True
    
    return service

@pytest.fixture(scope="function")
def mock_recommendation_engine():
    """Mock RecommendationEngine with test data"""
    engine = Mock(spec=RecommendationEngine)
    
    def mock_predict(responses, model_name=None):
        # Simple logic to return expected recommendations based on responses
        if responses.get('1') == 'Tank':
            if responses.get('2') == 'Easy':
                champion_name = 'Malphite'
            else:
                champion_name = 'Amumu'
        elif responses.get('1') == 'Marksman':
            champion_name = 'Jinx'
        elif responses.get('1') == 'Mage':
            champion_name = 'Lux'
        elif responses.get('1') == 'Support':
            champion_name = 'Thresh'
        else:
            champion_name = 'Malphite'  # Default
        
        # Find the champion
        champion = next(
            (c for c in get_test_champions() if c.name == champion_name), 
            get_test_champions()[0]
        )
        
        # Create mock recommendation
        recommendation = Mock()
        recommendation.champion = champion
        recommendation.confidence_score = 0.85
        recommendation.explanation = f"Test recommendation for {champion.name}"
        recommendation.match_reasons = ["Test reason 1", "Test reason 2"]
        
        return recommendation
    
    engine.predict_champion.side_effect = mock_predict
    engine.get_alternative_recommendations.return_value = [
        mock_predict({'1': 'Tank', '2': 'Easy'}),
        mock_predict({'1': 'Support', '2': 'Moderate'})
    ]
    engine.get_model_info.return_value = {
        'available_models': ['random_forest', 'knn'],
        'best_model': 'random_forest',
        'model_accuracy': 0.85
    }
    
    return engine

@pytest.fixture(scope="function")
def mock_services(mock_champion_service, mock_question_service, mock_recommendation_engine):
    """Mock all services together"""
    with patch('services.champion_service.ChampionService', return_value=mock_champion_service), \
         patch('services.question_service.QuestionService', return_value=mock_question_service), \
         patch('services.recommendation_engine.RecommendationEngine', return_value=mock_recommendation_engine):
        yield {
            'champion_service': mock_champion_service,
            'question_service': mock_question_service,
            'recommendation_engine': mock_recommendation_engine
        }

@pytest.fixture(scope="function")
def sample_session_data():
    """Provide sample session data for testing"""
    return {
        'session_id': 'test_session_123',
        'responses': {
            '1': 'Tank',
            '2': 'Easy',
            '3': 'Team fights',
            '4': 'Melee',
            '5': 'Medium'
        },
        'current_question': 6,
        'total_questions': 9,
        'started_at': '2024-01-15T10:30:00'
    }

@pytest.fixture(scope="function")
def complete_session_data():
    """Provide complete session data for testing"""
    return get_test_session('complete_session')

@pytest.fixture(scope="function")
def partial_session_data():
    """Provide partial session data for testing"""
    return get_test_session('partial_session')

@pytest.fixture(scope="function")
def mock_cache():
    """Mock cache for testing"""
    cache_data = {}
    
    cache = Mock()
    cache.get.side_effect = lambda key: cache_data.get(key)
    cache.set.side_effect = lambda key, value, timeout=None: cache_data.update({key: value})
    cache.delete.side_effect = lambda key: cache_data.pop(key, None)
    cache.clear.side_effect = lambda: cache_data.clear()
    
    return cache

@pytest.fixture(scope="function")
def mock_ml_models():
    """Mock ML models for testing"""
    models = {}
    
    # Mock random forest model
    rf_model = Mock()
    rf_model.predict.return_value = ['Malphite']
    rf_model.predict_proba.return_value = [[0.1, 0.8, 0.1]]
    models['random_forest'] = rf_model
    
    # Mock KNN model
    knn_model = Mock()
    knn_model.predict.return_value = ['Jinx']
    knn_model.predict_proba.return_value = [[0.2, 0.7, 0.1]]
    models['knn'] = knn_model
    
    return models

@pytest.fixture(scope="function")
def test_user_responses():
    """Provide various test user response patterns"""
    return {
        'tank_easy': get_response_pattern('tank_easy'),
        'marksman_advanced': get_response_pattern('marksman_advanced'),
        'mage_control': get_response_pattern('mage_control'),
        'support_utility': get_response_pattern('support_utility'),
        'partial': {
            '1': 'Tank',
            '2': 'Easy',
            '3': 'Team fights'
        },
        'invalid': {
            '1': 'InvalidRole',
            '2': 'InvalidDifficulty'
        }
    }

@pytest.fixture(scope="function")
def performance_test_data():
    """Provide data for performance testing"""
    return {
        'concurrent_users': 10,
        'requests_per_user': 5,
        'test_duration': 30,  # seconds
        'response_patterns': [
            get_response_pattern('tank_easy'),
            get_response_pattern('marksman_advanced'),
            get_response_pattern('mage_control'),
            get_response_pattern('support_utility')
        ]
    }

@pytest.fixture(scope="function")
def database_mock():
    """Mock database operations"""
    db_data = {
        'sessions': {},
        'recommendations': {},
        'user_history': {}
    }
    
    db = Mock()
    db.save_session.side_effect = lambda session: db_data['sessions'].update({session.session_id: session})
    db.get_session.side_effect = lambda session_id: db_data['sessions'].get(session_id)
    db.save_recommendation.side_effect = lambda rec: db_data['recommendations'].update({rec.id: rec})
    
    return db

@pytest.fixture(autouse=True)
def reset_caches():
    """Reset all caches before each test"""
    # Clear any global caches or state
    yield
    # Cleanup after test

@pytest.fixture(scope="function")
def error_scenarios():
    """Provide various error scenarios for testing"""
    return {
        'network_error': Exception("Network connection failed"),
        'timeout_error': TimeoutError("Request timed out"),
        'validation_error': ValueError("Invalid input data"),
        'ml_error': RuntimeError("ML model prediction failed"),
        'data_error': FileNotFoundError("Data file not found")
    }

# Pytest configuration
def pytest_configure(config):
    """Configure pytest with custom markers"""
    config.addinivalue_line(
        "markers", "slow: marks tests as slow (deselect with '-m \"not slow\"')"
    )
    config.addinivalue_line(
        "markers", "integration: marks tests as integration tests"
    )
    config.addinivalue_line(
        "markers", "e2e: marks tests as end-to-end tests"
    )
    config.addinivalue_line(
        "markers", "performance: marks tests as performance tests"
    )

def pytest_collection_modifyitems(config, items):
    """Modify test collection to add markers automatically"""
    for item in items:
        # Mark slow tests
        if "performance" in item.nodeid or "load" in item.nodeid:
            item.add_marker(pytest.mark.slow)
        
        # Mark integration tests
        if "integration" in item.nodeid:
            item.add_marker(pytest.mark.integration)
        
        # Mark end-to-end tests
        if "end_to_end" in item.nodeid or "e2e" in item.nodeid:
            item.add_marker(pytest.mark.e2e)

# Custom assertions
def assert_valid_champion(champion):
    """Assert that a champion object is valid"""
    assert champion is not None
    assert hasattr(champion, 'name')
    assert hasattr(champion, 'role')
    assert hasattr(champion, 'difficulty')
    assert isinstance(champion.difficulty, int)
    assert 1 <= champion.difficulty <= 10

def assert_valid_recommendation(recommendation):
    """Assert that a recommendation object is valid"""
    assert recommendation is not None
    assert hasattr(recommendation, 'champion')
    assert hasattr(recommendation, 'confidence_score')
    assert hasattr(recommendation, 'explanation')
    assert 0.0 <= recommendation.confidence_score <= 1.0
    assert_valid_champion(recommendation.champion)

def assert_valid_session(session):
    """Assert that a session object is valid"""
    assert session is not None
    assert hasattr(session, 'session_id')
    assert hasattr(session, 'responses')
    assert hasattr(session, 'current_question')
    assert isinstance(session.responses, dict)
    assert isinstance(session.current_question, int)

# Make custom assertions available to all tests
pytest.assert_valid_champion = assert_valid_champion
pytest.assert_valid_recommendation = assert_valid_recommendation
pytest.assert_valid_session = assert_valid_session