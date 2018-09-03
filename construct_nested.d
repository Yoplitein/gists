import std.traits;

Type construct(Type)(RepresentationTypeTuple!Type args)
{
    
    struct Flat
    {
        typeof(args) fields;
    }
    
    static union Conv
    {
        Type obj;
        Flat flat;
    }
    
    Conv conv;
    conv.flat.fields = args;
    
    return conv.obj;
}

unittest
{
    struct Parent
    {
        int x;
    }
    
    struct Child
    {
        Parent parent;
        int y;
    }
    
    static assert(__traits(compiles, construct!Child(1, 2)));
    
    Child child = construct!Child(1, 2);
    
    assert(child.parent.x == 1);
    assert(child.y == 2);
}
