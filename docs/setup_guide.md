# LoL Champion Recommender - Setup Guide

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Development Setup](#development-setup)
5. [Production Setup](#production-setup)
6. [Testing](#testing)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### System Requirements

- **Python**: 3.8 or higher
- **Operating System**: Windows 10+, macOS 10.14+, or Linux (Ubuntu 18.04+)
- **Memory**: Minimum 2GB RAM, recommended 4GB+
- **Storage**: Minimum 1GB free space
- **Network**: Internet connection for initial setup and data downloads

### Required Software

1. **Python 3.8+**
   ```bash
   # Check Python version
   python --version
   # or
   python3 --version
   ```

2. **pip** (Python package manager)
   ```bash
   # Check pip version
   pip --version
   ```

3. **Git** (for cloning repository)
   ```bash
   # Check Git version
   git --version
   ```

### Optional but Recommended

- **Virtual Environment Tool**: `venv`, `virtualenv`, or `conda`
- **Code Editor**: VS Code, PyCharm, or similar
- **Browser**: Chrome, Firefox, Safari, or Edge (for testing)

---

## Installation

### 1. Clone the Repository

```bash
# Clone the repository
git clone https://github.com/your-username/lol-champion-recommender.git

# Navigate to project directory
cd lol-champion-recommender
```

### 2. Create Virtual Environment

#### Using venv (Recommended)
```bash
# Create virtual environment
python -m venv venv

# Activate virtual environment
# On Windows:
venv\Scripts\activate
# On macOS/Linux:
source venv/bin/activate
```

#### Using conda
```bash
# Create conda environment
conda create -n lol-recommender python=3.9

# Activate environment
conda activate lol-recommender
```

### 3. Install Dependencies

```bash
# Install required packages
pip install -r requirements.txt

# Verify installation
pip list
```

### 4. Verify Installation

```bash
# Run basic tests
python -m pytest tests/test_models.py -v

# Check if Flask app starts
python app.py --check
```

---

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Copy example environment file
cp .env.example .env
```

Edit `.env` with your settings:

```bash
# Flask Configuration
SECRET_KEY=your-secret-key-here
FLASK_ENV=development  # or production
FLASK_DEBUG=True       # Set to False in production

# Application Settings
APP_NAME=LoL Champion Recommender
APP_VERSION=1.0.0

# Cache Configuration
CACHE_DEFAULT_TTL=3600
CACHE_MAX_MEMORY_SIZE=2000
CACHE_COMPRESSION=true

# Performance Settings
SEND_FILE_MAX_AGE_DEFAULT=31536000
MAX_CONTENT_LENGTH=16777216

# Logging
LOG_LEVEL=INFO
LOG_FILE=logs/app.log

# ML Model Settings
ML_MODEL_PATH=models/
DEFAULT_MODEL=random_forest
MODEL_CACHE_TTL=1800

# Data Settings
CHAMPION_DATA_PATH=data/champions.json
QUESTION_DATA_PATH=data/questions.json
```

### Configuration Files

#### config.py
The main configuration file contains environment-specific settings:

```python
import os
from dotenv import load_dotenv

load_dotenv()

class Config:
    SECRET_KEY = os.environ.get('SECRET_KEY') or 'dev-secret-key'
    FLASK_ENV = os.environ.get('FLASK_ENV') or 'development'
    
    # Cache settings
    CACHE_DEFAULT_TTL = int(os.environ.get('CACHE_DEFAULT_TTL', 3600))
    CACHE_MAX_MEMORY_SIZE = int(os.environ.get('CACHE_MAX_MEMORY_SIZE', 2000))
    
    # ML settings
    ML_MODEL_PATH = os.environ.get('ML_MODEL_PATH', 'models/')
    DEFAULT_MODEL = os.environ.get('DEFAULT_MODEL', 'random_forest')

class DevelopmentConfig(Config):
    DEBUG = True
    TESTING = False

class ProductionConfig(Config):
    DEBUG = False
    TESTING = False

class TestingConfig(Config):
    TESTING = True
    WTF_CSRF_ENABLED = False
```

---

## Development Setup

### 1. Initialize Data

```bash
# Create necessary directories
mkdir -p data models logs cache

# Download champion data (if not included)
python scripts/download_data.py

# Initialize ML models
python scripts/train_models.py
```

### 2. Run Development Server

```bash
# Start Flask development server
python app.py

# Or using Flask CLI
export FLASK_APP=app.py
flask run

# With specific host and port
flask run --host=0.0.0.0 --port=5000
```

### 3. Development Tools

#### Code Formatting
```bash
# Install development dependencies
pip install black flake8 isort

# Format code
black .

# Check code style
flake8 .

# Sort imports
isort .
```

#### Pre-commit Hooks
```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

### 4. Database Setup (if applicable)

```bash
# Initialize database
python scripts/init_db.py

# Run migrations
python scripts/migrate_db.py
```

---

## Production Setup

### 1. Server Requirements

#### Minimum Specifications
- **CPU**: 2 cores
- **RAM**: 4GB
- **Storage**: 10GB SSD
- **Network**: 100 Mbps

#### Recommended Specifications
- **CPU**: 4+ cores
- **RAM**: 8GB+
- **Storage**: 20GB+ SSD
- **Network**: 1 Gbps

### 2. Production Dependencies

```bash
# Install production WSGI server
pip install gunicorn

# Install production cache (optional)
pip install redis

# Install monitoring tools
pip install prometheus-client
```

### 3. WSGI Configuration

Create `wsgi.py`:

```python
from app import app

if __name__ == "__main__":
    app.run()
```

Create `gunicorn.conf.py`:

```python
# Gunicorn configuration
bind = "0.0.0.0:8000"
workers = 4
worker_class = "sync"
worker_connections = 1000
max_requests = 1000
max_requests_jitter = 100
timeout = 30
keepalive = 2
preload_app = True

# Logging
accesslog = "logs/access.log"
errorlog = "logs/error.log"
loglevel = "info"

# Process naming
proc_name = "lol-recommender"

# Security
limit_request_line = 4094
limit_request_fields = 100
limit_request_field_size = 8190
```

### 4. Run Production Server

```bash
# Using Gunicorn
gunicorn -c gunicorn.conf.py wsgi:app

# Or with custom settings
gunicorn --bind 0.0.0.0:8000 --workers 4 wsgi:app
```

### 5. Reverse Proxy (Nginx)

Create `/etc/nginx/sites-available/lol-recommender`:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /static {
        alias /path/to/your/app/static;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Security headers
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/lol-recommender /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 6. SSL Certificate (Let's Encrypt)

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

### 7. Process Management (systemd)

Create `/etc/systemd/system/lol-recommender.service`:

```ini
[Unit]
Description=LoL Champion Recommender
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/path/to/your/app
Environment=PATH=/path/to/your/app/venv/bin
ExecStart=/path/to/your/app/venv/bin/gunicorn -c gunicorn.conf.py wsgi:app
ExecReload=/bin/kill -s HUP $MAINPID
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable lol-recommender
sudo systemctl start lol-recommender
sudo systemctl status lol-recommender
```

---

## Testing

### Running Tests

```bash
# Run all tests
python -m pytest

# Run with verbose output
python -m pytest -v

# Run specific test file
python -m pytest tests/test_models.py

# Run with coverage
python -m pytest --cov=. --cov-report=html

# Run performance tests
python -m pytest tests/test_performance.py -v
```

### Test Categories

1. **Unit Tests**: Test individual components
   ```bash
   python -m pytest tests/test_models.py tests/test_services.py
   ```

2. **Integration Tests**: Test component interactions
   ```bash
   python -m pytest tests/test_recommendation_integration.py
   ```

3. **End-to-End Tests**: Test complete workflows
   ```bash
   python -m pytest tests/test_end_to_end.py
   ```

4. **Performance Tests**: Test system performance
   ```bash
   python -m pytest tests/test_performance.py
   ```

### Test Configuration

Create `pytest.ini`:

```ini
[tool:pytest]
testpaths = tests
python_files = test_*.py
python_classes = Test*
python_functions = test_*
addopts = 
    -v
    --tb=short
    --strict-markers
    --disable-warnings
markers =
    slow: marks tests as slow
    integration: marks tests as integration tests
    e2e: marks tests as end-to-end tests
    performance: marks tests as performance tests
```

---

## Troubleshooting

### Common Issues

#### 1. Import Errors

**Problem**: `ModuleNotFoundError` when running the application

**Solution**:
```bash
# Check Python path
echo $PYTHONPATH

# Add current directory to path
export PYTHONPATH="${PYTHONPATH}:$(pwd)"

# Or install in development mode
pip install -e .
```

#### 2. Port Already in Use

**Problem**: `Address already in use` error

**Solution**:
```bash
# Find process using port 5000
lsof -i :5000

# Kill the process
kill -9 <PID>

# Or use different port
flask run --port=5001
```

#### 3. Permission Errors

**Problem**: Permission denied when writing files

**Solution**:
```bash
# Check file permissions
ls -la

# Fix permissions
chmod 755 .
chmod 644 *.py

# Create directories with correct permissions
mkdir -p logs cache
chmod 755 logs cache
```

#### 4. Memory Issues

**Problem**: Application runs out of memory

**Solution**:
```bash
# Check memory usage
free -h

# Reduce cache size in config
export CACHE_MAX_MEMORY_SIZE=1000

# Use disk cache instead of memory
export CACHE_TYPE=disk
```

#### 5. ML Model Errors

**Problem**: Model fails to load or predict

**Solution**:
```bash
# Check model files exist
ls -la models/

# Retrain models
python scripts/train_models.py

# Check model format
python -c "import pickle; print(pickle.load(open('models/random_forest.pkl', 'rb')))"
```

### Performance Issues

#### Slow Response Times

1. **Enable Caching**:
   ```python
   # In config.py
   CACHE_DEFAULT_TTL = 3600
   CACHE_MAX_MEMORY_SIZE = 2000
   ```

2. **Optimize Database Queries**:
   ```bash
   # Check query performance
   python scripts/profile_queries.py
   ```

3. **Use Production WSGI Server**:
   ```bash
   # Don't use Flask dev server in production
   gunicorn wsgi:app
   ```

#### High Memory Usage

1. **Monitor Memory**:
   ```bash
   # Check memory usage
   ps aux | grep python
   ```

2. **Optimize Cache**:
   ```python
   # Reduce cache size
   CACHE_MAX_MEMORY_SIZE = 1000
   ```

3. **Use Garbage Collection**:
   ```python
   import gc
   gc.collect()
   ```

### Debugging

#### Enable Debug Mode

```bash
# Set environment variables
export FLASK_ENV=development
export FLASK_DEBUG=True

# Or in .env file
FLASK_ENV=development
FLASK_DEBUG=True
```

#### Logging

```python
import logging

# Configure logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s %(levelname)s %(name)s %(message)s',
    handlers=[
        logging.FileHandler('logs/debug.log'),
        logging.StreamHandler()
    ]
)
```

#### Profiling

```bash
# Install profiling tools
pip install py-spy

# Profile running application
py-spy top --pid <PID>

# Generate flame graph
py-spy record -o profile.svg --pid <PID>
```

### Getting Help

1. **Check Logs**:
   ```bash
   tail -f logs/app.log
   tail -f logs/error.log
   ```

2. **Run Health Check**:
   ```bash
   curl http://localhost:5000/health
   ```

3. **Check System Status**:
   ```bash
   python scripts/system_check.py
   ```

4. **Contact Support**:
   - GitHub Issues: [Repository Issues](https://github.com/your-username/lol-champion-recommender/issues)
   - Email: support@your-domain.com
   - Documentation: [Full Documentation](docs/)

---

## Next Steps

After successful setup:

1. **Customize Configuration**: Adjust settings in `config.py` and `.env`
2. **Add Your Data**: Replace sample data with your champion and question datasets
3. **Train Models**: Run model training with your data
4. **Test Thoroughly**: Run all test suites to ensure everything works
5. **Deploy**: Follow production setup guide for deployment
6. **Monitor**: Set up monitoring and logging for production use

---

*Last updated: January 2024*
*Version: 1.0.0*