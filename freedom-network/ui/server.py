#!/usr/bin/env python3
import os
import sys
from http.server import HTTPServer, SimpleHTTPRequestHandler
import webbrowser
import time

# Get the directory where this script is
UI_DIR = os.path.dirname(os.path.abspath(__file__))
os.chdir(UI_DIR)

class MyHTTPRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        # Prevent caching
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        return super().end_headers()
    
    def log_message(self, format, *args):
        # Suppress default logging
        pass

PORT = 8000
SERVER_ADDRESS = ('127.0.0.1', PORT)

print("🌐 Freedom Browser Web Server")
print(f"📍 Starting web server on http://127.0.0.1:{PORT}")
print("Press Ctrl+C to stop\n")

server = HTTPServer(SERVER_ADDRESS, MyHTTPRequestHandler)

# Open browser
time.sleep(0.5)
url = f"http://127.0.0.1:{PORT}"
print(f"🚀 Opening browser at {url}...\n")
webbrowser.open(url)

try:
    server.serve_forever()
except KeyboardInterrupt:
    print("\n\n👋 Server stopped")
    sys.exit(0)
