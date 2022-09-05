auto builder(T)(T inst = T.init)
{
    enum isClass = is(T == class);
    static assert(is(T == struct) || isClass, "builder!T can only build structs and classes");
    
    static if(isClass)
        if(inst is null) inst = new T;
    
    static struct Builder
    {
        private T inst;

        T build()
        {
            return inst;
        }

        typeof(this) opDispatch(string prop)(typeof(mixin("inst.", prop)) value)
        {
            mixin("inst.", prop, " = value;");
            return this;
        }
    }
    return Builder(inst);
}

unittest
{
    static struct Foo
    {
        int x;
        string y;
    }
    auto x = builder!Foo
    	.x(10)
        .y("foo")
        .build
    ;
    assert(x.x == 10);
    assert(x.y == "foo");
    
    static class Bar
    {
        int x;
        string y;
    }
    auto y = builder!Bar
        .x(20)
        .y("bar")
        .build
    ;
    assert(y.x == 20);
    assert(y.y == "bar");
    
    static assert(!__traits(compiles, { auto b = builder!int; }));
    static assert(!__traits(compiles, { auto b = builder!Foo; b.x(""); }));
}