import core.thread;
import std.experimental.logger;
import std.stdio;
import std.datetime;
static import vibe.core.log;

//std.logger's default logger must be an instance of a class that implements this
interface StdLoggerBridge
{
    //required for VibeLogger to pass the logging data to std.logger's internal bits
    void writeVibeMessage(string mod, string func, string file, int line, LogLevel level, SysTime time, string text) @safe;
}

class StdLogger: FileLogger, StdLoggerBridge
{
    this(in string fn, const LogLevel lv = LogLevel.info) @safe
    {
        super(fn, lv);
    }

    this(File file, const LogLevel lv = LogLevel.info) @safe
    {
        super(file, lv);
    }
    
    void writeVibeMessage(string mod, string func, string file, int line, LogLevel level, SysTime time, string text) @safe
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
    this(vibe.core.log.LogLevel minLevel)
    {
        this.minLevel = minLevel;
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
            case trace:
                level = LogLevel.trace;
                
                break;
            case info:
                level = LogLevel.info;
                
                break;
            case warn:
                level = LogLevel.warning;
                
                break;
            case diagnostic:
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
        
        StdLogger logger = cast(StdLogger)sharedLog;
        string moduleName = line.mod;
        
        if(logger is null)
            throw new Exception("sharedLog does not implement writeVibeMessage");
        
        if(moduleName == "")
            moduleName = "???";
        
        logger.writeVibeMessage(
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
    //register our std.logger logger, with writeVibeMessage method
    sharedLog = new StdLogger(stdout, LogLevel.all);
    
    //disable default Vibe logger
    vibe.core.log.setLogLevel(vibe.core.log.LogLevel.none);
    
    //register our Vibe logger that passes messages to std.logger
    vibe.core.log.registerLogger(cast(shared)new VibeLogger(vibe.core.log.LogLevel.debug_));
}
