# League of Legends Champion Recommender - Project Overview

## Project Description

A **single-page web application** that recommends League of Legends champions to players using **three machine learning algorithms** working in parallel. The application analyzes user preferences through a 10-question questionnaire covering both gameplay style and psychological preferences, then recommends the top 5 champions tailored to the player's profile.

**Key Features:**

- 🤖 **Triple ML Algorithm System**: Random Forest, Decision Tree, and K-Nearest Neighbors (KNN) working together
- 📊 **Transparent Scoring**: Every recommendation shows individual scores from all 3 algorithms
- 🎮 **170+ Champions**: Complete database with detailed attributes (damage, toughness, control, mobility, utility)
- 📈 **Real-time Analytics**: Built-in analytics dashboard with evaluation metrics (Precision@K, MRR)
- 💾 **Client-side Storage**: All user data stored locally using localStorage
- 🚀 **Zero Dependencies**: Pure HTML/CSS/JavaScript - no frameworks, no build tools
- 📱 **Responsive Design**: Works seamlessly on desktop and mobile devices

**Live Application:** <https://abdullah-binmadhi.github.io/LOL-Recommender-System/>

---

## Tech Stack

### Core Technologies

- **HTML5**: Structure and content
- **CSS3**: Styling with custom properties, flexbox, and grid
- **Vanilla JavaScript (ES6+)**: All application logic, ML algorithms, and data processing

### Machine Learning

- **Custom ML Implementations**:
- **Custom ML Implementations**:
  - `SimpleRandomForest`: Weighted feature scoring with ensemble approach
  - `SimpleDecisionTree`: Hierarchical decision-based champion scoring
  - `SimpleKNN`: Distance-based similarity matching (k=5)
- **Score Aggregation**: Weighted combination (RF: 40%, DT: 30%, KNN: 30%)
- **Evaluation Metrics**: Precision@K, Mean Reciprocal Rank (MRR)

### AI Integration

- **Google Gemini AI** (Gemini 2.5 Flash): Generates comprehensive analysis reports of recommendations

### Data Storage

- **localStorage**: Client-side persistent storage for user data
- **JSON**: Champion database and questionnaire structure

### Deployment

- **GitHub Pages**: Primary hosting platform
- **Vercel**: Alternative hosting (configured)
- **Custom Domain Support**: CNAME ready

### Development Tools

- **No Build System**: Direct file editing and browser reload
- **Isolated Testing**: 23+ HTML test files in `/src/analytics/`
- **Browser DevTools**: Primary debugging interface

---

## Project Structure

