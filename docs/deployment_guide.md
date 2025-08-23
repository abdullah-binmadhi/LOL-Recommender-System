# LoL Champion Recommender - Deployment Guide

## Table of Contents

1. [Deployment Overview](#deployment-overview)
2. [Local Development Deployment](#local-development-deployment)
3. [Docker Deployment](#docker-deployment)
4. [Cloud Deployment](#cloud-deployment)
5. [Production Deployment](#production-deployment)
6. [Monitoring and Maintenance](#monitoring-and-maintenance)
7. [Scaling and Performance](#scaling-and-performance)
8. [Security Considerations](#security-considerations)

---

## Deployment Overview

### Deployment Options

1. **Local Development**: For development and testing
2. **Docker**: Containerized deployment for consistency
3. **Cloud Platforms**: AWS, Google Cloud, Azure, Heroku
4. **VPS/Dedicated Server**: Self-managed server deployment
5. **Kubernetes**: For large-scale, orchestrated deployments

### Architecture Components

```mermaid
graph TB
    A[Load Balancer] --> B[Web Server]
    B --> C[Flask Application]
    C --> D[ML Models]
    C --> E[Cache Layer]
    C --> F[Static Files]
    
    G[Monitoring] --> C
    H[Logging] --> C
    I[Backup] --> D
    I --> E
```

---

## Local Development Deployment

### Quick Start

```bash
# Clone repository
git clone https://github.com/your-username/lol-champion-recommender.git
cd lol-champion-recommender

# Setup virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Setup environment
cp .env.example .env
# Edit .env with your settings

# Initialize data and models
python scripts/setup_data.py
python scripts/train_models.py

# Run development server
python app.py
```

### Development Configuration

Create `.env` for development:

```bash
# Development Environment
FLASK_ENV=development
FLASK_DEBUG=True
SECRET_KEY=dev-secret-key-change-in-production

# Local paths
CHAMPION_DATA_PATH=data/champions.json
QUESTION_DATA_PATH=data/questions.json
ML_MODEL_PATH=models/

# Cache settings (development)
CACHE_DEFAULT_TTL=300
CACHE_MAX_MEMORY_SIZE=500
CACHE_COMPRESSION=false

# Logging
LOG_LEVEL=DEBUG
LOG_FILE=logs/development.log
```

---

## Docker Deployment

### Dockerfile

Create `Dockerfile`:

```dockerfile
FROM python:3.9-slim

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements first for better caching
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY . .

# Create necessary directories
RUN mkdir -p logs cache models data

# Set environment variables
ENV FLASK_APP=app.py
ENV FLASK_ENV=production
ENV PYTHONPATH=/app

# Expose port
EXPOSE 5000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:5000/health || exit 1

# Run application
CMD ["gunicorn", "--bind", "0.0.0.0:5000", "--workers", "4", "wsgi:app"]
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  web:
    build: .
    ports:
      - "5000:5000"
    environment:
      - FLASK_ENV=production
      - SECRET_KEY=${SECRET_KEY}
      - CACHE_DEFAULT_TTL=3600
    volumes:
      - ./data:/app/data:ro
      - ./models:/app/models:ro
      - ./logs:/app/logs
      - ./cache:/app/cache
    depends_on:
      - redis
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./static:/app/static:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - web
    restart: unless-stopped

volumes:
  redis_data:
```

### Build and Run

```bash
# Build image
docker build -t lol-recommender .

# Run with docker-compose
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f web

# Stop services
docker-compose down
```

### Production Docker Configuration

Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
  web:
    build: .
    environment:
      - FLASK_ENV=production
      - SECRET_KEY=${SECRET_KEY}
      - CACHE_DEFAULT_TTL=3600
      - CACHE_MAX_MEMORY_SIZE=2000
    volumes:
      - ./data:/app/data:ro
      - ./models:/app/models:ro
      - app_logs:/app/logs
      - app_cache:/app/cache
    deploy:
      replicas: 3
      resources:
        limits:
          memory: 1G
        reservations:
          memory: 512M
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    deploy:
      resources:
        limits:
          memory: 256M
    restart: unless-stopped

volumes:
  app_logs:
  app_cache:
  redis_data:
```

---

## Cloud Deployment

### Heroku Deployment

#### Setup Files

Create `Procfile`:
```
web: gunicorn wsgi:app
```

Create `runtime.txt`:
```
python-3.9.18
```

Create `app.json`:
```json
{
  "name": "LoL Champion Recommender",
  "description": "ML-powered League of Legends champion recommendation system",
  "repository": "https://github.com/your-username/lol-champion-recommender",
  "keywords": ["python", "flask", "machine-learning", "league-of-legends"],
  "env": {
    "SECRET_KEY": {
      "description": "Secret key for Flask sessions",
      "generator": "secret"
    },
    "FLASK_ENV": {
      "description": "Flask environment",
      "value": "production"
    }
  },
  "addons": [
    "heroku-redis:hobby-dev"
  ],
  "buildpacks": [
    {
      "url": "heroku/python"
    }
  ]
}
```

#### Deploy to Heroku

```bash
# Install Heroku CLI
# https://devcenter.heroku.com/articles/heroku-cli

# Login to Heroku
heroku login

# Create app
heroku create your-app-name

# Set environment variables
heroku config:set SECRET_KEY=$(python -c 'import secrets; print(secrets.token_hex())')
heroku config:set FLASK_ENV=production

# Add Redis addon
heroku addons:create heroku-redis:hobby-dev

# Deploy
git push heroku main

# Scale dynos
heroku ps:scale web=1

# View logs
heroku logs --tail
```

### AWS Deployment

#### Using AWS Elastic Beanstalk

Create `.ebextensions/01_packages.config`:
```yaml
packages:
  yum:
    git: []
    gcc: []
```

Create `.ebextensions/02_python.config`:
```yaml
option_settings:
  aws:elasticbeanstalk:container:python:
    WSGIPath: wsgi.py
  aws:elasticbeanstalk:application:environment:
    FLASK_ENV: production
    PYTHONPATH: /opt/python/current/app
```

Deploy:
```bash
# Install EB CLI
pip install awsebcli

# Initialize EB application
eb init

# Create environment
eb create production

# Deploy
eb deploy

# Open in browser
eb open
```

#### Using AWS ECS (Docker)

Create `task-definition.json`:
```json
{
  "family": "lol-recommender",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "512",
  "memory": "1024",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "web",
      "image": "your-account.dkr.ecr.region.amazonaws.com/lol-recommender:latest",
      "portMappings": [
        {
          "containerPort": 5000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "FLASK_ENV",
          "value": "production"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/lol-recommender",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

### Google Cloud Platform

#### Using Google App Engine

Create `app.yaml`:
```yaml
runtime: python39

env_variables:
  SECRET_KEY: "your-secret-key"
  FLASK_ENV: "production"

automatic_scaling:
  min_instances: 1
  max_instances: 10
  target_cpu_utilization: 0.6

resources:
  cpu: 1
  memory_gb: 1
  disk_size_gb: 10
```

Deploy:
```bash
# Install Google Cloud SDK
# https://cloud.google.com/sdk/docs/install

# Authenticate
gcloud auth login

# Set project
gcloud config set project your-project-id

# Deploy
gcloud app deploy

# View logs
gcloud app logs tail -s default
```

---

## Production Deployment

### Server Setup (Ubuntu 20.04)

#### Initial Server Configuration

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install required packages
sudo apt install -y python3 python3-pip python3-venv nginx git supervisor

# Create application user
sudo useradd -m -s /bin/bash lolapp
sudo usermod -aG sudo lolapp

# Switch to application user
sudo su - lolapp
```

#### Application Setup

```bash
# Clone repository
git clone https://github.com/your-username/lol-champion-recommender.git
cd lol-champion-recommender

# Create virtual environment
python3 -m venv venv
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt
pip install gunicorn

# Setup directories
mkdir -p logs cache
chmod 755 logs cache

# Setup environment
cp .env.example .env
# Edit .env with production settings

# Initialize data
python scripts/setup_data.py
python scripts/train_models.py

# Test application
python app.py --test
```

#### Gunicorn Configuration

Create `/home/lolapp/lol-champion-recommender/gunicorn.conf.py`:

```python
# Server socket
bind = "127.0.0.1:8000"
backlog = 2048

# Worker processes
workers = 4
worker_class = "sync"
worker_connections = 1000
timeout = 30
keepalive = 2
max_requests = 1000
max_requests_jitter = 50

# Restart workers after this many requests, with up to 50 requests variation
preload_app = True

# Logging
accesslog = "/home/lolapp/lol-champion-recommender/logs/access.log"
errorlog = "/home/lolapp/lol-champion-recommender/logs/error.log"
loglevel = "info"
access_log_format = '%(h)s %(l)s %(u)s %(t)s "%(r)s" %(s)s %(b)s "%(f)s" "%(a)s" %(D)s'

# Process naming
proc_name = "lol-recommender"

# Server mechanics
daemon = False
pidfile = "/home/lolapp/lol-champion-recommender/gunicorn.pid"
user = "lolapp"
group = "lolapp"
tmp_upload_dir = None

# SSL (if using HTTPS directly with Gunicorn)
# keyfile = "/path/to/keyfile"
# certfile = "/path/to/certfile"
```

#### Supervisor Configuration

Create `/etc/supervisor/conf.d/lol-recommender.conf`:

```ini
[program:lol-recommender]
command=/home/lolapp/lol-champion-recommender/venv/bin/gunicorn -c gunicorn.conf.py wsgi:app
directory=/home/lolapp/lol-champion-recommender
user=lolapp
autostart=true
autorestart=true
redirect_stderr=true
stdout_logfile=/home/lolapp/lol-champion-recommender/logs/supervisor.log
environment=PATH="/home/lolapp/lol-champion-recommender/venv/bin"
```

Start supervisor:
```bash
sudo supervisorctl reread
sudo supervisorctl update
sudo supervisorctl start lol-recommender
sudo supervisorctl status
```

#### Nginx Configuration

Create `/etc/nginx/sites-available/lol-recommender`:

```nginx
upstream app_server {
    server 127.0.0.1:8000 fail_timeout=0;
}

server {
    listen 80;
    server_name your-domain.com www.your-domain.com;
    
    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com www.your-domain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Security headers
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json;

    # Client max body size
    client_max_body_size 16M;

    # Static files
    location /static {
        alias /home/lolapp/lol-champion-recommender/static;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Main application
    location / {
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header Host $http_host;
        proxy_redirect off;
        proxy_buffering off;
        proxy_pass http://app_server;
    }

    # Health check endpoint
    location /health {
        access_log off;
        proxy_pass http://app_server;
    }
}
```

Enable site:
```bash
sudo ln -s /etc/nginx/sites-available/lol-recommender /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Monitoring and Maintenance

### Application Monitoring

#### Health Check Endpoint

Add to `app.py`:
```python
@app.route('/health')
def health_check():
    """Health check endpoint for monitoring"""
    try:
        # Check database connection
        # Check ML models
        # Check cache
        
        return jsonify({
            'status': 'healthy',
            'timestamp': datetime.now().isoformat(),
            'version': '1.0.0',
            'checks': {
                'database': 'ok',
                'ml_models': 'ok',
                'cache': 'ok'
            }
        }), 200
    except Exception as e:
        return jsonify({
            'status': 'unhealthy',
            'error': str(e),
            'timestamp': datetime.now().isoformat()
        }), 500
```

#### Monitoring Script

Create `scripts/monitor.py`:
```python
#!/usr/bin/env python3
"""
Application monitoring script
"""

import requests
import time
import logging
from datetime import datetime

def check_health():
    """Check application health"""
    try:
        response = requests.get('http://localhost:5000/health', timeout=10)
        if response.status_code == 200:
            data = response.json()
            if data['status'] == 'healthy':
                logging.info("Health check passed")
                return True
            else:
                logging.error(f"Health check failed: {data}")
                return False
        else:
            logging.error(f"Health check returned {response.status_code}")
            return False
    except Exception as e:
        logging.error(f"Health check exception: {e}")
        return False

def main():
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s %(levelname)s %(message)s',
        handlers=[
            logging.FileHandler('logs/monitor.log'),
            logging.StreamHandler()
        ]
    )
    
    while True:
        if not check_health():
            # Send alert (email, Slack, etc.)
            logging.critical("Application is unhealthy!")
        
        time.sleep(60)  # Check every minute

if __name__ == '__main__':
    main()
```

### Log Management

#### Log Rotation

Create `/etc/logrotate.d/lol-recommender`:
```
/home/lolapp/lol-champion-recommender/logs/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 lolapp lolapp
    postrotate
        supervisorctl restart lol-recommender
    endscript
}
```

#### Centralized Logging

For production, consider using:
- **ELK Stack** (Elasticsearch, Logstash, Kibana)
- **Fluentd** with cloud logging services
- **Sentry** for error tracking

### Backup Strategy

#### Database Backup (if applicable)

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/home/lolapp/backups"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p $BACKUP_DIR

# Backup models
tar -czf $BACKUP_DIR/models_$DATE.tar.gz models/

# Backup data
tar -czf $BACKUP_DIR/data_$DATE.tar.gz data/

# Backup configuration
cp .env $BACKUP_DIR/env_$DATE

# Clean old backups (keep 30 days)
find $BACKUP_DIR -name "*.tar.gz" -mtime +30 -delete
find $BACKUP_DIR -name "env_*" -mtime +30 -delete

echo "Backup completed: $DATE"
```

Add to crontab:
```bash
# Daily backup at 2 AM
0 2 * * * /home/lolapp/lol-champion-recommender/scripts/backup.sh
```

---

## Scaling and Performance

### Horizontal Scaling

#### Load Balancer Configuration

Using HAProxy (`/etc/haproxy/haproxy.cfg`):
```
global
    daemon
    maxconn 4096

defaults
    mode http
    timeout connect 5000ms
    timeout client 50000ms
    timeout server 50000ms

frontend web_frontend
    bind *:80
    default_backend web_servers

backend web_servers
    balance roundrobin
    server web1 127.0.0.1:8001 check
    server web2 127.0.0.1:8002 check
    server web3 127.0.0.1:8003 check
```

#### Multiple Gunicorn Instances

Create multiple supervisor configurations:
```ini
[program:lol-recommender-8001]
command=/home/lolapp/lol-champion-recommender/venv/bin/gunicorn -b 127.0.0.1:8001 -w 2 wsgi:app
directory=/home/lolapp/lol-champion-recommender
user=lolapp
autostart=true
autorestart=true

[program:lol-recommender-8002]
command=/home/lolapp/lol-champion-recommender/venv/bin/gunicorn -b 127.0.0.1:8002 -w 2 wsgi:app
directory=/home/lolapp/lol-champion-recommender
user=lolapp
autostart=true
autorestart=true
```

### Vertical Scaling

#### Resource Optimization

1. **Memory Optimization**:
   ```python
   # Optimize cache settings
   CACHE_MAX_MEMORY_SIZE = 4000  # Increase for more memory
   CACHE_DEFAULT_TTL = 7200      # Longer TTL for stable data
   ```

2. **CPU Optimization**:
   ```python
   # Increase worker processes
   workers = multiprocessing.cpu_count() * 2 + 1
   ```

3. **I/O Optimization**:
   ```python
   # Use async workers for I/O bound tasks
   worker_class = "gevent"
   worker_connections = 1000
   ```

### Caching Strategy

#### Redis Configuration

```redis
# /etc/redis/redis.conf

# Memory settings
maxmemory 2gb
maxmemory-policy allkeys-lru

# Persistence
save 900 1
save 300 10
save 60 10000

# Network
bind 127.0.0.1
port 6379
timeout 300

# Security
requirepass your-redis-password
```

#### Application Cache Configuration

```python
# config.py
import redis

class ProductionConfig(Config):
    # Redis cache
    CACHE_TYPE = 'redis'
    CACHE_REDIS_URL = 'redis://localhost:6379/0'
    CACHE_DEFAULT_TTL = 3600
    
    # Session storage
    SESSION_TYPE = 'redis'
    SESSION_REDIS = redis.from_url('redis://localhost:6379/1')
```

---

## Security Considerations

### Application Security

#### Environment Variables

```bash
# Production .env
SECRET_KEY=your-very-long-random-secret-key
FLASK_ENV=production
FLASK_DEBUG=False

# Database credentials (if applicable)
DATABASE_URL=postgresql://user:password@localhost/dbname

# API keys
EXTERNAL_API_KEY=your-api-key

# Security settings
WTF_CSRF_ENABLED=True
SESSION_COOKIE_SECURE=True
SESSION_COOKIE_HTTPONLY=True
```

#### Security Headers

```python
# app.py
from flask_talisman import Talisman

# Configure security headers
Talisman(app, {
    'force_https': True,
    'strict_transport_security': True,
    'content_security_policy': {
        'default-src': "'self'",
        'script-src': "'self' 'unsafe-inline'",
        'style-src': "'self' 'unsafe-inline'",
        'img-src': "'self' data:",
    }
})
```

### Server Security

#### Firewall Configuration

```bash
# UFW firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 'Nginx Full'
sudo ufw enable
```

#### SSL/TLS Configuration

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

#### System Updates

```bash
# Automatic security updates
sudo apt install unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

### Monitoring Security

#### Fail2Ban Configuration

```bash
# Install Fail2Ban
sudo apt install fail2ban

# Configure for Nginx
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local

# Edit /etc/fail2ban/jail.local
[nginx-http-auth]
enabled = true
port = http,https
logpath = /var/log/nginx/error.log

[nginx-limit-req]
enabled = true
port = http,https
logpath = /var/log/nginx/error.log
maxretry = 10
```

---

## Troubleshooting Deployment

### Common Issues

#### 1. Application Won't Start

```bash
# Check logs
sudo supervisorctl tail -f lol-recommender

# Check Gunicorn directly
cd /home/lolapp/lol-champion-recommender
source venv/bin/activate
gunicorn -b 127.0.0.1:8000 wsgi:app

# Check Python path
python -c "import sys; print(sys.path)"
```

#### 2. 502 Bad Gateway

```bash
# Check if Gunicorn is running
ps aux | grep gunicorn

# Check Nginx error logs
sudo tail -f /var/log/nginx/error.log

# Test upstream connection
curl http://127.0.0.1:8000/health
```

#### 3. High Memory Usage

```bash
# Monitor memory
htop
free -h

# Check for memory leaks
python -m memory_profiler app.py

# Restart application
sudo supervisorctl restart lol-recommender
```

#### 4. SSL Certificate Issues

```bash
# Check certificate status
sudo certbot certificates

# Test SSL configuration
openssl s_client -connect your-domain.com:443

# Renew certificate
sudo certbot renew --dry-run
```

### Performance Troubleshooting

#### Slow Response Times

1. **Check Application Performance**:
   ```bash
   # Profile application
   python -m cProfile -o profile.stats app.py
   ```

2. **Check Database Performance** (if applicable):
   ```bash
   # Monitor queries
   tail -f logs/app.log | grep "SLOW"
   ```

3. **Check Network**:
   ```bash
   # Test network latency
   ping your-domain.com
   traceroute your-domain.com
   ```

#### High CPU Usage

```bash
# Check processes
top -p $(pgrep -f gunicorn)

# Profile CPU usage
py-spy top --pid $(pgrep -f gunicorn)

# Check for infinite loops
strace -p $(pgrep -f gunicorn)
```

---

*Last updated: January 2024*
*Version: 1.0.0*