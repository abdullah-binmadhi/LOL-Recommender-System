#!/usr/bin/env python3
"""
Performance test runner for the LoL Champion Recommender
"""

import sys
import time
import subprocess
import concurrent.futures
from typing import Dict, List, Any
import requests
import json

def run_unit_performance_tests():
    """Run unit performance tests"""
    print("Running unit performance tests...")
    
    try:
        result = subprocess.run([
            sys.executable, '-m', 'pytest', 
            'tests/test_performance.py', 
            '-v', '--tb=short'
        ], capture_output=True, text=True, timeout=300)
        
        print("STDOUT:", result.stdout)
        if result.stderr:
            print("STDERR:", result.stderr)
        
        return result.returncode == 0
    except subprocess.TimeoutExpired:
        print("Performance tests timed out!")
        return False
    except Exception as e:
        print(f"Error running performance tests: {e}")
        return False

def run_load_test(base_url: str = "http://localhost:5000", num_requests: int = 50, concurrency: int = 5):
    """Run load test against the application"""
    print(f"Running load test with {num_requests} requests, {concurrency} concurrent...")
    
    def make_request(request_id: int) -> Dict[str, Any]:
        """Make a single request"""
        start_time = time.time()
        
        try:
            session = requests.Session()
            
            # Start questionnaire
            response = session.get(f"{base_url}/start")
            if response.status_code not in [200, 302]:
                return {"id": request_id, "success": False, "error": f"Start failed: {response.status_code}"}
            
            # Submit answers
            answers = [
                ("1", "Tank"),
                ("2", "Easy"), 
                ("3", "Team fights")
            ]
            
            for question_id, answer in answers:
                response = session.post(f"{base_url}/answer", data={
                    "answer": answer,
                    "action": "next"
                })
                if response.status_code not in [200, 302]:
                    return {"id": request_id, "success": False, "error": f"Answer {question_id} failed: {response.status_code}"}
            
            # Get recommendation
            response = session.get(f"{base_url}/recommendation")
            if response.status_code not in [200, 302]:
                return {"id": request_id, "success": False, "error": f"Recommendation failed: {response.status_code}"}
            
            duration = time.time() - start_time
            return {"id": request_id, "success": True, "duration": duration}
            
        except Exception as e:
            duration = time.time() - start_time
            return {"id": request_id, "success": False, "error": str(e), "duration": duration}
    
    # Run concurrent requests
    start_time = time.time()
    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as executor:
        futures = [executor.submit(make_request, i) for i in range(num_requests)]
        results = [future.result() for future in concurrent.futures.as_completed(futures)]
    
    total_time = time.time() - start_time
    
    # Analyze results
    successful_requests = [r for r in results if r["success"]]
    failed_requests = [r for r in results if not r["success"]]
    
    if successful_requests:
        durations = [r["duration"] for r in successful_requests]
        avg_duration = sum(durations) / len(durations)
        min_duration = min(durations)
        max_duration = max(durations)
        
        # Calculate percentiles
        sorted_durations = sorted(durations)
        p95_duration = sorted_durations[int(len(sorted_durations) * 0.95)]
        p99_duration = sorted_durations[int(len(sorted_durations) * 0.99)]
    else:
        avg_duration = min_duration = max_duration = p95_duration = p99_duration = 0
    
    print(f"\nLoad Test Results:")
    print(f"Total requests: {num_requests}")
    print(f"Successful requests: {len(successful_requests)}")
    print(f"Failed requests: {len(failed_requests)}")
    print(f"Success rate: {len(successful_requests)/num_requests*100:.1f}%")
    print(f"Total time: {total_time:.2f}s")
    print(f"Requests per second: {num_requests/total_time:.2f}")
    
    if successful_requests:
        print(f"\nResponse Times:")
        print(f"Average: {avg_duration:.3f}s")
        print(f"Min: {min_duration:.3f}s")
        print(f"Max: {max_duration:.3f}s")
        print(f"95th percentile: {p95_duration:.3f}s")
        print(f"99th percentile: {p99_duration:.3f}s")
    
    if failed_requests:
        print(f"\nFailure Details:")
        for failure in failed_requests[:5]:  # Show first 5 failures
            print(f"Request {failure['id']}: {failure.get('error', 'Unknown error')}")
    
    # Performance criteria
    success_rate = len(successful_requests) / num_requests
    performance_good = (
        success_rate >= 0.95 and  # 95% success rate
        avg_duration <= 2.0 and   # Average response under 2s
        p95_duration <= 5.0       # 95th percentile under 5s
    )
    
    return performance_good

