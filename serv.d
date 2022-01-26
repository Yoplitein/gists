/++dub.sdl:
name "serv"
dependency "vibe-core" version="*"
dependency "vibe-d:http" version="~>0.9.4"
versions "VibeDefaultMain"
+/

import core.time: hours;
import std.algorithm: countUntil;
import std.exception: assumeUnique;
import std.functional: toDelegate;
import std.stdio: stderr, writefln;

import vibe.core.args: readOption;
import vibe.core.core: runTask, exitEventLoop;
import vibe.http.fileserver;
import vibe.http.server;

immutable ushort port;
immutable string[string] extraHeaders;
shared static this()
{
	ushort _port = 8123;
	readOption("p|port", &_port, "Port to listen on");
	port = _port;
	
	bool noDefaultHeaders = false;
	readOption("n|no-default-headers", &noDefaultHeaders, "Don't add default headers (enabling cross-origin isolation)");
	
	string[string] headers;
	if(!noDefaultHeaders)
		headers = [
			"Cross-Origin-Embedder-Policy": "require-corp",
			"Cross-Origin-Opener-Policy": "same-origin",
		];
	
	string[] headersStrs;
	readOption("x|header", &headersStrs, "Add a header to responses");
	
	size_t malformed = -1;
	foreach(index, header; headersStrs)
	{
		const split = header.countUntil("=");
		if(split == -1)
		{
			malformed = index;
			break;
		}
		const name = header[0 .. split];
		const value = header[split + 1 .. $];
		if(name.length == 0 || value.length == 0)
		{
			malformed = index;
			break;
		}
		headers[name] = value;
	}
	if(malformed != -1)
	{
		stderr.writefln("Malformed header: `%s`\nExpected `name=value`", headersStrs[malformed]);
		runTask({ exitEventLoop(); });
	}
	else
	{
		headers.rehash;
		extraHeaders = headers.assumeUnique;
	}
}

shared static this()
{
	auto sfOpts = new HTTPFileServerSettings;
	sfOpts.options = HTTPFileServerOption.serveIndexHTML | HTTPFileServerOption.failIfNotFound;
	sfOpts.maxAge = 1.hours;
	sfOpts.preWriteCallback = toDelegate(&addHeaders);
	auto sfServer = serveStaticFiles(".", sfOpts);
	
	auto httpOpts = new HTTPServerSettings;
	httpOpts.bindAddresses = ["0.0.0.0"];
	httpOpts.port = port;
	httpOpts.accessLogToConsole = true;
	listenHTTP(httpOpts, sfServer);
}

void addHeaders(scope HTTPServerRequest req, scope HTTPServerResponse res, ref string path) @safe
{
	foreach(header; extraHeaders.byKeyValue)
		res.headers[header.key] = header.value;
}
