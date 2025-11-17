#!/usr/bin/env python3
"""
Create Test Dataset with Ground Truth

This script helps create a properly formatted test dataset
with ground truth labels for model evaluation.
"""

import json
import csv
from pathlib import Path
from typing import List, Dict, Any


def load_champions() -> Dict[str, Any]:
    """Load champion data"""
    champion_path = Path("../src/data/champions.json")
    if champion_path.exists():
        with open(champion_path, 'r') as f:
            return json.load(f)
    return {}


def get_champions_by_role(champions: Dict[str, Any], role: str) -> List[str]:
    """Get all champions for a specific role"""
    return [name for name, data in champions.items() 
            if data.get('role') == role]


def create_sample_test_data() -> List[Dict[str, Any]]:
    """
    Create sample test dataset with ground truth
    
    Returns:
        List of test user dictionaries
    """
    champions = load_champions()
    
    test_data = [
        # User 1: Mage preference
        {
            "id": "test_user_001",
            "role": "Mage",
            "difficulty": "Medium (4-6)",
            "playstyle": "High Damage Output",
            "ground_truth": get_champions_by_role(champions, "Mage")[:5]
        },
        # User 2: Fighter preference
        {
            "id": "test_user_002",
            "role": "Fighter",
            "difficulty": "Easy (1-3)",
            "playstyle": "Tanky and Durable",
            "ground_truth": get_champions_by_role(champions, "Fighter")[:5]
        },
        # User 3: Support preference
        {
            "id": "test_user_003",
            "role": "Support",
            "difficulty": "Medium (4-6)",
            "playstyle": "Support Team",
            "ground_truth": get_champions_by_role(champions, "Support")[:5]
        },
        # User 4: Assassin preference
        {
            "id": "test_user_004",
            "role": "Assassin",
            "difficulty": "Hard (7-8)",
            "playstyle": "High Damage Output",
            "ground_truth": get_champions_by_role(champions, "Assassin")[:5]
        },
        # User 5: Tank preference
        {
            "id": "test_user_005",
            "role": "Tank",
            "difficulty": "Easy (1-3)",
            "playstyle": "Tanky and Durable",
            "ground_truth": get_champions_by_role(champions, "Tank")[:5]
        },
        # User 6: Marksman preference
        {
            "id": "test_user_006",
            "role": "Marksman",
            "difficulty": "Medium (4-6)",
            "playstyle": "High Damage Output",
            "ground_truth": get_champions_by_role(champions, "Marksman")[:5]
        },
        # User 7: No preference
        {
            "id": "test_user_007",
            "role": "No Preference",
            "difficulty": "Easy (1-3)",
            "playstyle": "Balanced/Hybrid",
            "ground_truth": []  # Will be calculated based on difficulty
        },
        # User 8: Mage with high difficulty
        {
            "id": "test_user_008",
            "role": "Mage",
            "difficulty": "Very Hard (9-10)",
            "playstyle": "High Damage Output",
            "ground_truth": ["Azir", "Zoe", "Twisted Fate", "Orianna"]
        },
        # User 9: Fighter with medium difficulty
        {
            "id": "test_user_009",
            "role": "Fighter",
            "difficulty": "Medium (4-6)",
            "playstyle": "Balanced/Hybrid",
            "ground_truth": ["Garen", "Darius", "Jax", "Irelia"]
        },
        # User 10: Support with easy difficulty
        {
            "id": "test_user_010",
            "role": "Support",
            "difficulty": "Easy (1-3)",
            "playstyle": "Support Team",
            "ground_truth": ["Soraka", "Janna", "Sona", "Lulu"]
        }
    ]
    
    return test_data


def save_test_data_json(test_data: List[Dict[str, Any]], filename: str = "test_dataset.json"):
    """Save test data to JSON file"""
    output_path = Path(filename)
    with open(output_path, 'w') as f:
        json.dump(test_data, f, indent=2)
    print(f"✓ Saved test dataset to {output_path}")
    print(f"  Total users: {len(test_data)}")


def save_test_data_csv(test_data: List[Dict[str, Any]], filename: str = "test_dataset.csv"):
    """Save test data to CSV file"""
    output_path = Path(filename)
    
    with open(output_path, 'w', newline='') as f:
        fieldnames = ['id', 'role', 'difficulty', 'playstyle', 'ground_truth']
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        
        writer.writeheader()
        for user in test_data:
            # Convert ground_truth list to comma-separated string
            user_copy = user.copy()
            user_copy['ground_truth'] = ','.join(user['ground_truth'])
            writer.writerow(user_copy)
    
    print(f"✓ Saved test dataset to {output_path}")


def convert_existing_data_to_test_format(input_file: str = "user_data_backup.json",
                                         output_file: str = "test_dataset_from_backup.json"):
    """
    Convert existing user data to test format by adding ground truth
    
    Args:
        input_file: Path to existing user data
        output_file: Path to save converted test data
    """
    input_path = Path(input_file)
    if not input_path.exists():
        print(f"Input file not found: {input_file}")
        return
    
    with open(input_path, 'r') as f:
        existing_data = json.load(f)
    
    champions = load_champions()
    test_data = []
    
    for user in existing_data:
        # Extract the champion they were recommended
        recommended = user.get('recommended_champion', '')
        
        # Create ground truth based on their role preference
        role = user.get('role', user.get('1', 'No Preference'))
        ground_truth = get_champions_by_role(champions, role)[:5]
        
        # If they have a recommended champion, include it in ground truth
        if recommended and recommended not in ground_truth:
            ground_truth.append(recommended)
        
        test_user = {
            "id": user.get('id', f"user_{len(test_data)+1:03d}"),
            "role": role,
            "difficulty": user.get('difficulty', user.get('2', 'Medium (4-6)')),
            "playstyle": user.get('playstyle', user.get('3', 'Balanced/Hybrid')),
            "ground_truth": ground_truth
        }
        test_data.append(test_user)
    
    # Save converted data
    output_path = Path(output_file)
    with open(output_path, 'w') as f:
        json.dump(test_data, f, indent=2)
    
    print(f"✓ Converted {len(test_data)} users to test format")
    print(f"✓ Saved to {output_path}")


def main():
    """Main function"""
    print("="*60)
    print("TEST DATASET CREATOR")
    print("="*60)
    
    print("\nOptions:")
    print("1. Create sample test dataset (10 users)")
    print("2. Convert existing user_data_backup.json to test format")
    print("3. Both")
    
    choice = input("\nEnter choice (1/2/3): ").strip()
    
    if choice in ['1', '3']:
        print("\n[Creating sample test dataset...]")
        test_data = create_sample_test_data()
        save_test_data_json(test_data, "test_dataset_sample.json")
        save_test_data_csv(test_data, "test_dataset_sample.csv")
    
    if choice in ['2', '3']:
        print("\n[Converting existing data...]")
        convert_existing_data_to_test_format()
    
    print("\n" + "="*60)
    print("COMPLETE")
    print("="*60)
    print("\nNext steps:")
    print("1. Review the generated test dataset")
    print("2. Edit ground_truth values if needed")
    print("3. Run evaluation: python ml_evaluation_pipeline.py")


if __name__ == "__main__":
    main()
