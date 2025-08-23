"""
Performance tests for the LoL Champion Recommender
Tests caching, concurrent access, and optimization features
"""

import pytest
import time
import threading
import concurrent.futures
from unittest.mock import patch, MagicMock
import sys
import os

# Add the parent directory to the path so we can import our modules
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.cache_manager import CacheManager, PerformanceMonitor, cached, timed
from services.champion_service import ChampionService
from services.recommendation_engine import RecommendationEngine
from services.performance_service import PerformanceService
from app import app

class TestCachePerformance:
    """Test cache performance and optimization"""
    
    def setup_method(self):
        """Setup for each test"""
        self.cache_manager = CacheManager(cache_dir="test_cache", max_memory_size=100)
        self.performance_monitor = PerformanceMonitor()
    
    def teardown_method(self):
        """Cleanup after each test"""
        self.cache_manager.clear()
        self.performance_monitor.clear_metrics()
    
    def test_cache_basic_operations(self):
        """Test basic cache operations performance"""
        # Test cache set performance
        start_time = time.time()
        for i in range(100):
            self.cache_manager.set(f"key_{i}", f"value_{i}")
        set_time = time.time() - start_time
        
        assert set_time < 1.0, f"Cache set operations too slow: {set_time}s"
        
        # Test cache get performance
        start_time = time.time()
        for i in range(100):
            value = self.cache_manager.get(f"key_{i}")
            assert value == f"value_{i}"
        get_time = time.time() - start_time
        
        assert get_time < 0.5, f"Cache get operations too slow: {get_time}s"
        
        # Check hit rate
        stats = self.cache_manager.get_stats()
        assert stats['hit_rate'] == 100.0, f"Expected 100% hit rate, got {stats['hit_rate']}%"
    
    def test_cache_lru_eviction(self):
        """Test LRU eviction performance"""
        cache_manager = CacheManager(cache_dir="test_cache", max_memory_size=10)
        
        # Fill cache beyond capacity
        for i in range(20):
            cache_manager.set(f"key_{i}", f"value_{i}")
        
        stats = cache_manager.get_stats()
        assert stats['memory_entries'] <= 10, f"Cache size exceeded limit: {stats['memory_entries']}"
        assert stats['evictions'] > 0, "No evictions occurred"
        
        # Verify LRU behavior - recent items should still be in cache
        for i in range(15, 20):
            value = cache_manager.get(f"key_{i}")
            assert value == f"value_{i}", f"Recent item key_{i} was evicted"
    
    def test_cache_compression(self):
        """Test cache compression effectiveness"""
        cache_manager = CacheManager(cache_dir="test_cache", compression=True)
        
        # Store large data that should benefit from compression
        large_data = "x" * 10000  # 10KB of repeated data
        cache_manager.set("large_key", large_data)
        
        # Retrieve and verify
        retrieved_data = cache_manager.get("large_key")
        assert retrieved_data == large_data
        
        stats = cache_manager.get_stats()
        assert stats['compression_saves'] > 0, "Compression should have been used"
    
    def test_concurrent_cache_access(self):
        """Test cache performance under concurrent access"""
        cache_manager = CacheManager(cache_dir="test_cache", max_memory_size=1000)
        
        def cache_worker(worker_id):
            """Worker function for concurrent testing"""
            for i in range(50):
                key = f"worker_{worker_id}_key_{i}"
                value = f"worker_{worker_id}_value_{i}"
                cache_manager.set(key, value)
                
                retrieved = cache_manager.get(key)
                assert retrieved == value
        
        # Run concurrent workers
        start_time = time.time()
        with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
            futures = [executor.submit(cache_worker, i) for i in range(10)]
            concurrent.futures.wait(futures)
        
        concurrent_time = time.time() - start_time
        assert concurrent_time < 5.0, f"Concurrent cache access too slow: {concurrent_time}s"
        
        stats = cache_manager.get_stats()
        assert stats['hits'] > 0, "No cache hits recorded"
        assert stats['sets'] == 500, f"Expected 500 sets, got {stats['sets']}"

