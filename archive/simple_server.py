#!/usr/bin/env python3
"""
Ultra-simple HTTP server for the LoL Champion Recommender
Uses only Python standard library - no external dependencies!
"""

import http.server
import socketserver
import json
import urllib.parse
from pathlib import Path
import webbrowser
import threading
import time

PORT = 8082

class ChampionRecommenderHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        # Redirect root to champion-recommender.html
        if self.path == '/' or self.path == '/index.html':
            self.path = '/champion-recommender.html'
        
        # Serve champion-recommender.html when requested
        if self.path == '/champion-recommender.html':
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            
            # Read and serve the champion-recommender.html file
            try:
                with open('champion-recommender.html', 'r', encoding='utf-8') as f:
                    content = f.read()
                self.wfile.write(content.encode('utf-8'))
            except FileNotFoundError:
                self.wfile.write(b'<h1>Champion recommender file not found. Please make sure champion-recommender.html exists.</h1>')
        else:
            # Serve static files normally
            super().do_GET()

def open_browser():
    """Open browser after a short delay"""
    time.sleep(1.5)
    webbrowser.open(f'http://localhost:{PORT}')

def main():
    print("🎮 LoL Champion Recommender - Simple Server")
    print("=" * 50)
    print(f"🌐 Server starting on http://localhost:{PORT}")
    print("📱 The website will open automatically in your browser")
    print("⚡ Press Ctrl+C to stop the server")
    print("=" * 50)
    
    # Start browser opening in background
    browser_thread = threading.Thread(target=open_browser)
    browser_thread.daemon = True
    browser_thread.start()
    
    # Start server
    with socketserver.TCPServer(("", PORT), ChampionRecommenderHandler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped. Thanks for trying the LoL Champion Recommender!")

if __name__ == "__main__":
    main()