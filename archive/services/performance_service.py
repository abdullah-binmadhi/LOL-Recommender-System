"""
Performance optimization service for the LoL Champion Recommender
"""

import time
import threading
from typing import Dict, List, Any, Optional
from datetime import datetime, timedelta
from utils.cache_manager import cache_manager, performance_monitor, cache_preloader
from services.champion_service import ChampionService
from services.question_service import QuestionService
from services.recommendation_engine import RecommendationEngine
import logging

class PerformanceService:
    """Service for monitoring and optimizing application performance"""
    
    def __init__(self):
        self.champion_service = ChampionService()
        self.question_service = QuestionService()
        self.recommendation_engine = None  # Lazy load to avoid circular imports
        self.optimization_tasks = []
        self.monitoring_active = False
        self._lock = threading.RLock()
        
        # Setup logging
        self.logger = logging.getLogger(__name__)
        
        # Performance targets
        self.performance_targets = {
            'cache_hit_rate': 80.0,  # Target 80% cache hit rate
            'avg_response_time': 0.5,  # Target 500ms average response
            'ml_prediction_time': 0.3,  # Target 300ms for ML predictions
            'data_loading_time': 0.2   # Target 200ms for data loading
        }
    
    def start_monitoring(self):
        """Start performance monitoring"""
        with self._lock:
            if not self.monitoring_active:
                self.monitoring_active = True
                self._start_monitoring_thread()
                self.logger.info("Performance monitoring started")
    
    def stop_monitoring(self):
        """Stop performance monitoring"""
        with self._lock:
            self.monitoring_active = False
            self.logger.info("Performance monitoring stopped")
    
    def _start_monitoring_thread(self):
        """Start background monitoring thread"""
        def monitor_worker():
            while self.monitoring_active:
                try:
                    self._perform_health_check()
                    self._optimize_if_needed()
                    time.sleep(60)  # Check every minute
                except Exception as e:
                    self.logger.error(f"Monitoring error: {e}")
                    time.sleep(60)
        
        monitor_thread = threading.Thread(target=monitor_worker, daemon=True)
        monitor_thread.start()
    
    def _perform_health_check(self):
        """Perform system health check"""
        try:
            # Check cache performance
            cache_stats = cache_manager.get_stats()
            
            # Check if cache hit rate is below target
            if cache_stats['hit_rate'] < self.performance_targets['cache_hit_rate']:
                self._schedule_cache_optimization()
            
            # Check performance metrics
            perf_metrics = performance_monitor.get_metrics()
            avg_response = perf_metrics.get('avg_response_time', 0)
            
            if avg_response > self.performance_targets['avg_response_time']:
                self._schedule_performance_optimization()
            
        except Exception as e:
            self.logger.error(f"Health check error: {e}")
    
    def _schedule_cache_optimization(self):
        """Schedule cache optimization tasks"""
        if 'cache_optimization' not in [task['type'] for task in self.optimization_tasks]:
            self.optimization_tasks.append({
                'type': 'cache_optimization',
                'scheduled_at': datetime.now(),
                'priority': 'medium'
            })
    
    def _schedule_performance_optimization(self):
        """Schedule performance optimization tasks"""
        if 'performance_optimization' not in [task['type'] for task in self.optimization_tasks]:
            self.optimization_tasks.append({
                'type': 'performance_optimization',
                'scheduled_at': datetime.now(),
                'priority': 'high'
            })
    
    def _optimize_if_needed(self):
        """Execute pending optimization tasks"""
        if not self.optimization_tasks:
            return
        
        # Sort by priority and age
        priority_order = {'high': 3, 'medium': 2, 'low': 1}
        self.optimization_tasks.sort(
            key=lambda x: (priority_order.get(x['priority'], 1), x['scheduled_at'])
        )
        
        # Execute highest priority task
        task = self.optimization_tasks.pop(0)
        
        try:
            if task['type'] == 'cache_optimization':
                self._execute_cache_optimization()
            elif task['type'] == 'performance_optimization':
                self._execute_performance_optimization()
            
            self.logger.info(f"Executed optimization task: {task['type']}")
            
        except Exception as e:
            self.logger.error(f"Optimization task failed: {e}")
    
    def _execute_cache_optimization(self):
        """Execute cache optimization strategies"""
        # Preload frequently accessed data
        self._preload_common_data()
        
        # Clean up expired entries
        cache_manager._cleanup_expired()
        
        # Adjust cache settings if needed
        cache_stats = cache_manager.get_stats()
        if cache_stats.get('evictions', 0) > cache_stats.get('sets', 0) * 0.3:
            # Too many evictions, suggest increasing cache size
            self.logger.warning("High cache eviction rate detected")
    
    def _execute_performance_optimization(self):
        """Execute performance optimization strategies"""
        # Clear old performance data
        performance_monitor.clear_metrics()
        
        # Warm up critical paths
        self._warm_up_critical_paths()
    
    def _preload_common_data(self):
        """Preload commonly accessed data into cache"""
        try:
            # Preload all champions
            self.champion_service.get_all_champions()
            
            # Preload questions
            self.question_service.get_all_questions()
            
            # Preload common champion lookups
            champions = self.champion_service.get_all_champions()
            for champion in champions[:20]:  # Top 20 champions
                self.champion_service.get_champion_by_name(champion.name)
            
            self.logger.info("Data preloading completed")
            
        except Exception as e:
            self.logger.error(f"Data preloading failed: {e}")
    
    def _warm_up_critical_paths(self):
        """Warm up critical application paths"""
        try:
            # Warm up ML models if available
            if self.recommendation_engine is None:
                from services.recommendation_engine import RecommendationEngine
                self.recommendation_engine = RecommendationEngine()
            
            # Test prediction with sample data
            sample_responses = {
                '1': 'Tank',
                '2': 'Easy',
                '3': 'Team fights'
            }
            
            self.recommendation_engine.predict_champion(sample_responses)
            self.logger.info("Critical paths warmed up")
            
        except Exception as e:
            self.logger.error(f"Critical path warm-up failed: {e}")
    
    def get_performance_dashboard(self) -> Dict[str, Any]:
        """Get comprehensive performance dashboard data"""
        try:
            cache_stats = cache_manager.get_stats()
            perf_report = performance_monitor.get_performance_report()
            cache_report = cache_manager.get_performance_report()
            
            # Calculate health scores
            health_scores = self._calculate_health_scores(cache_stats, perf_report)
            
            return {
                'timestamp': datetime.now().isoformat(),
                'health_scores': health_scores,
                'cache_performance': cache_stats,
                'operation_performance': perf_report,
                'cache_analysis': cache_report,
                'optimization_tasks': len(self.optimization_tasks),
                'monitoring_active': self.monitoring_active,
                'recommendations': self._get_optimization_recommendations(cache_stats, perf_report)
            }
            
        except Exception as e:
            self.logger.error(f"Dashboard generation failed: {e}")
            return {'error': str(e)}
    
    def _calculate_health_scores(self, cache_stats: Dict, perf_report: Dict) -> Dict[str, float]:
        """Calculate health scores for different system components"""
        scores = {}
        
        # Cache health score
        hit_rate = cache_stats.get('hit_rate', 0)
        cache_score = min(100, (hit_rate / self.performance_targets['cache_hit_rate']) * 100)
        scores['cache'] = round(cache_score, 1)
        
        # Performance health score
        avg_time = perf_report.get('summary', {}).get('avg_response_time', 0)
        if avg_time > 0:
            perf_score = min(100, (self.performance_targets['avg_response_time'] / avg_time) * 100)
        else:
            perf_score = 100
        scores['performance'] = round(perf_score, 1)
        
        # Overall health score
        scores['overall'] = round((scores['cache'] + scores['performance']) / 2, 1)
        
        return scores
    
    def _get_optimization_recommendations(self, cache_stats: Dict, perf_report: Dict) -> List[str]:
        """Get optimization recommendations based on current performance"""
        recommendations = []
        
        # Cache recommendations
        if cache_stats.get('hit_rate', 0) < 70:
            recommendations.append("Increase cache TTL or preload more data to improve hit rate")
        
        if cache_stats.get('evictions', 0) > cache_stats.get('sets', 0) * 0.2:
            recommendations.append("Consider increasing cache memory size to reduce evictions")
        
        # Performance recommendations
        slow_ops = perf_report.get('slow_operations', [])
        if slow_ops:
            recommendations.append(f"Optimize slow operations: {', '.join([op['operation'] for op in slow_ops[:3]])}")
        
        # Alert recommendations
        alerts = perf_report.get('summary', {}).get('alerts', [])
        if len(alerts) > 5:
            recommendations.append("High number of performance alerts - investigate bottlenecks")
        
        return recommendations
    
    def force_optimization(self, optimization_type: str = 'all') -> Dict[str, Any]:
        """Force immediate optimization"""
        results = {'executed': [], 'errors': []}
        
        try:
            if optimization_type in ['all', 'cache']:
                self._execute_cache_optimization()
                results['executed'].append('cache_optimization')
            
            if optimization_type in ['all', 'performance']:
                self._execute_performance_optimization()
                results['executed'].append('performance_optimization')
            
            if optimization_type in ['all', 'preload']:
                self._preload_common_data()
                results['executed'].append('data_preload')
            
            return results
            
        except Exception as e:
            results['errors'].append(str(e))
            return results
    
    def clear_all_caches(self) -> Dict[str, Any]:
        """Clear all caches and reset performance metrics"""
        try:
            cleared_entries = cache_manager.clear()
            performance_monitor.clear_metrics()
            
            return {
                'success': True,
                'cleared_entries': cleared_entries,
                'message': 'All caches cleared and metrics reset'
            }
            
        except Exception as e:
            return {
                'success': False,
                'error': str(e)
            }
    
    def get_cache_analysis(self) -> Dict[str, Any]:
        """Get detailed cache analysis"""
        try:
            return cache_manager.get_performance_report()
        except Exception as e:
            return {'error': str(e)}

# Global performance service instance
performance_service = PerformanceService()