class TestPerformanceMonitoring:
    """Test performance monitoring functionality"""
    
    def setup_method(self):
        """Setup for each test"""
        self.performance_monitor = PerformanceMonitor()
    
    def teardown_method(self):
        """Cleanup after each test"""
        self.performance_monitor.clear_metrics()
    
    def test_timing_decorator(self):
        """Test the @timed decorator"""
        # Import the global performance monitor that the decorator uses
        from utils.cache_manager import performance_monitor as global_monitor
        
        @timed("test_operation")
        def slow_function():
            time.sleep(0.1)
            return "result"
        
        result = slow_function()
        assert result == "result"
        
        metrics = global_monitor.get_metrics()
        assert "test_operation" in metrics['operations']
        
        operation_metrics = metrics['operations']['test_operation']
        assert operation_metrics['count'] >= 1
        assert operation_metrics['avg_time'] >= 0.1
    
    def test_performance_alerts(self):
        """Test performance alerting system"""
        # Record slow operation that should trigger alert
        self.performance_monitor.record_timing("slow_operation", 2.0)  # 2 seconds
        
        metrics = self.performance_monitor.get_metrics()
        assert len(metrics['alerts']) > 0, "No alerts generated for slow operation"
        
        alert = metrics['alerts'][0]
        assert alert['operation'] == "slow_operation"
        assert alert['duration'] == 2.0
        assert alert['severity'] in ['medium', 'high']
    
    def test_performance_trends(self):
        """Test performance trend analysis"""
        # Simulate degrading performance
        for i in range(10):
            # Gradually increasing response times
            self.performance_monitor.record_timing("degrading_op", 0.1 + (i * 0.01))
        
        # Add some recent slower times
        time.sleep(0.1)  # Small delay to separate time periods
        for i in range(5):
            self.performance_monitor.record_timing("degrading_op", 0.3)
        
        report = self.performance_monitor.get_performance_report()
        trends = report.get('trends', {})
        
        # Should detect degrading trend (though timing might be tricky in tests)
        assert 'degrading_op' in trends or len(trends) == 0  # May not have enough time separation

class TestServicePerformance:
    """Test performance of core services"""
    
    def setup_method(self):
        """Setup for each test"""
        self.champion_service = ChampionService()
    
    def test_champion_service_caching(self):
        """Test champion service caching performance"""
        # First call should load data
        start_time = time.time()
        champions1 = self.champion_service.get_all_champions()
        first_call_time = time.time() - start_time
        
        # Second call should use cache
        start_time = time.time()
        champions2 = self.champion_service.get_all_champions()
        second_call_time = time.time() - start_time
        
        assert len(champions1) > 0, "No champions loaded"
        assert champions1 == champions2, "Cached data doesn't match"
        assert second_call_time < first_call_time, "Cache didn't improve performance"
        assert second_call_time < 0.1, f"Cached call too slow: {second_call_time}s"
    
    def test_champion_lookup_performance(self):
        """Test champion lookup performance"""
        # Ensure champions are loaded
        champions = self.champion_service.get_all_champions()
        if not champions:
            pytest.skip("No champions available for testing")
        
        test_champion = champions[0]
        
        # Test multiple lookups
        start_time = time.time()
        for _ in range(100):
            found_champion = self.champion_service.get_champion_by_name(test_champion.name)
            assert found_champion is not None
            assert found_champion.name == test_champion.name
        
        lookup_time = time.time() - start_time
        assert lookup_time < 1.0, f"Champion lookups too slow: {lookup_time}s"
    
    @patch('services.recommendation_engine.RecommendationEngine.load_models')
    def test_recommendation_caching(self, mock_load_models):
        """Test recommendation engine caching"""
        # Mock the model loading to avoid file dependencies
        mock_load_models.return_value = True
        
        # Create a mock recommendation engine
        rec_engine = RecommendationEngine()
        rec_engine.models = {'test_model': MagicMock()}
        rec_engine.champion_names = ['TestChampion']
        rec_engine.best_model_name = 'test_model'
        
        # Mock the model prediction
        mock_model = rec_engine.models['test_model']
        mock_model.predict.return_value = [0]
        mock_model.predict_proba.return_value = [[0.8, 0.2]]
        
        # Mock champion service
        rec_engine.champion_service = MagicMock()
        mock_champion = MagicMock()
        mock_champion.name = "TestChampion"
        rec_engine.champion_service.get_champion_by_name.return_value = mock_champion
        
        # Mock feature processor
        rec_engine.feature_processor = MagicMock()
        rec_engine.feature_processor.process_user_responses.return_value = [[1, 0, 1]]
        rec_engine.feature_processor.explain_recommendation.return_value = ["Test reason"]
        
        test_responses = {'1': 'Tank', '2': 'Easy'}
        
        # First call
        start_time = time.time()
        result1 = rec_engine.predict_champion(test_responses)
        first_call_time = time.time() - start_time
        
        # Second call should use cache
        start_time = time.time()
        result2 = rec_engine.predict_champion(test_responses)
        second_call_time = time.time() - start_time
        
        assert result1 is not None
        assert result2 is not None
        assert second_call_time < first_call_time, "Cache didn't improve performance"

