# LOL Champion Recommender System - Usage Guide

## Overview
This guide explains how to use the enhanced League of Legends Champion Recommender System, which now includes 10 questions (5 game-related and 5 psychological/behavioral) to provide more personalized champion recommendations.

## System Requirements
- A modern web browser (Chrome, Firefox, Safari, or Edge)
- Python 3.8+ (for local development and data storage)

## Quick Start

### Option 1: Direct Browser Access (No Server Required)
1. Navigate to the project directory
2. Open `src/index.html` directly in your web browser
3. Complete the registration form and questionnaire

### Option 2: Local Server (Recommended for Data Storage)
1. Open a terminal
2. Navigate to the project directory
3. Start the HTTP server:
   ```bash
   cd /Users/abdullahbinmadhi/Desktop/LOL-Recommender-System
   python -m http.server 8080
   ```
4. Open your browser to http://localhost:8080/src/

## Registration Process

### Personal Information Form
The enhanced registration form collects the following information:
- **Name** (required) - Your display name
- **Email** (required) - For user identification
- **Age** (required) - For demographic analysis
- **Gender** (optional) - For demographic analysis
- **Phone** (optional) - For contact purposes
- **LoL Experience** (required) - Your experience level with the game

## Questionnaire

### Game-Related Questions (5)
1. **What is your preferred difficulty?**
   - Easy (1–3)
   - Medium (4–6)
   - Hard (7–8)
   - Very Hard (9–10)

2. **What is your preferred role?**
   - Tank
   - Fighter
   - Assassin
   - Mage
   - Marksman
   - Support

3. **What is your preferred position?**
   - Top
   - Jungle
   - Mid
   - Bot
   - Support

4. **What playstyle do you prefer?**
   - High Damage Output
   - Tanky and Durable
   - Support Team
   - Balanced/Hybrid

5. **Do you prefer range or melee?**
   - Ranged
   - Melee
   - No Preference

### Psychological & Behavioral Questions (5)

6. **How do you usually respond under pressure in a game?**
   - Stay calm and strategic
   - Take charge and lead
   - Get aggressive and take risks
   - Play cautiously to avoid mistakes
   - *Dimension: Emotional stability & risk-taking*

7. **What type of character aesthetic appeals to you most?**
   - Heroic
   - Mysterious
   - Dark and edgy
   - Cute or playful
   - Monstrous or non-human
   - *Dimension: Aesthetic & identity preference*

8. **When cooperating with others, how do you prefer to contribute to the team?**
   - Lead and make decisions
   - Support and enable others
   - Balance between both
   - Stay independent and focus on my role
   - *Dimension: Teamwork & leadership orientation*

9. **Which character identity do you usually prefer?**
   - Male
   - Female
   - Non-human
   - No preference
   - *Dimension: Character identity & self-expression*

10. **When faced with a difficult challenge, what best describes your approach?**
    - Analyze carefully before acting
    - Jump in and adapt on the fly
    - Follow the team's lead
    - Focus on long-term improvement
    - *Dimension: Cognitive style & problem-solving preference*

## Data Storage and Export

### Local Storage
All user data is automatically stored in your browser's localStorage for persistence between sessions.

### CSV Export
To export your data as a CSV file:
1. Press **Ctrl+Shift+E** at any time
2. The CSV file will be downloaded automatically
3. The file will be named `champion_recommender_users.csv`

### Backend Storage (Optional)
For local development with backend storage:
1. Start the data server:
   ```bash
   cd PYTHON_TESTING
   python data_server.py
   ```
2. The server will run on http://localhost:5001
3. Data will be automatically saved to `PYTHON_TESTING/champion_recommender_users.csv`

## CSV File Structure

The exported CSV file contains the following columns:
- ID
- Full Name
- Email
- Phone
- Age
- Gender
- LoL Experience
- Registration Date
- Session ID
- Recommended Champion
- Winning Algorithm
- Confidence Score
- Random Forest Champion
- Random Forest Confidence
- Decision Tree Champion
- Decision Tree Confidence
- KNN Champion
- KNN Confidence
- Consensus Level
- User Answers
- Completion Date
- Pressure Response
- Aesthetic Preference
- Team Contribution
- Character Identity
- Problem Solving Approach

## Troubleshooting

### Common Issues

1. **Page not loading**
   - Ensure you're opening `src/index.html` and not the root `index.html`
   - Check that all files are in their correct locations

2. **Data not saving**
   - Make sure your browser allows localStorage
   - Check browser console for JavaScript errors (F12)

3. **Export not working**
   - Ensure JavaScript is enabled
   - Try using a different browser

4. **Server errors**
   - Make sure all Python dependencies are installed
   - Check that the required ports are available

### Getting Help
If you encounter any issues:
1. Check the browser console for error messages (F12)
2. Verify all files are in their correct locations
3. Ensure you're using a modern browser
4. Contact the development team for assistance

## Development Information

### Project Structure
```
LOL-Recommender-System/
├── src/
│   ├── index.html          # Main application
│   ├── data/
│   │   └── questions.json  # Questionnaire data
│   └── styles/
├── PYTHON_TESTING/         # Backend and data management
│   ├── data_server.py      # Flask server for data storage
│   ├── user_data_manager.py # CSV management
│   └── champion_recommender_users.csv # Data file
└── README.md               # Project documentation
```

### Modifying Questions
To modify the questionnaire:
1. Edit `src/data/questions.json`
2. The changes will be reflected automatically in the web application
3. Ensure the JSON structure remains valid

### Adding New Features
To extend the system:
1. Add new fields to the registration form in `src/index.html`
2. Update the data handling in JavaScript
3. Modify `user_data_manager.py` to include new CSV columns
4. Update `data_server.py` to handle new data fields

## Privacy and Security

### Data Collection
- All data is stored locally in your browser
- No personal information is transmitted over the internet
- Data is only saved to your local machine

### Data Usage
- Data is used solely for providing champion recommendations
- All data remains on your device
- No third-party analytics or tracking

### Data Deletion
- To delete your data, clear your browser's localStorage
- Or delete the `champion_recommender_users.csv` file

## Feedback and Support

For feedback, suggestions, or bug reports, please contact the development team or create an issue in the GitHub repository.

Enjoy finding your perfect League of Legends champion!