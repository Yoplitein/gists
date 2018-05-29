public import std.typecons: Yes, No;

struct ClampedNumber(NumericT)
{
    import std.traits: isNumeric;
    import std.typecons: Flag;
    
    alias Numeric = NumericT;
    alias TrackUnclamped = Flag!"trackUnclamped";
    
    static assert(isNumeric!Numeric, Numeric.stringof ~ " is not a numeric type!");
    
    private Numeric _value;
    private Numeric _unclampedValue;
    Numeric min;
    Numeric max;
    TrackUnclamped trackUnclamped;
    
    @disable this();
    
    this(Numeric value, Numeric min, Numeric max, TrackUnclamped trackUnclamped = No.trackUnclamped)
    {
        this.min = min;
        this.max = max;
        this.trackUnclamped = trackUnclamped;
        this = value;
    }
    
    Other opCast(Other)()
    if(isNumeric!Other)
    {
        return cast(Other)value;
    }
    
    Numeric opAssign(Other)(Other value)
    if(isNumeric!Other)
    {
        import std.algorithm: min, max;
        
        if(trackUnclamped)
            this._unclampedValue = value;
        
        return this._value = max(min(value, this.max), this.min);
    }
    
    template opOpAssign(string op)
    {
        mixin clampedNumberArithmetic!(op, false);
        alias opOpAssign = opArithmetic;
    }
    
    template opBinary(string op)
    {
        mixin clampedNumberArithmetic!(op, true);
        alias opBinary = opArithmetic;
    }
    
    bool opEquals(Other)(Other value)
    if(isNumeric!Other)
    {
        return this.value == value;
    }
    
    bool opEquals(OtherNumeric)(ClampedNumber!OtherNumeric other)
    {
        auto thisValue = trackUnclamped ? _unclampedValue : _value;
        auto thatValue = other.trackUnclamped ? other._unclampedValue : other._value;
        
        return
            this.trackUnclamped == other.trackUnclamped &&
            thisValue == thatValue &&
            this.max == other.max &&
            this.min == other.min
        ;
    }
    
    @property Numeric value()
    {
        return this._value;
    }
    
    @property Numeric unclampedValue()
    {
        return this._unclampedValue;
    }
}

private mixin template clampedNumberArithmetic(string op, bool copy)
{
    typeof(this) opArithmetic(Other)(Other value)
    if(isNumeric!Other)
    {
        static if(copy)
        {
            enum target = "result";
            typeof(this) result = this;
        }
        else
            enum target = "this";
        
        if(trackUnclamped)
            mixin(target ~ " = _unclampedValue " ~ op ~ " value;");
        else
            mixin(target ~ " = _value " ~ op ~ " value;");
        
        mixin("return " ~ target ~ ";");
    }
    
    typeof(this) opArithmetic(OtherNumeric)(ClampedNumber!OtherNumeric other)
    {
        static if(copy)
        {
            enum target = "result";
            typeof(this) result = this;
        }
        else
            enum target = "this";
        
        auto thisValue = trackUnclamped ? _unclampedValue : _value;
        auto thatValue = other.trackUnclamped ? other._unclampedValue : other._value;
        
        mixin(target ~ " = thisValue " ~ op ~ " thatValue;");
        mixin("return " ~ target ~ ";");
    }
}

///Test clamping.
unittest
{
    //clamping
    auto num = ClampedNumber!int(1, 0, 10);
    num = -1;
    
    assert(num == 0);
    
    num = 20;
    
    assert(num == 10);
}

///Test operators.
unittest
{
    auto num = ClampedNumber!int(1, 0, 10);
    num -= 2;
    
    //opEquals
    assert(num == 0);
    assert(num == ClampedNumber!int(0, 0, 10));
    
    assert(num == 0);
    
    num += 20;
    
    assert(num == 10);
    
    num -= 1;
    
    assert(num == 9);
}

///Test unclamped tracking.
unittest
{
    auto num = ClampedNumber!int(1, 0, 10, Yes.trackUnclamped);
    
    num -= 1;
    
    assert(num == 0);
    assert(num.unclampedValue == 0);
    
    num -= 1;
    
    assert(num == 0);
    assert(num.unclampedValue == -1);
    
    num += 1;
    
    assert(num == 0);
    assert(num.unclampedValue == 0);
    
    num += 1;
    
    assert(num == 1);
    assert(num.unclampedValue == 1);
    
    num = -1;
    
    assert(num == 0);
    assert(num.unclampedValue == -1);
}

///Test arithmetic on pairs of ClampedNumbers
unittest
{
    auto x = ClampedNumber!int(1, 0, 10, Yes.trackUnclamped);
    auto y = ClampedNumber!int(2, 0, 10, Yes.trackUnclamped);
    auto z = x + y;
    
    assert(z == 3);
    
    x = 20;
    y = 20;
    z = x + y;
    
    assert(z.value == 10);
    assert(z.unclampedValue == 40);
    
    y.trackUnclamped = No.trackUnclamped;
    z = x + y;
    
    assert(z.trackUnclamped);
    assert(z.value == 10);
    assert(z.unclampedValue == 30);
}
