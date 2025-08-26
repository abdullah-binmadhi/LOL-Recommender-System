"""
Comprehensive error handling utilities for the LoL Champion Recommender
"""

import logging
import traceback
from functools import wraps
from flask import flash, redirect, url_for, request, jsonify
from datetime import datetime
import os

# Configure logging
def setup_logging():
    """Setup application logging"""
    # Create logs directory if it doesn't exist
    os.makedirs('logs', exist_ok=True)
    
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler('logs/app.log'),
            logging.StreamHandler()
        ]
    )
    
    # Configure different log levels for different components
    logging.getLogger('werkzeug').setLevel(logging.WARNING)  # Reduce Flask logs
    logging.getLogger('urllib3').setLevel(logging.WARNING)   # Reduce HTTP logs

# Initialize logging
setup_logging()

logger = logging.getLogger(__name__)

class AppError(Exception):
    """Base application error class"""
    def __init__(self, message, error_code=None, details=None):
        self.message = message
        self.error_code = error_code
        self.details = details or {}
        self.timestamp = datetime.now()
        super().__init__(self.message)

class ValidationError(AppError):
    """Validation error for user input"""
    pass

class DataError(AppError):
    """Data processing or loading error"""
    pass

class MLError(AppError):
    """Machine learning model error"""
    pass

class SessionError(AppError):
    """Session management error"""
    pass

def handle_errors(fallback_route='index', flash_message=None):
    """Decorator for comprehensive error handling"""
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            try:
                return func(*args, **kwargs)
            except ValidationError as e:
                logger.warning(f"Validation error in {func.__name__}: {e.message}")
                flash(e.message, 'error')
                return redirect(url_for(fallback_route))
            except SessionError as e:
                logger.warning(f"Session error in {func.__name__}: {e.message}")
                flash(e.message, 'warning')
                return redirect(url_for('index'))
            except (DataError, MLError) as e:
                logger.error(f"System error in {func.__name__}: {e.message}")
                flash(flash_message or 'System error occurred. Please try again.', 'error')
                return redirect(url_for(fallback_route))
            except Exception as e:
                logger.error(f"Unexpected error in {func.__name__}: {str(e)}")
                logger.error(traceback.format_exc())
                flash('An unexpected error occurred. Please try again.', 'error')
                return redirect(url_for('index'))
        return wrapper
    return decorator

def handle_api_errors(func):
    """Decorator for API error handling"""
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except ValidationError as e:
            logger.warning(f"API validation error in {func.__name__}: {e.message}")
            return jsonify({
                'error': e.message,
                'error_code': e.error_code or 'VALIDATION_ERROR',
                'details': e.details
            }), 400
        except SessionError as e:
            logger.warning(f"API session error in {func.__name__}: {e.message}")
            return jsonify({
                'error': e.message,
                'error_code': e.error_code or 'SESSION_ERROR'
            }), 401
        except (DataError, MLError) as e:
            logger.error(f"API system error in {func.__name__}: {e.message}")
            return jsonify({
                'error': 'System error occurred',
                'error_code': e.error_code or 'SYSTEM_ERROR',
                'details': e.details
            }), 500
        except Exception as e:
            logger.error(f"API unexpected error in {func.__name__}: {str(e)}")
            logger.error(traceback.format_exc())
            return jsonify({
                'error': 'Internal server error',
                'error_code': 'INTERNAL_ERROR'
            }), 500
    return wrapper

def validate_session(session):
    """Validate session data"""
    if 'session_id' not in session:
        raise SessionError("No active session found. Please start a new questionnaire.")
    
    if 'responses' not in session:
        raise SessionError("Session data corrupted. Please start over.")
    
    return True

def validate_responses(responses, required_count):
    """Validate user responses"""
    if not responses:
        raise ValidationError("No responses found. Please complete the questionnaire.")
    
    if len(responses) < required_count:
        raise ValidationError(f"Incomplete questionnaire. Please answer all {required_count} questions.")
    
    return True

def validate_question_answer(question, answer):
    """Validate a specific question answer"""
    if not answer:
        raise ValidationError("Please select an answer before continuing.")
    
    if not question.is_valid_answer(answer):
        raise ValidationError(f"Invalid answer '{answer}' for question: {question.text}")
    
    return True

def log_user_action(action, session_id=None, details=None):
    """Log user actions for debugging and analytics"""
    log_data = {
        'action': action,
        'timestamp': datetime.now().isoformat(),
        'session_id': session_id,
        'ip_address': request.remote_addr if request else None,
        'user_agent': request.headers.get('User-Agent') if request else None,
        'details': details or {}
    }
    
    logger.info(f"User action: {action}", extra=log_data)

def create_error_response(error_type, message, details=None):
    """Create standardized error response"""
    return {
        'error': True,
        'error_type': error_type,
        'message': message,
        'details': details or {},
        'timestamp': datetime.now().isoformat()
    }



def handle_ml_errors(func):
    """Specific error handling for ML operations"""
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except FileNotFoundError as e:
            raise MLError(f"ML model files not found: {str(e)}", 'MODEL_NOT_FOUND')
        except ImportError as e:
            raise MLError(f"ML dependencies missing: {str(e)}", 'DEPENDENCIES_MISSING')
        except ValueError as e:
            if "n_splits" in str(e):
                raise MLError("Insufficient training data for model validation", 'INSUFFICIENT_DATA')
            raise MLError(f"ML model validation error: {str(e)}", 'VALIDATION_ERROR')
        except Exception as e:
            raise MLError(f"ML processing error: {str(e)}", 'PROCESSING_ERROR')
    return wrapper

def handle_data_errors(func):
    """Specific error handling for data operations"""
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except FileNotFoundError as e:
            raise DataError(f"Data file not found: {str(e)}", 'FILE_NOT_FOUND')
        except PermissionError as e:
            raise DataError(f"Permission denied accessing data: {str(e)}", 'PERMISSION_DENIED')
        except UnicodeDecodeError as e:
            raise DataError(f"Data encoding error: {str(e)}", 'ENCODING_ERROR')
        except KeyError as e:
            raise DataError(f"Missing required data field: {str(e)}", 'MISSING_FIELD')
        except Exception as e:
            raise DataError(f"Data processing error: {str(e)}", 'PROCESSING_ERROR')
    return wrapper

# Initialize logging when module is imported
setup_logging()