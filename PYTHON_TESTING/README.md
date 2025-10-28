# Champion Recommender Data Storage - PYTHON_TESTING

This folder contains the Python server and CSV file management system for the Champion Recommender application.

## 📁 Files in this folder:
- `user_data_manager.py` - Python class to manage CSV file creation and updates
- `data_server.py` - Flask server to handle user registration and results
- `requirements.txt` - Python dependencies
- `champion_recommender_users.csv` - CSV file with all user data (auto-created)
- `user_data_backup.json` - JSON backup of user data (auto-created)
- `evaluation_metrics.py` - Implementation of Precision@K and MRR evaluation metrics
- `test_evaluation_metrics.py` - Test suite for evaluation metrics
- `evaluate_recommendations.py` - Command-line tool for evaluating recommendation performance

## 🚀 Quick Setup

### Step 1: Install Dependencies
```bash
cd PYTHON_TESTING
pip install -r requirements.txt
```

### Step 2: Start the Server
```bash
python data_server.py
```

You should see:
```
Starting Champion Recommender Data Server...
Server will be available at: http://localhost:5000
CSV files will be saved in: /path/to/PYTHON_TESTING/champion_recommender_users.csv
```

### Step 3: Use the Champion Recommender
Open your HTML file and register users. The CSV file will be automatically created and updated in this PYTHON_TESTING folder.

## 📊 CSV File Structure

The CSV file contains the following columns:

### User Information:
- ID
- Full Name
- Email
- Phone
- Age
- Gender
- LoL Experience
- Registration Date
- Session ID

### Questionnaire Results (added when user completes):
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

### Evaluation Metrics (added automatically):
- Precision@1
- Precision@3
- Precision@5
- MRR

## 🔧 API Endpoints

- `POST /api/register_user` - Register new user and update CSV
- `POST /api/update_results` - Update user with questionnaire results
- `GET /api/user_count` - Get total number of users
- `GET /api/download_csv` - Download the CSV file
- `GET /api/csv_path` - Get the full path to CSV file

## 📈 How it Works

1. **User Registration**: When someone fills out the form, data is sent to the server
2. **CSV Update**: Server immediately updates the CSV file in this folder
3. **Results Update**: When user completes questionnaire, results are added to the same CSV row
4. **Evaluation Metrics**: Precision@K and MRR metrics are automatically calculated and added to the CSV
5. **Real-time Updates**: CSV file is updated in real-time as users register and complete questionnaires

## 📊 Evaluation Metrics

The system automatically calculates two key evaluation metrics for each user:

### Precision@K
Measures the proportion of recommended champions in the top-K results that are actually relevant to the user.

### Mean Reciprocal Rank (MRR)
Measures how early the first relevant champion appears in the ranking.

These metrics help evaluate:
- Overall relevance of top-ranked champions for user preferences
- How quickly relevant champions are recommended for faster adoption

## 🧪 Testing Evaluation Metrics

### Run Evaluation Tests:
```bash
python test_evaluation_metrics.py
```

### Evaluate Recommendation Performance:
```bash
python evaluate_recommendations.py
```

### Update Existing CSV with Metrics:
```bash
python update_csv_with_metrics.py
```

## 🔍 Checking Your Data

### View CSV File Location:
```bash
python -c "from user_data_manager import UserDataManager; print(UserDataManager().get_csv_path())"
```

### Add Test User:
```bash
python user_data_manager.py
# Choose option 1 to add a sample user
# Choose option 5 to calculate evaluation metrics
```

### Check User Count:
Visit: http://localhost:5000/api/user_count

## 📁 File Locations

The CSV file will be saved as:
```
/your/path/to/PYTHON_TESTING/champion_recommender_users.csv
```

You can open this file in Excel, Google Sheets, or any CSV viewer to see all user data and results.

## 🛠️ Troubleshooting

### Server won't start:
- Make sure port 5000 is available
- Check if Flask is installed: `pip install flask flask-cors`

### CSV file not updating:
- Make sure the server is running
- Check browser console for connection errors
- Verify the HTML file is connecting to http://localhost:5000

### Can't find CSV file:
- Check the server startup message for the exact path
- Look in this PYTHON_TESTING folder
- Use the API endpoint: http://localhost:5000/api/csv_path

## ✅ Success Indicators

When everything is working correctly, you should see:
- ✅ "User data saved to PYTHON_TESTING folder successfully!" in browser console
- ✅ "Results saved to PYTHON_TESTING CSV file successfully!" after completing questionnaire
- 📊 CSV file in this folder with user data and evaluation metrics
- 📁 File path shown in console messages

The CSV file will be automatically updated every time a user registers or completes the questionnaire, including the evaluation metrics for each user!