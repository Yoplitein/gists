import std.algorithm;
import std.datetime;
import std.range;
import std.stdio;
import std.string;

void main()
{
    stdin
        .byLine
        .map!(line => line.split("/"))
        .filter!(line => line[1] != line[2])
        .map!(
            line => "commit `%s` committed `%s` but authored `%s` (%s earlier)"
                .format(
                    line[0].take(7),
                    line[1],
                    line[2],
                    findEarlier(
                        line[1].idup,
                        line[2].idup
                    )
                )
        )
        .join("\n")
        .writeln
    ;
}

SysTime parseDate(string date)
{
    string[] bits = date.split(" ");
    string dayStr = bits[0];
    string month = bits[1];
    string day = bits[2];
    string time = bits[3];
    string year = bits[4];
    string tz = bits[5];
    
    return "%s, %s %s %s %s %s".format(dayStr, day, month, year, time, tz).parseRFC822DateTime;
}

string findEarlier(string commitDate, string authorDate)
{
    SysTime cd = commitDate.parseDate;
    SysTime ad = authorDate.parseDate;
    
    return cd < ad ? "commit date" : "author date";
}