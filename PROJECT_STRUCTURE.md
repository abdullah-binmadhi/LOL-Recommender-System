# Project Structure Documentation

This document provides a comprehensive overview of the current project structure and organization.

## Current Structure

```
├── .github/
│   └── workflows/
│       └── pages.yml
├── archive/                   # Historical files (not used in production)
├── docs/                      # Comprehensive documentation
│   ├── api_documentation.md
│   ├── deployment_guide.md
│   ├── setup_guide.md
│   ├── troubleshooting_guide.md
│   ├── user_guide.md
│   ├── README.md
│   └── repository_structure.md
├── src/                       # Source code (production files)
│   ├── index.html             # Main application
│   ├── data/                  # JSON data files
│   │   ├── champion_counters.json
│   │   ├── champion_guides.json
│   │   ├── champion_tips.json
│   │   ├── champions.json
│   │   └── questions.json
│   ├── assets/                # Images and media
│   ├── components/            # (Future use)
│   ├── styles/                # CSS files
│   │   └── styles.css
│   └── README.md              # Source code documentation
├── PYTHON_TESTING/            # Python development environment
├── .gitignore
├── .env.example
├── README.md                  # Project overview
└── index.html                 # Redirect to src/index.html
```

## Key Directories Explained

### src/ - Source Code (Production)

This is where all the production code lives:

- **index.html**: The main application file containing all HTML, CSS, and JavaScript
- **data/**: JSON files containing champion data, questions, and other information
- **assets/**: Images, icons, and other media files
- **styles/**: CSS files for styling
- **components/**: Reserved for future component-based organization

### docs/ - Documentation

Comprehensive documentation for all aspects of the project:

- User guides
- Developer documentation
- Deployment instructions
- Troubleshooting guides
- API documentation

### archive/ - Historical Files

Files that are no longer used but kept for reference:

- Previous versions of the application
- Old implementation attempts
- Data files in different formats
- Experimental features
- Development environment files

### PYTHON_TESTING/ - Development Environment

Python scripts and server files for local development:

- Data collection scripts
- Testing utilities
- Local server implementations

## File Management Guidelines

### Adding New Files

1. **Production Code**: Add to appropriate directory in `src/`
2. **Documentation**: Add to `docs/` with clear naming
3. **Development Tools**: Add to `scripts/` or `PYTHON_TESTING/`
4. **Tests**: Add to `tests/`

### Updating Existing Files

1. Make changes in feature branches
2. Update documentation when making significant changes
3. Test thoroughly before committing
4. Follow existing naming and organization conventions

### Removing Files

1. Move to `archive/` instead of deleting if there's any potential future use
2. Update documentation if removing key files
3. Ensure no references to removed files remain in the codebase

## Best Practices

1. **Keep it Clean**: Only essential files in root directory
2. **Organize by Purpose**: Group files by their function
3. **Document Everything**: Maintain up-to-date documentation
4. **Use Descriptive Names**: Clear, consistent naming conventions
5. **Version Control**: Use Git effectively with meaningful commit messages

## Deployment Considerations

For GitHub Pages deployment:

1. Only files in the repository are deployed
2. The `src/index.html` is the main entry point
3. All data files must be in the repository
4. No server-side processing is available (static site only)