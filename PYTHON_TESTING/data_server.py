#!/usr/bin/env python3
"""
Flask server to handle user data storage for Champion Recommender
Automatically saves user data to CSV file in PYTHON_TESTING folder
"""

from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
import json
import os
from datetime import datetime
from pathlib import Path
from user_data_manager import UserDataManager

app = Flask(__name__)
CORS(app)  # Enable CORS for all routes

# Initialize the user data manager
data_manager = UserDataManager()

@app.route('/api/register_user', methods=['POST'])
def register_user():
    """Register a new user and update CSV file"""
    try:
        user_data = request.json
        
        # Validate required fields
        required_fields = ['fullName', 'email', 'age']
        for field in required_fields:
            if not user_data.get(field):
                return jsonify({'error': f'Missing required field: {field}'}), 400
        
        # Convert to our format
        formatted_data = {
            'full_name': user_data['fullName'],
            'email': user_data['email'],
            'phone': user_data.get('phone', ''),
            'age': int(user_data['age']),
            'gender': user_data.get('gender', ''),
            'lol_experience': user_data.get('experience', ''),
            'session_id': user_data.get('sessionId', f'session_{datetime.now().timestamp()}')
        }
        
        # Add user to database and update CSV
        data_manager.add_user(formatted_data)
        
        return jsonify({
            'success': True, 
            'message': 'User registered successfully and CSV updated',
            'session_id': formatted_data['session_id'],
            'csv_path': data_manager.get_csv_path()
        })
        
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/update_results', methods=['POST'])
def update_results():
    """Update user with recommendation results and update CSV"""
    try:
        data = request.json
        session_id = data.get('sessionId')
        
        if not session_id:
            return jsonify({'error': 'Missing session ID'}), 400
        
        results_data = {
            'recommended_champion': data.get('recommendedChampion'),
            'winning_algorithm': data.get('winningAlgorithm'),
            'confidence_score': data.get('confidence'),
            'random_forest_champion': data.get('randomForestChampion'),
            'random_forest_confidence': data.get('randomForestConfidence'),
            'decision_tree_champion': data.get('decisionTreeChampion'),
            'decision_tree_confidence': data.get('decisionTreeConfidence'),
            'knn_champion': data.get('knnChampion'),
            'knn_confidence': data.get('knnConfidence'),
            'consensus_level': data.get('consensusLevel'),
            'user_answers': data.get('userAnswers'),
            'completion_timestamp': data.get('completionDate')
        }
        
        success = data_manager.update_user_results(session_id, results_data)
        
        if success:
            return jsonify({
                'success': True, 
                'message': 'Results updated successfully and CSV updated',
                'csv_path': data_manager.get_csv_path()
            })
        else:
            return jsonify({'error': 'User not found'}), 404
            
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/user_count', methods=['GET'])
def get_user_count():
    """Get total number of registered users"""
    count = data_manager.get_user_count()
    return jsonify({
        'count': count,
        'csv_path': data_manager.get_csv_path()
    })

@app.route('/api/download_csv', methods=['GET'])
def download_csv():
    """Download the CSV file"""
    try:
        # Create/update the CSV file
        data_manager.create_csv_file()
        
        # Check if file exists
        if os.path.exists(data_manager.csv_file):
            return send_file(
                data_manager.csv_file,
                as_attachment=True,
                download_name='champion_recommender_users.csv',
                mimetype='text/csv'
            )
        else:
            return jsonify({'error': 'CSV file not found'}), 404
            
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/csv_path', methods=['GET'])
def get_csv_path():
    """Get the full path to the CSV file"""
    return jsonify({
        'csv_path': data_manager.get_csv_path(),
        'exists': os.path.exists(data_manager.csv_file),
        'user_count': data_manager.get_user_count()
    })

@app.route('/', methods=['GET'])
def home():
    """Home page with server status"""
    return jsonify({
        'message': 'Champion Recommender Data Server is running!',
        'csv_location': data_manager.get_csv_path(),
        'endpoints': {
            'POST /api/register_user': 'Register a new user',
            'POST /api/update_results': 'Update user with results',
            'GET /api/user_count': 'Get total user count',
            'GET /api/download_csv': 'Download CSV file',
            'GET /api/csv_path': 'Get CSV file path'
        },
        'total_users': data_manager.get_user_count()
    })

if __name__ == '__main__':
    print("Starting Champion Recommender Data Server...")
    print(f"Server will be available at: http://localhost:5001")
    print(f"CSV files will be saved in: {data_manager.get_csv_path()}")
    print(f"Working directory: {Path(__file__).parent.absolute()}")
    print("\nEndpoints:")
    print("- http://localhost:5001/api/user_count (GET user count)")
    print("- http://localhost:5001/api/download_csv (Download CSV file)")
    print("- http://localhost:5001/api/csv_path (Get CSV file location)")
    print("\nPress Ctrl+C to stop the server")
    
    app.run(debug=True, host='0.0.0.0', port=5001)