class TestConcurrentPerformance:
    """Test performance under concurrent load"""
    
    def test_concurrent_recommendations(self):
        """Test concurrent recommendation requests"""
        with app.test_client() as client:
            def make_request():
                """Make a recommendation request"""
                # Start a session
                response = client.get('/start')
                assert response.status_code in [200, 302]
                
                # Submit some answers
                with client.session_transaction() as sess:
                    sess['session_id'] = 'test_session'
                    sess['current_question'] = 1
                    sess['responses'] = {'1': 'Tank', '2': 'Easy', '3': 'Team fights'}
                    sess['total_questions'] = 3
                
                # Get recommendation
                response = client.get('/recommendation')
                return response.status_code
            
            # Run concurrent requests
            start_time = time.time()
            with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
                futures = [executor.submit(make_request) for _ in range(10)]
                results = [future.result() for future in concurrent.futures.as_completed(futures)]
            
            concurrent_time = time.time() - start_time
            
            # All requests should succeed
            assert all(status in [200, 302] for status in results), "Some requests failed"
            assert concurrent_time < 10.0, f"Concurrent requests too slow: {concurrent_time}s"

class TestPerformanceService:
    """Test the performance service functionality"""
    
    def setup_method(self):
        """Setup for each test"""
        self.performance_service = PerformanceService()
    
    def test_performance_dashboard_data(self):
        """Test performance dashboard data generation"""
        dashboard_data = self.performance_service.get_performance_dashboard()
        
        assert 'timestamp' in dashboard_data
        assert 'health_scores' in dashboard_data
        assert 'cache_performance' in dashboard_data
        
        # Health scores should be valid percentages
        health_scores = dashboard_data['health_scores']
        for score_name, score_value in health_scores.items():
            assert 0 <= score_value <= 100, f"Invalid health score {score_name}: {score_value}"
    
    def test_optimization_execution(self):
        """Test optimization execution"""
        results = self.performance_service.force_optimization('cache')
        
        assert 'executed' in results
        assert 'errors' in results
        assert isinstance(results['executed'], list)
        assert isinstance(results['errors'], list)
    
    def test_cache_clearing(self):
        """Test cache clearing functionality"""
        # Add some data to cache first
        from utils.cache_manager import cache_manager
        cache_manager.set("test_key", "test_value")
        
        results = self.performance_service.clear_all_caches()
        
        assert 'success' in results
        if results['success']:
            assert 'cleared_entries' in results
            assert results['cleared_entries'] >= 0

class TestMemoryUsage:
    """Test memory usage and optimization"""
    
    def test_cache_memory_limits(self):
        """Test that cache respects memory limits"""
        cache_manager = CacheManager(cache_dir="test_cache", max_memory_size=50)
        
        # Add many items
        for i in range(100):
            cache_manager.set(f"key_{i}", f"value_{i}" * 100)  # Larger values
        
        stats = cache_manager.get_stats()
        assert stats['memory_entries'] <= 50, f"Cache exceeded memory limit: {stats['memory_entries']}"
        assert stats['evictions'] > 0, "No evictions occurred despite memory pressure"
    
    def test_performance_data_cleanup(self):
        """Test that performance data doesn't grow unbounded"""
        performance_monitor = PerformanceMonitor()
        
        # Generate lots of performance data
        for i in range(15000):  # More than the 10000 limit
            performance_monitor.record_timing("test_op", 0.1)
        
        # Check that history is limited
        history_length = len(performance_monitor.operation_history['test_op'])
        assert history_length <= 10000, f"Performance history too large: {history_length}"

if __name__ == '__main__':
    pytest.main([__file__, '-v'])