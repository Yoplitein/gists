import std.algorithm;
import std.random;
import std.range;
import std.stdio;
import std.string;
import std.utf;

void main(string[] args)
{
    if(args.length < 2)
    {
        writeln("nope");
        
        return;
    }
    
    args
        .drop(1)
        .map!(
            word => word
                .enumerate
                .map!(
                    tup => (tup[0] & 1) ?
                        randomFormatting.format(tup[1]) :
                        "%s".format(tup[1])
                )
                .join
        )
        .map!(word => word.randomCaps)
        .join(" ")
        .writeln
    ;
}

string randomFormatting()
{
    static immutable options = [
        "**",
        "*",
        "__",
        "~~",
        "`",
    ];
    string option = options.randomCover.front;
    
    return "%s%%s%s".format(option, option);
}

string randomCaps(string word)
{
    return word
        .map!(c => randomBool ? c : c.toUpper)
        .array
        .toUTF8
    ;
}

bool randomBool()
{
    return uniform!"[]"(0, 1) == 1;
}
