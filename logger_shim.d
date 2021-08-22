import core.thread;
import std.experimental.logger;
import std.datetime;
static import vibe.core.log;

// has to be in a separate class as Logger.LogEntry is protected :facepalm:
class StdLogger: StdForwardLogger
{
    @safe:
    
    this()
    {
        super(LogLevel.all);
    }
    
    void writeVibeMessage(string mod, string func, string file, int line, LogLevel level, SysTime time, string text)
    {
        LogEntry entry;
        entry.file = file;
        entry.line = line;
        entry.funcName = func;
        entry.prettyFuncName = func;
        entry.moduleName = mod;
        entry.logLevel = level;
        //entry.threadId = thread; //no way to convert a Thread to a Tid
        entry.timestamp = time;
        entry.msg = text;
        entry.logger = this;
        
        writeLogMsg(entry);
    }
}

class VibeLogger: vibe.core.log.Logger
{
    private StdLogger stdLogger;
    
    this(vibe.core.log.LogLevel minLevel)
    {
        this.minLevel = minLevel;
        stdLogger = new StdLogger();
    }
    
    override void log(ref vibe.core.log.LogLine line) @safe
    {
        LogLevel level;
        
        switch(line.level) with(vibe.core.log.LogLevel)
        {
            default:
                level = LogLevel.all;
                
                break;
            case debugV:
            case debug_:
            case diagnostic:
            case trace:
                level = LogLevel.trace;
                
                break;
            case info:
                level = LogLevel.info;
                
                break;
            case warn:
                level = LogLevel.warning;
                
                break;
            case error:
                level = LogLevel.error;
                
                break;
            case critical:
                level = LogLevel.critical;
                
                break;
            case fatal:
                level = LogLevel.fatal;
                
                break;
        }
        
        string moduleName = line.mod;
        if(moduleName == "")
            moduleName = "???";
        
        stdLogger.writeVibeMessage(
            moduleName,
            line.func,
            line.file,
            line.line,
            level,
            line.time,
            line.text,
        );
    }
}

shared static this()
{
    //disable default Vibe logger
    vibe.core.log.setLogLevel(vibe.core.log.LogLevel.none);
    
    //register our Vibe logger that passes messages to std.logger
    vibe.core.log.registerLogger(cast(shared)new VibeLogger(vibe.core.log.LogLevel.debug_));
}