from flask import Flask, render_template, request, session, redirect, url_for, flash, jsonify
import os
import uuid
from datetime import datetime
from dotenv import load_dotenv
from config import config
from services.question_service import QuestionService
from services.champion_service import ChampionService
from services.recommendation_engine import RecommendationEngine
from services.champion_details_service import ChampionDetailsService
from models.user_response import UserSession, UserResponse
from utils.error_handler import (
    handle_errors, handle_api_errors, validate_session, validate_responses,
    validate_question_answer, log_user_action, AppError, ValidationError,
    SessionError, DataError, MLError
)
from utils.cache_manager import cache_manager, performance_monitor, timed
from services.performance_service import performance_service

# Load environment variables
load_dotenv()

app = Flask(__name__)

# Load configuration
config_name = os.environ.get('FLASK_ENV', 'development')
app.config.from_object(config.get(config_name, config['default']))

# Configure static file caching
@app.after_request
def after_request(response):
    """Add performance headers"""
    # Cache static files
    if request.endpoint == 'static':
        response.cache_control.max_age = app.config['SEND_FILE_MAX_AGE_DEFAULT']
        response.cache_control.public = True
    
    # Add performance headers
    response.headers['X-Content-Type-Options'] = 'nosniff'
    response.headers['X-Frame-Options'] = 'DENY'
    response.headers['X-XSS-Protection'] = '1; mode=block'
    
    return response

# Initialize services
question_service = QuestionService()
champion_service = ChampionService()
recommendation_engine = RecommendationEngine()
champion_details_service = ChampionDetailsService()

# Start performance monitoring
performance_service.start_monitoring()

@app.route('/')
def index():
    """Landing page with start button"""
    # Clear any existing session
    session.clear()
    return render_template('index.html')

@app.route('/start')
@handle_errors(fallback_route='index', flash_message='Error starting questionnaire. Please try again.')
def start_questionnaire():
    """Initialize questionnaire session"""
    # Validate that questions are available
    total_questions = question_service.get_total_questions()
    if total_questions == 0:
        raise DataError('No questions available. Please contact support.', 'NO_QUESTIONS')
    
    # Create new session
    session_id = str(uuid.uuid4())
    session['session_id'] = session_id
    session['current_question'] = 1
    session['responses'] = {}
    session['started_at'] = datetime.now().isoformat()
    session['total_questions'] = total_questions
    
    log_user_action('questionnaire_started', session_id, {'total_questions': total_questions})
    flash(f'Questionnaire started! {total_questions} questions to go.', 'info')
    return redirect(url_for('questionnaire'))

@app.route('/questionnaire')
@handle_errors(fallback_route='index', flash_message='Error loading question. Please restart the questionnaire.')
@timed("questionnaire_load")
def questionnaire():
    """Display current question"""
    validate_session(session)
    
    current_question_id = session.get('current_question', 1)
    total_questions = session.get('total_questions', question_service.get_total_questions())
    
    # Check if questionnaire is complete
    if current_question_id > total_questions:
        return redirect(url_for('recommendation'))
    
    question = question_service.get_question_by_id(current_question_id)
    if not question:
        raise DataError('Question not found. Please restart the questionnaire.', 'QUESTION_NOT_FOUND')
    
    # Calculate progress
    progress = ((current_question_id - 1) / total_questions) * 100
    
    # Get previous answer if user is navigating back
    responses = session.get('responses', {})
    previous_answer = responses.get(str(current_question_id))
    
    log_user_action('question_viewed', session.get('session_id'), {
        'question_id': current_question_id,
        'progress': progress
    })
    
    return render_template('question.html', 
                         question=question,
                         current_question=current_question_id,
                         total_questions=total_questions,
                         progress=progress,
                         previous_answer=previous_answer,
                         can_go_back=current_question_id > 1)

