# LoL Champion Recommender - Troubleshooting Guide

## Table of Contents

1. [Quick Diagnostics](#quick-diagnostics)
2. [Application Issues](#application-issues)
3. [Performance Problems](#performance-problems)
4. [Data and ML Issues](#data-and-ml-issues)
5. [Deployment Issues](#deployment-issues)
6. [Development Issues](#development-issues)
7. [System Administration](#system-administration)
8. [Monitoring and Alerts](#monitoring-and-alerts)

---

## Quick Diagnostics

### Health Check Commands

```bash
# Basic application health
curl -f http://localhost:5000/health

# Check if all services are running
python scripts/system_check.py

# Test ML model functionality
python scripts/test_ml_models.py

# Verify data integrity
python scripts/verify_data.py
```

### Log Locations

```bash
# Application logs
tail -f logs/app.log

# Error logs
tail -f logs/error.log

# Access logs (if using Gunicorn)
tail -f logs/access.log

# System logs
sudo journalctl -u lol-recommender -f
```

### Common Status Checks

```bash
# Check process status
ps aux | grep -E "(python|gunicorn|flask)"

# Check port usage
netstat -tlnp | grep :5000

# Check memory usage
free -h
df -h

# Check system load
uptime
top
```

---

## Application Issues

### Startup Problems

#### Issue: Application Won't Start

**Symptoms**:
- Process exits immediately
- "Address already in use" error
- Import errors
- Configuration errors

**Diagnostic Steps**:
```bash
# Check if port is in use
lsof -i :5000

# Test Python imports
python -c "import app; print('OK')"

# Check configuration
python -c "from config import Config; print(Config.SECRET_KEY)"

# Verify dependencies
pip check
```

**Solutions**:
```bash
# Kill process using port
kill -9 $(lsof -t -i:5000)

# Fix import paths
export PYTHONPATH="${PYTHONPATH}:$(pwd)"

# Reinstall dependencies
pip install -r requirements.txt --force-reinstall

# Use different port
flask run --port=5001
```

#### Issue: Module Import Errors

**Symptoms**:
- `ModuleNotFoundError`
- `ImportError`
- Missing dependencies

**Diagnostic Steps**:
```bash
# Check Python path
python -c "import sys; print('\n'.join(sys.path))"

# Verify virtual environment
which python
pip list

# Test specific imports
python -c "import flask, sklearn, pandas"
```

**Solutions**:
```bash
# Activate virtual environment
source venv/bin/activate

# Install missing packages
pip install -r requirements.txt

# Fix Python path
export PYTHONPATH="${PYTHONPATH}:$(pwd)"

# Reinstall in development mode
pip install -e .
```

### Runtime Errors

#### Issue: 500 Internal Server Error

**Symptoms**:
- Server returns HTTP 500
- Error pages displayed
- Application crashes

**Diagnostic Steps**:
```bash
# Check error logs
tail -50 logs/error.log

# Enable debug mode
export FLASK_DEBUG=True

# Test with minimal configuration
python -c "from app import app; app.run(debug=True)"
```

**Solutions**:
```bash
# Fix configuration errors
# Check .env file and config.py

# Handle missing data files
python scripts/setup_data.py

# Restart application
sudo systemctl restart lol-recommender

# Clear cache if corrupted
rm -rf cache/*
```

#### Issue: Session Problems

**Symptoms**:
- Sessions not persisting
- User progress lost
- Session timeout errors

**Diagnostic Steps**:
```bash
# Check session configuration
python -c "from app import app; print(app.config['SECRET_KEY'])"

# Test session functionality
curl -c cookies.txt -b cookies.txt http://localhost:5000/start
```

**Solutions**:
```bash
# Set proper secret key
export SECRET_KEY=$(python -c 'import secrets; print(secrets.token_hex())')

# Check session storage
# Ensure Redis is running if using Redis sessions
redis-cli ping

# Clear session data
rm -rf flask_session/
```

### API Endpoint Issues

#### Issue: API Returns Errors

**Symptoms**:
- 400 Bad Request
- 404 Not Found
- Malformed JSON responses

**Diagnostic Steps**:
```bash
# Test API endpoints
curl -X POST -H "Content-Type: application/json" \
  -d '{"responses":{"1":"Tank","2":"Easy"}}' \
  http://localhost:5000/api/recommendation

# Check API logs
grep "API" logs/app.log

# Validate JSON
echo '{"test":"data"}' | python -m json.tool
```

**Solutions**:
```bash
# Fix JSON formatting
# Ensure proper Content-Type headers
# Validate request data structure

# Check route definitions
python -c "from app import app; print(app.url_map)"

# Test with curl
curl -v http://localhost:5000/api/recommendation
```

---

## Performance Problems

### Slow Response Times

#### Issue: Pages Load Slowly

**Symptoms**:
- Long page load times (>5 seconds)
- Timeouts
- Poor user experience

**Diagnostic Steps**:
```bash
# Measure response times
time curl http://localhost:5000/

# Check system resources
htop
iotop

# Profile application
python -m cProfile -o profile.stats app.py
```

**Solutions**:
```bash
# Enable caching
export CACHE_DEFAULT_TTL=3600

# Optimize database queries
# Add indexes, optimize SQL

# Use production WSGI server
gunicorn wsgi:app

# Enable compression
# Configure gzip in Nginx
```

#### Issue: High Memory Usage

**Symptoms**:
- System runs out of memory
- OOM killer activates
- Swap usage high

**Diagnostic Steps**:
```bash
# Check memory usage
free -h
ps aux --sort=-%mem | head

# Monitor memory over time
watch -n 5 'free -h'

# Check for memory leaks
python -m memory_profiler app.py
```

**Solutions**:
```bash
# Reduce cache size
export CACHE_MAX_MEMORY_SIZE=1000

# Optimize ML models
# Use smaller models or model compression

# Add more RAM
# Or use swap file
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

#### Issue: High CPU Usage

**Symptoms**:
- CPU usage consistently >80%
- System becomes unresponsive
- Slow application performance

**Diagnostic Steps**:
```bash
# Check CPU usage
top -p $(pgrep -f python)

# Profile CPU usage
py-spy top --pid $(pgrep -f gunicorn)

# Check for infinite loops
strace -p $(pgrep -f python)
```

**Solutions**:
```bash
# Optimize algorithms
# Cache expensive computations

# Scale horizontally
# Add more worker processes
gunicorn -w 4 wsgi:app

# Use async workers for I/O bound tasks
gunicorn -k gevent wsgi:app
```

### Cache Issues

#### Issue: Cache Not Working

**Symptoms**:
- Slow repeated requests
- High database load
- Cache miss rate >50%

**Diagnostic Steps**:
```bash
# Check cache status
curl http://localhost:5000/admin/cache-stats

# Test cache manually
python -c "from utils.cache_manager import cache; cache.set('test', 'value'); print(cache.get('test'))"

# Check Redis (if using)
redis-cli info stats
```

**Solutions**:
```bash
# Restart cache service
sudo systemctl restart redis

# Clear corrupted cache
redis-cli FLUSHALL

# Check cache configuration
python -c "from config import Config; print(Config.CACHE_TYPE)"

# Increase cache size
export CACHE_MAX_MEMORY_SIZE=2000
```

---

## Data and ML Issues

### Data Loading Problems

#### Issue: Champion Data Not Loading

**Symptoms**:
- "No champions available" error
- Empty champion lists
- Data validation errors

**Diagnostic Steps**:
```bash
# Check data files exist
ls -la data/

# Validate JSON structure
python -m json.tool data/champions.json

# Test data loading
python -c "from services.champion_service import ChampionService; cs = ChampionService(); print(len(cs.get_all_champions()))"
```

**Solutions**:
```bash
# Download/restore data files
python scripts/download_data.py

# Fix JSON formatting
python scripts/validate_data.py --fix

# Check file permissions
chmod 644 data/*.json

# Regenerate data if corrupted
python scripts/generate_sample_data.py
```

#### Issue: Question Data Problems

**Symptoms**:
- Questionnaire won't load
- Missing questions
- Validation errors

**Diagnostic Steps**:
```bash
# Check question data
python -c "from services.question_service import QuestionService; qs = QuestionService(); print(qs.get_total_questions())"

# Validate question structure
python scripts/validate_questions.py

# Check for missing fields
grep -n "required" data/questions.json
```

**Solutions**:
```bash
# Restore question data
cp data/questions.json.backup data/questions.json

# Regenerate questions
python scripts/generate_questions.py

# Fix validation rules
python scripts/fix_question_validation.py
```

### ML Model Issues

#### Issue: Model Won't Load

**Symptoms**:
- "Model not found" errors
- Prediction failures
- ML service unavailable

**Diagnostic Steps**:
```bash
# Check model files
ls -la models/

# Test model loading
python -c "import pickle; model = pickle.load(open('models/random_forest.pkl', 'rb')); print(type(model))"

# Check model format
file models/*.pkl
```

**Solutions**:
```bash
# Retrain models
python scripts/train_models.py

# Download pre-trained models
python scripts/download_models.py

# Check model compatibility
python scripts/verify_models.py

# Use fallback model
export DEFAULT_MODEL=simple_classifier
```

#### Issue: Poor Prediction Quality

**Symptoms**:
- Low confidence scores
- Irrelevant recommendations
- User complaints about accuracy

**Diagnostic Steps**:
```bash
# Test model accuracy
python scripts/evaluate_models.py

# Check training data quality
python scripts/analyze_training_data.py

# Compare model performance
python scripts/model_comparison.py
```

**Solutions**:
```bash
# Retrain with more data
python scripts/collect_training_data.py
python scripts/train_models.py

# Tune hyperparameters
python scripts/hyperparameter_tuning.py

# Use ensemble methods
export DEFAULT_MODEL=ensemble

# Update feature engineering
python scripts/update_features.py
```

#### Issue: Model Training Fails

**Symptoms**:
- Training script crashes
- Out of memory errors
- Convergence failures

**Diagnostic Steps**:
```bash
# Check training data
python scripts/validate_training_data.py

# Monitor memory during training
python -m memory_profiler scripts/train_models.py

# Check for data issues
python scripts/analyze_data_quality.py
```

**Solutions**:
```bash
# Reduce training data size
python scripts/sample_training_data.py --size 10000

# Use simpler models
export MODEL_TYPE=linear

# Increase memory
# Or use incremental learning
python scripts/incremental_training.py

# Fix data preprocessing
python scripts/preprocess_data.py --clean
```

---

## Deployment Issues

### Docker Problems

#### Issue: Container Won't Start

**Symptoms**:
- Container exits immediately
- Build failures
- Port binding errors

**Diagnostic Steps**:
```bash
# Check container logs
docker logs lol-recommender

# Inspect container
docker inspect lol-recommender

# Test build process
docker build -t lol-recommender . --no-cache
```

**Solutions**:
```bash
# Fix Dockerfile issues
# Check base image compatibility
# Ensure all dependencies are installed

# Use different port
docker run -p 5001:5000 lol-recommender

# Check resource limits
docker run --memory=2g lol-recommender

# Debug interactively
docker run -it lol-recommender /bin/bash
```

#### Issue: Docker Compose Problems

**Symptoms**:
- Services won't start
- Network connectivity issues
- Volume mounting problems

**Diagnostic Steps**:
```bash
# Check compose file
docker-compose config

# View service logs
docker-compose logs web

# Check service status
docker-compose ps
```

**Solutions**:
```bash
# Recreate services
docker-compose down
docker-compose up --build

# Fix volume permissions
sudo chown -R 1000:1000 ./data

# Check network connectivity
docker-compose exec web ping redis

# Update compose file
docker-compose -f docker-compose.yml -f docker-compose.override.yml up
```

### Production Deployment Issues

#### Issue: Nginx Configuration Problems

**Symptoms**:
- 502 Bad Gateway
- SSL certificate errors
- Static files not serving

**Diagnostic Steps**:
```bash
# Test Nginx configuration
sudo nginx -t

# Check Nginx logs
sudo tail -f /var/log/nginx/error.log

# Test upstream connection
curl http://127.0.0.1:8000/health
```

**Solutions**:
```bash
# Fix configuration syntax
sudo nginx -t

# Restart Nginx
sudo systemctl restart nginx

# Check SSL certificates
sudo certbot certificates

# Fix file permissions
sudo chown -R www-data:www-data /var/www/html
```

#### Issue: Systemd Service Problems

**Symptoms**:
- Service won't start
- Automatic restart failures
- Permission errors

**Diagnostic Steps**:
```bash
# Check service status
sudo systemctl status lol-recommender

# View service logs
sudo journalctl -u lol-recommender -f

# Test service file
sudo systemd-analyze verify lol-recommender.service
```

**Solutions**:
```bash
# Fix service file
sudo systemctl daemon-reload

# Check user permissions
sudo -u lolapp whoami

# Fix file paths
# Ensure all paths in service file are absolute

# Restart service
sudo systemctl restart lol-recommender
```

---

## Development Issues

### Environment Setup Problems

#### Issue: Virtual Environment Issues

**Symptoms**:
- Package conflicts
- Wrong Python version
- Import errors

**Diagnostic Steps**:
```bash
# Check Python version
python --version

# Check virtual environment
which python
pip list

# Test package imports
python -c "import flask, sklearn, pandas"
```

**Solutions**:
```bash
# Recreate virtual environment
rm -rf venv
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# Use specific Python version
python3.9 -m venv venv

# Fix package conflicts
pip install --upgrade pip
pip install -r requirements.txt --force-reinstall
```

#### Issue: Database Migration Problems

**Symptoms**:
- Migration failures
- Schema conflicts
- Data corruption

**Diagnostic Steps**:
```bash
# Check migration status
python scripts/check_migrations.py

# Validate database schema
python scripts/validate_schema.py

# Check data integrity
python scripts/check_data_integrity.py
```

**Solutions**:
```bash
# Reset database
python scripts/reset_database.py

# Run migrations manually
python scripts/migrate_database.py

# Backup and restore
python scripts/backup_database.py
python scripts/restore_database.py
```

### Testing Issues

#### Issue: Tests Failing

**Symptoms**:
- Unit test failures
- Integration test errors
- Test environment issues

**Diagnostic Steps**:
```bash
# Run specific test
python -m pytest tests/test_models.py -v

# Check test configuration
cat pytest.ini

# Run with debugging
python -m pytest --pdb tests/test_failing.py
```

**Solutions**:
```bash
# Update test fixtures
python scripts/update_test_fixtures.py

# Fix test environment
export FLASK_ENV=testing

# Mock external dependencies
# Update test mocks and fixtures

# Run tests in isolation
python -m pytest tests/test_models.py::TestChampion::test_creation
```

---

## System Administration

### Log Management

#### Issue: Log Files Growing Too Large

**Symptoms**:
- Disk space running out
- Slow log processing
- Performance degradation

**Diagnostic Steps**:
```bash
# Check log file sizes
du -sh logs/*

# Check disk usage
df -h

# Find large files
find logs/ -size +100M -ls
```

**Solutions**:
```bash
# Set up log rotation
sudo cp /etc/logrotate.d/lol-recommender.conf /etc/logrotate.d/

# Compress old logs
gzip logs/*.log.1

# Clean old logs
find logs/ -name "*.log.*" -mtime +30 -delete

# Reduce log level
export LOG_LEVEL=WARNING
```

#### Issue: Log Analysis Problems

**Symptoms**:
- Can't find relevant log entries
- Log format issues
- Missing timestamps

**Diagnostic Steps**:
```bash
# Search logs
grep -n "ERROR" logs/app.log

# Check log format
head -10 logs/app.log

# Analyze log patterns
awk '{print $1}' logs/access.log | sort | uniq -c
```

**Solutions**:
```bash
# Standardize log format
# Update logging configuration

# Use structured logging
export LOG_FORMAT=json

# Set up centralized logging
# Configure ELK stack or similar

# Use log analysis tools
tail -f logs/app.log | grep ERROR
```

### Security Issues

#### Issue: Security Vulnerabilities

**Symptoms**:
- Security scanner alerts
- Suspicious access patterns
- Unauthorized access attempts

**Diagnostic Steps**:
```bash
# Check for security updates
pip audit

# Scan for vulnerabilities
safety check

# Check access logs
grep -E "(404|403|500)" logs/access.log
```

**Solutions**:
```bash
# Update dependencies
pip install --upgrade -r requirements.txt

# Apply security patches
sudo apt update && sudo apt upgrade

# Configure firewall
sudo ufw enable
sudo ufw allow 22
sudo ufw allow 80
sudo ufw allow 443

# Set up fail2ban
sudo apt install fail2ban
```

#### Issue: SSL/TLS Problems

**Symptoms**:
- Certificate errors
- HTTPS not working
- Mixed content warnings

**Diagnostic Steps**:
```bash
# Check certificate status
openssl s_client -connect your-domain.com:443

# Test SSL configuration
curl -I https://your-domain.com

# Check certificate expiration
sudo certbot certificates
```

**Solutions**:
```bash
# Renew certificates
sudo certbot renew

# Fix certificate chain
sudo certbot install --cert-name your-domain.com

# Update SSL configuration
# Check Nginx SSL settings

# Force HTTPS
# Add HSTS headers
```

---

## Monitoring and Alerts

### Setting Up Monitoring

#### Basic Monitoring Script

Create `scripts/monitor.py`:
```python
#!/usr/bin/env python3
import requests
import time
import logging
import smtplib
from email.mime.text import MIMEText

def check_health():
    try:
        response = requests.get('http://localhost:5000/health', timeout=10)
        return response.status_code == 200
    except:
        return False

def send_alert(message):
    # Configure email alerts
    msg = MIMEText(message)
    msg['Subject'] = 'LoL Recommender Alert'
    msg['From'] = 'alerts@your-domain.com'
    msg['To'] = 'admin@your-domain.com'
    
    # Send email (configure SMTP settings)
    # smtp_server.send_message(msg)

def main():
    logging.basicConfig(level=logging.INFO)
    
    while True:
        if not check_health():
            logging.error("Health check failed!")
            send_alert("Application health check failed")
        
        time.sleep(60)

if __name__ == '__main__':
    main()
```

#### System Monitoring

```bash
# CPU and Memory monitoring
#!/bin/bash
# monitor_system.sh

while true; do
    CPU=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    MEM=$(free | grep Mem | awk '{printf("%.1f", $3/$2 * 100.0)}')
    
    if (( $(echo "$CPU > 80" | bc -l) )); then
        echo "High CPU usage: $CPU%"
        # Send alert
    fi
    
    if (( $(echo "$MEM > 80" | bc -l) )); then
        echo "High memory usage: $MEM%"
        # Send alert
    fi
    
    sleep 300  # Check every 5 minutes
done
```

### Alert Configuration

#### Email Alerts

```python
# alerts.py
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

class AlertManager:
    def __init__(self, smtp_server, smtp_port, username, password):
        self.smtp_server = smtp_server
        self.smtp_port = smtp_port
        self.username = username
        self.password = password
    
    def send_alert(self, subject, message, recipients):
        msg = MIMEMultipart()
        msg['From'] = self.username
        msg['To'] = ', '.join(recipients)
        msg['Subject'] = subject
        
        msg.attach(MIMEText(message, 'plain'))
        
        try:
            server = smtplib.SMTP(self.smtp_server, self.smtp_port)
            server.starttls()
            server.login(self.username, self.password)
            server.send_message(msg)
            server.quit()
            return True
        except Exception as e:
            print(f"Failed to send alert: {e}")
            return False
```

#### Slack Integration

```python
# slack_alerts.py
import requests
import json

def send_slack_alert(webhook_url, message):
    payload = {
        'text': message,
        'username': 'LoL Recommender Bot',
        'icon_emoji': ':warning:'
    }
    
    try:
        response = requests.post(webhook_url, json=payload)
        return response.status_code == 200
    except Exception as e:
        print(f"Failed to send Slack alert: {e}")
        return False
```

### Performance Monitoring

#### Application Metrics

```python
# metrics.py
import time
import psutil
from functools import wraps

class MetricsCollector:
    def __init__(self):
        self.metrics = {}
    
    def time_function(self, func_name):
        def decorator(func):
            @wraps(func)
            def wrapper(*args, **kwargs):
                start_time = time.time()
                result = func(*args, **kwargs)
                end_time = time.time()
                
                execution_time = end_time - start_time
                if func_name not in self.metrics:
                    self.metrics[func_name] = []
                self.metrics[func_name].append(execution_time)
                
                return result
            return wrapper
        return decorator
    
    def get_system_metrics(self):
        return {
            'cpu_percent': psutil.cpu_percent(),
            'memory_percent': psutil.virtual_memory().percent,
            'disk_usage': psutil.disk_usage('/').percent
        }
    
    def get_performance_report(self):
        report = {}
        for func_name, times in self.metrics.items():
            report[func_name] = {
                'avg_time': sum(times) / len(times),
                'max_time': max(times),
                'min_time': min(times),
                'call_count': len(times)
            }
        return report
```

---

## Emergency Procedures

### Application Down

1. **Immediate Response**:
   ```bash
   # Check if process is running
   ps aux | grep -E "(python|gunicorn)"
   
   # Restart application
   sudo systemctl restart lol-recommender
   
   # Check logs for errors
   tail -50 logs/error.log
   ```

2. **If Restart Fails**:
   ```bash
   # Check system resources
   free -h
   df -h
   
   # Kill hung processes
   sudo pkill -f python
   
   # Start manually for debugging
   cd /path/to/app
   source venv/bin/activate
   python app.py
   ```

3. **Escalation**:
   - Contact system administrator
   - Check external dependencies
   - Consider rollback to previous version

### Data Corruption

1. **Immediate Actions**:
   ```bash
   # Stop application
   sudo systemctl stop lol-recommender
   
   # Backup current state
   cp -r data/ data_backup_$(date +%Y%m%d_%H%M%S)
   
   # Restore from backup
   cp -r data_backup_latest/* data/
   ```

2. **Validation**:
   ```bash
   # Validate data integrity
   python scripts/validate_data.py
   
   # Test application
   python scripts/test_basic_functionality.py
   ```

### Security Incident

1. **Immediate Response**:
   ```bash
   # Block suspicious IPs
   sudo ufw deny from <suspicious_ip>
   
   # Check access logs
   grep <suspicious_ip> logs/access.log
   
   # Change passwords/keys
   export SECRET_KEY=$(python -c 'import secrets; print(secrets.token_hex())')
   ```

2. **Investigation**:
   ```bash
   # Analyze logs
   grep -E "(404|403|500)" logs/access.log
   
   # Check for unauthorized changes
   git status
   git diff
   
   # Scan for malware
   sudo rkhunter --check
   ```

---

*Last updated: January 2024*
*Version: 1.0.0*

*For emergency support, contact: emergency@lol-recommender.com*