#!/usr/bin/env python3
import http.server as server
port = 8000
server.SimpleHTTPRequestHandler.extensions_map['.wasm'] = 'application/wasm'
s = server.HTTPServer(('localhost', port), server.SimpleHTTPRequestHandler)
s.serve_forever()