```text
LOL-Recommender-System/
│
├── index.html                          # Root redirect to src/index.html
├── README.md                           # Project overview and quick start
├── USAGE_GUIDE.md                      # Detailed user guide
├── PROJECT_STRUCTURE.md                # Technical structure documentation
├── PROJECT_OVERVIEW.md                 # This file
├── AI_REPORT_IMPLEMENTATION.md         # Gemini AI integration docs
│
├── _config.yml                         # Jekyll configuration for GitHub Pages
├── vercel.json                         # Vercel deployment configuration
├── package.json                        # Project metadata (no actual npm dependencies)
│
├── src/                                # Main application source
│   ├── index.html                      # THE MAIN APP (~8,260 lines)
│   │                                   # Contains: HTML + CSS + JavaScript + ML algorithms
│   ├── README.md                       # Source directory documentation
│   ├── saveRecommendation.js           # Legacy file (functionality moved to index.html)
│   │
│   ├── data/                           # Application data files
│   │   ├── champions.json              # Champion database (~170 champions)
│   │   └── questions.json              # Questionnaire structure (10 questions)
│   │
│   ├── analytics/                      # Testing & analytics dashboard
│   │   ├── analytics.html              # Main analytics dashboard
│   │   ├── simple-analytics.html       # Simplified analytics view
│   │   ├── debug-analytics.html        # Debug-focused analytics
│   │   ├── README.md                   # Analytics documentation
│   │   │
│   │   ├── test_random_forest_predictall.html       # RF algorithm tests
│   │   ├── test_decision_tree_predictall.html       # DT algorithm tests
│   │   ├── test_knn_predictall.html                 # KNN algorithm tests
│   │   ├── test_score_aggregator.html               # Score aggregation tests
│   │   ├── test_integration_tests.html              # Full workflow tests
│   │   ├── test_evaluation_metrics.html             # Precision@K, MRR tests
│   │   ├── test_evaluation_metrics_comprehensive.html
│   │   ├── test_display_unified_recommendations.html
│   │   ├── test_calculation_details.html
│   │   └── ... (23+ test files total)
│   │
│   ├── assets/                         # Static assets (if needed)
│   ├── components/                     # Component-related files (minimal usage)
│   └── styles/                         # External stylesheets (if separated)
│
├── docs/                               # Current project documentation
│   ├── README.md                       # Documentation index
│   ├── api_documentation.md            # API and function reference
│   ├── setup_guide.md                  # Development setup instructions
│   ├── user_guide.md                   # End-user instructions
│   ├── deployment_guide.md             # Deployment procedures
│   ├── troubleshooting_guide.md        # Common issues and solutions
│   └── repository_structure.md         # Repository organization
│
├── docs-archive/                       # Historical implementation documents
│   ├── README.md                       # Archive index
│   ├── FINAL_IMPLEMENTATION_REPORT.md  # Complete implementation summary
│   ├── FINAL_ORGANIZATION_SUMMARY.md   # Project organization details
│   ├── PROJECT_COMPLETION_SUMMARY.md   # Final completion report
│   ├── UNIFIED_ML_DESIGN_COMPLETE.md   # ML system design documentation
│   ├── ML_EVALUATION_SUMMARY.md        # Evaluation metrics implementation
│   ├── SCORE_AGGREGATOR_IMPLEMENTATION.md
│   ├── KNN_IMPLEMENTATION_SUMMARY.md
│   ├── DEPLOYMENT_SUMMARY.md
│   ├── TEST_RESULTS_SUMMARY.md
│   ├── TASK_*_SUMMARY.md               # Individual task summaries (Tasks 5-16)
│   ├── test_dataset_sample.csv         # Sample test data
│   ├── test_dataset_sample.json
│   ├── LoL_champion_data.csv           # Original champion data
│   └── ... (30+ historical documents)
│
├── data/                               # Root-level data backup
│   └── champions.json                  # Champion database (duplicate)
│
├── public/                             # Public assets (alternative structure)
│   └── data/
│       └── champions.json              # Champion database (duplicate)
│
├── components/                         # React-like components (unused in main app)
│   ├── Layout.js
│   └── Navigation.js
│
├── lib/                                # Library files (if needed)
└── pages/                              # Additional pages (if needed)
```

---

## Key Architecture Components

### 1. Monolithic Main Application (`src/index.html`)

The entire application is self-contained in a single HTML file (~8,260 lines):

- **Lines 1-500**: HTML structure and questionnaire UI
- **Lines 500-2000**: CSS styling with custom properties
- **Lines 2000-2580**: Core JavaScript application logic
- **Lines 2580-3056**: Champion database and data structures
- **Lines 3057-3322**: `SimpleRandomForest` ML algorithm
- **Lines 3323-3721**: `SimpleDecisionTree` ML algorithm
- **Lines 3722-3945**: `SimpleKNN` ML algorithm
- **Lines 3946-4095**: `ScoreAggregator` for combining scores
- **Lines 4095-4790**: Results display and UI rendering
- **Lines 4790-5000+**: Gemini AI integration and reporting

### 2. Data Structures

#### Champion Object

```javascript
{
  "Ahri": {
    role: "Mage",
    positions: ["Mid"],
    difficulty: 5,
    damage: 7,
    toughness: 3,
    control: 6,
    mobility: 8,
    utility: 4,
    // ... additional attributes
  }
}
```

#### ML Results Object

```javascript
mlResults = {
  scores: {
    randomForest: { "Ahri": 85.3, "Zed": 78.2, ... },
    decisionTree: { "Ahri": 82.1, "Zed": 75.8, ... },
    knn: { "Ahri": 88.4, "Zed": 81.3, ... }
  },
  aggregated: { "Ahri": 85.6, "Zed": 78.4, ... },
  top5: [
    {
      championName: "Ahri",
      rfScore: 85.3,
      dtScore: 82.1,
      knnScore: 88.4,
      averageScore: 85.3,
      weightedScore: 85.6,
      // ... full champion data
    },
    // ... 4 more champions
  ]
}
```

