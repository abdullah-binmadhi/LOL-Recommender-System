#!/usr/bin/env python3
"""
Comprehensive test runner for the LoL Champion Recommender
Runs all test suites with proper reporting and coverage analysis
"""

import os
import sys
import subprocess
import argparse
import time
from pathlib import Path

# Add the parent directory to the path
sys.path.insert(0, str(Path(__file__).parent.parent))

def run_command(command, description=""):
    """Run a command and return the result"""
    print(f"\n{'='*60}")
    print(f"Running: {description or command}")
    print(f"{'='*60}")
    
    start_time = time.time()
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    end_time = time.time()
    
    print(f"Duration: {end_time - start_time:.2f} seconds")
    print(f"Exit code: {result.returncode}")
    
    if result.stdout:
        print("\nSTDOUT:")
        print(result.stdout)
    
    if result.stderr:
        print("\nSTDERR:")
        print(result.stderr)
    
    return result.returncode == 0, result

def check_prerequisites():
    """Check if all prerequisites are installed"""
    print("Checking prerequisites...")
    
    required_packages = [
        'pytest',
        'pytest-cov',
        'pytest-html',
        'pytest-xdist',
        'coverage'
    ]
    
    missing_packages = []
    for package in required_packages:
        try:
            __import__(package.replace('-', '_'))
        except ImportError:
            missing_packages.append(package)
    
    if missing_packages:
        print(f"Missing packages: {', '.join(missing_packages)}")
        print("Install with: pip install " + " ".join(missing_packages))
        return False
    
    print("All prerequisites are installed.")
    return True

def run_unit_tests(args):
    """Run unit tests"""
    command = [
        "python", "-m", "pytest",
        "tests/test_models.py",
        "tests/test_services.py",
        "-v"
    ]
    
    if args.coverage:
        command.extend(["--cov=models", "--cov=services", "--cov-report=term-missing"])
    
    if args.html_report:
        command.extend(["--html=reports/unit_tests.html", "--self-contained-html"])
    
    if args.parallel:
        command.extend(["-n", "auto"])
    
    return run_command(" ".join(command), "Unit Tests")

def run_integration_tests(args):
    """Run integration tests"""
    command = [
        "python", "-m", "pytest",
        "tests/test_recommendation_integration.py",
        "tests/test_ml_components.py",
        "-v", "-m", "integration"
    ]
    
    if args.coverage:
        command.extend(["--cov=services", "--cov=ml", "--cov-report=term-missing"])
    
    if args.html_report:
        command.extend(["--html=reports/integration_tests.html", "--self-contained-html"])
    
    return run_command(" ".join(command), "Integration Tests")

def run_end_to_end_tests(args):
    """Run end-to-end tests"""
    command = [
        "python", "-m", "pytest",
        "tests/test_end_to_end.py",
        "-v", "-m", "e2e"
    ]
    
    if args.coverage:
        command.extend(["--cov=app", "--cov-report=term-missing"])
    
    if args.html_report:
        command.extend(["--html=reports/e2e_tests.html", "--self-contained-html"])
    
    # E2E tests should not run in parallel
    return run_command(" ".join(command), "End-to-End Tests")

def run_performance_tests(args):
    """Run performance tests"""
    if args.skip_slow:
        print("Skipping performance tests (--skip-slow flag)")
        return True, None
    
    command = [
        "python", "-m", "pytest",
        "tests/test_performance.py",
        "-v", "-m", "performance",
        "--tb=short"
    ]
    
    if args.html_report:
        command.extend(["--html=reports/performance_tests.html", "--self-contained-html"])
    
    return run_command(" ".join(command), "Performance Tests")

def run_accessibility_tests(args):
    """Run accessibility tests"""
    command = [
        "python", "-m", "pytest",
        "tests/test_accessibility.py",
        "-v"
    ]
    
    if args.html_report:
        command.extend(["--html=reports/accessibility_tests.html", "--self-contained-html"])
    
    return run_command(" ".join(command), "Accessibility Tests")

