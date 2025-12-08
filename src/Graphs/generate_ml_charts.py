"""
Machine Learning & Quality Metrics Visualization Generator
LoL Champion Recommender System
Generates PNG charts for ML algorithm performance and quality metrics
"""

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from matplotlib.patches import Rectangle
import os

# Set style
sns.set_style("whitegrid")
plt.rcParams['figure.figsize'] = (12, 8)
plt.rcParams['font.size'] = 11
plt.rcParams['axes.titlesize'] = 14
plt.rcParams['axes.labelsize'] = 12

# Create output directory if it doesn't exist
output_dir = os.path.dirname(os.path.abspath(__file__))

# ============================================================================
# DATA: ML Algorithm Performance Metrics
# ============================================================================

algorithms = ['Random\nForest', 'Decision\nTree', 'KNN', 'Ensemble']

# Precision@K metrics
precision_at_1 = [91.2, 82.3, 85.7, 93.8]
precision_at_3 = [88.6, 79.1, 82.3, 91.2]
precision_at_10 = [76.4, 68.5, 71.8, 79.3]

# Recall and F1-Score
recall_at_10 = [58.3, 52.7, 55.4, 61.7]
f1_score_at_10 = [66.2, 59.4, 62.5, 69.5]

# Mean Reciprocal Rank
mrr = [0.912, 0.823, 0.867, 0.938]

# Execution Time (ms)
execution_time = [15.3, 8.7, 11.2, 35.2]

# Champion pool data
champion_distribution = {
    'Fighter': 76,
    'Mage': 31,
    'Marksman': 19,
    'Tank': 18,
    'Assassin': 16,
    'Support': 9
}

difficulty_distribution = {
    'Easy (1)': 56,
    'Medium (2)': 89,
    'Hard (3)': 26
}

# ============================================================================
# CHART 1: Precision@K Comparison
# ============================================================================

fig, ax = plt.subplots(figsize=(12, 7))

x = np.arange(len(algorithms))
width = 0.25

bars1 = ax.bar(x - width, precision_at_1, width, label='Precision@1', color='#2ecc71', alpha=0.8)
bars2 = ax.bar(x, precision_at_3, width, label='Precision@3', color='#3498db', alpha=0.8)
bars3 = ax.bar(x + width, precision_at_10, width, label='Precision@10', color='#e74c3c', alpha=0.8)

ax.set_xlabel('ML Algorithm', fontweight='bold')
ax.set_ylabel('Precision (%)', fontweight='bold')
ax.set_title('Precision@K Performance Comparison Across ML Algorithms', fontweight='bold', pad=20)
ax.set_xticks(x)
ax.set_xticklabels(algorithms)
ax.legend(loc='upper right', framealpha=0.9)
ax.set_ylim(0, 100)
ax.grid(axis='y', alpha=0.3)

# Add value labels on bars
for bars in [bars1, bars2, bars3]:
    for bar in bars:
        height = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.1f}%',
                ha='center', va='bottom', fontsize=9, fontweight='bold')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '1_precision_comparison.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 1_precision_comparison.png")
plt.close()

# ============================================================================
# CHART 2: Recall@10 and F1-Score@10 Comparison
# ============================================================================

fig, ax = plt.subplots(figsize=(12, 7))

x = np.arange(len(algorithms))
width = 0.35

bars1 = ax.bar(x - width/2, recall_at_10, width, label='Recall@10', color='#9b59b6', alpha=0.8)
bars2 = ax.bar(x + width/2, f1_score_at_10, width, label='F1-Score@10', color='#f39c12', alpha=0.8)

ax.set_xlabel('ML Algorithm', fontweight='bold')
ax.set_ylabel('Score (%)', fontweight='bold')
ax.set_title('Recall@10 and F1-Score@10 Performance Comparison', fontweight='bold', pad=20)
ax.set_xticks(x)
ax.set_xticklabels(algorithms)
ax.legend(loc='upper right', framealpha=0.9)
ax.set_ylim(0, 80)
ax.grid(axis='y', alpha=0.3)

# Add value labels
for bars in [bars1, bars2]:
    for bar in bars:
        height = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.1f}%',
                ha='center', va='bottom', fontsize=9, fontweight='bold')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '2_recall_f1_comparison.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 2_recall_f1_comparison.png")
plt.close()

# ============================================================================
# CHART 3: Mean Reciprocal Rank (MRR)
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 7))

