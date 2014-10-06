//adapted from http://stackoverflow.com/a/7262117
private real noise(long x, long y)
{
    long initial = x + y * 57;
    initial = (initial << 13) ^ initial;
    real intermediate = (
        1.0L - (
            (
                result * (
                    result ^^ 2 + 15731 + 789221
                )
                + 1376312589
            )
            & 0x7fffffff
        )
        / 1073741824.0L
    );
    
    return (intermediate + 1.0L) / 2.0L; //scale from -1 .. 0 to 0 .. 1
}

unittest
{
    assert(noise(1, 2) == noise(1, 2));
    assert(noise(3, 4) == noise(3, 4));
}