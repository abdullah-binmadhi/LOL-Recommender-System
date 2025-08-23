"""
Comprehensive caching system for the LoL Champion Recommender
Enhanced with performance monitoring and advanced caching strategies
"""

import json
import pickle
import hashlib
import time
import os
import gzip
import logging
from typing import Any, Optional, Dict, Callable, List, Tuple
from functools import wraps, lru_cache
from datetime import datetime, timedelta
import threading
from collections import defaultdict, OrderedDict

class CacheManager:
    """Thread-safe cache manager with multiple storage backends and performance optimization"""
    
    def __init__(self, cache_dir: str = "cache", default_ttl: int = 3600, 
                 max_memory_size: int = 1000, compression: bool = True):
        self.cache_dir = cache_dir
        self.default_ttl = default_ttl
        self.max_memory_size = max_memory_size
        self.compression = compression
        
        # LRU cache for memory management
        self.memory_cache = OrderedDict()
        self.cache_access_times = {}
        
        # Enhanced statistics
        self.cache_stats = {
            'hits': 0,
            'misses': 0,
            'sets': 0,
            'deletes': 0,
            'evictions': 0,
            'disk_reads': 0,
            'disk_writes': 0,
            'compression_saves': 0
        }
        
        # Performance tracking
        self.operation_times = defaultdict(list)
        self._lock = threading.RLock()
        
        # Ensure cache directory exists
        os.makedirs(cache_dir, exist_ok=True)
        
        # Start cleanup thread
        self._start_cleanup_thread()
        
        # Setup logging
        self.logger = logging.getLogger(__name__)
    
    def _evict_lru_if_needed(self):
        """Evict least recently used items if memory cache is full"""
        while len(self.memory_cache) >= self.max_memory_size:
            # Remove least recently used item
            lru_key = next(iter(self.memory_cache))
            del self.memory_cache[lru_key]
            if lru_key in self.cache_access_times:
                del self.cache_access_times[lru_key]
            self.cache_stats['evictions'] += 1
    
    def _update_access_time(self, key: str):
        """Update access time for LRU tracking"""
        self.cache_access_times[key] = time.time()
        # Move to end in OrderedDict (most recently used)
        if key in self.memory_cache:
            self.memory_cache.move_to_end(key)
    
    def _generate_key(self, key: str, namespace: str = "default") -> str:
        """Generate a cache key with namespace"""
        full_key = f"{namespace}:{key}"
        return hashlib.md5(full_key.encode()).hexdigest()
    
    def _is_expired(self, cache_entry: Dict) -> bool:
        """Check if cache entry is expired"""
        if 'expires_at' not in cache_entry:
            return False
        return datetime.now() > datetime.fromisoformat(cache_entry['expires_at'])
    
    def get(self, key: str, namespace: str = "default") -> Optional[Any]:
        """Get value from cache with performance tracking"""
        start_time = time.time()
        
        with self._lock:
            cache_key = self._generate_key(key, namespace)
            
            # Try memory cache first
            if cache_key in self.memory_cache:
                entry = self.memory_cache[cache_key]
                if not self._is_expired(entry):
                    self._update_access_time(cache_key)
                    self.cache_stats['hits'] += 1
                    self.operation_times['memory_get'].append(time.time() - start_time)
                    return entry['value']
                else:
                    del self.memory_cache[cache_key]
                    if cache_key in self.cache_access_times:
                        del self.cache_access_times[cache_key]
            
            # Try disk cache
            cache_file = os.path.join(self.cache_dir, f"{cache_key}.cache")
            compressed_file = os.path.join(self.cache_dir, f"{cache_key}.cache.gz")
            
            # Try compressed file first if compression is enabled
            if self.compression and os.path.exists(compressed_file):
                try:
                    with gzip.open(compressed_file, 'rb') as f:
                        entry = pickle.load(f)
                    
                    if not self._is_expired(entry):
                        # Load back into memory cache
                        self._evict_lru_if_needed()
                        self.memory_cache[cache_key] = entry
                        self._update_access_time(cache_key)
                        self.cache_stats['hits'] += 1
                        self.cache_stats['disk_reads'] += 1
                        self.operation_times['disk_get'].append(time.time() - start_time)
                        return entry['value']
                    else:
                        os.remove(compressed_file)
                except Exception as e:
                    self.logger.warning(f"Error reading compressed cache file: {e}")
                    try:
                        os.remove(compressed_file)
                    except:
                        pass
            
            # Try uncompressed file
            elif os.path.exists(cache_file):
                try:
                    with open(cache_file, 'rb') as f:
                        entry = pickle.load(f)
                    
                    if not self._is_expired(entry):
                        # Load back into memory cache
                        self._evict_lru_if_needed()
                        self.memory_cache[cache_key] = entry
                        self._update_access_time(cache_key)
                        self.cache_stats['hits'] += 1
                        self.cache_stats['disk_reads'] += 1
                        self.operation_times['disk_get'].append(time.time() - start_time)
                        return entry['value']
                    else:
                        os.remove(cache_file)
                except Exception as e:
                    self.logger.warning(f"Error reading cache file: {e}")
                    try:
                        os.remove(cache_file)
                    except:
                        pass
            
            self.cache_stats['misses'] += 1
            self.operation_times['cache_miss'].append(time.time() - start_time)
            return None
    
    def set(self, key: str, value: Any, ttl: Optional[int] = None, 
            namespace: str = "default", disk_cache: bool = True) -> None:
        """Set value in cache with compression and performance tracking"""
        start_time = time.time()
        
        with self._lock:
            cache_key = self._generate_key(key, namespace)
            ttl = ttl or self.default_ttl
            
            expires_at = datetime.now() + timedelta(seconds=ttl)
            entry = {
                'value': value,
                'expires_at': expires_at.isoformat(),
                'created_at': datetime.now().isoformat(),
                'size': len(pickle.dumps(value))  # Track size
            }
            
            # Evict if needed before adding
            self._evict_lru_if_needed()
            
            # Store in memory cache
            self.memory_cache[cache_key] = entry
            self._update_access_time(cache_key)
            
            # Store in disk cache if requested
            if disk_cache:
                try:
                    serialized_data = pickle.dumps(entry)
                    
                    if self.compression:
                        # Try compression and use if it saves space
                        compressed_data = gzip.compress(serialized_data)
                        if len(compressed_data) < len(serialized_data) * 0.8:  # Only if 20%+ savings
                            cache_file = os.path.join(self.cache_dir, f"{cache_key}.cache.gz")
                            with gzip.open(cache_file, 'wb') as f:
                                f.write(serialized_data)
                            self.cache_stats['compression_saves'] += 1
                        else:
                            cache_file = os.path.join(self.cache_dir, f"{cache_key}.cache")
                            with open(cache_file, 'wb') as f:
                                f.write(serialized_data)
                    else:
                        cache_file = os.path.join(self.cache_dir, f"{cache_key}.cache")
                        with open(cache_file, 'wb') as f:
                            f.write(serialized_data)
                    
                    self.cache_stats['disk_writes'] += 1
                    
                except Exception as e:
                    self.logger.warning(f"Error writing to disk cache: {e}")
            
            self.cache_stats['sets'] += 1
            self.operation_times['cache_set'].append(time.time() - start_time)
    
    def delete(self, key: str, namespace: str = "default") -> bool:
        """Delete value from cache"""
        with self._lock:
            cache_key = self._generate_key(key, namespace)
            deleted = False
            
            # Remove from memory cache
            if cache_key in self.memory_cache:
                del self.memory_cache[cache_key]
                deleted = True
            
            # Remove from disk cache
            cache_file = os.path.join(self.cache_dir, f"{cache_key}.cache")
            if os.path.exists(cache_file):
                try:
                    os.remove(cache_file)
                    deleted = True
                except Exception:
                    pass
            
            if deleted:
                self.cache_stats['deletes'] += 1
            
            return deleted
    
    def clear(self, namespace: Optional[str] = None) -> int:
        """Clear cache entries"""
        with self._lock:
            cleared = 0
            
            if namespace:
                # Clear specific namespace
                prefix = f"{namespace}:"
                keys_to_remove = []
                
                for cache_key in list(self.memory_cache.keys()):
                    # Check if this key belongs to the namespace
                    for mem_key, entry in self.memory_cache.items():
                        if mem_key.startswith(hashlib.md5(prefix.encode()).hexdigest()[:8]):
                            keys_to_remove.append(mem_key)
                
                for key in keys_to_remove:
                    if key in self.memory_cache:
                        del self.memory_cache[key]
                        cleared += 1
            else:
                # Clear all
                cleared = len(self.memory_cache)
                self.memory_cache.clear()
                
                # Clear disk cache
                try:
                    for filename in os.listdir(self.cache_dir):
                        if filename.endswith('.cache'):
                            os.remove(os.path.join(self.cache_dir, filename))
                except Exception:
                    pass
            
            return cleared
    
    def get_stats(self) -> Dict[str, Any]:
        """Get comprehensive cache statistics"""
        with self._lock:
            total_requests = self.cache_stats['hits'] + self.cache_stats['misses']
            hit_rate = (self.cache_stats['hits'] / total_requests * 100) if total_requests > 0 else 0
            
            # Calculate disk cache files
            disk_files = []
            try:
                for f in os.listdir(self.cache_dir):
                    if f.endswith('.cache') or f.endswith('.cache.gz'):
                        disk_files.append(f)
            except:
                pass
            
            # Calculate average operation times
            avg_times = {}
            for operation, times in self.operation_times.items():
                if times:
                    avg_times[f'avg_{operation}_time_ms'] = round(sum(times) / len(times) * 1000, 2)
            
            # Memory usage estimation
            memory_size_bytes = sum(entry.get('size', 0) for entry in self.memory_cache.values())
            
            return {
                **self.cache_stats,
                'hit_rate': round(hit_rate, 2),
                'memory_entries': len(self.memory_cache),
                'memory_size_mb': round(memory_size_bytes / (1024 * 1024), 2),
                'disk_entries': len(disk_files),
                'max_memory_size': self.max_memory_size,
                'compression_enabled': self.compression,
                **avg_times
            }
    
    def get_performance_report(self) -> Dict[str, Any]:
        """Get detailed performance report"""
        with self._lock:
            stats = self.get_stats()
            
            # Top accessed keys
            top_keys = sorted(
                self.cache_access_times.items(), 
                key=lambda x: x[1], 
                reverse=True
            )[:10]
            
            # Cache efficiency metrics
            total_ops = stats['hits'] + stats['misses']
            efficiency_score = (stats['hits'] / total_ops * 100) if total_ops > 0 else 0
            
            return {
                'basic_stats': stats,
                'top_accessed_keys': [key for key, _ in top_keys],
                'efficiency_score': round(efficiency_score, 2),
                'recommendations': self._get_optimization_recommendations(stats)
            }
    
    def _get_optimization_recommendations(self, stats: Dict) -> List[str]:
        """Generate optimization recommendations based on stats"""
        recommendations = []
        
        if stats['hit_rate'] < 50:
            recommendations.append("Low hit rate - consider increasing TTL or cache size")
        
        if stats['evictions'] > stats['sets'] * 0.3:
            recommendations.append("High eviction rate - consider increasing max_memory_size")
        
        if stats['disk_reads'] > stats['hits'] * 0.5:
            recommendations.append("High disk usage - consider increasing memory cache size")
        
        if not stats['compression_enabled'] and stats['disk_entries'] > 100:
            recommendations.append("Enable compression to save disk space")
        
        return recommendations
    
    def _cleanup_expired(self) -> int:
        """Clean up expired cache entries"""
        with self._lock:
            cleaned = 0
            
            # Clean memory cache
            expired_keys = []
            for key, entry in self.memory_cache.items():
                if self._is_expired(entry):
                    expired_keys.append(key)
            
            for key in expired_keys:
                del self.memory_cache[key]
                cleaned += 1
            
            # Clean disk cache
            try:
                for filename in os.listdir(self.cache_dir):
                    if filename.endswith('.cache'):
                        cache_file = os.path.join(self.cache_dir, filename)
                        try:
                            with open(cache_file, 'rb') as f:
                                entry = pickle.load(f)
                            
                            if self._is_expired(entry):
                                os.remove(cache_file)
                                cleaned += 1
                        except Exception:
                            # Remove corrupted files
                            try:
                                os.remove(cache_file)
                                cleaned += 1
                            except:
                                pass
            except Exception:
                pass
            
            return cleaned
    
    def _start_cleanup_thread(self):
        """Start background cleanup thread"""
        def cleanup_worker():
            while True:
                time.sleep(300)  # Clean up every 5 minutes
                try:
                    self._cleanup_expired()
                except Exception:
                    pass
        
        cleanup_thread = threading.Thread(target=cleanup_worker, daemon=True)
        cleanup_thread.start()