@app.route('/answer', methods=['POST'])
@handle_errors(fallback_route='questionnaire', flash_message='Error processing your answer. Please try again.')
def submit_answer():
    """Process submitted answer"""
    validate_session(session)
    
    current_question_id = session.get('current_question', 1)
    answer = request.form.get('answer')
    action = request.form.get('action', 'next')  # next, previous, or finish
    
    # Validate answer if provided (not required for 'previous' action)
    if answer:
        question = question_service.get_question_by_id(current_question_id)
        if not question:
            raise DataError('Question not found', 'QUESTION_NOT_FOUND')
        
        validate_question_answer(question, answer)
        
        # Store answer
        responses = session.get('responses', {})
        responses[str(current_question_id)] = answer
        session['responses'] = responses
        session['last_answered'] = current_question_id
        
        log_user_action('answer_submitted', session.get('session_id'), {
            'question_id': current_question_id,
            'answer': answer,
            'action': action
        })
    elif action != 'previous':
        raise ValidationError('Please select an answer before continuing.')
    
    # Handle navigation
    total_questions = session.get('total_questions', question_service.get_total_questions())
    
    if action == 'previous' and current_question_id > 1:
        session['current_question'] = current_question_id - 1
        return redirect(url_for('questionnaire'))
    elif action == 'next' or action == 'finish':
        if current_question_id < total_questions:
            session['current_question'] = current_question_id + 1
            return redirect(url_for('questionnaire'))
        else:
            # All questions answered, mark as completed
            session['completed_at'] = datetime.now().isoformat()
            log_user_action('questionnaire_completed', session.get('session_id'), {
                'total_responses': len(session.get('responses', {}))
            })
            return redirect(url_for('recommendation'))
    else:
        return redirect(url_for('questionnaire'))

@app.route('/recommendation')
@handle_errors(fallback_route='index', flash_message='Error generating recommendation. Please try again.')
@timed("recommendation_generation")
def recommendation():
    """Display recommendation results using ML models"""
    validate_session(session)
    
    responses = session.get('responses', {})
    total_questions = question_service.get_total_questions()
    validate_responses(responses, total_questions)
    
    try:
        # Get ML-powered recommendation
        recommendation_result = recommendation_engine.predict_champion(responses)
        
        # Store recommendation in session for alternatives
        session['last_recommendation'] = recommendation_result.champion.name
        session['recommendation_time'] = datetime.now().isoformat()
        
        # Get model info for display
        model_info = recommendation_engine.get_model_info()
        
        log_user_action('recommendation_generated', session.get('session_id'), {
            'champion': recommendation_result.champion.name,
            'confidence': recommendation_result.confidence_score,
            'model_used': model_info.get('best_model', 'unknown')
        })
        
        return render_template('recommendation.html',
                             champion=recommendation_result.champion,
                             responses=responses,
                             confidence=recommendation_result.confidence_score * 100,
                             explanation=recommendation_result.explanation,
                             match_reasons=recommendation_result.match_reasons,
                             model_info=model_info)
                             
    except MLError as e:
        # ML-specific error handling with fallback
        log_user_action('ml_error_fallback', session.get('session_id'), {'error': str(e)})
        
        # Fallback to simple recommendation
        champions = champion_service.get_all_champions()
        if not champions:
            raise DataError('No champions available', 'NO_CHAMPIONS')
        
        fallback_champion = champions[0]
        flash('Using fallback recommendation due to ML system issue.', 'warning')
        
        return render_template('recommendation.html',
                             champion=fallback_champion,
                             responses=responses,
                             confidence=75.0,
                             explanation="Fallback recommendation due to system maintenance",
                             match_reasons=["System fallback - please try again later"],
                             model_info={'available_models': [], 'models_loaded': False})

@app.route('/previous')
def previous_question():
    """Navigate to previous question"""
    if 'session_id' not in session:
        return redirect(url_for('index'))
    
    current_question_id = session.get('current_question', 1)
    if current_question_id > 1:
        session['current_question'] = current_question_id - 1
        flash('Moved to previous question.', 'info')
    else:
        flash('Already at the first question.', 'warning')
    
    return redirect(url_for('questionnaire'))

