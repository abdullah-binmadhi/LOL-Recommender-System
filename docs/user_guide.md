# LoL Champion Recommender - User Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Using the Recommender](#using-the-recommender)
3. [Understanding Results](#understanding-results)
4. [Advanced Features](#advanced-features)
5. [Tips and Best Practices](#tips-and-best-practices)
6. [Frequently Asked Questions](#frequently-asked-questions)
7. [Troubleshooting](#troubleshooting)

---

## Getting Started

### What is the LoL Champion Recommender?

The LoL Champion Recommender is an intelligent web application that helps League of Legends players discover champions that match their playstyle preferences. Using machine learning algorithms, it analyzes your answers to a series of questions and recommends champions that best fit your preferred way of playing.

### Who Should Use This Tool?

- **New Players**: Discover champions that match your gaming preferences
- **Returning Players**: Find new champions to try after breaks from the game
- **Experienced Players**: Explore champions outside your comfort zone
- **Role Switchers**: Find suitable champions when trying new roles

### System Requirements

- **Web Browser**: Chrome, Firefox, Safari, or Edge (latest versions)
- **Internet Connection**: Required for initial loading
- **JavaScript**: Must be enabled
- **Screen Resolution**: Minimum 1024x768 (mobile-friendly)

---

## Using the Recommender

### Step 1: Starting Your Session

1. **Visit the Homepage**
   - Navigate to the LoL Champion Recommender website
   - You'll see a welcome screen with information about the tool
   - Click the "Start Recommendation" button to begin

2. **Session Creation**
   - The system automatically creates a unique session for you
   - Your progress is saved as you go through the questionnaire
   - You can safely close and reopen your browser without losing progress

### Step 2: Answering Questions

The questionnaire consists of 9 carefully designed questions that help determine your playstyle preferences:

#### Question Types

1. **Role Preference** (Question 1)
   - **Options**: Tank, Fighter, Assassin, Mage, Marksman, Support
   - **Purpose**: Determines your preferred champion role
   - **Tip**: Choose the role you enjoy most or want to learn

2. **Difficulty Level** (Question 2)
   - **Options**: Easy, Moderate, Hard, Expert
   - **Purpose**: Matches champions to your skill level
   - **Tip**: Be honest about your current skill level

3. **Gameplay Style** (Question 3)
   - **Options**: Team fights, Solo plays, Supporting allies, Farming, Roaming
   - **Purpose**: Identifies how you prefer to impact games
   - **Tip**: Think about what you find most enjoyable in games

4. **Range Preference** (Question 4)
   - **Options**: Melee, Ranged, No preference
   - **Purpose**: Determines if you prefer close or long-range combat
   - **Tip**: Consider your comfort level with positioning

5. **Damage Importance** (Question 5)
   - **Options**: Very Low, Low, Medium, High, Very High
   - **Purpose**: Measures how important dealing damage is to you
   - **Tip**: Think about whether you prefer to be the damage dealer or support others

6. **Playstyle Approach** (Question 6)
   - **Options**: Defensive, Offensive, Balanced
   - **Purpose**: Determines your preferred approach to gameplay
   - **Tip**: Consider whether you prefer to initiate fights or react to them

7. **Ability Preference** (Question 7)
   - **Options**: Crowd Control, Burst Damage, Sustained Damage, Healing/Shielding, Mobility
   - **Purpose**: Identifies what type of abilities you enjoy using
   - **Tip**: Think about what feels most satisfying to execute

8. **Experience Level** (Question 8)
   - **Options**: Beginner, Intermediate, Advanced, Expert
   - **Purpose**: Helps calibrate recommendations to your overall game knowledge
   - **Tip**: Consider your overall League of Legends experience, not just with specific champions

9. **Motivation** (Question 9)
   - **Options**: Winning games, Learning new champions, Having fun, Improving skills, Support team
   - **Purpose**: Understands what drives you to play
   - **Tip**: Choose what motivates you most when playing League of Legends

#### Navigation Features

- **Next Button**: Proceed to the next question after selecting an answer
- **Previous Button**: Go back to modify previous answers
- **Progress Bar**: Shows your completion progress
- **Skip Option**: Available for non-essential questions (if implemented)

### Step 3: Getting Your Recommendation

After completing all questions:

1. **Processing**
   - The ML algorithm analyzes your responses
   - Multiple models may be consulted for the best recommendation
   - Processing typically takes 1-3 seconds

2. **Results Display**
   - Your recommended champion is displayed prominently
   - Champion image, name, and title are shown
   - Detailed explanation of why this champion was chosen

---

## Understanding Results

### Recommendation Display

#### Champion Information

1. **Basic Details**
   - **Name**: The champion's name (e.g., "Malphite")
   - **Title**: The champion's lore title (e.g., "Shard of the Monolith")
   - **Role**: Primary role classification
   - **Difficulty**: Skill level required (1-10 scale)

2. **Attributes**
   - **Damage**: Offensive capability rating
   - **Toughness**: Defensive/survivability rating
   - **Control**: Crowd control and utility rating
   - **Mobility**: Movement and positioning rating
   - **Utility**: Team support capability rating

3. **Champion Details**
   - **Range Type**: Melee or Ranged combat style
   - **Position**: Preferred lane/position
   - **Tags**: Additional role classifications

#### Recommendation Explanation

The system provides detailed explanations for why a champion was recommended:

1. **Confidence Score**
   - Percentage indicating how well the champion matches your preferences
   - Higher scores indicate better matches
   - Scores above 80% are considered excellent matches

2. **Match Reasons**
   - Specific aspects that align with your answers
   - Examples: "Role matches your preference for Tank champions"
   - Helps you understand the recommendation logic

3. **Detailed Analysis**
   - Explanation of how your answers led to this recommendation
   - Mentions specific question responses that influenced the choice
   - Provides context for the ML model's decision

### Champion Abilities

Each recommended champion includes detailed ability information:

1. **Passive Ability**
   - Always-active ability that defines the champion's unique mechanics
   - Provides strategic advantages throughout the game

2. **Q, W, E Abilities**
   - Basic abilities with cooldowns
   - Form the core of the champion's gameplay kit
   - Each has specific purposes (damage, utility, mobility, etc.)

3. **Ultimate Ability (R)**
   - Most powerful ability with longest cooldown
   - Often defines the champion's team fight role
   - Usually the most impactful ability in crucial moments

### Playing Tips

The recommendation includes guidance on how to play the champion:

1. **Playstyle Tips**
   - How to approach early, mid, and late game
   - Positioning advice
   - Key gameplay patterns to follow

2. **Strengths and Weaknesses**
   - What the champion excels at
   - Situations to avoid or be cautious in
   - How to maximize the champion's potential

3. **Learning Path**
   - Suggested order for learning abilities
   - Practice recommendations
   - Common mistakes to avoid

---

## Advanced Features

### Alternative Recommendations

If you want to explore more options:

1. **Get Alternatives Button**
   - Click to see 3-5 additional champion recommendations
   - Each alternative includes its own confidence score and explanation
   - Alternatives are ranked by compatibility with your preferences

2. **Alternative Display**
   - Shows champions with different strengths but similar overall compatibility
   - Helps you explore different playstyles within your preferences
   - Includes brief explanations for each alternative

### Retaking the Questionnaire

You can restart the process at any time:

1. **Retake Button**
   - Available on the results page
   - Clears your current session and starts fresh
   - Previous session data is saved in your browser history

2. **Session History**
   - View your previous recommendations (if enabled)
   - Compare different sessions
   - Track your evolving preferences over time

### Champion Details Page

Click on any recommended champion to see detailed information:

1. **Extended Champion Information**
   - Complete ability descriptions
   - Lore and background information
   - Professional play statistics (if available)

2. **External Resources**
   - Links to champion guides
   - Official League of Legends champion page
   - Community resources and builds

### API Access

For developers or advanced users:

1. **REST API Endpoints**
   - Get recommendations programmatically
   - Access champion data
   - Integrate with other tools

2. **Response Formats**
   - JSON responses for easy integration
   - Detailed error messages
   - Consistent data structures

---

## Tips and Best Practices

### Getting Better Recommendations

1. **Be Honest About Your Preferences**
   - Answer based on what you actually enjoy, not what you think is "best"
   - Consider your current skill level realistically
   - Think about what motivates you to play

2. **Consider Your Goals**
   - Are you looking to improve at a specific role?
   - Do you want to try something completely new?
   - Are you preparing for ranked play or just having fun?

3. **Think About Your Playtime**
   - How much time do you have to learn a new champion?
   - Are you looking for immediate impact or long-term mastery?
   - Consider your practice schedule and commitment level

### Using Recommendations Effectively

1. **Start with High-Confidence Matches**
   - Begin with recommendations that have 80%+ confidence scores
   - These are most likely to match your preferences
   - Build confidence before trying lower-scored recommendations

2. **Read the Explanations**
   - Understand why each champion was recommended
   - Use the reasoning to guide your learning approach
   - Pay attention to specific strengths mentioned

3. **Try Multiple Recommendations**
   - Don't limit yourself to just the top recommendation
   - Explore alternatives to find your perfect match
   - Different champions may suit different moods or game modes

### Learning New Champions

1. **Practice in Safe Environments**
   - Start with AI games or practice tool
   - Try ARAM for team fight practice
   - Use normal games before ranked play

2. **Focus on Fundamentals First**
   - Learn ability combos and cooldowns
   - Understand power spikes and item builds
   - Practice positioning and decision-making

3. **Watch and Learn**
   - Study high-level gameplay videos
   - Watch professional players use the champion
   - Learn from educational content creators

### Troubleshooting Poor Matches

If a recommendation doesn't feel right:

1. **Review Your Answers**
   - Consider if your answers accurately reflected your preferences
   - Think about whether your preferences have changed
   - Retake the questionnaire with more thought

2. **Try Alternatives**
   - Look at the alternative recommendations
   - Sometimes the second or third choice is better
   - Each person's preferences are unique

3. **Give It Time**
   - Some champions require more practice to appreciate
   - Initial impressions may not reflect long-term enjoyment
   - Try the champion in different game modes

---

## Frequently Asked Questions

### General Questions

**Q: How accurate are the recommendations?**
A: The system has been trained on extensive champion data and player preferences. Most users find recommendations with 70%+ confidence scores to be good matches. However, personal preference is subjective, and some experimentation may be needed.

**Q: Can I retake the questionnaire?**
A: Yes! You can retake the questionnaire as many times as you want. Your preferences may change over time, or you might want to explore different aspects of your playstyle.

**Q: How long does the questionnaire take?**
A: The questionnaire typically takes 3-5 minutes to complete. Take your time to consider each answer thoughtfully for the best results.

**Q: Do I need to create an account?**
A: No account is required. The system uses browser sessions to track your progress, so you can complete the questionnaire without any registration.

### Technical Questions

**Q: Why isn't the website loading?**
A: Ensure you have a stable internet connection and JavaScript enabled. Try refreshing the page or clearing your browser cache. If problems persist, try a different browser.

**Q: Can I use this on mobile devices?**
A: Yes! The website is fully responsive and works on smartphones and tablets. The experience is optimized for touch interfaces.

**Q: Is my data saved?**
A: Your session data is temporarily stored in your browser. No personal information is collected or stored permanently on our servers.

**Q: Can I share my results?**
A: You can share your recommendation by copying the URL or taking a screenshot. The system doesn't currently have built-in sharing features.

### Recommendation Questions

**Q: Why did I get a champion I don't like?**
A: The ML model makes predictions based on patterns in the data. If a recommendation doesn't feel right, try the alternatives or retake the questionnaire with different answers that better reflect your preferences.

**Q: Can I get recommendations for specific roles only?**
A: The first question asks about role preference, which heavily influences the recommendation. However, the system considers all your answers to provide the best overall match.

**Q: Why are some champions never recommended?**
A: The system only recommends champions that are included in the training data. Some champions may be excluded if they're very new, reworked, or don't have sufficient data for accurate recommendations.

**Q: How often is the champion data updated?**
A: Champion data is updated regularly to reflect game changes, new champions, and balance updates. The ML models are retrained periodically to maintain accuracy.

### Troubleshooting Questions

**Q: The questionnaire seems stuck on one question.**
A: Try refreshing the page. Your progress should be saved. If the problem persists, try clearing your browser cache or using a different browser.

**Q: I'm getting error messages.**
A: Most errors are temporary. Try refreshing the page or waiting a few minutes. If errors persist, the issue may be on our end and should resolve shortly.

**Q: The recommendation page won't load.**
A: This might happen if your session expired or there was a processing error. Try retaking the questionnaire or refreshing the page.

---

## Troubleshooting

### Common Issues and Solutions

#### Website Not Loading

**Symptoms**: Page won't load, connection errors, blank screen

**Solutions**:
1. Check your internet connection
2. Try refreshing the page (Ctrl+F5 or Cmd+Shift+R)
3. Clear browser cache and cookies
4. Try a different browser
5. Disable browser extensions temporarily
6. Check if JavaScript is enabled

#### Questionnaire Problems

**Symptoms**: Can't proceed to next question, answers not saving, progress lost

**Solutions**:
1. Ensure JavaScript is enabled
2. Try clicking the answer option again
3. Refresh the page (progress should be saved)
4. Clear browser cache if problems persist
5. Try using a different browser

#### Recommendation Not Loading

**Symptoms**: Stuck on "Processing..." screen, error after completing questionnaire

**Solutions**:
1. Wait a few more seconds (processing can take up to 10 seconds)
2. Refresh the page
3. Check browser console for error messages (F12 → Console)
4. Try retaking the questionnaire
5. Contact support if the issue persists

#### Mobile Device Issues

**Symptoms**: Layout problems, buttons not working, text too small

**Solutions**:
1. Rotate device to landscape mode
2. Zoom out if content appears too large
3. Try using a different mobile browser
4. Clear mobile browser cache
5. Ensure you have the latest browser version

### Performance Issues

#### Slow Loading

**Causes and Solutions**:
1. **Slow Internet**: Try a faster connection or wait for better connectivity
2. **Server Load**: Wait a few minutes and try again
3. **Large Images**: Allow time for champion images to load
4. **Browser Issues**: Clear cache, restart browser, or try a different one

#### Memory Issues

**Symptoms**: Browser becomes slow, tabs crash, system becomes unresponsive

**Solutions**:
1. Close other browser tabs
2. Restart your browser
3. Clear browser cache and cookies
4. Restart your computer if necessary
5. Try using a different browser

### Error Messages

#### "Session Expired"

**Meaning**: Your session has timed out due to inactivity
**Solution**: Click "Start Over" to begin a new session

#### "Invalid Response"

**Meaning**: There was an error processing your answer
**Solution**: Try selecting the answer again or refresh the page

#### "Recommendation Failed"

**Meaning**: The ML system encountered an error
**Solution**: Try retaking the questionnaire or contact support

#### "Network Error"

**Meaning**: Connection to the server was lost
**Solution**: Check your internet connection and try again

### Browser Compatibility

#### Supported Browsers

- **Chrome**: Version 80+
- **Firefox**: Version 75+
- **Safari**: Version 13+
- **Edge**: Version 80+

#### Unsupported Browsers

- Internet Explorer (any version)
- Very old versions of any browser
- Browsers with JavaScript disabled

### Getting Help

If you continue to experience issues:

1. **Check the FAQ**: Many common issues are addressed above
2. **Try Basic Troubleshooting**: Refresh, clear cache, try different browser
3. **Contact Support**: 
   - Email: support@lol-recommender.com
   - Include: Browser version, operating system, description of the problem
   - Screenshots: Helpful for visual issues

4. **Report Bugs**:
   - GitHub Issues: [Repository Issues Page]
   - Include: Steps to reproduce, expected vs actual behavior
   - Browser Console Logs: Press F12 → Console tab, copy any error messages

### System Status

Check our status page for known issues:
- **Status Page**: status.lol-recommender.com
- **Maintenance Windows**: Announced in advance
- **Incident Reports**: Real-time updates during outages

---

*Last updated: January 2024*
*Version: 1.0.0*

*For additional support, visit our [documentation](docs/) or contact our support team.*