import std.algorithm;
import std.array;
import std.datetime;
import std.getopt;
import std.range;
import std.socket;
import std.stdio;
import std.string;

class ReadFailed: Exception
{
    this()
    {
        super("");
    }
}

ubyte[] encode(string message, bool newline = true, bool nul = true)
{
    Appender!string buffer;
    
    buffer.reserve(message.length + 5);
    buffer.put("\xff\xff\xff\xff");
    buffer.put(message);
    
    if(newline)
        buffer.put("\n");
    
    if(nul)
        buffer.put("\x00");
    
    return cast(ubyte[])buffer.data;
}

string decode(ubyte[] data, bool newline = true, bool nul = true)
{
    auto result = cast(string)(data[4 .. $]);
    
    if(nul)
        result = result.strip('\x00');
    
    if(newline)
        result = result.strip('\n');
    
    return result;
}

string b2c(string x)
{
    return x.replace("\x00", "\\x00").replace("\n", "\\n");
}

ubyte[] readAll(UdpSocket socket)
{
    Appender!(ubyte[]) buffer;
    enum chunkSize = 1024;
    ubyte[chunkSize] chunk;
    
    while(true)
    {
        int readBytes = socket.receive(chunk);
        
        if(readBytes <= 0)
            throw new ReadFailed;
        
        buffer.put(chunk[0 .. readBytes]);
        
        if(readBytes < chunkSize)
            break;
        
        stdout.flush;
    }
    
    return buffer.data;
}

string rcon(string host, ushort port, string password, string command)
{
    auto server = parseAddress(host, port);
    auto socket = new UdpSocket;
    
    scope(exit) socket.close;
    socket.setOption(SocketOptionLevel.SOCKET, SocketOption.RCVTIMEO, 5.seconds);
    socket.sendTo(encode("challenge rcon"), server);
    
    string challengeID = socket
        .readAll
        .decode
        .split(" ")
        .drop(2)
        .front
        .strip
    ;
    
    socket.sendTo(encode("rcon %s \"%s\" %s".format(challengeID, password, command), false, true), server);
    
    return socket
        .readAll
        .decode
        .drop(1) //leading l
    ;
}

int main(string[] args)
{
    string host = "127.0.0.1";
    ushort port = 27015;
    string password;
    string command;
    GetoptResult parsed;
    
    try
        parsed = args.getopt(
            config.passThrough,
            "H|host", "Host to connect to", &host,
            "port", "Port to connect on", &port,
            config.required,
            "p|password", "Password to authenticate with", &password,
        );
    catch(GetOptException err)
    {
        writeln("Password required.");
        
        return 1;
    }
    
    if(parsed.helpWanted)
    {
        defaultGetoptPrinter("rcon.d -p <password> <command>", parsed.options);
        
        return 0;
    }
    
    try
        writeln(rcon(host, port, password, args.drop(1).join(" ")));
    catch(ReadFailed err)
    {
        writeln("Server did not send a response (is it up?)");
        
        return 2;
    }
    
    return 0;
}
