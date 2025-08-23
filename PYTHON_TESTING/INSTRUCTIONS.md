# 📊 How to Get Your User Data in CSV Format

## ✅ **Current Status:**
- ✅ CSV file created: `champion_recommender_users.csv` (clean, no fake users)
- ✅ HTML updated to save data locally
- ✅ Manual export function added

## 🚀 **How to Use:**

### **Method 1: Automatic Server (Recommended)**
1. **Start the server:**
   ```bash
   cd PYTHON_TESTING
   python data_server.py
   ```
   
2. **Use your champion recommender** - data will automatically save to CSV

### **Method 2: Manual Export (If server doesn't work)**
1. **Register users** in your champion recommender (data saves to browser)
2. **Export CSV** using either:
   - Press `Ctrl+Shift+A` (Admin Panel) → Click OK
   - Press `Ctrl+Shift+E` (Direct export)
3. **CSV file downloads** to your Downloads folder

## 📁 **File Locations:**
- **Server CSV**: `PYTHON_TESTING/champion_recommender_users.csv`
- **Manual Export**: Downloads folder → `champion_recommender_users.csv`

## 🔧 **Troubleshooting:**

### **Server Issues:**
- **Port 5000 busy?** → Server now uses port 5001
- **Server won't start?** → Use Method 2 (Manual Export)
- **No data in CSV?** → Check browser console for connection errors

### **No Data Showing:**
- **Register yourself** in the champion recommender
- **Complete the questionnaire** to get full data
- **Check localStorage** - data should be there even if server fails

## 📊 **CSV Columns:**
Your CSV will include:
- **User Info**: Name, Email, Phone, Age, Gender, Experience
- **Registration**: Date, Session ID  
- **Results**: Recommended Champion, Algorithm Used, Confidence
- **Detailed Results**: All 3 algorithm results, consensus level, user answers

## ✨ **Quick Test:**
1. Open your champion recommender HTML file
2. Register with your real details
3. Complete the questionnaire
4. Press `Ctrl+Shift+A` → Click OK
5. CSV file downloads with your data!

## 🎯 **Success Indicators:**
- ✅ Browser console shows: "User data saved successfully"
- ✅ CSV file contains your actual registration
- ✅ No fake/sample users in the file

The CSV file is now clean and ready for your real user data! 📈