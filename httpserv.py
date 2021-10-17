import http.server as httpd

exts = httpd.SimpleHTTPRequestHandler.extensions_map
exts.update({
	".html": "text/html",
	".js": "application/javascript",
})

serv = httpd.HTTPServer(('', 8123), httpd.SimpleHTTPRequestHandler)
serv.serve_forever()