# Global cache instance with configurable settings
def create_cache_manager():
    """Create cache manager with configuration"""
    try:
        from config import config
        config_name = os.environ.get('FLASK_ENV', 'development')
        app_config = config.get(config_name, config['default'])
        
        return CacheManager(
            cache_dir="cache",
            default_ttl=getattr(app_config, 'CACHE_DEFAULT_TTL', 3600),
            max_memory_size=getattr(app_config, 'CACHE_MAX_MEMORY_SIZE', 2000),
            compression=getattr(app_config, 'CACHE_COMPRESSION', True)
        )
    except ImportError:
        # Fallback if config not available
        return CacheManager(
            cache_dir="cache",
            default_ttl=3600,
            max_memory_size=2000,
            compression=True
        )

cache_manager = create_cache_manager()

def cached(ttl: int = 3600, namespace: str = "default", 
           key_func: Optional[Callable] = None, disk_cache: bool = True,
           ignore_args: List[str] = None):
    """Enhanced decorator for caching function results"""
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            # Generate cache key
            if key_func:
                cache_key = key_func(*args, **kwargs)
            else:
                # Default key generation with argument filtering
                key_parts = [func.__name__]
                
                # Add positional args
                key_parts.extend(str(arg) for arg in args)
                
                # Add keyword args, excluding ignored ones
                filtered_kwargs = kwargs.copy()
                if ignore_args:
                    for arg in ignore_args:
                        filtered_kwargs.pop(arg, None)
                
                key_parts.extend(f"{k}={v}" for k, v in sorted(filtered_kwargs.items()))
                cache_key = "|".join(key_parts)
            
            # Try to get from cache
            cached_result = cache_manager.get(cache_key, namespace)
            if cached_result is not None:
                return cached_result
            
            # Execute function and cache result
            result = func(*args, **kwargs)
            cache_manager.set(cache_key, result, ttl, namespace, disk_cache)
            
            return result
        
        # Add cache management methods to the function
        wrapper.cache_clear = lambda: cache_manager.clear(namespace)
        wrapper.cache_info = lambda: cache_manager.get_stats()
        wrapper.cache_performance = lambda: cache_manager.get_performance_report()
        
        return wrapper
    return decorator