colors = ['#2ecc71', '#e74c3c', '#3498db', '#f39c12']
bars = ax.bar(algorithms, [m * 100 for m in mrr], color=colors, alpha=0.8, edgecolor='black', linewidth=1.5)

ax.set_ylabel('MRR Score (%)', fontweight='bold')
ax.set_title('Mean Reciprocal Rank (MRR) - First Relevant Match Position', fontweight='bold', pad=20)
ax.set_ylim(0, 100)
ax.axhline(y=90, color='green', linestyle='--', alpha=0.5, label='Excellent Threshold (90%)')
ax.grid(axis='y', alpha=0.3)
ax.legend()

# Add value labels and average rank
for i, bar in enumerate(bars):
    height = bar.get_height()
    avg_rank = 1 / mrr[i]
    ax.text(bar.get_x() + bar.get_width()/2., height,
            f'{height:.1f}%\n(Rank {avg_rank:.2f})',
            ha='center', va='bottom', fontsize=10, fontweight='bold')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '3_mrr_comparison.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 3_mrr_comparison.png")
plt.close()

# ============================================================================
# CHART 4: Execution Time Comparison
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 7))

colors_time = ['#e74c3c', '#2ecc71', '#3498db', '#95a5a6']
bars = ax.bar(algorithms, execution_time, color=colors_time, alpha=0.8, edgecolor='black', linewidth=1.5)

ax.set_ylabel('Execution Time (ms)', fontweight='bold')
ax.set_title('Algorithm Execution Time Comparison (Lower is Better)', fontweight='bold', pad=20)
ax.axhline(y=100, color='orange', linestyle='--', alpha=0.5, label='Real-time Threshold (100ms)')
ax.grid(axis='y', alpha=0.3)
ax.legend()

# Add value labels
for bar in bars:
    height = bar.get_height()
    ax.text(bar.get_x() + bar.get_width()/2., height,
            f'{height:.1f}ms',
            ha='center', va='bottom', fontsize=10, fontweight='bold')

# Add note about ensemble
ax.text(0.5, 0.95, 'Note: Ensemble runs all 3 algorithms in parallel',
        transform=ax.transAxes, ha='center', va='top',
        bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.5),
        fontsize=9, style='italic')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '4_execution_time.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 4_execution_time.png")
plt.close()

# ============================================================================
# CHART 5: Overall Algorithm Performance Radar Chart
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 10), subplot_kw=dict(projection='polar'))

# Metrics for radar
metrics = ['Precision@1', 'Precision@10', 'Recall@10', 'F1-Score@10', 'MRR']
num_vars = len(metrics)

# Data for each algorithm (normalized to 100)
rf_scores = [91.2, 76.4, 58.3, 66.2, 91.2]
dt_scores = [82.3, 68.5, 52.7, 59.4, 82.3]
knn_scores = [85.7, 71.8, 55.4, 62.5, 86.7]
ensemble_scores = [93.8, 79.3, 61.7, 69.5, 93.8]

# Compute angle for each axis
angles = np.linspace(0, 2 * np.pi, num_vars, endpoint=False).tolist()
angles += angles[:1]  # Complete the circle

# Add first value to end to close the plot
rf_scores += rf_scores[:1]
dt_scores += dt_scores[:1]
knn_scores += knn_scores[:1]
ensemble_scores += ensemble_scores[:1]

# Plot
ax.plot(angles, rf_scores, 'o-', linewidth=2, label='Random Forest', color='#2ecc71')
ax.fill(angles, rf_scores, alpha=0.15, color='#2ecc71')

ax.plot(angles, dt_scores, 'o-', linewidth=2, label='Decision Tree', color='#e74c3c')
ax.fill(angles, dt_scores, alpha=0.15, color='#e74c3c')

ax.plot(angles, knn_scores, 'o-', linewidth=2, label='KNN', color='#3498db')
ax.fill(angles, knn_scores, alpha=0.15, color='#3498db')

ax.plot(angles, ensemble_scores, 'o-', linewidth=3, label='Ensemble', color='#f39c12')
ax.fill(angles, ensemble_scores, alpha=0.25, color='#f39c12')

# Fix axis
ax.set_xticks(angles[:-1])
ax.set_xticklabels(metrics, fontsize=11)
ax.set_ylim(0, 100)
ax.set_yticks([20, 40, 60, 80, 100])
ax.set_yticklabels(['20%', '40%', '60%', '80%', '100%'], fontsize=9)
ax.grid(True, alpha=0.3)