@app.route('/skip')
def skip_question():
    """Skip current question (for optional questions)"""
    if 'session_id' not in session:
        return redirect(url_for('index'))
    
    current_question_id = session.get('current_question', 1)
    total_questions = session.get('total_questions', question_service.get_total_questions())
    
    # Mark as skipped
    responses = session.get('responses', {})
    responses[str(current_question_id)] = 'SKIPPED'
    session['responses'] = responses
    
    if current_question_id < total_questions:
        session['current_question'] = current_question_id + 1
        flash('Question skipped.', 'info')
        return redirect(url_for('questionnaire'))
    else:
        return redirect(url_for('recommendation'))

@app.route('/session-status')
def session_status():
    """Get current session status (for AJAX calls)"""
    if 'session_id' not in session:
        return jsonify({'status': 'no_session'})
    
    responses = session.get('responses', {})
    total_questions = session.get('total_questions', 0)
    current_question = session.get('current_question', 1)
    
    return jsonify({
        'status': 'active',
        'session_id': session.get('session_id'),
        'current_question': current_question,
        'total_questions': total_questions,
        'answered_questions': len(responses),
        'progress': (len(responses) / total_questions * 100) if total_questions > 0 else 0,
        'started_at': session.get('started_at'),
        'responses': responses
    })

@app.route('/alternatives')
def alternatives():
    """Get alternative champion recommendations with filtering options"""
    if 'session_id' not in session or 'responses' not in session:
        return redirect(url_for('index'))
    
    responses = session.get('responses', {})
    if len(responses) < question_service.get_total_questions():
        return redirect(url_for('questionnaire'))
    
    try:
        # Get filter parameters
        role_filter = request.args.get('role', '')
        difficulty_filter = request.args.get('difficulty', '')
        num_alternatives = int(request.args.get('num', 6))  # Default to 6 alternatives
        
        # Get alternative recommendations
        exclude_champions = [session.get('last_recommendation', '')]
        alternative_recommendations = recommendation_engine.get_alternative_recommendations(
            responses, exclude=exclude_champions, num_alternatives=num_alternatives
        )
        
        # Apply filters if specified
        if role_filter:
            alternative_recommendations = [
                alt for alt in alternative_recommendations 
                if alt.champion.role.lower() == role_filter.lower()
            ]
        
        if difficulty_filter:
            try:
                max_difficulty = int(difficulty_filter)
                alternative_recommendations = [
                    alt for alt in alternative_recommendations 
                    if alt.champion.difficulty <= max_difficulty
                ]
            except ValueError:
                pass
        
        # Get all available roles and difficulties for filter options
        all_champions = champion_service.get_all_champions()
        available_roles = sorted(list(set(c.role for c in all_champions)))
        available_difficulties = sorted(list(set(c.difficulty for c in all_champions)))
        
        return render_template('alternatives.html',
                             alternatives=alternative_recommendations,
                             responses=responses,
                             original_recommendation=session.get('last_recommendation'),
                             available_roles=available_roles,
                             available_difficulties=available_difficulties,
                             current_filters={
                                 'role': role_filter,
                                 'difficulty': difficulty_filter,
                                 'num': num_alternatives
                             })
                             
    except Exception as e:
        flash(f'Error getting alternatives: {str(e)}', 'error')
        return redirect(url_for('recommendation'))

