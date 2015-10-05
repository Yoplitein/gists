import std.meta;

/++
    Retrieves the attribute of type AttrType from symbol.
    
    If no such attribute exists on the symbol, AttrType.init is returned.
+/
template attribute(AttrType, alias symbol)
{
    template match(args...)
    {
        static if(!is(args[0]))
            enum match = is(AttrType == typeof(args[0]));
        else
            enum match = false;
    }
    
    alias matchingAttributes = Filter!(match, __traits(getAttributes, symbol));
    
    static if(matchingAttributes.length > 1)
        static assert(false, symbol.stringof ~ " has multiple attributes of type " ~ AttrType.stringof);
    else static if(matchingAttributes.length == 0)
        enum attribute = AttrType.init;
    else
        enum attribute = matchingAttributes[0];
}

unittest
{
    struct Data
    {
        string data;
    }
    
    @Data("abc")
    int symbol;
    
    int otherSymbol;
    
    @Data("abc")
    @Data("def")
    int ambiguousSymbol;
    
    Data attr = attribute!(Data, symbol);
    
    assert(attr == Data("abc"));
    
    attr = attribute!(Data, otherSymbol);
    
    assert(attr == Data.init);
    static assert(!__traits(compiles, attribute!(Data, ambiguousSymbol)));
}