### 3. ML Algorithm Flow

```text
User Answers (10 questions)
         ↓
Feature Extraction (damage pref, toughness pref, etc.)
         ↓
    ┌────┴────┬────────────┬────────────┐
    ↓         ↓            ↓            ↓
Random    Decision      KNN      (All run in parallel)
Forest      Tree                  
    ↓         ↓            ↓
  Score     Score        Score    (All champions scored 0-100)
    └────┬────┴────────────┘
         ↓
  Score Aggregator (weighted combination)
         ↓
  Diversity Filter (max 2 per role)
         ↓
   Top 5 Champions with transparent scoring
```

### 4. Testing Architecture

- **23+ Isolated Test Files**: Each tests specific functionality
- **No Test Runner**: Open HTML files directly in browser
- **Mock Data**: Each test file includes sample champions and user answers
- **Console Logging**: All test results logged to browser console

---

## Data Flow

1. **User Input**: 10-question questionnaire (5 gameplay + 5 psychological)
2. **Feature Extraction**: Answers mapped to numerical features
3. **ML Processing**: All 3 algorithms score all ~170 champions
4. **Score Aggregation**: Weighted combination (RF: 40%, DT: 30%, KNN: 30%)
5. **Diversity Filtering**: Ensure varied champion roles in top 5
6. **Results Display**: Show top 5 with transparent scoring breakdown
7. **AI Report Generation**: Optional Gemini AI analysis
8. **Data Export**: CSV export via Ctrl+Shift+E (localStorage → CSV)

---

## Development Workflow

### Local Development

```bash
# Option 1: Direct file open
open src/index.html

# Option 2: Local HTTP server (recommended)
python -m http.server 8080
# Visit: http://localhost:8080/src/
```

### Testing

```bash
# Open specific test file
open src/analytics/test_random_forest_predictall.html

# Open main analytics dashboard
open src/analytics/analytics.html
```

### Deployment Process

```bash
# Push to GitHub (auto-deploys to GitHub Pages)
git add .
git commit -m "Update application"
git push origin main

# Live within 2-3 minutes at:
# https://abdullah-binmadhi.github.io/LOL-Recommender-System/
```

---

## Key Design Decisions

1. **Monolithic Architecture**: Entire app in one file for maximum portability and zero dependencies
2. **No Build Tools**: Instant development cycle - edit and refresh browser
3. **Triple ML System**: Combines strengths of different algorithms for better recommendations
4. **Transparent Scoring**: Users see exactly how each algorithm scored each champion
5. **Client-side Only**: No backend required - all processing in browser
6. **localStorage Persistence**: User data persists across sessions without database
7. **Diversity Filter**: Prevents all recommendations from same role/position

---

## Performance Characteristics

- **Load Time**: < 2 seconds on modern browsers
- **ML Processing**: ~100-300ms for all 3 algorithms on ~170 champions
- **Memory Usage**: ~15-25 MB (champion database + ML results)
- **Storage**: ~50-100 KB localStorage per user session
- **Browser Compatibility**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+

---

## Future Enhancement Opportunities

- [ ] Add more ML algorithms (SVM, Neural Network)
- [ ] Implement A/B testing for algorithm weights
- [ ] Add user feedback loop for recommendation quality
- [ ] Create mobile-optimized questionnaire UI
- [ ] Add champion synergy analysis (team composition)
- [ ] Implement user preference learning over time
- [ ] Add multilingual support
- [ ] Create backend for cross-device sync

---

## License & Credits

- **License**: MIT (if applicable)
- **Champion Data**: Riot Games (League of Legends)
- **ML Algorithms**: Custom implementations
- **AI Integration**: Google Gemini AI

---

## Contact & Support

For issues, questions, or contributions, please refer to the GitHub repository:
<https://github.com/abdullah-binmadhi/LOL-Recommender-System>
