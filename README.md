# League of Legends Champion Recommender

<p align="center">
  <a href="https://abdullah-binmadhi.github.io/LOL-Recommender-System/">
    <img src="https://img.shields.io/badge/🎮_Live_Demo-Try_Now!-brightgreen?style=for-the-badge&logo=googlechrome" alt="Live Demo">
  </a>
  <a href="https://github.com/abdullah-binmadhi/LOL-Recommender-System/stargazers">
    <img src="https://img.shields.io/github/stars/abdullah-binmadhi/LOL-Recommender-System?style=for-the-badge&logo=github" alt="GitHub Stars">
  </a>
  <a href="https://github.com/abdullah-binmadhi/LOL-Recommender-System/issues">
    <img src="https://img.shields.io/github/issues/abdullah-binmadhi/LOL-Recommender-System?style=for-the-badge&logo=github" alt="GitHub Issues">
  </a>
</p>

<p align="center">
  <img src="https://opengraph.githubassets.com/1/abdullah-binmadhi/LOL-Recommender-System" alt="Project Banner" width="100%">
</p>

## Discover Your Perfect Champion

An **AI-powered champion recommendation system** that uses **3 machine learning algorithms** to suggest the perfect League of Legends champion based on your playstyle preferences.

> **No more guessing!** Get personalized champion recommendations backed by data science.

---

## Try It Live

### **[Launch Champion Recommender](https://abdullah-binmadhi.github.io/LOL-Recommender-System/)**

---

## Key Features

<div align="center">

| Feature | Description |
|--------|-------------|
| **Enhanced Questionnaire** | 10 questions (5 game-related + 5 psychological/behavioral) for deeper insights |
| **Unified ML System** | All 3 algorithms (Random Forest, Decision Tree, KNN) evaluate every champion |
| **Top 5 Recommendations** | Get 5 unique champions, each scored by all 3 algorithms |
| **Transparent Scoring** | See detailed breakdowns of how each algorithm calculated scores |
| **Smart Diversity** | Max 2 champions per role ensures variety in recommendations |
| **Visual Score Comparison** | Color-coded bars show algorithm scores side-by-side |
| **Expandable Details** | Click to see contributing factors, matched criteria, and penalties |
| **Data Export** | Export your results as CSV with Ctrl+Shift+E |
| **Mobile Friendly** | Works perfectly on desktop, tablet, and mobile devices |

</div>

---

## How It Works

1. **Answer 10 Insightful Questions** - Tell us about your preferred role, position, difficulty level, playstyle, range, and psychological preferences
2. **Enhanced Registration** - Provide additional personal information for better analysis
3. **AI Analysis** - Our 3 ML algorithms analyze your comprehensive preferences and score ALL 150+ champions
4. **Get Top 5 Champions** - Receive 5 personalized champion recommendations, each evaluated by all 3 algorithms
5. **Explore Detailed Scores** - See transparent scoring breakdowns showing how each algorithm evaluated each champion
6. **View Alternatives** - Explore other great champion options that match your playstyle

---

## Unified ML Recommendation System

### Overview

The system uses a **unified scoring approach** where all 3 machine learning algorithms evaluate every champion in the database, providing transparent and comprehensive recommendations.

### How It Works

#### 1. **Comprehensive Scoring**
Each of the 3 ML algorithms scores all 150+ champions based on your preferences:
- **Random Forest**: Weighted feature scoring with playstyle bonuses
- **Decision Tree**: Hierarchical decision-based scoring
- **K-Nearest Neighbors**: Distance-based similarity scoring

#### 2. **Score Aggregation**
The system aggregates scores from all algorithms for each champion:
```javascript
Champion Score = {
  randomForest: 85.3%,
  decisionTree: 78.2%,
  knn: 82.1%,
  average: 81.9%,
  weighted: 82.5%
}
```

#### 3. **Top 5 Selection**
- Champions are ranked by their aggregate scores
- Top 5 unique champions are selected
- Diversity filter ensures variety (max 2 per role)
- All recommendations are backed by data from all 3 algorithms

#### 4. **Transparent Display**
Each recommended champion shows:
- **Aggregate Score**: Overall match percentage
- **Individual Algorithm Scores**: See how each ML algorithm rated the champion
- **Visual Score Bars**: Color-coded bars for easy comparison
- **Calculation Details**: Expandable breakdown of scoring factors

### Algorithm Scoring Methods