@app.route('/api/alternatives')
def api_alternatives():
    """API endpoint for getting alternative recommendations"""
    if 'session_id' not in session or 'responses' not in session:
        return jsonify({'error': 'No active session'}), 400
    
    responses = session.get('responses', {})
    if len(responses) < question_service.get_total_questions():
        return jsonify({'error': 'Incomplete questionnaire'}), 400
    
    try:
        # Get parameters
        exclude_champions = request.args.getlist('exclude')
        if not exclude_champions:
            exclude_champions = [session.get('last_recommendation', '')]
        
        num_alternatives = int(request.args.get('num', 5))
        role_filter = request.args.get('role', '')
        
        # Get alternatives
        alternatives = recommendation_engine.get_alternative_recommendations(
            responses, exclude=exclude_champions, num_alternatives=num_alternatives
        )
        
        # Apply role filter if specified
        if role_filter:
            alternatives = [alt for alt in alternatives if alt.champion.role.lower() == role_filter.lower()]
        
        # Format response
        alternatives_data = []
        for alt in alternatives:
            alternatives_data.append({
                'champion': {
                    'name': alt.champion.name,
                    'title': alt.champion.title,
                    'role': alt.champion.role,
                    'difficulty': alt.champion.difficulty,
                    'attributes': getattr(alt.champion, 'attributes', {}),
                    'range_type': getattr(alt.champion, 'range_type', 'Unknown'),
                    'position': getattr(alt.champion, 'position', 'Any')
                },
                'confidence': alt.confidence_score,
                'explanation': alt.explanation,
                'match_reasons': alt.match_reasons
            })
        
        return jsonify({
            'alternatives': alternatives_data,
            'total': len(alternatives_data),
            'filters_applied': {
                'role': role_filter,
                'exclude': exclude_champions,
                'num_requested': num_alternatives
            }
        })
        
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/compare-models')
def compare_models():
    """Compare predictions from all available models"""
    if 'session_id' not in session or 'responses' not in session:
        return redirect(url_for('index'))
    
    responses = session.get('responses', {})
    if len(responses) < question_service.get_total_questions():
        return redirect(url_for('questionnaire'))
    
    try:
        # Get predictions from all models
        all_predictions = recommendation_engine.predict_with_all_models(responses)
        model_info = recommendation_engine.get_model_info()
        
        return render_template('model_comparison.html',
                             predictions=all_predictions,
                             responses=responses,
                             model_info=model_info)
                             
    except Exception as e:
        flash(f'Error comparing models: {str(e)}', 'error')
        return redirect(url_for('recommendation'))

@app.route('/api/recommendation', methods=['POST'])
@handle_api_errors
def api_recommendation():
    """API endpoint for getting recommendations"""
    try:
        data = request.get_json()
    except Exception as e:
        raise ValidationError('Invalid JSON format in request', 'INVALID_JSON')
    
    if not data or 'responses' not in data:
        raise ValidationError('Invalid request data: responses field required', 'MISSING_RESPONSES')
    
    responses = data['responses']
    if not isinstance(responses, dict) or not responses:
        raise ValidationError('Responses must be a non-empty dictionary', 'INVALID_RESPONSES')
    
    model_name = data.get('model', None)
    
    # Validate responses format
    for q_id, answer in responses.items():
        try:
            question_id = int(q_id)
            question = question_service.get_question_by_id(question_id)
            if question:
                validate_question_answer(question, answer)
        except (ValueError, TypeError):
            raise ValidationError(f'Invalid question ID: {q_id}', 'INVALID_QUESTION_ID')
    
    # Get recommendation
    recommendation_result = recommendation_engine.predict_champion(responses, model_name)
    
    # Get alternatives if requested
    alternatives = []
    if data.get('include_alternatives', False):
        alternatives = recommendation_engine.get_alternative_recommendations(
            responses, exclude=[recommendation_result.champion.name], num_alternatives=3
        )
    
    log_user_action('api_recommendation', None, {
        'model_name': model_name,
        'include_alternatives': data.get('include_alternatives', False),
        'champion': recommendation_result.champion.name
    })
    
    return jsonify({
        'success': True,
        'champion': {
            'name': recommendation_result.champion.name,
            'title': recommendation_result.champion.title,
            'role': recommendation_result.champion.role,
            'difficulty': recommendation_result.champion.difficulty,
            'attributes': getattr(recommendation_result.champion, 'attributes', {}),
            'range_type': getattr(recommendation_result.champion, 'range_type', 'Unknown'),
            'position': getattr(recommendation_result.champion, 'position', 'Any')
        },
        'confidence': recommendation_result.confidence_score,
        'explanation': recommendation_result.explanation,
        'match_reasons': recommendation_result.match_reasons,
        'alternatives': [
            {
                'name': alt.champion.name,
                'title': alt.champion.title,
                'role': alt.champion.role,
                'confidence': alt.confidence_score,
                'explanation': alt.explanation
            } for alt in alternatives
        ]
    })

