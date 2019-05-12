import gfm.math;

Vec project(Vec: Vector!(T, N), T, size_t N)(Vec a, Vec b)
{
    return b * scalarProject(a, b);
}

T scalarProject(Vec: Vector!(T, N), T, size_t N)(Vec a, Vec b)
{
    import std.math: cos;

    return a.magnitude * cos(angleBetween(a, b));
}

Vec planeProject(Vec: Vector!(T, N), T, size_t N)(Vec vec, Vec normal)
{
    return vec - vec.project(normal);
}