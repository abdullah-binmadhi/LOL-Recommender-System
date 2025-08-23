"""
Session test fixtures
Provides consistent session data for testing
"""

import uuid
from datetime import datetime, timedelta
from typing import Dict, List, Any, Optional
from models.user_response import UserSession, UserResponse

def generate_test_session_id() -> str:
    """Generate a test session ID"""
    return f"test_session_{uuid.uuid4().hex[:8]}"

def create_test_session(
    session_id: Optional[str] = None,
    responses: Optional[Dict[str, str]] = None,
    current_question: int = 1,
    completed: bool = False,
    created_at: Optional[datetime] = None
) -> UserSession:
    """Create a test user session"""
    if session_id is None:
        session_id = generate_test_session_id()
    
    if responses is None:
        responses = {}
    
    if created_at is None:
        created_at = datetime.now()
    
    # Convert responses dict to UserResponse objects
    user_responses = []
    for question_id, answer in responses.items():
        user_response = UserResponse(
            question_id=int(question_id),
            answer=answer,
            timestamp=created_at + timedelta(minutes=int(question_id))
        )
        user_responses.append(user_response)
    
    return UserSession(
        session_id=session_id,
        responses=user_responses,
        current_question=current_question,
        completed=completed,
        created_at=created_at
    )

# Pre-defined test sessions for different scenarios
TEST_SESSIONS = {
    "empty_session": {
        "responses": {},
        "current_question": 1,
        "completed": False
    },
    "partial_session": {
        "responses": {
            "1": "Tank",
            "2": "Easy",
            "3": "Team fights"
        },
        "current_question": 4,
        "completed": False
    },
    "complete_session": {
        "responses": {
            "1": "Tank",
            "2": "Easy", 
            "3": "Team fights",
            "4": "Melee",
            "5": "High",
            "6": "Defensive",
            "7": "Crowd Control",
            "8": "Beginner",
            "9": "Support team"
        },
        "current_question": 10,
        "completed": True
    },
    "tank_preference_session": {
        "responses": {
            "1": "Tank",
            "2": "Easy",
            "3": "Team fights",
            "4": "Melee",
            "5": "Medium",
            "6": "Defensive",
            "7": "Crowd Control",
            "8": "Beginner",
            "9": "Support team"
        },
        "current_question": 10,
        "completed": True
    },
    "damage_preference_session": {
        "responses": {
            "1": "Marksman",
            "2": "Moderate",
            "3": "Solo plays",
            "4": "Ranged",
            "5": "Very High",
            "6": "Offensive",
            "7": "Burst Damage",
            "8": "Intermediate",
            "9": "Winning games"
        },
        "current_question": 10,
        "completed": True
    },
    "support_preference_session": {
        "responses": {
            "1": "Support",
            "2": "Hard",
            "3": "Supporting allies",
            "4": "Ranged",
            "5": "Low",
            "6": "Defensive",
            "7": "Healing/Shielding",
            "8": "Advanced",
            "9": "Support team"
        },
        "current_question": 10,
        "completed": True
    },
    "mage_preference_session": {
        "responses": {
            "1": "Mage",
            "2": "Moderate",
            "3": "Team fights",
            "4": "Ranged",
            "5": "High",
            "6": "Offensive",
            "7": "Burst Damage",
            "8": "Intermediate",
            "9": "Learning new champions"
        },
        "current_question": 10,
        "completed": True
    }
}

def get_test_session(session_type: str) -> UserSession:
    """Get a pre-defined test session"""
    if session_type not in TEST_SESSIONS:
        raise ValueError(f"Unknown session type: {session_type}")
    
    session_data = TEST_SESSIONS[session_type]
    return create_test_session(**session_data)

def get_all_test_sessions() -> Dict[str, UserSession]:
    """Get all pre-defined test sessions"""
    return {name: get_test_session(name) for name in TEST_SESSIONS.keys()}

# Session state transitions for testing
SESSION_TRANSITIONS = [
    ("empty_session", "partial_session"),
    ("partial_session", "complete_session"),
    ("complete_session", "empty_session")  # Retake scenario
]

# Expected session progression
EXPECTED_PROGRESSION = {
    1: {"responses": {}, "current_question": 1},
    2: {"responses": {"1": "Tank"}, "current_question": 2},
    3: {"responses": {"1": "Tank", "2": "Easy"}, "current_question": 3},
    4: {"responses": {"1": "Tank", "2": "Easy", "3": "Team fights"}, "current_question": 4}
}

# Session validation rules
SESSION_VALIDATION_RULES = {
    "session_id_required": True,
    "session_id_format": "string",
    "responses_format": "dict",
    "current_question_min": 1,
    "current_question_max": 10,
    "started_at_required": True,
    "completed_at_when_completed": True
}

# Session timeout settings for testing
SESSION_TIMEOUTS = {
    "active_timeout": timedelta(hours=2),
    "inactive_timeout": timedelta(minutes=30),
    "cleanup_after": timedelta(days=1)
}

# Session history for testing retake functionality
SAMPLE_SESSION_HISTORY = [
    {
        "session_id": "session_001",
        "completed_at": datetime.now() - timedelta(days=1),
        "recommendation": "Malphite",
        "responses": {
            "1": "Tank",
            "2": "Easy",
            "3": "Team fights"
        }
    },
    {
        "session_id": "session_002", 
        "completed_at": datetime.now() - timedelta(hours=2),
        "recommendation": "Jinx",
        "responses": {
            "1": "Marksman",
            "2": "Moderate",
            "3": "Solo plays"
        }
    }
]

def create_session_with_history(current_session_type: str = "empty_session") -> Dict[str, Any]:
    """Create a session with history for testing"""
    current_session = get_test_session(current_session_type)
    
    return {
        "current_session": current_session,
        "session_history": SAMPLE_SESSION_HISTORY,
        "total_sessions": len(SAMPLE_SESSION_HISTORY) + 1
    }

# Performance test sessions
PERFORMANCE_TEST_SESSIONS = {
    "concurrent_user_1": get_test_session("tank_preference_session"),
    "concurrent_user_2": get_test_session("damage_preference_session"),
    "concurrent_user_3": get_test_session("support_preference_session"),
    "concurrent_user_4": get_test_session("mage_preference_session"),
    "concurrent_user_5": get_test_session("complete_session")
}

# Edge case sessions for testing
EDGE_CASE_SESSIONS = {
    "invalid_responses": {
        "responses": {
            "1": "InvalidRole",
            "2": "InvalidDifficulty",
            "3": "InvalidPlaystyle"
        },
        "current_question": 4,
        "completed": False
    },
    "missing_required_responses": {
        "responses": {
            "2": "Easy",
            "4": "Melee"
        },
        "current_question": 5,
        "completed": False
    },
    "out_of_order_responses": {
        "responses": {
            "5": "High",
            "2": "Easy",
            "8": "Beginner",
            "1": "Tank"
        },
        "current_question": 6,
        "completed": False
    }
}