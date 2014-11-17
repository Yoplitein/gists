import gl3n.util: is_matrix, is_vector;

auto to_gfm(MatrixType)(MatrixType matrix)
if(is_matrix!MatrixType)
{
    static import gfm.math;
    
    enum matrixSize = matrix.cols * matrix.rows;
    matrix.mt[matrixSize] data;
    matrix.mt *ptr = matrix.value_ptr;
    
    foreach(index; 0 .. matrixSize)
        data[index] = *ptr++;
    
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
    uniform.set(matrix.to_gfm);
}

void set(MatrixType)(GLUniform uniform, VectorType vector)
if(is_vector!VectorType)
{
    uniform.set(vector.to_gfm);
}*/