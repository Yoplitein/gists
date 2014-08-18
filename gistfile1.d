import gl3n.util: is_matrix, is_vector;

auto to_gfm(MatrixType)(MatrixType matrix)
if(is_matrix!MatrixType)
{
    static import gfm.math;
    
    float[] data;
    float *ptr = matrix.value_ptr;
    
    foreach(_; 0 .. matrix.cols * matrix.rows)
        data ~= [*ptr++];
    
    return gfm.math.Matrix!(matrix.mt, matrix.rows, matrix.cols)(data);
}

auto to_gfm(VectorType)(VectorType vector)
if(is_vector!VectorType)
{
    static import gfm.math;
    
    return gfm.math.Vector!(vector.vt, vector.dimension)(vector.vector);
}

//no way to prefer these over GLUniform.set. :(
/*void set(MatrixType)(GLUniform uniform, MatrixType matrix)
if(is_matrix!MatrixType)
{
    writeln("GLUniform.set for matrix");
    uniform.set(matrix.to_gfm);
}

void set(MatrixType)(GLUniform uniform, VectorType vector)
if(is_vector!VectorType)
{
    writeln("GLUniform.set for vector");
    uniform.set(vector.to_gfm);
}*/