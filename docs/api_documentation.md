# LoL Champion Recommender - API Documentation

## Overview

This document provides comprehensive API documentation for the LoL Champion Recommender application's internal services and external endpoints.

## Table of Contents

1. [Web API Endpoints](#web-api-endpoints)
2. [Internal Services](#internal-services)
3. [Data Models](#data-models)
4. [Error Handling](#error-handling)
5. [Performance Monitoring](#performance-monitoring)

---

## Web API Endpoints

### Authentication
All API endpoints are currently public. No authentication is required.

### Base URL
- Development: `http://localhost:5000`
- Production: `https://your-domain.com`

### Content Type
All API endpoints accept and return `application/json` unless otherwise specified.

---

### POST /api/recommendation

Get a champion recommendation based on user responses.

**Request Body:**
```json
{
  "responses": {
    "1": "Tank",
    "2": "Easy",
    "3": "Team fights"
  },
  "model": "random_forest",  // Optional: specific model to use
  "include_alternatives": true  // Optional: include alternative recommendations
}
```

**Response (Success - 200):**
```json
{
  "success": true,
  "champion": {
    "name": "Malphite",
    "title": "Shard of the Monolith",
    "role": "Tank",
    "difficulty": 2,
    "attributes": {
      "damage": 4,
      "toughness": 9,
      "control": 7,
      "mobility": 5,
      "utility": 7,
      "difficulty": 2
    },
    "range_type": "Melee",
    "position": "Top"
  },
  "confidence": 0.85,
  "explanation": "Random Forest analysis shows Malphite is ideal because: Perfect role alignment with Tank, Matches your preference for easier champions",
  "match_reasons": [
    "Role matches your preference for Tank champions",
    "Low difficulty suitable for beginners",
    "Strong team fight presence"
  ],
  "alternatives": [
    {
      "name": "Amumu",
      "title": "the Sad Mummy",
      "role": "Tank",
      "confidence": 0.78,
      "explanation": "Strong alternative tank with excellent team fight control"
    }
  ]
}
```

**Response (Error - 400):**
```json
{
  "success": false,
  "error": "Invalid request data: responses field required",
  "error_code": "MISSING_RESPONSES"
}
```

---

### GET /api/alternatives

Get alternative champion recommendations for the current session.

**Query Parameters:**
- `exclude` (array): Champion names to exclude from alternatives
- `num` (integer): Number of alternatives to return (default: 5)
- `role` (string): Filter alternatives by role

**Example:** `/api/alternatives?exclude=Malphite&exclude=Amumu&num=3&role=Tank`

**Response (Success - 200):**
```json
{
  "alternatives": [
    {
      "champion": {
        "name": "Shen",
        "title": "the Eye of Twilight",
        "role": "Tank",
        "difficulty": 3,
        "attributes": {...},
        "range_type": "Melee",
        "position": "Top"
      },
      "confidence": 0.82,
      "explanation": "Ensemble analysis identified Shen as your best match based on overall compatibility",
      "match_reasons": [
        "Tank role alignment",
        "Moderate difficulty level",
        "Strong utility for team"
      ]
    }
  ],
  "total": 3,
  "filters_applied": {
    "role": "Tank",
    "exclude": ["Malphite", "Amumu"],
    "num_requested": 3
  }
}
```

---

### GET /session-status

Get current session status and progress.

**Response (Success - 200):**
```json
{
  "status": "active",
  "session_id": "uuid-string",
  "current_question": 3,
  "total_questions": 9,
  "answered_questions": 2,
  "progress": 22.2,
  "started_at": "2024-01-15T10:30:00",
  "responses": {
    "1": "Tank",
    "2": "Easy"
  }
}
```

---

### GET /admin/cache-stats

Get cache performance statistics (admin endpoint).

**Response (Success - 200):**
```json
{
  "cache_stats": {
    "hits": 1250,
    "misses": 45,
    "sets": 890,
    "deletes": 12,
    "evictions": 5,
    "hit_rate": 96.5,
    "memory_entries": 450,
    "disk_entries": 890,
    "memory_size_mb": 12.5,
    "compression_enabled": true
  },
  "performance_report": {
    "basic_stats": {...},
    "top_accessed_keys": ["champion_data", "ml_predictions"],
    "efficiency_score": 96.5,
    "recommendations": [
      "Cache performance is excellent",
      "Consider increasing TTL for static data"
    ]
  }
}
```

---

### POST /admin/optimize

Force performance optimization (admin endpoint).

**Request Body:**
```json
{
  "type": "all"  // Options: "all", "cache", "performance", "preload"
}
```

**Response (Success - 200):**
```json
{
  "executed": ["cache_optimization", "performance_optimization"],
  "errors": []
}
```

---

## Internal Services

### ChampionService

Manages champion data loading and retrieval.

#### Methods

##### `get_all_champions() -> List[Champion]`
Returns all available champions.

**Caching:** 2 hours TTL, disk cache enabled
**Performance:** ~0.0001s with cache, ~0.001s without cache

```python
champion_service = ChampionService()
champions = champion_service.get_all_champions()
```

##### `get_champion_by_name(name: str) -> Optional[Champion]`
Retrieves a champion by name (case insensitive).

**Parameters:**
- `name`: Champion name to search for

**Returns:** Champion object or None if not found

```python
champion = champion_service.get_champion_by_name("Malphite")
```

##### `get_champions_by_role(role: str) -> List[Champion]`
Retrieves all champions with a specific role.

**Parameters:**
- `role`: Role to filter by (Tank, Fighter, Assassin, Mage, Marksman, Support)

```python
tanks = champion_service.get_champions_by_role("Tank")
```

##### `get_champions_by_difficulty(min_diff: int, max_diff: int) -> List[Champion]`
Retrieves champions within a difficulty range.

**Parameters:**
- `min_diff`: Minimum difficulty (1-10)
- `max_diff`: Maximum difficulty (1-10)

```python
easy_champions = champion_service.get_champions_by_difficulty(1, 3)
```

---

### RecommendationEngine

Handles ML-based champion recommendations.

#### Methods

##### `predict_champion(user_responses: Dict[str, str], model_name: Optional[str] = None) -> ChampionRecommendation`
Predicts the best champion for user responses.

**Caching:** 30 minutes TTL, cache key based on responses and model
**Performance:** ~0.0001s with cache, ~0.003s without cache

**Parameters:**
- `user_responses`: Dictionary mapping question IDs to answers
- `model_name`: Optional specific model to use

**Returns:** ChampionRecommendation object

```python
responses = {"1": "Tank", "2": "Easy", "3": "Team fights"}
recommendation = engine.predict_champion(responses)
```

##### `get_alternative_recommendations(user_responses: Dict[str, str], exclude: List[str] = None, num_alternatives: int = 3) -> List[ChampionRecommendation]`
Gets alternative champion recommendations.

**Parameters:**
- `user_responses`: User's questionnaire responses
- `exclude`: Champion names to exclude from results
- `num_alternatives`: Number of alternatives to return

```python
alternatives = engine.get_alternative_recommendations(
    responses, 
    exclude=["Malphite"], 
    num_alternatives=5
)
```

##### `get_model_info() -> Dict[str, Any]`
Returns information about loaded ML models.

```python
info = engine.get_model_info()
# Returns: {'available_models': ['random_forest', 'knn'], 'best_model': 'random_forest', ...}
```

---

### QuestionService

Manages questionnaire questions and validation.

#### Methods

##### `get_all_questions() -> List[Question]`
Returns all questionnaire questions.

**Caching:** 1 hour TTL

##### `get_question_by_id(question_id: int) -> Optional[Question]`
Retrieves a specific question by ID.

##### `get_total_questions() -> int`
Returns the total number of questions.

##### `validate_answer(question: Question, answer: str) -> bool`
Validates an answer for a specific question.

---

### PerformanceService

Monitors and optimizes application performance.

#### Methods

##### `get_performance_dashboard() -> Dict[str, Any]`
Returns comprehensive performance dashboard data.

```python
dashboard = performance_service.get_performance_dashboard()
```

##### `force_optimization(optimization_type: str = 'all') -> Dict[str, Any]`
Forces immediate performance optimization.

**Parameters:**
- `optimization_type`: Type of optimization ('all', 'cache', 'performance', 'preload')

##### `clear_all_caches() -> Dict[str, Any]`
Clears all application caches.

---

## Data Models

### Champion

Represents a League of Legends champion.

```python
class Champion:
    id: str                    # Unique identifier
    name: str                  # Champion name
    title: str                 # Champion title
    role: str                  # Primary role
    difficulty: int            # Difficulty rating (1-10)
    tags: List[str]           # Champion tags
    attributes: Dict[str, float]  # Attribute ratings
    abilities: List[Ability]   # Champion abilities
    range_type: str           # "Melee" or "Ranged"
    position: str             # Preferred position
```

### Question

Represents a questionnaire question.

```python
class Question:
    id: int                   # Question ID
    text: str                 # Question text
    question_type: str        # Type of question
    options: List[str]        # Available answer options
    required: bool            # Whether answer is required
    category: str             # Question category
```

### ChampionRecommendation

Represents a champion recommendation result.

```python
class ChampionRecommendation:
    champion: Champion        # Recommended champion
    confidence_score: float   # Confidence (0.0-1.0)
    explanation: str          # Human-readable explanation
    match_reasons: List[str]  # Specific match reasons
```

### UserSession

Represents a user's questionnaire session.

```python
class UserSession:
    session_id: str           # Unique session identifier
    user_responses: Dict[str, str]  # Question ID -> Answer mapping
    current_question: int     # Current question number
    started_at: datetime      # Session start time
    completed_at: Optional[datetime]  # Completion time
```

---

## Error Handling

### Error Response Format

All API errors follow this format:

```json
{
  "success": false,
  "error": "Human-readable error message",
  "error_code": "MACHINE_READABLE_CODE",
  "details": {
    "field": "Additional error details"
  }
}
```

### Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `INVALID_JSON` | Request body is not valid JSON | 400 |
| `MISSING_RESPONSES` | Required responses field missing | 400 |
| `INVALID_RESPONSES` | Responses format is invalid | 400 |
| `INVALID_QUESTION_ID` | Question ID is not valid | 400 |
| `NO_SESSION` | No active session found | 400 |
| `INCOMPLETE_QUESTIONNAIRE` | Questionnaire not completed | 400 |
| `QUESTION_NOT_FOUND` | Question does not exist | 404 |
| `CHAMPION_NOT_FOUND` | Champion does not exist | 404 |
| `NO_CHAMPIONS` | No champions available | 500 |
| `NO_QUESTIONS` | No questions available | 500 |
| `ML_ERROR` | Machine learning system error | 500 |
| `DATA_ERROR` | Data loading error | 500 |

### Custom Exception Classes

```python
class AppError(Exception):
    """Base application error"""
    pass

class ValidationError(AppError):
    """Input validation error"""
    pass

class SessionError(AppError):
    """Session-related error"""
    pass

class DataError(AppError):
    """Data loading/processing error"""
    pass

class MLError(AppError):
    """Machine learning error"""
    pass
```

---

## Performance Monitoring

### Metrics Tracked

1. **Cache Performance**
   - Hit rate, miss rate
   - Memory usage, disk usage
   - Operation timing

2. **Response Times**
   - Page load times
   - API response times
   - ML prediction times

3. **System Health**
   - Error rates
   - Concurrent users
   - Resource usage

### Performance Targets

| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| Cache Hit Rate | >80% | <70% |
| Average Response Time | <500ms | >1000ms |
| ML Prediction Time | <300ms | >500ms |
| API Response Time | <200ms | >400ms |
| Error Rate | <1% | >5% |

### Monitoring Endpoints

- `GET /admin/performance` - Performance dashboard
- `GET /admin/cache-stats` - Cache statistics
- `GET /admin/performance-metrics` - Detailed metrics

### Optimization Features

- Automatic cache optimization
- Performance alert system
- Trend analysis and recommendations
- Manual optimization triggers

---

## Rate Limiting

Currently, no rate limiting is implemented. For production deployment, consider implementing:

- Per-IP rate limiting
- Session-based rate limiting
- API key-based quotas

---

## Caching Strategy

### Cache Layers

1. **Memory Cache** (L1)
   - Fastest access
   - Limited size (configurable)
   - LRU eviction

2. **Disk Cache** (L2)
   - Persistent storage
   - Compression enabled
   - Automatic cleanup

### Cache Keys

- Champion data: `champions:all`
- Champion lookup: `champion_lookup:{name}`
- ML predictions: `ml_predictions:{response_hash}_{model}`
- Question data: `questions:all`

### TTL Settings

- Champion data: 2 hours
- ML predictions: 30 minutes
- Question data: 1 hour
- Performance metrics: 5 minutes

---

## Security Considerations

### Input Validation
- All user inputs are validated
- SQL injection prevention (not applicable - no SQL database)
- XSS prevention in templates

### Session Security
- Secure session cookies
- Session timeout
- CSRF protection (configurable)

### Headers
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`

---

## Deployment Notes

### Environment Variables

```bash
# Required
SECRET_KEY=your-secret-key-here

# Optional Performance Settings
CACHE_DEFAULT_TTL=3600
CACHE_MAX_MEMORY_SIZE=2000
CACHE_COMPRESSION=true
SEND_FILE_MAX_AGE_DEFAULT=31536000

# Flask Environment
FLASK_ENV=production  # or development
```

### Health Check Endpoint

Use `GET /` for basic health checks. Returns 200 if application is running.

### Logging

Application logs include:
- User actions and errors
- Performance metrics
- Cache operations
- ML model usage

---

## Support and Troubleshooting

### Common Issues

1. **Slow Performance**
   - Check cache hit rates
   - Monitor memory usage
   - Review performance dashboard

2. **ML Predictions Failing**
   - Verify model files exist
   - Check training data
   - Review error logs

3. **Session Issues**
   - Clear browser cookies
   - Check session configuration
   - Verify secret key

### Debug Mode

Enable debug mode for development:
```bash
export FLASK_ENV=development
```

This enables:
- Detailed error pages
- Auto-reload on code changes
- Debug toolbar (if installed)

---

*Last updated: January 2024*
*Version: 1.0.0*