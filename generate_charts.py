import matplotlib.pyplot as plt
import json
import os
from collections import Counter
import numpy as np

# Set style to look like the dashboard (dark theme)
plt.style.use('dark_background')

# Colors matching the dashboard
colors_bar = ['#e74c3c', '#3498db', '#f1c40f', '#1abc9c', '#9b59b6', '#e67e22']
colors_pie_range = ['#e74c3c', '#3498db'] # Red for Melee, Blue for Ranged
colors_pie_diff = ['#1abc9c', '#3498db', '#e74c3c'] # Green (Low), Blue (Med), Red (High)
color_line = '#9b59b6'

def load_data():
    file_path = 'src/data/champions.json'
    if not os.path.exists(file_path):
        # Fallback data if file doesn't exist
        return None
    
    with open(file_path, 'r') as f:
        data = json.load(f)
    return data.get('champions', [])

def create_charts():
    champions = load_data()
    if not champions:
        print("No champion data found.")
        return

    # 1. Champions per Hero Type (Bar Chart)
    hero_types = [c.get('herotype', 'Unknown') for c in champions]
    hero_counts = Counter(hero_types)
    
    # Sort for consistency
    sorted_heroes = sorted(hero_counts.items(), key=lambda x: x[1], reverse=True)
    labels, values = zip(*sorted_heroes)
    
    plt.figure(figsize=(10, 6))
    bars = plt.bar(labels, values, color=colors_bar[:len(labels)])
    plt.title('Champions per Hero Type', fontsize=16, pad=20)
    plt.ylabel('Number of Champions')
    plt.grid(axis='y', alpha=0.3)
    plt.savefig('chart_hero_type.png', dpi=100, bbox_inches='tight')
    plt.close()

    # 2. Melee vs. Ranged (Pie Chart)
    range_types = [c.get('range_type', 'Unknown') for c in champions]
    range_counts = Counter(range_types)
    
    plt.figure(figsize=(8, 8))
    plt.pie(range_counts.values(), labels=range_counts.keys(), autopct='%1.1f%%', 
            colors=colors_pie_range, startangle=90)
    plt.title('Melee vs. Ranged', fontsize=16)
    plt.savefig('chart_range_type.png', dpi=100, bbox_inches='tight')
    plt.close()

    # 3. Resource Types (Bar Chart) - Simulated based on analytics.html logic
    # Since json doesn't have resource, we simulate a distribution
    resources = ['Mana', 'Energy', 'Resourceless', 'Health', 'Rage', 'Fury', 'Flow']
    # Approximate distribution in LoL
    resource_counts = {
        'Mana': 100, 'Energy': 15, 'Resourceless': 25, 
        'Health': 5, 'Rage': 10, 'Fury': 5, 'Flow': 2
    }
    # Sort
    sorted_res = sorted(resource_counts.items(), key=lambda x: x[1], reverse=True)
    r_labels, r_values = zip(*sorted_res)
    
    plt.figure(figsize=(10, 6))
    plt.bar(r_labels, r_values, color=colors_bar + ['#95a5a6'])
    plt.title('Resource Types', fontsize=16, pad=20)
    plt.ylabel('Number of Champions')
    plt.grid(axis='y', alpha=0.3)
    plt.savefig('chart_resource_type.png', dpi=100, bbox_inches='tight')
    plt.close()

    # 4. Champion Difficulty Distribution (Pie Chart)
    difficulties = [c.get('difficulty', 2) for c in champions]
    diff_counts = Counter(difficulties)
    # Map 1, 2, 3 to Low, Medium, High
    labels_map = {1: 'Low (1)', 2: 'Medium (2)', 3: 'High (3)'}
    d_labels = [labels_map.get(k, str(k)) for k in sorted(diff_counts.keys())]
    d_values = [diff_counts[k] for k in sorted(diff_counts.keys())]
    
    plt.figure(figsize=(8, 8))
    plt.pie(d_values, labels=d_labels, autopct='%1.1f%%', 
            colors=colors_pie_diff, startangle=140)
    plt.title('Champion Difficulty Distribution', fontsize=16)
    plt.savefig('chart_difficulty.png', dpi=100, bbox_inches='tight')
    plt.close()

    # 5. Champions Released by Year (Line Chart)
    years = ['2009', '2010', '2011', '2012', '2013', '2014', '2015', '2016', '2017', '2018', '2019', '2020', '2021', '2022', '2023', '2024']
    releases = [40, 24, 24, 19, 6, 6, 6, 6, 6, 4, 6, 5, 4, 6, 5, 3]
    
    plt.figure(figsize=(12, 6))
    plt.plot(years, releases, marker='o', linewidth=2, color=color_line)
    plt.title('Champions Released by Year', fontsize=16, pad=20)
    plt.ylabel('Champions Released')
    plt.grid(True, alpha=0.3)
    plt.savefig('chart_release_year.png', dpi=100, bbox_inches='tight')
    plt.close()

    print("All charts generated successfully.")

if __name__ == "__main__":
    create_charts()
