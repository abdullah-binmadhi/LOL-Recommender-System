# New Features Implementation

## Overview
This update adds 5 new psychological and behavioral questions to the League of Legends Champion Recommender System, expanding the questionnaire from 5 to 10 questions. It also enhances the data storage system to capture more detailed user information.

## New Questions Added

### Game-Related Questions (5)
1. What is your preferred difficulty?
   Options: Easy (1–3), Medium (4–6), Hard (7–8), Very Hard (9–10)
2. What is your preferred role?
   Options: Tank, Fighter, Assassin, Mage, Marksman, Support
3. What is your preferred position?
   Options: Top, Jungle, Mid, Bot, Support
4. What playstyle do you prefer?
   Options: High Damage Output, Tanky and Durable, Support Team, Balanced/Hybrid
5. Do you prefer range or melee?
   Options: Ranged, Melee, No Preference

### Psychological & Behavioral Questions (5)
6. How do you usually respond under pressure in a game?
   Options: Stay calm and strategic, Take charge and lead, Get aggressive and take risks, Play cautiously to avoid mistakes
   Dimension: Emotional stability & risk-taking

7. What type of character aesthetic appeals to you most?
   Options: Heroic, Mysterious, Dark and edgy, Cute or playful, Monstrous or non-human
   Dimension: Aesthetic & identity preference

8. When cooperating with others, how do you prefer to contribute to the team?
   Options: Lead and make decisions, Support and enable others, Balance between both, Stay independent and focus on my role
   Dimension: Teamwork & leadership orientation

9. Which character identity do you usually prefer?
   Options: Male, Female, Non-human, No preference
   Dimension: Character identity & self-expression

10. When faced with a difficult challenge, what best describes your approach?
    Options: Analyze carefully before acting, Jump in and adapt on the fly, Follow the team's lead, Focus on long-term improvement
    Dimension: Cognitive style & problem-solving preference

## Enhanced Data Storage

### User Registration Form
The registration form now collects additional information:
- Email (required)
- Age (required)
- Gender (optional)
- Phone (optional)

### CSV Storage Structure
The CSV file now includes additional columns:
- Pressure Response
- Aesthetic Preference
- Team Contribution
- Character Identity
- Problem Solving Approach

## Implementation Details

### Files Modified
1. `src/data/questions.json` - Added the 5 new questions
2. `src/index.html` - Updated the user registration form and JavaScript logic
3. `PYTHON_TESTING/user_data_manager.py` - Updated CSV structure
4. `PYTHON_TESTING/data_server.py` - Updated data handling

### Data Flow
1. User completes enhanced registration form
2. User answers all 10 questions
3. Data is stored in localStorage and optionally sent to backend server
4. Results are saved to CSV with all new fields
5. Users can export data manually with Ctrl+Shift+E

## Testing
A test script (`PYTHON_TESTING/test_new_questions.py`) is included to verify:
- Correct number of questions (10)
- Presence of all question IDs
- Proper CSV structure with new columns

## Usage Instructions
1. Open `src/index.html` in a web browser
2. Complete the enhanced registration form
3. Answer all 10 questions
4. View recommendations
5. Export data using Ctrl+Shift+E when needed