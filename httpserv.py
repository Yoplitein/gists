from http.server import HTTPServer, SimpleHTTPRequestHandler
from sys import argv

# teach the request handler about common MIME types
# fixes e.g. JS/WASM/etc. failing to load in some browsers
mimeTypes = {}
mimeTypes.update({
	".html": "text/html",
	".css": "text/css",
	".json": "text/json",
	".js": "application/javascript",
	".wasm": "application/wasm",
})
mimeTypes.update({f".{p[0]}": f"image/{p[1] or p[0]}" for p in [("jpg", "jpeg"), ("jpeg", None), ("png", None), ("gif", None)]})
mimeTypes.update({f".{ext}": f"font/{ext}" for ext in ["ttf", "otf", "woff", "woff2"]})
SimpleHTTPRequestHandler.extensions_map.update(mimeTypes)

staticHeaders = {
	# enable cross-origin isolation, enabling access to `SharedArrayBuffer`, `performance.now()`, etc.
	"Cross-Origin-Embedder-Policy": "require-corp",
	"Cross-Origin-Opener-Policy": "same-origin",
}

class ReqHandler(SimpleHTTPRequestHandler):
	def send_response(self, *args, **kwargs):
		super().send_response(*args, **kwargs)
		for name, val in staticHeaders.items():
			self.send_header(name, val)

try:
	port = 8123 if len(argv) < 2 else int(argv[1])
	assert port > 0 and port <= 0xffff
	
	serv = HTTPServer(('', port), ReqHandler)
	print(f"Server running at http://localhost:{port}")
	serv.serve_forever()
except KeyboardInterrupt: pass
except (ValueError, AssertionError):
	print(f"{argv[1]} is not a number/valid port")

print("Server shut down")
