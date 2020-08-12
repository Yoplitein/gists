/++dub.sdl:
name "dnstest"

versions "VibeDefaultMain"

dependency "vibe-core" version="~>1.9.3"
+/

import std.socket: AddressFamily;

import vibe.core.core;
import vibe.core.log;
import vibe.core.net;

shared static this()
{
    runTask(
        {
            scope(exit) exitEventLoop;
            
            try logInfo("%s", resolveHost("")); // enforce L41
            catch(Throwable err) logError("%s %s", typeid(err), err.msg);
            
            try logInfo("%s", resolveHost("192.168.1.1", AddressFamily.INET6)); // enforce L44
            catch(Throwable err) logError("%s %s", typeid(err), err.msg);
            
            try logInfo("%s", resolveHost("foo", AddressFamily.UNSPEC, false)); // enforce L54
            catch(Throwable err) logError("%s %s", typeid(err), err.msg);
            
            try logInfo("%s", resolveHost("bogus.example.com", AddressFamily.UNSPEC)); // enforce L75
            catch(Throwable err) logError("%s %s", typeid(err), err.msg);
            
            try listenTCP(5000, (stream) => stream.close(), "0.0.0.1");
            catch(Throwable err) logError("%s %s", typeid(err), err.msg);
        }
    );
}