def run_error_handling_tests(args):
    """Run error handling tests"""
    command = [
        "python", "-m", "pytest",
        "tests/test_error_handling.py",
        "-v"
    ]
    
    if args.html_report:
        command.extend(["--html=reports/error_handling_tests.html", "--self-contained-html"])
    
    return run_command(" ".join(command), "Error Handling Tests")

def run_all_tests_combined(args):
    """Run all tests in a single command"""
    command = [
        "python", "-m", "pytest",
        "tests/",
        "-v"
    ]
    
    if args.coverage:
        command.extend([
            "--cov=app",
            "--cov=models", 
            "--cov=services",
            "--cov=ml",
            "--cov=utils",
            "--cov-report=term-missing",
            "--cov-report=html:reports/coverage_html"
        ])
    
    if args.html_report:
        command.extend(["--html=reports/all_tests.html", "--self-contained-html"])
    
    if args.parallel and not args.e2e_only:
        command.extend(["-n", "auto"])
    
    if args.skip_slow:
        command.extend(["-m", "not slow"])
    
    if args.unit_only:
        command.extend(["-m", "not integration and not e2e and not performance"])
    elif args.integration_only:
        command.extend(["-m", "integration"])
    elif args.e2e_only:
        command.extend(["-m", "e2e"])
    elif args.performance_only:
        command.extend(["-m", "performance"])
    
    return run_command(" ".join(command), "All Tests Combined")

def generate_coverage_report():
    """Generate comprehensive coverage report"""
    print("\nGenerating coverage report...")
    
    # Generate HTML coverage report
    success, result = run_command(
        "coverage html -d reports/coverage_html",
        "HTML Coverage Report"
    )
    
    if success:
        print("Coverage report generated at: reports/coverage_html/index.html")
    
    # Generate XML coverage report (for CI/CD)
    run_command(
        "coverage xml -o reports/coverage.xml",
        "XML Coverage Report"
    )
    
    # Print coverage summary
    run_command("coverage report", "Coverage Summary")
    
    return success

def run_code_quality_checks():
    """Run code quality checks"""
    print("\nRunning code quality checks...")
    
    # Check if tools are available
    tools = ['flake8', 'black', 'isort']
    available_tools = []
    
    for tool in tools:
        try:
            subprocess.run([tool, '--version'], capture_output=True, check=True)
            available_tools.append(tool)
        except (subprocess.CalledProcessError, FileNotFoundError):
            print(f"Warning: {tool} not available")
    
    results = []
    
    # Run flake8 (linting)
    if 'flake8' in available_tools:
        success, _ = run_command(
            "flake8 app.py models/ services/ ml/ utils/ --max-line-length=88 --extend-ignore=E203,W503",
            "Code Linting (flake8)"
        )
        results.append(('flake8', success))
    
    # Check black formatting
    if 'black' in available_tools:
        success, _ = run_command(
            "black --check --diff app.py models/ services/ ml/ utils/",
            "Code Formatting Check (black)"
        )
        results.append(('black', success))
    
    # Check import sorting
    if 'isort' in available_tools:
        success, _ = run_command(
            "isort --check-only --diff app.py models/ services/ ml/ utils/",
            "Import Sorting Check (isort)"
        )
        results.append(('isort', success))
    
    return results

def create_reports_directory():
    """Create reports directory if it doesn't exist"""
    reports_dir = Path("reports")
    reports_dir.mkdir(exist_ok=True)
    return reports_dir

def print_summary(results):
    """Print test results summary"""
    print("\n" + "="*80)
    print("TEST RESULTS SUMMARY")
    print("="*80)
    
    total_tests = len(results)
    passed_tests = sum(1 for _, success in results if success)
    failed_tests = total_tests - passed_tests
    
    for test_name, success in results:
        status = "PASSED" if success else "FAILED"
        print(f"{test_name:<30} {status}")
    
    print("-" * 80)
    print(f"Total: {total_tests}, Passed: {passed_tests}, Failed: {failed_tests}")
    
    if failed_tests == 0:
        print("\n🎉 All tests passed!")
        return True
    else:
        print(f"\n❌ {failed_tests} test suite(s) failed.")
        return False

