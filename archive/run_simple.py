#!/usr/bin/env python3
"""
Simple startup script for the LoL Champion Recommender
Bypasses ML model training for quick demo
"""

import os
import sys
from flask import Flask, render_template, request, redirect, url_for, session, jsonify
from datetime import datetime
import uuid

# Add current directory to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

# Simple Flask app for demo
app = Flask(__name__)
app.secret_key = 'demo-secret-key-change-in-production'

# Sample data for demo
SAMPLE_CHAMPIONS = [
    {
        'name': 'Malphite',
        'title': 'Shard of the Monolith',
        'role': 'Tank',
        'difficulty': 2,
        'description': 'A massive creature of living stone, Malphite struggles to impose blessed order on a chaotic world.'
    },
    {
        'name': 'Jinx',
        'title': 'the Loose Cannon',
        'role': 'Marksman',
        'difficulty': 6,
        'description': 'A manic and impulsive criminal from Zaun, Jinx lives to wreak havoc without care for the consequences.'
    },
    {
        'name': 'Lux',
        'title': 'the Lady of Luminosity',
        'role': 'Mage',
        'difficulty': 5,
        'description': 'Luxanna Crownguard hails from Demacia, an insular realm where magical abilities are viewed with fear and suspicion.'
    },
    {
        'name': 'Thresh',
        'title': 'the Chain Warden',
        'role': 'Support',
        'difficulty': 7,
        'description': 'Sadistic and cunning, Thresh is an ambitious and restless spirit of the Shadow Isles.'
    }
]

SAMPLE_QUESTIONS = [
    {
        'id': 1,
        'text': 'What role do you prefer to play?',
        'options': ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
    },
    {
        'id': 2,
        'text': 'What difficulty level are you comfortable with?',
        'options': ['Easy', 'Moderate', 'Hard', 'Expert']
    },
    {
        'id': 3,
        'text': 'What type of gameplay do you enjoy most?',
        'options': ['Team fights', 'Solo plays', 'Supporting allies', 'Farming', 'Roaming']
    }
]

@app.route('/')
def index():
    """Home page"""
    return render_template('index.html')

@app.route('/start')
def start_questionnaire():
    """Start a new questionnaire session"""
    session['session_id'] = str(uuid.uuid4())
    session['current_question'] = 1
    session['responses'] = {}
    session['started_at'] = datetime.now().isoformat()
    return redirect(url_for('questionnaire'))

@app.route('/questionnaire')
def questionnaire():
    """Display current question"""
    if 'session_id' not in session:
        return redirect(url_for('index'))
    
    current_q = session.get('current_question', 1)
    
    if current_q > len(SAMPLE_QUESTIONS):
        return redirect(url_for('recommendation'))
    
    question = SAMPLE_QUESTIONS[current_q - 1]
    progress = (current_q - 1) / len(SAMPLE_QUESTIONS) * 100
    
    return render_template('questionnaire.html', 
                         question=question, 
                         current_question=current_q,
                         total_questions=len(SAMPLE_QUESTIONS),
                         progress=progress)

@app.route('/answer', methods=['POST'])
def submit_answer():
    """Submit an answer and move to next question"""
    if 'session_id' not in session:
        return redirect(url_for('index'))
    
    answer = request.form.get('answer')
    action = request.form.get('action', 'next')
    current_q = session.get('current_question', 1)
    
    if action == 'next' and answer:
        # Save the answer
        session['responses'][str(current_q)] = answer
        session['current_question'] = current_q + 1
        session.modified = True
    elif action == 'previous' and current_q > 1:
        session['current_question'] = current_q - 1
        session.modified = True
    
    return redirect(url_for('questionnaire'))

@app.route('/recommendation')
def recommendation():
    """Show recommendation based on answers"""
    if 'session_id' not in session or not session.get('responses'):
        return redirect(url_for('index'))
    
    responses = session.get('responses', {})
    
    # Simple recommendation logic based on first answer (role preference)
    preferred_role = responses.get('1', 'Tank')
    
    # Find a champion that matches the preferred role
    recommended_champion = None
    for champion in SAMPLE_CHAMPIONS:
        if champion['role'] == preferred_role:
            recommended_champion = champion
            break
    
    # Default to first champion if no match
    if not recommended_champion:
        recommended_champion = SAMPLE_CHAMPIONS[0]
    
    # Create explanation
    explanation = f"Based on your preference for {preferred_role} champions, we recommend {recommended_champion['name']}!"
    confidence = 85  # Mock confidence score
    
    return render_template('recommendation.html',
                         champion=recommended_champion,
                         explanation=explanation,
                         confidence=confidence,
                         responses=responses)

@app.route('/retake')
def retake():
    """Clear session and start over"""
    session.clear()
    return redirect(url_for('index'))

@app.route('/health')
def health_check():
    """Health check endpoint"""
    return jsonify({
        'status': 'healthy',
        'timestamp': datetime.now().isoformat(),
        'version': '1.0.0-demo'
    })

if __name__ == '__main__':
    print("🎮 Starting LoL Champion Recommender (Demo Mode)")
    print("📝 This is a simplified version for demonstration")
    print("🌐 Open your browser and go to: http://localhost:5001")
    print("⚡ Press Ctrl+C to stop the server")
    print("-" * 50)
    
    app.run(host='0.0.0.0', port=5001, debug=False)