#!/usr/bin/env python3
"""
Command-line tool to evaluate champion recommendation performance
"""

import argparse
import sys
from pathlib import Path

# Add the current directory to the path
sys.path.append(str(Path(__file__).parent))

from evaluation_metrics import evaluate_model_performance

def main():
    """Main function"""
    parser = argparse.ArgumentParser(description='Evaluate champion recommendation performance')
    parser.add_argument('--data-file', default='user_data_backup.json',
                        help='Path to user data JSON file (default: user_data_backup.json)')
    parser.add_argument('--output-format', choices=['text', 'json'], default='text',
                        help='Output format (default: text)')
    
    args = parser.parse_args()
    
    print("Champion Recommender System - Performance Evaluation")
    print("=" * 55)
    
    # Run evaluation
    results = evaluate_model_performance(args.data_file)
    
    if not results:
        print("Evaluation failed!")
        sys.exit(1)
    
    if args.output_format == 'json':
        import json
        print(json.dumps(results, indent=2))
    else:
        # Results already printed by evaluate_model_performance function
        pass
    
    print("\nEvaluation completed successfully!")

if __name__ == "__main__":
    main()