# Repository Structure

This document describes the organization of the LoL Champion Recommender repository.

## Directory Structure

```
├── .github/
│   └── workflows/
│       └── pages.yml          # GitHub Pages deployment workflow
├── docs/                      # Project documentation
│   ├── api_documentation.md
│   ├── deployment_guide.md
│   ├── setup_guide.md
│   ├── troubleshooting_guide.md
│   ├── user_guide.md
│   ├── README.md              # Documentation index
│   └── repository_structure.md # This file
├── src/                       # Source code
│   ├── index.html             # Main application file
│   ├── data/                  # JSON data files
│   │   ├── champion_counters.json
│   │   ├── champion_guides.json
│   │   ├── champion_tips.json
│   │   ├── champions.json
│   │   └── questions.json
│   ├── assets/                # Images and other assets
│   ├── components/            # Future components directory
│   └── styles/                # Future styles directory
├── scripts/                   # Utility scripts
├── tests/                     # Test files
├── PYTHON_TESTING/            # Python development files
├── .gitignore                 # Git ignore file
├── .env.example              # Environment variables example
├── README.md                  # Project overview
└── index.html                 # Redirect to src/index.html
```

## File Descriptions

### Core Application Files

- **src/index.html**: The main application file containing all HTML, CSS, and JavaScript
- **src/data/**: Contains all JSON data files used by the application
  - `champions.json`: Complete champion database with attributes
  - `questions.json`: Questionnaire data
  - `champion_counters.json`: Champion matchup data
  - `champion_guides.json`: Champion guides and strategies
  - `champion_tips.json`: Champion tips and tricks

### Documentation Files

- **docs/**: Comprehensive documentation for users, developers, and administrators
- **README.md**: Project overview and quick start guide

### Development Files

- **PYTHON_TESTING/**: Python scripts for local development and data collection
- **scripts/**: Utility scripts for development
- **tests/**: Test files for the application

### Configuration Files

- **.github/workflows/pages.yml**: GitHub Pages deployment configuration
- **.gitignore**: Specifies files and directories to ignore in Git
- **.env.example**: Example environment variables file

## Best Practices

1. **Keep the root directory clean**: Only essential files should be in the root
2. **Organize by purpose**: Files are grouped by their function
3. **Separate documentation**: All documentation is in the docs/ directory
4. **Clear naming conventions**: Files and directories have descriptive names
5. **Future-ready structure**: The structure allows for easy expansion

## Maintenance Guidelines

1. **Add new features**: Place new code in appropriate directories
2. **Update documentation**: Keep docs/ up to date with changes
3. **Clean up old files**: Remove unused files periodically
4. **Follow the structure**: Maintain consistency when adding new files