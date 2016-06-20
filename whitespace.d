#!/usr/bin/env rdmd

import std.algorithm;
import std.file;
import std.range;
import std.regex;
import std.stdio;

void main(string[] args)
{
    if(args.length == 1)
    {
        writeln("Usage: whitespace <folders>");
        
        return;
    }
    
    string[] folders = args.drop(1);
    auto files = folders
        .map!(f => dirEntries(f, SpanMode.breadth))
        .joiner
        .filter!(f => !f.isDir)
        .map!(f => f.name)
    ;
    
    foreach(file; files)
    {
        string contents = file.readText;
        
        if(contents.empty)
            continue;
        
        if(contents.canFind("\r\n"))
        {
            writefln("%s => uses Windows style EOL", file);
            
            contents = contents.replace("\r\n", "\n");
        }
        
        string[] lines = contents.split("\n");
        alias re = ctRegex!(r"\S[ \t]+$");
        
        if(!lines[$ - 1].empty)
            writefln("%s => no blank line at end of file", file);
        
        foreach(index, line; lines)
        {
            bool noTrailingSpaces = line.matchAll(re).empty;
            
            if(!noTrailingSpaces)
                writefln("%s:%s => trailing whitespace", file, index + 1);
            
            if(index == 0 || index == lines.length - 1)
                continue;
            
            if(line.length != 0)
                continue;
            
            if(lines[index - 1].startsWith(" ") || lines[index + 1].startsWith(" "))
                writefln("%s:%s => missing spaces on a blank line", file, index + 1);
        }
    }
}
