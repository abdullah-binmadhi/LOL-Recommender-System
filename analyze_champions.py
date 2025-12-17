import json
import os
from collections import Counter

def analyze_data():
    file_path = 'src/data/champions.json'
    if not os.path.exists(file_path):
        print(f"Error: {file_path} not found.")
        return

    with open(file_path, 'r') as f:
        data = json.load(f)

    champions = data.get('champions', [])
    total_champions = len(champions)

    # 1. Hero Type Analysis
    hero_types = [c.get('herotype', 'Unknown') for c in champions]
    hero_counts = Counter(hero_types)
    
    # 2. Range Type Analysis
    range_types = [c.get('range_type', 'Unknown') for c in champions]
    range_counts = Counter(range_types)

    # 3. Difficulty Analysis
    difficulties = [c.get('difficulty', 0) for c in champions]
    difficulty_counts = Counter(difficulties)

    # 4. Release Year (Hardcoded from analytics.html)
    # releases = [40, 24, 24, 19, 6, 6, 6, 6, 6, 4, 6, 5, 4, 6, 5, 3]
    # years = ['2009', '2010', '2011', '2012', '2013', '2014', '2015', '2016', '2017', '2018', '2019', '2020', '2021', '2022', '2023', '2024']
    
    print("Analysis Results:")
    print(f"Total Champions: {total_champions}")
    print("\nHero Types:")
    for type, count in hero_counts.most_common():
        print(f"  {type}: {count} ({count/total_champions*100:.1f}%)")
        
    print("\nRange Types:")
    for type, count in range_counts.most_common():
        print(f"  {type}: {count} ({count/total_champions*100:.1f}%)")

    print("\nDifficulty Levels:")
    for level, count in sorted(difficulty_counts.items()):
        print(f"  Level {level}: {count} ({count/total_champions*100:.1f}%)")

if __name__ == "__main__":
    analyze_data()
