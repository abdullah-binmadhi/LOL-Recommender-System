"""
Response test fixtures
Provides consistent response data for testing ML models and recommendations
"""

from typing import Dict, List, Any, Tuple
import numpy as np

# Standard response patterns for different champion types
RESPONSE_PATTERNS = {
    "tank_easy": {
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
    "tank_advanced": {
        "1": "Tank",
        "2": "Hard",
        "3": "Team fights", 
        "4": "Melee",
        "5": "Medium",
        "6": "Defensive",
        "7": "Crowd Control",
        "8": "Advanced",
        "9": "Improving skills"
    },
    "marksman_beginner": {
        "1": "Marksman",
        "2": "Easy",
        "3": "Farming",
        "4": "Ranged",
        "5": "Very High",
        "6": "Offensive",
        "7": "Sustained Damage",
        "8": "Beginner",
        "9": "Learning new champions"
    },
    "marksman_advanced": {
        "1": "Marksman",
        "2": "Hard",
        "3": "Solo plays",
        "4": "Ranged", 
        "5": "Very High",
        "6": "Offensive",
        "7": "Burst Damage",
        "8": "Expert",
        "9": "Winning games"
    },
    "mage_control": {
        "1": "Mage",
        "2": "Moderate",
        "3": "Team fights",
        "4": "Ranged",
        "5": "High",
        "6": "Offensive",
        "7": "Crowd Control",
        "8": "Intermediate",
        "9": "Having fun"
    },
    "mage_burst": {
        "1": "Mage",
        "2": "Hard",
        "3": "Solo plays",
        "4": "Ranged",
        "5": "Very High",
        "6": "Offensive", 
        "7": "Burst Damage",
        "8": "Advanced",
        "9": "Winning games"
    },
    "support_utility": {
        "1": "Support",
        "2": "Moderate",
        "3": "Supporting allies",
        "4": "Ranged",
        "5": "Low",
        "6": "Defensive",
        "7": "Healing/Shielding",
        "8": "Intermediate",
        "9": "Support team"
    },
    "support_engage": {
        "1": "Support",
        "2": "Hard",
        "3": "Team fights",
        "4": "Melee",
        "5": "Medium",
        "6": "Offensive",
        "7": "Crowd Control",
        "8": "Advanced",
        "9": "Winning games"
    },
    "fighter_bruiser": {
        "1": "Fighter",
        "2": "Moderate",
        "3": "Solo plays",
        "4": "Melee",
        "5": "High",
        "6": "Balanced",
        "7": "Sustained Damage",
        "8": "Intermediate",
        "9": "Improving skills"
    },
    "assassin_burst": {
        "1": "Assassin",
        "2": "Hard",
        "3": "Solo plays",
        "4": "Melee",
        "5": "Very High",
        "6": "Offensive",
        "7": "Burst Damage",
        "8": "Advanced",
        "9": "Winning games"
    }
}

# Expected recommendations for each response pattern
EXPECTED_RECOMMENDATIONS = {
    "tank_easy": "Malphite",
    "tank_advanced": "Amumu",
    "marksman_beginner": "Jinx",
    "marksman_advanced": "Jinx",
    "mage_control": "Lux",
    "mage_burst": "Lux", 
    "support_utility": "Thresh",
    "support_engage": "Thresh",
    "fighter_bruiser": None,  # No fighter in test data
    "assassin_burst": None   # No assassin in test data
}

# Confidence score ranges for different patterns
CONFIDENCE_RANGES = {
    "tank_easy": (0.8, 1.0),      # High confidence - clear match
    "tank_advanced": (0.7, 0.9),  # Good confidence
    "marksman_beginner": (0.7, 0.9),
    "marksman_advanced": (0.8, 1.0),
    "mage_control": (0.7, 0.9),
    "mage_burst": (0.6, 0.8),     # Lower confidence - less clear
    "support_utility": (0.8, 1.0),
    "support_engage": (0.6, 0.8),
    "fighter_bruiser": (0.0, 0.5), # Low confidence - no good match
    "assassin_burst": (0.0, 0.5)
}

def get_response_pattern(pattern_name: str) -> Dict[str, str]:
    """Get a specific response pattern"""
    if pattern_name not in RESPONSE_PATTERNS:
        raise ValueError(f"Unknown response pattern: {pattern_name}")
    return RESPONSE_PATTERNS[pattern_name].copy()

def get_all_response_patterns() -> Dict[str, Dict[str, str]]:
    """Get all response patterns"""
    return {name: pattern.copy() for name, pattern in RESPONSE_PATTERNS.items()}

def get_expected_recommendation(pattern_name: str) -> str:
    """Get expected recommendation for a response pattern"""
    return EXPECTED_RECOMMENDATIONS.get(pattern_name)

def get_confidence_range(pattern_name: str) -> Tuple[float, float]:
    """Get expected confidence range for a response pattern"""
    return CONFIDENCE_RANGES.get(pattern_name, (0.0, 1.0))

# Partial response patterns for testing incomplete questionnaires
PARTIAL_PATTERNS = {
    "tank_partial": {
        "1": "Tank",
        "2": "Easy",
        "3": "Team fights"
    },
    "damage_partial": {
        "1": "Marksman", 
        "2": "Moderate",
        "3": "Solo plays",
        "4": "Ranged"
    },
    "support_partial": {
        "1": "Support",
        "2": "Hard",
        "3": "Supporting allies",
        "4": "Ranged",
        "5": "Low"
    }
}

# Invalid response patterns for testing error handling
INVALID_PATTERNS = {
    "invalid_role": {
        "1": "InvalidRole",
        "2": "Easy",
        "3": "Team fights"
    },
    "invalid_difficulty": {
        "1": "Tank",
        "2": "InvalidDifficulty", 
        "3": "Team fights"
    },
    "empty_responses": {},
    "none_values": {
        "1": None,
        "2": "Easy",
        "3": "Team fights"
    },
    "wrong_types": {
        "1": 123,
        "2": ["Easy"],
        "3": {"answer": "Team fights"}
    }
}

# Response variations for testing model robustness
RESPONSE_VARIATIONS = {
    "case_variations": {
        "1": "tank",      # lowercase
        "2": "EASY",      # uppercase
        "3": "Team Fights" # mixed case
    },
    "whitespace_variations": {
        "1": " Tank ",    # leading/trailing spaces
        "2": "Easy",
        "3": "Team fights"
    },
    "alternative_answers": {
        "1": "Tank",
        "2": "Beginner",  # Alternative to "Easy"
        "3": "Teamfights" # No space
    }
}

# Feature vectors for ML testing (encoded responses)
FEATURE_VECTORS = {
    "tank_easy": np.array([1, 0, 0, 0, 0, 0,  # Role: Tank
                          1, 0, 0, 0,          # Difficulty: Easy
                          1, 0, 0, 0, 0,       # Playstyle: Team fights
                          1, 0, 0,             # Range: Melee
                          0, 0, 1, 0, 0,       # Damage: Medium
                          1, 0, 0,             # Style: Defensive
                          1, 0, 0, 0, 0,       # Ability: Crowd Control
                          1, 0, 0, 0,          # Experience: Beginner
                          0, 0, 0, 0, 1]),     # Motivation: Support team
    
    "marksman_advanced": np.array([0, 0, 0, 0, 1, 0,  # Role: Marksman
                                  0, 0, 1, 0,          # Difficulty: Hard
                                  0, 1, 0, 0, 0,       # Playstyle: Solo plays
                                  0, 1, 0,             # Range: Ranged
                                  0, 0, 0, 0, 1,       # Damage: Very High
                                  0, 1, 0,             # Style: Offensive
                                  0, 1, 0, 0, 0,       # Ability: Burst Damage
                                  0, 0, 0, 1,          # Experience: Expert
                                  1, 0, 0, 0, 0])      # Motivation: Winning games
}

def get_feature_vector(pattern_name: str) -> np.ndarray:
    """Get encoded feature vector for a response pattern"""
    if pattern_name not in FEATURE_VECTORS:
        raise ValueError(f"No feature vector available for pattern: {pattern_name}")
    return FEATURE_VECTORS[pattern_name].copy()

# Training data for ML model testing
TRAINING_DATA = {
    "features": [
        FEATURE_VECTORS["tank_easy"],
        FEATURE_VECTORS["marksman_advanced"]
    ],
    "labels": [
        "Malphite",
        "Jinx"
    ]
}

def get_training_data() -> Tuple[np.ndarray, List[str]]:
    """Get training data for ML model testing"""
    features = np.array(TRAINING_DATA["features"])
    labels = TRAINING_DATA["labels"]
    return features, labels

# Response quality metrics for testing
RESPONSE_QUALITY = {
    "complete_responses": ["tank_easy", "marksman_advanced", "mage_control"],
    "partial_responses": ["tank_partial", "damage_partial", "support_partial"],
    "invalid_responses": ["invalid_role", "invalid_difficulty", "empty_responses"],
    "edge_case_responses": ["case_variations", "whitespace_variations"]
}

def get_responses_by_quality(quality: str) -> List[str]:
    """Get response pattern names by quality category"""
    return RESPONSE_QUALITY.get(quality, [])

# Performance test responses (for load testing)
PERFORMANCE_TEST_RESPONSES = [
    get_response_pattern("tank_easy"),
    get_response_pattern("marksman_advanced"), 
    get_response_pattern("mage_control"),
    get_response_pattern("support_utility"),
    get_response_pattern("tank_advanced")
]

def get_performance_test_responses() -> List[Dict[str, str]]:
    """Get responses for performance testing"""
    return [pattern.copy() for pattern in PERFORMANCE_TEST_RESPONSES]

# A/B testing response variants
AB_TEST_VARIANTS = {
    "variant_a": {
        "tank_preference": get_response_pattern("tank_easy"),
        "damage_preference": get_response_pattern("marksman_beginner")
    },
    "variant_b": {
        "tank_preference": get_response_pattern("tank_advanced"),
        "damage_preference": get_response_pattern("marksman_advanced")
    }
}

def get_ab_test_variant(variant: str) -> Dict[str, Dict[str, str]]:
    """Get A/B test variant responses"""
    if variant not in AB_TEST_VARIANTS:
        raise ValueError(f"Unknown A/B test variant: {variant}")
    return AB_TEST_VARIANTS[variant]