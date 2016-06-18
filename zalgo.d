import std.algorithm;
import std.array;
import std.file: writeFile = write;
import std.getopt;
import std.random;
import std.range;
import std.stdio;
import std.string;
import std.uni;
import std.utf;

void main(string[] args)
{
    size_t min = 5;
    size_t max = 10;
    GetoptResult parsed = args.getopt(
        "min", "Minimum number of combining characters to apply", &min,
        "max", "Maximum number of combining characters to apply", &max,
    );
    
    if(parsed.helpWanted)
    {
        defaultGetoptPrinter("zalgo [options] <text>", parsed.options);
        
        return;
    }
    
    if(min >= max)
    {
        writefln("Maximum must be larger than minimum);
        
        return;
    }
    
    if(args.length == 1)
    {
        writeln("No text to zalgo");
        
        return;
    }
    
    string text = args
        .drop(1)
        .join(" ")
    ;
    dstring special = iota(0x300, 0x34f)
        .chain(iota(0x350, 0x370))
        .map!(x => cast(dchar)x)
        .array
        .idup
    ;
    auto result = appender!dstring;
    
    foreach(dchar chr; text)
    {
        result.put(chr);
        
        foreach(_; 0 .. uniform!"[]"(min, max))
            result.put(special.randomCover.front);
    }
    
    writeFile("out.txt", result.data.toUTF8);
}