def cache_warm_up(func: Callable, warm_up_data: List[Tuple], namespace: str = "default"):
    """Warm up cache with predefined data"""
    for args, kwargs in warm_up_data:
        try:
            func(*args, **kwargs)
        except Exception as e:
            logging.warning(f"Cache warm-up failed for {func.__name__}: {e}")

class CachePreloader:
    """Preload frequently accessed data into cache"""
    
    def __init__(self, cache_manager: CacheManager):
        self.cache_manager = cache_manager
        self.preload_tasks = []
    
    def add_preload_task(self, func: Callable, args: tuple, kwargs: dict, 
                        namespace: str = "default", ttl: int = 3600):
        """Add a function to preload into cache"""
        self.preload_tasks.append({
            'func': func,
            'args': args,
            'kwargs': kwargs,
            'namespace': namespace,
            'ttl': ttl
        })
    
    def execute_preload(self):
        """Execute all preload tasks"""
        for task in self.preload_tasks:
            try:
                result = task['func'](*task['args'], **task['kwargs'])
                # Generate cache key similar to cached decorator
                key_parts = [task['func'].__name__]
                key_parts.extend(str(arg) for arg in task['args'])
                key_parts.extend(f"{k}={v}" for k, v in sorted(task['kwargs'].items()))
                cache_key = "|".join(key_parts)
                
                self.cache_manager.set(
                    cache_key, result, task['ttl'], task['namespace']
                )
            except Exception as e:
                logging.warning(f"Preload failed for {task['func'].__name__}: {e}")

