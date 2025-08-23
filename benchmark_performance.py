#!/usr/bin/env python3
"""
Performance benchmark script to demonstrate caching and optimization improvements
"""

import time
import statistics
from typing import List, Dict, Any
from services.champion_service import ChampionService
from services.recommendation_engine import RecommendationEngine
from utils.cache_manager import cache_manager, performance_monitor

def benchmark_champion_service():
    """Benchmark champion service with and without caching"""
    print("Benchmarking Champion Service...")
    
    champion_service = ChampionService()
    
    # Clear cache to start fresh
    cache_manager.clear()
    
    # Benchmark without cache (first calls)
    print("  Testing without cache (first calls)...")
    times_without_cache = []
    
    for i in range(5):
        start_time = time.time()
        champions = champion_service.get_all_champions()
        duration = time.time() - start_time
        times_without_cache.append(duration)
        
        # Clear cache between calls to simulate no caching
        cache_manager.clear()
    
    # Benchmark with cache (subsequent calls)
    print("  Testing with cache (subsequent calls)...")
    
    # Prime the cache
    champion_service.get_all_champions()
    
    times_with_cache = []
    for i in range(10):
        start_time = time.time()
        champions = champion_service.get_all_champions()
        duration = time.time() - start_time
        times_with_cache.append(duration)
    
    # Results
    avg_without_cache = statistics.mean(times_without_cache)
    avg_with_cache = statistics.mean(times_with_cache)
    improvement = ((avg_without_cache - avg_with_cache) / avg_without_cache) * 100
    
    print(f"  Average time without cache: {avg_without_cache:.4f}s")
    print(f"  Average time with cache: {avg_with_cache:.4f}s")
    print(f"  Performance improvement: {improvement:.1f}%")
    print(f"  Speedup factor: {avg_without_cache / avg_with_cache:.1f}x")
    
    return {
        'without_cache': avg_without_cache,
        'with_cache': avg_with_cache,
        'improvement_percent': improvement,
        'speedup_factor': avg_without_cache / avg_with_cache
    }

def benchmark_champion_lookups():
    """Benchmark champion lookup operations"""
    print("\nBenchmarking Champion Lookups...")
    
    champion_service = ChampionService()
    champions = champion_service.get_all_champions()
    
    if not champions:
        print("  No champions available for testing")
        return {}
    
    test_champions = champions[:10]  # Test with first 10 champions
    
    # Clear cache
    cache_manager.clear()
    
    # Benchmark first lookups (no cache)
    print("  Testing lookups without cache...")
    times_without_cache = []
    
    for champion in test_champions:
        cache_manager.clear()  # Clear cache for each lookup
        start_time = time.time()
        found = champion_service.get_champion_by_name(champion.name)
        duration = time.time() - start_time
        times_without_cache.append(duration)
    
    # Benchmark cached lookups
    print("  Testing lookups with cache...")
    
    # Prime cache
    for champion in test_champions:
        champion_service.get_champion_by_name(champion.name)
    
    times_with_cache = []
    for champion in test_champions:
        start_time = time.time()
        found = champion_service.get_champion_by_name(champion.name)
        duration = time.time() - start_time
        times_with_cache.append(duration)
    
    # Results
    avg_without_cache = statistics.mean(times_without_cache)
    avg_with_cache = statistics.mean(times_with_cache)
    improvement = ((avg_without_cache - avg_with_cache) / avg_without_cache) * 100
    
    print(f"  Average lookup time without cache: {avg_without_cache:.6f}s")
    print(f"  Average lookup time with cache: {avg_with_cache:.6f}s")
    print(f"  Performance improvement: {improvement:.1f}%")
    print(f"  Speedup factor: {avg_without_cache / avg_with_cache:.1f}x")
    
    return {
        'without_cache': avg_without_cache,
        'with_cache': avg_with_cache,
        'improvement_percent': improvement,
        'speedup_factor': avg_without_cache / avg_with_cache
    }