#### Random Forest Scoring
- **Role Match**: 40 points (exact role match)
- **Position Match**: 30 points (lane compatibility)
- **Difficulty Match**: 20 points (skill level alignment)
- **Damage Match**: 15 points (damage output preference)
- **Toughness Match**: 15 points (survivability preference)
- **Playstyle Bonus**: 20 points (aggressive/defensive/supportive)
- **Psychological Match**: 30 points (behavioral preferences)
- **Total**: Normalized to 0-100%

#### Decision Tree Scoring
- **Hierarchical Evaluation**: Top-down decision logic
- **Level 1**: Role match (50 points)
- **Level 2**: Position match (40 points, if role matches)
- **Level 3**: Difficulty match (30 points, if position matches)
- **Level 4**: Attribute matches (20 points each)
- **Bonuses**: Psychological preferences (25 points each)
- **Penalties**: Applied for mismatches
- **Total**: Normalized to 0-100%

#### K-Nearest Neighbors Scoring
- **Distance-Based**: Lower distance = higher score
- **Role Distance**: 8 points penalty for mismatch
- **Position Distance**: 6 points penalty for mismatch
- **Numerical Distances**: Weighted differences in attributes
  - Difficulty: 0.5 weight
  - Damage: 0.3 weight
  - Toughness: 0.3 weight
- **Psychological Distances**: 2 points each
- **Total**: Distance converted to 0-100% score (inverse)

### Example Output

```
Top 5 Recommended Champions:

1. Ahri (81.9% Overall Match)
   🌳 Random Forest: 85.3% ████████████████████
   🎯 Decision Tree: 78.2% ███████████████
   ⚡ KNN: 82.1% ████████████████
   
2. Lux (80.1% Overall Match)
   🌳 Random Forest: 78.2% ███████████████
   🎯 Decision Tree: 84.1% ████████████████████
   ⚡ KNN: 77.8% ███████████████

... (3 more champions)
```

### Benefits

✅ **More Options**: 5 champions instead of 3  
✅ **Transparency**: See exactly how each algorithm scored each champion  
✅ **Confidence**: Multiple algorithms agreeing increases recommendation quality  
✅ **Diversity**: Variety in roles and playstyles  
✅ **Explainability**: Detailed breakdowns of scoring factors  
✅ **Accuracy**: All 150+ champions evaluated, not just top picks from each algorithm

---

## Project Structure

```
├── src/                    # Source code
│   ├── index.html         # Main application
│   ├── data/              # Champion data files
│   └── assets/            # Images and other assets
├── PYTHON_TESTING/        # Backend and data management
│   ├── data_server.py     # Flask server for data storage
│   ├── user_data_manager.py # CSV management
│   └── champion_recommender_users.csv # Data file
├── docs/                  # Documentation
├── .github/               # GitHub configurations
└── README.md              # This file
```

For detailed documentation about the project structure, see [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) and [ORGANIZATION_SUMMARY.md](ORGANIZATION_SUMMARY.md).

---

## Deployment Options

### GitHub Pages (Current)
The project is currently deployed on GitHub Pages at: https://abdullah-binmadhi.github.io/LOL-Recommender-System/

### Vercel Deployment
This project is also compatible with Vercel deployment. To deploy on Vercel:

1. Fork this repository
2. Connect your GitHub repository to Vercel
3. Vercel will automatically detect the configuration and deploy the site

The project includes:
- `vercel.json` for routing configuration
- `package.json` for deployment metadata

No build step is required as this is a static site.

---

## Next.js Development Setup

This project also includes a Next.js application for enhanced functionality:

1. Navigate to the project directory
2. Install dependencies with `npm install`
3. Start the development server with `npm run dev`
4. Access the application at http://localhost:3000

See [NEXTJS_README.md](NEXTJS_README.md) for detailed instructions.

---

## Built With

<div align="center">

| Technology | Purpose |
|------------|---------|
| **HTML5** | Structure and content |
| **CSS3** | Styling and animations |
| **JavaScript (ES6+)** | Interactivity and logic |
| **Custom ML Algorithms** | Champion recommendation engine |
| **LocalStorage** | Client-side data persistence |
| **GitHub Pages** | Hosting and deployment |

</div>

---

## License

This project is for **educational purposes**. League of Legends is a trademark of Riot Games.

<p align="center">
  <img src="https://img.shields.io/badge/Made_With_❤️_For-LoL_Community-blue?style=for-the-badge" alt="Made for LoL Community">