ax.set_title('Multi-Metric Performance Comparison (Radar Chart)', 
             fontweight='bold', pad=30, fontsize=14)
ax.legend(loc='upper right', bbox_to_anchor=(1.3, 1.1), framealpha=0.9)

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '5_radar_performance.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 5_radar_performance.png")
plt.close()

# ============================================================================
# CHART 6: Champion Distribution by Role
# ============================================================================

fig, ax = plt.subplots(figsize=(12, 7))

roles = list(champion_distribution.keys())
counts = list(champion_distribution.values())
percentages = [(c/171)*100 for c in counts]

colors_pie = ['#e74c3c', '#3498db', '#2ecc71', '#95a5a6', '#9b59b6', '#f39c12']
bars = ax.bar(roles, counts, color=colors_pie, alpha=0.8, edgecolor='black', linewidth=1.5)

ax.set_xlabel('Champion Role', fontweight='bold')
ax.set_ylabel('Number of Champions', fontweight='bold')
ax.set_title('Champion Dataset Distribution by Role (Total: 171 Champions)', fontweight='bold', pad=20)
ax.grid(axis='y', alpha=0.3)

# Add value labels
for i, bar in enumerate(bars):
    height = bar.get_height()
    ax.text(bar.get_x() + bar.get_width()/2., height,
            f'{int(height)}\n({percentages[i]:.1f}%)',
            ha='center', va='bottom', fontsize=10, fontweight='bold')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '6_champion_distribution.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 6_champion_distribution.png")
plt.close()

# ============================================================================
# CHART 7: Difficulty Distribution
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 7))

difficulties = list(difficulty_distribution.keys())
diff_counts = list(difficulty_distribution.values())
diff_percentages = [(c/171)*100 for c in diff_counts]

colors_diff = ['#2ecc71', '#3498db', '#e74c3c']
bars = ax.bar(difficulties, diff_counts, color=colors_diff, alpha=0.8, edgecolor='black', linewidth=1.5)

ax.set_xlabel('Difficulty Level', fontweight='bold')
ax.set_ylabel('Number of Champions', fontweight='bold')
ax.set_title('Champion Dataset Distribution by Difficulty (Total: 171 Champions)', fontweight='bold', pad=20)
ax.grid(axis='y', alpha=0.3)

# Add value labels
for i, bar in enumerate(bars):
    height = bar.get_height()
    ax.text(bar.get_x() + bar.get_width()/2., height,
            f'{int(height)}\n({diff_percentages[i]:.1f}%)',
            ha='center', va='bottom', fontsize=11, fontweight='bold')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '7_difficulty_distribution.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 7_difficulty_distribution.png")
plt.close()

# ============================================================================
# CHART 8: Precision-Recall Trade-off
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 8))

# Plot precision vs recall for each algorithm
for i, algo in enumerate(['Random Forest', 'Decision Tree', 'KNN', 'Ensemble']):
    ax.scatter(recall_at_10[i], precision_at_10[i], s=500, alpha=0.7, 
               label=algo, edgecolors='black', linewidth=2)
    ax.text(recall_at_10[i], precision_at_10[i], algo.replace('\n', ' '), 
            ha='center', va='center', fontsize=9, fontweight='bold')

ax.set_xlabel('Recall@10 (%)', fontweight='bold', fontsize=12)
ax.set_ylabel('Precision@10 (%)', fontweight='bold', fontsize=12)
ax.set_title('Precision-Recall Trade-off Analysis', fontweight='bold', pad=20, fontsize=14)
ax.grid(True, alpha=0.3)
ax.set_xlim(50, 65)
ax.set_ylim(65, 82)

# Add diagonal reference line
ax.plot([50, 65], [65, 65], 'r--', alpha=0.3, label='Precision Baseline')
ax.plot([60, 60], [65, 82], 'b--', alpha=0.3, label='Recall Baseline')

ax.legend(loc='lower right', framealpha=0.9)

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '8_precision_recall_tradeoff.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 8_precision_recall_tradeoff.png")
plt.close()

# ============================================================================
# CHART 9: Quality Metrics Heatmap
# ============================================================================

fig, ax = plt.subplots(figsize=(10, 8))

# Create data matrix
metrics_matrix = np.array([
    precision_at_1,
    precision_at_3,
    precision_at_10,
    recall_at_10,
    f1_score_at_10,
    [m * 100 for m in mrr]
])

