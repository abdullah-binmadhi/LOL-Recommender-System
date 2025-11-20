# GitHub Copilot Instructions - LoL Champion Recommender

## Project Overview
A **single-page web application** that recommends League of Legends champions using **3 ML algorithms** (Random Forest, Decision Tree, KNN). The entire app is **self-contained in `src/index.html`** (~8,260 lines) - no build tools, no frameworks, pure HTML/CSS/JavaScript.

**Live:** https://abdullah-binmadhi.github.io/LOL-Recommender-System/

## Critical Architecture Understanding

### The Monolithic Design
- **`src/index.html`** contains EVERYTHING: HTML structure, CSS styles, JavaScript logic, and all 3 ML algorithm implementations
- This is intentional - the app runs standalone in any browser without dependencies
- DO NOT suggest splitting into modules/components unless explicitly requested
- When editing, search for specific class/function names to locate code sections

### Data Flow
1. User answers 10 questions (5 game-related, 5 psychological) → stored in `userAnswers` object
2. Three ML algorithms each score ALL ~170 champions independently:
   - `SimpleRandomForest` (lines ~3057+): Weighted feature scoring
   - `SimpleDecisionTree` (lines ~3323+): Hierarchical decision-based scoring  
   - `SimpleKNN` (lines ~3722+): Distance-based similarity scoring
3. `ScoreAggregator` (lines ~3946+) combines scores from all 3 algorithms
4. Top 5 champions selected with diversity filter (max 2 per role)
5. Results displayed with transparent scoring breakdown per algorithm

### Global Data Structures
```javascript
// Champions database - loaded inline around line 2582
const allChampions = {
  "Ahri": { role: "Mage", positions: ["Mid"], difficulty: 5, damage: 7, toughness: 3, ... },
  // ~170 champions total
}

// ML results structure - populated by runAllAlgorithms()
mlResults = {
  scores: { randomForest: {...}, decisionTree: {...}, knn: {...} },
  aggregated: {...},  // Combined scores for all champions
  top5: [...]         // Top 5 recommendations with full details
}
```

## Core Development Workflows

### Testing Strategy
The `/src/analytics/` directory contains **23+ HTML test files** for isolated testing:
- `test_random_forest_predictall.html` - Test RF algorithm in isolation
- `test_integration_tests.html` - Full workflow tests
- `test_evaluation_metrics.html` - Precision@K and MRR metrics
- Open test files directly in browser - no test runner needed

**Creating Tests:** Copy an existing test file, modify mock data, open in browser

### Making ML Algorithm Changes
1. Locate algorithm class (search for `class SimpleRandomForest`)
2. Each algorithm MUST implement:
   - `predictAll(features)` - Returns scores for ALL champions
   - `calculateChampionScore(features, champion)` - Core scoring logic
   - `normalizeScore(rawScore)` - Converts to 0-100 scale
3. Test in isolation using analytics test files before modifying main app
4. Score normalization ranges:
   - Random Forest: 0-170 raw → 0-100 normalized
   - Decision Tree: 0-210 raw → 0-100 normalized
   - KNN: Distance-based → inverse normalized to 0-100

### Data Export & Storage
- User data stored in `localStorage` under key `'championRecommenderUsers'`
- Export triggered via **Ctrl+Shift+E** keyboard shortcut
- Format: CSV with 25+ columns including all answers + recommendations
- NO backend required - all data handling is client-side

### Adding/Modifying Questions
1. Edit `src/data/questions.json` - structured with id, text, options, weight, feature_mapping
2. Update question rendering logic in `src/index.html` (search for `loadQuestions()`)
3. Ensure `userAnswers` object captures new question responses
4. Update ML algorithm scoring if new features are added

## Project-Specific Conventions

### Naming Patterns
- ML Classes: `SimpleRandomForest`, `SimpleDecisionTree`, `SimpleKNN` (not "Random Forest Classifier")
- Champion properties: `championName`, `damage`, `toughness`, `control`, `mobility`, `utility`
- Scores: Always 0-100 scale after normalization, stored as floats (e.g., `85.3`)
- Functions: camelCase (e.g., `calculateChampionScore`, `displayUnifiedRecommendations`)

### Scoring Transparency
Every recommendation must show:
- Individual scores from all 3 algorithms (RF, DT, KNN)
- Average score (arithmetic mean)
- Weighted score (RF: 40%, DT: 30%, KNN: 30%)
- Visual score bars with color coding
- Expandable "Details" showing contributing factors

### File Organization
```
src/
├── index.html              # THE MAIN APP (~8,260 lines)
├── data/
│   ├── champions.json      # Champion database (external JSON)
│   └── questions.json      # Questionnaire structure
├── analytics/              # Isolated test files (23+ HTML files)
│   ├── test_*.html         # Unit/integration tests
│   └── analytics.html      # Main analytics dashboard
└── assets/                 # Images (if needed)

docs-archive/               # Historical implementation docs - READ ONLY
```

## Common Pitfalls

1. **Don't assume there's a build step** - Changes to `src/index.html` are immediately live
2. **Don't use npm/imports** - This is vanilla JavaScript, no module system
3. **localStorage is the database** - No backend API, no server-side persistence
4. **All 3 algorithms must score identically** - If you change champion data, test ALL algorithms
5. **Diversity filter is critical** - Top 5 must have max 2 champions per role
6. **Questions load from JSON** - Don't hardcode questions in HTML

## Key Integration Points

### Google Gemini AI Integration
- API key embedded for AI report generation (search for `generateAIReport()` around line 4790)
- Model: Gemini 2.5 Flash
- Used for: Generating comprehensive analysis reports of recommendations
- Handles errors gracefully with fallback messages

### GitHub Pages Deployment
- Entry point: `/index.html` (root) redirects to `/src/index.html`
- Configuration: `_config.yml`, `vercel.json` (alternative hosting)
- No build/deploy step - push to `main` branch auto-deploys
- Live URL updates within minutes of push

## Quick Reference

### Finding Code Sections
- ML Algorithms: Lines 3057-3945
- Score Aggregation: Lines 3946-4095  
- Results Display: Search for `displayUnifiedRecommendations`
- Evaluation Metrics: Search for `class EvaluationMetrics`
- Question Loading: Search for `loadQuestions()`
- Data Export: Search for `exportToCSV` or `Ctrl+Shift+E`

### Running the App Locally
```bash
# Option 1: Direct file open
open src/index.html

# Option 2: Local server (recommended)
python -m http.server 8080
# Then visit: http://localhost:8080/src/
```

### Debugging Tips
- Open browser console - all ML execution is logged
- Check `mlResults` global variable after questionnaire completion
- Use analytics test files to isolate issues
- Validation errors displayed inline with red borders
- Check `localStorage` for persisted user data

## When Making Changes

1. **Read context first** - Find existing implementation patterns in `src/index.html`
2. **Test in isolation** - Use/create analytics test file before modifying main app
3. **Maintain transparency** - Ensure scoring logic is explainable and visible to users
4. **Preserve backward compatibility** - User data in localStorage must remain valid
5. **Update documentation** - Only update `README.md` or `USAGE_GUIDE.md`, NOT docs-archive