# Global preloader instance
cache_preloader = CachePreloader(cache_manager)

def cache_key_for_user_responses(responses: Dict) -> str:
    """Generate cache key for user responses"""
    # Sort responses for consistent key generation
    sorted_responses = sorted(responses.items())
    return hashlib.md5(str(sorted_responses).encode()).hexdigest()

def cache_key_for_champion(champion_name: str, detail_type: str = "basic") -> str:
    """Generate cache key for champion data"""
    return f"champion_{champion_name.lower()}_{detail_type}"

class PerformanceMonitor:
    """Advanced performance monitoring with alerting and optimization suggestions"""
    
    def __init__(self, alert_threshold: float = 1.0):
        self.metrics = {}
        self.alert_threshold = alert_threshold
        self.alerts = []
        self.operation_history = defaultdict(list)
        self._lock = threading.RLock()
        
        # Performance thresholds
        self.thresholds = {
            'ml_prediction': 0.5,  # 500ms
            'data_loading': 0.3,   # 300ms
            'cache_operation': 0.1, # 100ms
            'db_query': 0.2        # 200ms
        }
    
    def record_timing(self, operation: str, duration: float, metadata: Dict = None):
        """Record operation timing with metadata"""
        with self._lock:
            if operation not in self.metrics:
                self.metrics[operation] = {
                    'count': 0,
                    'total_time': 0,
                    'min_time': float('inf'),
                    'max_time': 0,
                    'avg_time': 0,
                    'p95_time': 0,
                    'p99_time': 0,
                    'recent_times': []
                }
            
            metric = self.metrics[operation]
            metric['count'] += 1
            metric['total_time'] += duration
            metric['min_time'] = min(metric['min_time'], duration)
            metric['max_time'] = max(metric['max_time'], duration)
            metric['avg_time'] = metric['total_time'] / metric['count']
            
            # Keep recent times for percentile calculation
            metric['recent_times'].append(duration)
            if len(metric['recent_times']) > 1000:  # Keep last 1000 measurements
                metric['recent_times'] = metric['recent_times'][-1000:]
            
            # Calculate percentiles
            if len(metric['recent_times']) >= 10:
                sorted_times = sorted(metric['recent_times'])
                metric['p95_time'] = sorted_times[int(len(sorted_times) * 0.95)]
                metric['p99_time'] = sorted_times[int(len(sorted_times) * 0.99)]
            
            # Store in history with timestamp
            self.operation_history[operation].append({
                'timestamp': time.time(),
                'duration': duration,
                'metadata': metadata or {}
            })
            
            # Keep history limited
            if len(self.operation_history[operation]) > 10000:
                self.operation_history[operation] = self.operation_history[operation][-5000:]
            
            # Check for performance alerts
            self._check_performance_alert(operation, duration)
    
    def _check_performance_alert(self, operation: str, duration: float):
        """Check if operation duration exceeds thresholds"""
        threshold = self.thresholds.get(operation, self.alert_threshold)
        
        if duration > threshold:
            alert = {
                'timestamp': datetime.now().isoformat(),
                'operation': operation,
                'duration': duration,
                'threshold': threshold,
                'severity': 'high' if duration > threshold * 2 else 'medium'
            }
            
            self.alerts.append(alert)
            
            # Keep only recent alerts
            if len(self.alerts) > 100:
                self.alerts = self.alerts[-50:]
    
    def get_metrics(self) -> Dict[str, Any]:
        """Get comprehensive performance metrics"""
        with self._lock:
            return {
                'operations': dict(self.metrics),
                'alerts': self.alerts[-10:],  # Last 10 alerts
                'total_operations': sum(m['count'] for m in self.metrics.values()),
                'avg_response_time': self._calculate_overall_avg_time()
            }
    
    def _calculate_overall_avg_time(self) -> float:
        """Calculate overall average response time"""
        total_time = sum(m['total_time'] for m in self.metrics.values())
        total_count = sum(m['count'] for m in self.metrics.values())
        return total_time / total_count if total_count > 0 else 0
    
    def get_performance_report(self) -> Dict[str, Any]:
        """Generate comprehensive performance report"""
        with self._lock:
            report = {
                'summary': self.get_metrics(),
                'slow_operations': self._get_slow_operations(),
                'trends': self._analyze_trends(),
                'recommendations': self._generate_recommendations()
            }
            return report
    
    def _get_slow_operations(self) -> List[Dict]:
        """Identify operations that are consistently slow"""
        slow_ops = []
        for op_name, metric in self.metrics.items():
            if metric['avg_time'] > self.thresholds.get(op_name, self.alert_threshold):
                slow_ops.append({
                    'operation': op_name,
                    'avg_time': metric['avg_time'],
                    'count': metric['count'],
                    'p95_time': metric.get('p95_time', 0)
                })
        
        return sorted(slow_ops, key=lambda x: x['avg_time'], reverse=True)
    
    def _analyze_trends(self) -> Dict[str, Any]:
        """Analyze performance trends over time"""
        trends = {}
        current_time = time.time()
        
        for operation, history in self.operation_history.items():
            if len(history) < 10:
                continue
            
            # Get recent vs older performance
            recent_cutoff = current_time - 3600  # Last hour
            recent_times = [h['duration'] for h in history if h['timestamp'] > recent_cutoff]
            older_times = [h['duration'] for h in history if h['timestamp'] <= recent_cutoff]
            
            if recent_times and older_times:
                recent_avg = sum(recent_times) / len(recent_times)
                older_avg = sum(older_times) / len(older_times)
                
                trend = 'improving' if recent_avg < older_avg else 'degrading'
                change_pct = ((recent_avg - older_avg) / older_avg) * 100
                
                trends[operation] = {
                    'trend': trend,
                    'change_percent': round(change_pct, 2),
                    'recent_avg': round(recent_avg, 3),
                    'older_avg': round(older_avg, 3)
                }
        
        return trends
    
    def _generate_recommendations(self) -> List[str]:
        """Generate performance optimization recommendations"""
        recommendations = []
        
        # Check cache performance
        cache_stats = cache_manager.get_stats()
        if cache_stats['hit_rate'] < 70:
            recommendations.append("Improve cache hit rate by increasing TTL or preloading data")
        
        # Check slow operations
        slow_ops = self._get_slow_operations()
        if slow_ops:
            recommendations.append(f"Optimize slow operations: {', '.join([op['operation'] for op in slow_ops[:3]])}")
        
        # Check memory usage
        if cache_stats.get('evictions', 0) > cache_stats.get('sets', 0) * 0.3:
            recommendations.append("Increase cache memory size to reduce evictions")
        
        # Check alert frequency
        recent_alerts = [a for a in self.alerts if 
                        datetime.fromisoformat(a['timestamp']) > datetime.now() - timedelta(hours=1)]
        if len(recent_alerts) > 10:
            recommendations.append("High alert frequency - investigate performance bottlenecks")
        
        return recommendations
    
    def clear_metrics(self):
        """Clear all metrics and history"""
        with self._lock:
            self.metrics.clear()
            self.operation_history.clear()
            self.alerts.clear()

# Global performance monitor
performance_monitor = PerformanceMonitor()

def timed(operation_name: str):
    """Decorator for timing function execution"""
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            start_time = time.time()
            try:
                result = func(*args, **kwargs)
                return result
            finally:
                duration = time.time() - start_time
                performance_monitor.record_timing(operation_name, duration)
        return wrapper
    return decorator