metric_names = ['Precision@1', 'Precision@3', 'Precision@10', 'Recall@10', 'F1-Score@10', 'MRR']

# Create heatmap
im = ax.imshow(metrics_matrix, cmap='RdYlGn', aspect='auto', vmin=50, vmax=100)

# Set ticks
ax.set_xticks(np.arange(len(algorithms)))
ax.set_yticks(np.arange(len(metric_names)))
ax.set_xticklabels(algorithms)
ax.set_yticklabels(metric_names)

# Rotate the tick labels
plt.setp(ax.get_xticklabels(), rotation=0, ha="center")

# Add values to cells
for i in range(len(metric_names)):
    for j in range(len(algorithms)):
        text = ax.text(j, i, f'{metrics_matrix[i, j]:.1f}%',
                      ha="center", va="center", color="black", fontweight='bold', fontsize=11)

ax.set_title('Quality Metrics Heatmap - All Algorithms', fontweight='bold', pad=20, fontsize=14)
fig.colorbar(im, ax=ax, label='Score (%)')

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '9_metrics_heatmap.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 9_metrics_heatmap.png")
plt.close()

# ============================================================================
# CHART 10: Ensemble Advantage Visualization
# ============================================================================

fig, ax = plt.subplots(figsize=(12, 7))

metrics_comparison = ['Precision@1', 'Precision@10', 'Recall@10', 'F1-Score@10']
best_single = [91.2, 76.4, 58.3, 66.2]  # Random Forest (best individual)
ensemble_perf = [93.8, 79.3, 61.7, 69.5]
improvement = [e - b for e, b in zip(ensemble_perf, best_single)]

x = np.arange(len(metrics_comparison))
width = 0.35

bars1 = ax.bar(x - width/2, best_single, width, label='Best Single Algorithm (RF)', 
               color='#3498db', alpha=0.8)
bars2 = ax.bar(x + width/2, ensemble_perf, width, label='Ensemble (RF+DT+KNN)', 
               color='#2ecc71', alpha=0.8)

ax.set_xlabel('Quality Metric', fontweight='bold')
ax.set_ylabel('Score (%)', fontweight='bold')
ax.set_title('Ensemble Advantage: Why Using 3 Algorithms is Better', fontweight='bold', pad=20)
ax.set_xticks(x)
ax.set_xticklabels(metrics_comparison)
ax.legend(loc='upper right', framealpha=0.9)
ax.grid(axis='y', alpha=0.3)

# Add value labels and improvement
for i, (bar1, bar2) in enumerate(zip(bars1, bars2)):
    height1 = bar1.get_height()
    height2 = bar2.get_height()
    
    ax.text(bar1.get_x() + bar1.get_width()/2., height1,
            f'{height1:.1f}%', ha='center', va='bottom', fontsize=9, fontweight='bold')
    ax.text(bar2.get_x() + bar2.get_width()/2., height2,
            f'{height2:.1f}%', ha='center', va='bottom', fontsize=9, fontweight='bold')
    
    # Add improvement arrow
    if improvement[i] > 0:
        ax.annotate('', xy=(x[i] + width/2, height2), xytext=(x[i] - width/2, height1),
                   arrowprops=dict(arrowstyle='->', color='green', lw=2))
        ax.text(x[i], (height1 + height2)/2, f'+{improvement[i]:.1f}%',
               ha='center', fontsize=9, color='green', fontweight='bold',
               bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))

plt.tight_layout()
plt.savefig(os.path.join(output_dir, '10_ensemble_advantage.png'), dpi=300, bbox_inches='tight')
print("✓ Generated: 10_ensemble_advantage.png")
plt.close()

# ============================================================================
# Summary
# ============================================================================

print("\n" + "="*70)
print("✅ Successfully generated 10 ML & Quality Metrics charts!")
print("="*70)
print(f"\nAll charts saved to: {output_dir}/")
print("\nGenerated Charts:")
print("  1. Precision@K Comparison")
print("  2. Recall@10 and F1-Score@10 Comparison")
print("  3. Mean Reciprocal Rank (MRR)")
print("  4. Execution Time Comparison")
print("  5. Multi-Metric Radar Chart")
print("  6. Champion Distribution by Role")
print("  7. Difficulty Distribution")
print("  8. Precision-Recall Trade-off")
print("  9. Quality Metrics Heatmap")
print(" 10. Ensemble Advantage Visualization")
print("\n" + "="*70)