def test_cache_performance():
    """Test cache performance specifically"""
    print("Testing cache performance...")
    
    try:
        from utils.cache_manager import cache_manager
        
        # Test cache operations
        start_time = time.time()
        
        # Set operations
        for i in range(1000):
            cache_manager.set(f"perf_test_key_{i}", f"value_{i}")
        
        set_time = time.time() - start_time
        print(f"Cache set operations (1000 items): {set_time:.3f}s")
        
        # Get operations
        start_time = time.time()
        hits = 0
        for i in range(1000):
            value = cache_manager.get(f"perf_test_key_{i}")
            if value:
                hits += 1
        
        get_time = time.time() - start_time
        print(f"Cache get operations (1000 items): {get_time:.3f}s")
        print(f"Cache hit rate: {hits/1000*100:.1f}%")
        
        # Get cache stats
        stats = cache_manager.get_stats()
        print(f"Cache statistics: {json.dumps(stats, indent=2)}")
        
        # Performance criteria
        cache_good = (
            set_time < 1.0 and    # Set operations under 1s
            get_time < 0.5 and    # Get operations under 0.5s
            hits >= 950           # At least 95% hit rate
        )
        
        return cache_good
        
    except Exception as e:
        print(f"Cache performance test failed: {e}")
        return False

def main():
    """Main performance test runner"""
    print("LoL Champion Recommender - Performance Test Suite")
    print("=" * 50)
    
    results = {}
    
    # Run unit performance tests
    print("\n1. Unit Performance Tests")
    print("-" * 30)
    results['unit_tests'] = run_unit_performance_tests()
    
    # Test cache performance
    print("\n2. Cache Performance Tests")
    print("-" * 30)
    results['cache_performance'] = test_cache_performance()
    
    # Ask if user wants to run load tests
    print("\n3. Load Tests")
    print("-" * 30)
    
    run_load_tests = input("Run load tests? This requires the app to be running (y/N): ").lower().startswith('y')
    
    if run_load_tests:
        base_url = input("Enter base URL (default: http://localhost:5000): ").strip()
        if not base_url:
            base_url = "http://localhost:5000"
        
        try:
            # Test if app is running
            response = requests.get(base_url, timeout=5)
            if response.status_code == 200:
                results['load_test'] = run_load_test(base_url)
            else:
                print(f"App not responding correctly at {base_url}")
                results['load_test'] = False
        except requests.exceptions.RequestException as e:
            print(f"Cannot connect to app at {base_url}: {e}")
            results['load_test'] = False
    else:
        results['load_test'] = None
    
    # Summary
    print("\n" + "=" * 50)
    print("PERFORMANCE TEST SUMMARY")
    print("=" * 50)
    
    for test_name, result in results.items():
        if result is None:
            status = "SKIPPED"
        elif result:
            status = "PASSED"
        else:
            status = "FAILED"
        
        print(f"{test_name.replace('_', ' ').title()}: {status}")
    
    # Overall result
    passed_tests = sum(1 for r in results.values() if r is True)
    total_tests = sum(1 for r in results.values() if r is not None)
    
    if total_tests > 0:
        print(f"\nOverall: {passed_tests}/{total_tests} tests passed")
        
        if passed_tests == total_tests:
            print("🎉 All performance tests passed!")
            return 0
        else:
            print("⚠️  Some performance tests failed. Check the output above.")
            return 1
    else:
        print("No tests were run.")
        return 1

if __name__ == "__main__":
    sys.exit(main())