def main():
    """Main test runner function"""
    parser = argparse.ArgumentParser(description="Run LoL Champion Recommender tests")
    
    # Test selection options
    parser.add_argument("--unit-only", action="store_true", help="Run only unit tests")
    parser.add_argument("--integration-only", action="store_true", help="Run only integration tests")
    parser.add_argument("--e2e-only", action="store_true", help="Run only end-to-end tests")
    parser.add_argument("--performance-only", action="store_true", help="Run only performance tests")
    parser.add_argument("--skip-slow", action="store_true", help="Skip slow tests")
    
    # Execution options
    parser.add_argument("--parallel", action="store_true", help="Run tests in parallel")
    parser.add_argument("--coverage", action="store_true", help="Generate coverage report")
    parser.add_argument("--html-report", action="store_true", help="Generate HTML test reports")
    parser.add_argument("--code-quality", action="store_true", help="Run code quality checks")
    parser.add_argument("--separate", action="store_true", help="Run test suites separately")
    
    # Output options
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")
    parser.add_argument("--quiet", "-q", action="store_true", help="Quiet output")
    
    args = parser.parse_args()
    
    # Check prerequisites
    if not check_prerequisites():
        sys.exit(1)
    
    # Create reports directory
    create_reports_directory()
    
    # Store results
    results = []
    
    print("Starting LoL Champion Recommender Test Suite")
    print(f"Python version: {sys.version}")
    print(f"Working directory: {os.getcwd()}")
    
    start_time = time.time()
    
    try:
        if args.separate:
            # Run test suites separately
            if not any([args.unit_only, args.integration_only, args.e2e_only, args.performance_only]):
                # Run all suites separately
                success, _ = run_unit_tests(args)
                results.append(("Unit Tests", success))
                
                success, _ = run_integration_tests(args)
                results.append(("Integration Tests", success))
                
                success, _ = run_end_to_end_tests(args)
                results.append(("End-to-End Tests", success))
                
                success, _ = run_performance_tests(args)
                results.append(("Performance Tests", success))
                
                success, _ = run_accessibility_tests(args)
                results.append(("Accessibility Tests", success))
                
                success, _ = run_error_handling_tests(args)
                results.append(("Error Handling Tests", success))
            else:
                # Run specific suite
                if args.unit_only:
                    success, _ = run_unit_tests(args)
                    results.append(("Unit Tests", success))
                elif args.integration_only:
                    success, _ = run_integration_tests(args)
                    results.append(("Integration Tests", success))
                elif args.e2e_only:
                    success, _ = run_end_to_end_tests(args)
                    results.append(("End-to-End Tests", success))
                elif args.performance_only:
                    success, _ = run_performance_tests(args)
                    results.append(("Performance Tests", success))
        else:
            # Run all tests in a single command
            success, _ = run_all_tests_combined(args)
            results.append(("All Tests", success))
        
        # Run code quality checks if requested
        if args.code_quality:
            quality_results = run_code_quality_checks()
            results.extend(quality_results)
        
        # Generate coverage report if requested
        if args.coverage:
            success = generate_coverage_report()
            results.append(("Coverage Report", success))
    
    except KeyboardInterrupt:
        print("\n\nTest execution interrupted by user.")
        sys.exit(1)
    except Exception as e:
        print(f"\n\nUnexpected error during test execution: {e}")
        sys.exit(1)
    
    end_time = time.time()
    total_time = end_time - start_time
    
    # Print summary
    all_passed = print_summary(results)
    
    print(f"\nTotal execution time: {total_time:.2f} seconds")
    
    if args.html_report:
        print(f"\nHTML reports available in: reports/")
    
    if args.coverage:
        print(f"Coverage report available at: reports/coverage_html/index.html")
    
    # Exit with appropriate code
    sys.exit(0 if all_passed else 1)

if __name__ == "__main__":
    main()