import os
from dotenv import load_dotenv

load_dotenv()

class Config:
    SECRET_KEY = os.environ.get('SECRET_KEY') or 'dev-secret-key-change-in-production'
    DATA_FOLDER = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'data')
    MODEL_FOLDER = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'models', 'trained')
    
    # Performance settings
    CACHE_DEFAULT_TTL = int(os.environ.get('CACHE_DEFAULT_TTL', 3600))
    CACHE_MAX_MEMORY_SIZE = int(os.environ.get('CACHE_MAX_MEMORY_SIZE', 2000))
    CACHE_COMPRESSION = os.environ.get('CACHE_COMPRESSION', 'true').lower() == 'true'
    
    # Static file optimization
    SEND_FILE_MAX_AGE_DEFAULT = int(os.environ.get('SEND_FILE_MAX_AGE_DEFAULT', 31536000))  # 1 year
    
class DevelopmentConfig(Config):
    DEBUG = True
    SEND_FILE_MAX_AGE_DEFAULT = 0  # No caching in development
    
class ProductionConfig(Config):
    DEBUG = False
    
    # Production optimizations
    CACHE_DEFAULT_TTL = int(os.environ.get('CACHE_DEFAULT_TTL', 7200))  # 2 hours
    CACHE_MAX_MEMORY_SIZE = int(os.environ.get('CACHE_MAX_MEMORY_SIZE', 5000))  # Larger cache
    
    # Enable gzip compression for static files
    COMPRESS_MIMETYPES = [
        'text/html',
        'text/css',
        'text/xml',
        'application/json',
        'application/javascript',
        'application/xml+rss',
        'application/atom+xml',
        'image/svg+xml'
    ]

config = {
    'development': DevelopmentConfig,
    'production': ProductionConfig,
    'default': DevelopmentConfig
}