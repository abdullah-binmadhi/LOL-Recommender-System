# Project Structure

This document describes the organization of the League of Legends Champion Recommender System.

## Root Directory Structure

```
LOL-Recommender-System/
├── .github/                 # GitHub Actions and workflows
├── .git/                    # Git repository data
├── .vscode/                 # VS Code settings
├── src/                     # Main application source code
├── docs/                    # GitHub Pages documentation
├── docs-archive/            # Archived development documentation
├── components/              # Legacy components (can be removed)
├── data/                    # Root-level data (legacy)
├── lib/                     # Libraries (legacy)
├── pages/                   # Next.js pages (legacy)
├── public/                  # Public assets (legacy)
├── node_modules/            # npm dependencies
├── index.html               # GitHub Pages entry point
├── README.md                # Project overview and setup
├── USAGE_GUIDE.md          # User guide
├── package.json            # npm dependencies and scripts
├── _config.yml             # GitHub Pages Jekyll configuration
└── .gitignore              # Git ignore rules

```

## Source Code (`/src/`)

**Main application files - This is the active codebase**

```
src/
├── index.html              # Main application (Single Page App)
├── data/                   # Application data
│   ├── questions.json      # Questionnaire questions
│   └── champion_counters.json  # Champion matchup data
├── analytics/              # Analytics dashboards and tests
│   ├── analytics.html      # Main analytics dashboard
│   ├── debug-analytics.html
│   ├── simple-analytics.html
│   ├── test_*.html         # Various test files
│   └── README.md
├── assets/                 # Images, icons, etc.
├── components/             # Reusable components
└── styles/                 # CSS stylesheets
```

## Documentation (`/docs/`)

**GitHub Pages documentation site**

- Used for project documentation visible at: https://abdullah-binmadhi.github.io/LOL-Recommender-System/

## Archive (`/docs-archive/`)

**Development documentation and test data**

Contains:
- TASK_*.md - Task completion summaries
- DEPLOYMENT_*.md - Deployment documentation
- ML_*.md - ML implementation docs
- TEST_*.md - Test results
- test_dataset_sample.* - Sample test data
- LoL_champion_data.csv - Original data backup

These files are kept for reference but are not needed for the application to run.

## Key Files

### Application Files
- **`/src/index.html`** - Main application file (7000+ lines)
  - Contains all HTML, CSS, and JavaScript
  - Includes 3 ML algorithms (Random Forest, Decision Tree, KNN)
  - Includes questionnaire system
  - Includes recommendation display

### Configuration Files
- **`package.json`** - npm dependencies
- **`_config.yml`** - GitHub Pages configuration
- **`.gitignore`** - Git ignore rules

### Documentation
- **`README.md`** - Project setup and overview
- **`USAGE_GUIDE.md`** - How to use the application

## Data Flow

1. User opens `/index.html` (GitHub Pages entry)
2. Redirects to `/src/index.html` (main app)
3. User completes questionnaire (data from `/src/data/questions.json`)
4. ML algorithms process answers
5. Recommendations displayed with champion data
6. Results can be saved to localStorage
7. Analytics available at `/src/analytics/analytics.html`

## Legacy Folders (Can be removed)

- `/components/` - Old component structure
- `/lib/` - Old libraries
- `/pages/` - Next.js pages (project no longer uses Next.js)
- `/public/` - Old public assets
- `/data/` - Root-level data (moved to `/src/data/`)

## Development vs Production

### Development
- Edit `/src/index.html` for main application
- Edit `/src/data/*.json` for data changes
- Test using local file or live server

### Production (GitHub Pages)
- Push to `main` branch
- GitHub Pages automatically deploys
- Live at: https://abdullah-binmadhi.github.io/LOL-Recommender-System/

## File Organization Summary

| Location | Purpose | Status |
|----------|---------|--------|
| `/src/` | Main application | **Active** |
| `/docs/` | GitHub Pages docs | **Active** |
| `/index.html` | Entry point | **Active** |
| `README.md` | Documentation | **Active** |
| `USAGE_GUIDE.md` | User guide | **Active** |
| `/docs-archive/` | Dev documentation | Archived |
| `/src/analytics/` | Analytics & tests | Development only |
| `/components/`, `/lib/`, `/pages/`, `/public/` | Legacy code | Can be removed |

## Maintenance

### To clean up further:
```bash
# Remove legacy folders
rm -rf components lib pages public data

# Keep only essential files for production
```

### To add new features:
1. Edit `/src/index.html`
2. Update `/src/data/` if needed
3. Test locally
4. Commit and push to deploy

## Notes

- Application is a single-page app (SPA) - everything in `/src/index.html`
- No build process required - pure HTML/CSS/JavaScript
- GitHub Pages serves static files
- Analytics and tests are in `/src/analytics/` but not deployed
- All development documentation moved to `/docs-archive/`
