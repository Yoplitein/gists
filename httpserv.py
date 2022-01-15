from sys import argv
import http.server as httpd

exts = httpd.SimpleHTTPRequestHandler.extensions_map
exts.update({
	".html": "text/html",
	".js": "application/javascript",
})

try:
	port = 8123 if len(argv) < 2 else int(argv[1])
	assert port > 0 and port <= 0xffff
	
	serv = httpd.HTTPServer(('', port), httpd.SimpleHTTPRequestHandler)
	print(f"Server running at http://localhost:{port}")
	serv.serve_forever()
except KeyboardInterrupt: pass
except (ValueError, AssertionError):
	print(f"{argv[1]} is not a number/valid port")

print("Server shut down")