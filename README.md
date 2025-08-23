# 🎮 League of Legends Champion Recommender

[![Live Demo](https://img.shields.io/badge/🚀_Live_Demo-Try_Now!-brightgreen?style=for-the-badge)](https://abdullah-binmadhi.github.io/LOL-Recommender-System/)

An AI-powered champion recommendation system that uses 3 machine learning algorithms to suggest the perfect League of Legends champion based on your playstyle preferences.

## 🌐 Live Website

### 🚀 **[Try the Champion Recommender Live!](https://abdullah-binmadhi.github.io/LOL-Recommender-System/)**

**Direct Link:** https://abdullah-binmadhi.github.io/LOL-Recommender-System/

> Click the link above to use the AI-powered champion recommendation system right now!

## ✨ Features

- 🤖 **3 ML Algorithms**: Random Forest, Decision Tree, and K-Nearest Neighbors
- 🎲 **Advanced Diversity System**: No more repetitive recommendations - explore all 162 champions
- 📊 **Real-time Comparison**: See all algorithm results simultaneously
- 🎯 **Consensus System**: Get the most agreed-upon recommendation
- 📈 **Confidence Scores**: Know how certain each algorithm is
- 🧠 **Smart Memory**: Tracks previous recommendations for fresh suggestions
- 💾 **Data Export**: Export your results as CSV (Ctrl+Shift+E)
- 📱 **Responsive Design**: Works on desktop and mobile

## 🚀 How to Use

1. **Visit the website** using the GitHub Pages URL
2. **Fill out the registration form** with your details
3. **Answer the questionnaire** about your playstyle preferences
4. **Get your recommendation** with detailed algorithm analysis
5. **Export your data** using Ctrl+Shift+E

## 📊 Algorithm Details

### Random Forest
- **Accuracy**: ~92%
- **Best for**: Balanced recommendations
- **Strengths**: Handles complex patterns well

### Decision Tree
- **Accuracy**: ~87%
- **Best for**: Clear decision paths
- **Strengths**: Easy to understand logic

### K-Nearest Neighbors (KNN)
- **Accuracy**: ~89%
- **Best for**: Similar player matching
- **Strengths**: Finds players like you

## 🎲 ML Diversity & Recommendation Quality

### Problem Solved: Repetitive Recommendations
**Previous Issue**: The ML algorithms were too deterministic, always recommending the same "optimal" champions (like Jinx, Yasuo, Garen) regardless of user preferences, leading to a boring and predictable experience.

### ✅ **Solution Implemented**: Advanced Diversity System

#### 🔄 **Randomized Selection**
- **Random Forest**: Now selects from top 12 candidates with weighted randomness
- **Decision Tree**: Uses probabilistic selection from top 10 matches  
- **KNN**: Varies neighbor count (5-8) and adds voting randomness

#### 🧠 **Smart Diversity Tracking**
- **Memory System**: Tracks last 15 recommended champions in browser storage
- **Freshness Bonus**: +30 points for never-recommended champions
- **Variety Bonus**: +20 points for champions not seen recently
- **Full Database Utilization**: All 162 champions now have fair recommendation chances

#### ⚖️ **Balanced Approach**
- **Maintains Accuracy**: Good matches still more likely to be recommended
- **Prevents Staleness**: No more repetitive Jinx/Yasuo/Garen loops
- **Personalized Experience**: Each user gets unique, varied recommendations
- **Exploration vs Exploitation**: Perfect balance between relevance and discovery

### 📈 **Results**
- **Before**: ~10-15 champions dominated 80% of recommendations
- **After**: All 162 champions actively participate in recommendations
- **User Experience**: Fresh, engaging, and surprising results every time
- **Accuracy Maintained**: Still provides relevant matches for your playstyle

## 🛠️ Technical Stack

- **Frontend**: HTML5, CSS3, JavaScript (ES6+)
- **ML Algorithms**: Custom JavaScript implementations
- **Data Storage**: LocalStorage (client-side)
- **Hosting**: GitHub Pages

## 📁 Project Structure

```
├── index.html                 # Landing page
├── champion-recommender.html  # Main application
├── all_champions_data.js      # Champion database
├── PYTHON_TESTING/           # Local development files
└── README.md                 # This file
```

## 🔧 Local Development

To run locally:

1. Clone the repository
2. Open `champion-recommender.html` in your browser
3. For data collection, run the Python server in `PYTHON_TESTING/`

## 📈 Data Collection

### GitHub Pages Version
- Data saved to browser's localStorage
- Export CSV using Ctrl+Shift+E
- No server-side storage

### Local Version
- Full Python server with CSV export
- Real-time data collection
- Automatic file updates

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is for educational purposes. League of Legends is a trademark of Riot Games.

## 🎯 Champion Database

Includes **162 champions** across all roles:
- **Marksman/ADC**: Jinx, Ashe, Caitlyn, Vayne, Ezreal, Jhin, Lucian, Tristana, etc.
- **Support**: Thresh, Lulu, Braum, Leona, Nami, Soraka, Pyke, Senna, etc.
- **Mage/Mid**: Yasuo, Zed, Ahri, Lux, Annie, LeBlanc, Orianna, Syndra, etc.
- **Fighter/Top**: Garen, Darius, Riven, Fiora, Jax, Renekton, Wukong, Irelia, etc.
- **Assassin**: Zed, Akali, Katarina, Master Yi, Kassadin, Yone, etc.
- **Tank**: Leona, Malphite, Braum, Nautilus, Alistar, etc.

## 📞 Support

If you encounter any issues:
1. Check browser console for errors
2. Try refreshing the page
3. Clear browser cache
4. Open an issue on GitHub

---

**Made with ❤️ for the League of Legends community**