@app.route('/champion/<champion_name>')
@handle_errors(fallback_route='index', flash_message='Error loading champion details.')
def champion_details(champion_name):
    """Display detailed champion information"""
    champion = champion_service.get_champion_by_name(champion_name)
    if not champion:
        flash(f'Champion "{champion_name}" not found.', 'error')
        return redirect(url_for('index'))
    
    # Get comprehensive champion details
    details = champion_details_service.get_champion_details(champion_name)
    
    log_user_action('champion_details_viewed', session.get('session_id'), {
        'champion': champion_name
    })
    
    return render_template('champion_details.html',
                         champion=champion,
                         details=details)

@app.route('/retake')
def retake():
    """Reset questionnaire and start over with session history"""
    # Store previous session data for comparison
    if 'session_id' in session and 'responses' in session:
        previous_session = {
            'session_id': session.get('session_id'),
            'responses': session.get('responses', {}),
            'last_recommendation': session.get('last_recommendation'),
            'completed_at': datetime.now().isoformat()
        }
        
        # Store in session history (keep last 3 sessions)
        session_history = session.get('session_history', [])
        session_history.append(previous_session)
        if len(session_history) > 3:
            session_history = session_history[-3:]
        
        # Clear current session but keep history
        old_session_data = dict(session)
        session.clear()
        session['session_history'] = session_history
        
        flash('Previous session saved. Starting fresh questionnaire!', 'info')
    else:
        session.clear()
        flash('Questionnaire reset. Ready for a new champion search!', 'info')
    
    return redirect(url_for('index'))

@app.route('/session-history')
def session_history():
    """View previous questionnaire sessions"""
    if 'session_history' not in session:
        flash('No previous sessions found.', 'info')
        return redirect(url_for('index'))
    
    history = session.get('session_history', [])
    
    # Get champion names for each session
    enhanced_history = []
    for hist_session in history:
        try:
            if hist_session.get('last_recommendation'):
                champion = champion_service.get_champion_by_name(hist_session['last_recommendation'])
                hist_session['champion'] = champion
            enhanced_history.append(hist_session)
        except Exception:
            enhanced_history.append(hist_session)
    
    return render_template('session_history.html', 
                         history=enhanced_history,
                         current_session=session.get('session_id'))

@app.route('/compare-with-previous/<int:history_index>')
def compare_with_previous(history_index):
    """Compare current responses with a previous session"""
    if 'session_history' not in session or 'responses' not in session:
        flash('No sessions to compare.', 'warning')
        return redirect(url_for('index'))
    
    history = session.get('session_history', [])
    if history_index >= len(history):
        flash('Invalid session selected.', 'error')
        return redirect(url_for('session_history'))
    
    current_responses = session.get('responses', {})
    previous_session = history[history_index]
    previous_responses = previous_session.get('responses', {})
    
    # Get recommendations for both sessions
    try:
        current_recommendation = recommendation_engine.predict_champion(current_responses)
        previous_recommendation = recommendation_engine.predict_champion(previous_responses)
        
        # Compare responses
        comparison_data = {
            'current': {
                'responses': current_responses,
                'recommendation': current_recommendation,
                'session_id': session.get('session_id')
            },
            'previous': {
                'responses': previous_responses,
                'recommendation': previous_recommendation,
                'session_id': previous_session.get('session_id'),
                'completed_at': previous_session.get('completed_at')
            }
        }
        
        return render_template('session_comparison.html', 
                             comparison=comparison_data,
                             questions=question_service.get_all_questions())
                             
    except Exception as e:
        flash(f'Error comparing sessions: {str(e)}', 'error')
        return redirect(url_for('session_history'))