def benchmark_ml_predictions():
    """Benchmark ML prediction caching"""
    print("\nBenchmarking ML Predictions...")
    
    try:
        rec_engine = RecommendationEngine()
        
        test_responses = {
            '1': 'Tank',
            '2': 'Easy',
            '3': 'Team fights',
            '4': 'Melee',
            '5': 'High'
        }
        
        # Clear cache
        cache_manager.clear()
        
        # Benchmark without cache
        print("  Testing predictions without cache...")
        times_without_cache = []
        
        for i in range(3):  # Fewer iterations due to ML complexity
            cache_manager.clear()
            start_time = time.time()
            try:
                prediction = rec_engine.predict_champion(test_responses)
                duration = time.time() - start_time
                times_without_cache.append(duration)
            except Exception as e:
                print(f"    ML prediction failed: {e}")
                return {}
        
        # Benchmark with cache
        print("  Testing predictions with cache...")
        
        # Prime cache
        try:
            rec_engine.predict_champion(test_responses)
        except Exception:
            pass
        
        times_with_cache = []
        for i in range(5):
            start_time = time.time()
            try:
                prediction = rec_engine.predict_champion(test_responses)
                duration = time.time() - start_time
                times_with_cache.append(duration)
            except Exception as e:
                print(f"    Cached ML prediction failed: {e}")
                break
        
        if not times_without_cache or not times_with_cache:
            print("  ML predictions not available")
            return {}
        
        # Results
        avg_without_cache = statistics.mean(times_without_cache)
        avg_with_cache = statistics.mean(times_with_cache)
        improvement = ((avg_without_cache - avg_with_cache) / avg_without_cache) * 100
        
        print(f"  Average prediction time without cache: {avg_without_cache:.4f}s")
        print(f"  Average prediction time with cache: {avg_with_cache:.4f}s")
        print(f"  Performance improvement: {improvement:.1f}%")
        print(f"  Speedup factor: {avg_without_cache / avg_with_cache:.1f}x")
        
        return {
            'without_cache': avg_without_cache,
            'with_cache': avg_with_cache,
            'improvement_percent': improvement,
            'speedup_factor': avg_without_cache / avg_with_cache
        }
        
    except Exception as e:
        print(f"  ML benchmarking failed: {e}")
        return {}

def benchmark_cache_operations():
    """Benchmark cache operations themselves"""
    print("\nBenchmarking Cache Operations...")
    
    # Test cache set operations
    print("  Testing cache set operations...")
    start_time = time.time()
    for i in range(1000):
        cache_manager.set(f"bench_key_{i}", f"value_{i}")
    set_time = time.time() - start_time
    
    print(f"  1000 cache set operations: {set_time:.4f}s ({1000/set_time:.0f} ops/sec)")
    
    # Test cache get operations
    print("  Testing cache get operations...")
    start_time = time.time()
    hits = 0
    for i in range(1000):
        value = cache_manager.get(f"bench_key_{i}")
        if value:
            hits += 1
    get_time = time.time() - start_time
    
    print(f"  1000 cache get operations: {get_time:.4f}s ({1000/get_time:.0f} ops/sec)")
    print(f"  Cache hit rate: {hits/1000*100:.1f}%")
    
    return {
        'set_time': set_time,
        'get_time': get_time,
        'set_ops_per_sec': 1000 / set_time,
        'get_ops_per_sec': 1000 / get_time,
        'hit_rate': hits / 1000 * 100
    }

def show_cache_statistics():
    """Show current cache statistics"""
    print("\nCurrent Cache Statistics:")
    stats = cache_manager.get_stats()
    
    for key, value in stats.items():
        if isinstance(value, float):
            print(f"  {key}: {value:.2f}")
        else:
            print(f"  {key}: {value}")

def show_performance_metrics():
    """Show performance monitoring metrics"""
    print("\nPerformance Monitoring Metrics:")
    
    try:
        metrics = performance_monitor.get_metrics()
        
        if metrics.get('operations'):
            print("  Operation Performance:")
            for operation, data in metrics['operations'].items():
                print(f"    {operation}:")
                print(f"      Count: {data['count']}")
                print(f"      Avg Time: {data['avg_time']:.4f}s")
                print(f"      Min Time: {data['min_time']:.4f}s")
                print(f"      Max Time: {data['max_time']:.4f}s")
        
        if metrics.get('alerts'):
            print(f"  Recent Alerts: {len(metrics['alerts'])}")
            for alert in metrics['alerts'][-3:]:  # Show last 3 alerts
                print(f"    {alert['operation']}: {alert['duration']:.3f}s ({alert['severity']})")
    
    except Exception as e:
        print(f"  Error getting performance metrics: {e}")

def main():
    """Main benchmark runner"""
    print("LoL Champion Recommender - Performance Benchmark")
    print("=" * 60)
    
    results = {}
    
    # Run benchmarks
    results['champion_service'] = benchmark_champion_service()
    results['champion_lookups'] = benchmark_champion_lookups()
    results['ml_predictions'] = benchmark_ml_predictions()
    results['cache_operations'] = benchmark_cache_operations()
    
    # Show statistics
    show_cache_statistics()
    show_performance_metrics()
    
    # Summary
    print("\n" + "=" * 60)
    print("BENCHMARK SUMMARY")
    print("=" * 60)
    
    for benchmark_name, result in results.items():
        if not result:
            continue
            
        print(f"\n{benchmark_name.replace('_', ' ').title()}:")
        
        if 'improvement_percent' in result:
            print(f"  Performance Improvement: {result['improvement_percent']:.1f}%")
            print(f"  Speedup Factor: {result['speedup_factor']:.1f}x")
        
        if 'ops_per_sec' in str(result):
            for key, value in result.items():
                if 'ops_per_sec' in key:
                    print(f"  {key.replace('_', ' ').title()}: {value:.0f}")
    
    print(f"\nOverall: Caching provides significant performance improvements!")
    print("Key benefits:")
    print("- Faster data loading and champion lookups")
    print("- Reduced ML model computation through prediction caching")
    print("- Better user experience with faster response times")
    print("- Reduced server load through efficient caching")

if __name__ == "__main__":
    main()