@app.route('/restore-session/<int:history_index>')
def restore_session(history_index):
    """Restore a previous session"""
    if 'session_history' not in session:
        flash('No previous sessions found.', 'warning')
        return redirect(url_for('index'))
    
    history = session.get('session_history', [])
    if history_index >= len(history):
        flash('Invalid session selected.', 'error')
        return redirect(url_for('session_history'))
    
    # Restore the selected session
    previous_session = history[history_index]
    
    # Clear current session and restore previous one
    current_history = session.get('session_history', [])
    session.clear()
    
    session['session_id'] = str(uuid.uuid4())  # New session ID
    session['responses'] = previous_session.get('responses', {})
    session['current_question'] = len(previous_session.get('responses', {})) + 1
    session['total_questions'] = question_service.get_total_questions()
    session['session_history'] = current_history
    session['restored_from'] = previous_session.get('session_id')
    
    flash('Session restored successfully!', 'success')
    return redirect(url_for('recommendation'))

@app.route('/admin/performance')
def performance_dashboard():
    """Performance monitoring dashboard"""
    dashboard_data = performance_service.get_performance_dashboard()
    return render_template('admin/performance.html', data=dashboard_data)

@app.route('/admin/cache-stats')
def cache_stats():
    """Cache statistics endpoint"""
    stats = cache_manager.get_stats()
    performance_report = cache_manager.get_performance_report()
    
    return jsonify({
        'cache_stats': stats,
        'performance_report': performance_report
    })

@app.route('/admin/performance-metrics')
def performance_metrics():
    """Performance metrics endpoint"""
    metrics = performance_monitor.get_performance_report()
    return jsonify(metrics)

@app.route('/admin/optimize', methods=['POST'])
def force_optimization():
    """Force performance optimization"""
    optimization_type = request.json.get('type', 'all') if request.is_json else 'all'
    results = performance_service.force_optimization(optimization_type)
    return jsonify(results)

@app.route('/admin/clear-cache', methods=['POST'])
def clear_cache():
    """Clear all caches"""
    results = performance_service.clear_all_caches()
    return jsonify(results)

@app.errorhandler(404)
def not_found_error(error):
    log_user_action('404_error', session.get('session_id'), {
        'url': request.url,
        'referrer': request.referrer
    })
    return render_template('error.html', 
                         error_code=404, 
                         error_message="The page you're looking for doesn't exist"), 404

@app.errorhandler(500)
def internal_error(error):
    log_user_action('500_error', session.get('session_id'), {
        'url': request.url,
        'error': str(error)
    })
    return render_template('error.html', 
                         error_code=500, 
                         error_message="Internal server error occurred",
                         error_details=str(error) if app.debug else None), 500

@app.errorhandler(503)
def service_unavailable_error(error):
    log_user_action('503_error', session.get('session_id'), {
        'url': request.url
    })
    return render_template('error.html', 
                         error_code=503, 
                         error_message="Service temporarily unavailable"), 503

@app.errorhandler(ValidationError)
def handle_validation_error(error):
    log_user_action('validation_error', session.get('session_id'), {
        'error': error.message,
        'error_code': error.error_code
    })
    flash(error.message, 'error')
    return redirect(url_for('index'))

@app.errorhandler(SessionError)
def handle_session_error(error):
    log_user_action('session_error', session.get('session_id'), {
        'error': error.message,
        'error_code': error.error_code
    })
    flash(error.message, 'warning')
    return redirect(url_for('index'))

@app.errorhandler(MLError)
def handle_ml_error(error):
    log_user_action('ml_error', session.get('session_id'), {
        'error': error.message,
        'error_code': error.error_code
    })
    flash('AI system temporarily unavailable. Please try again later.', 'error')
    return redirect(url_for('index'))

@app.errorhandler(DataError)
def handle_data_error(error):
    log_user_action('data_error', session.get('session_id'), {
        'error': error.message,
        'error_code': error.error_code
    })
    flash('Data loading error. Please contact support if this persists.', 'error')
    return redirect(url_for('index'))

if __name__ == '__main__':
    app.run(debug=True)