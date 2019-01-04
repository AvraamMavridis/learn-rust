We define a Matrix as a type alias to `Vec<Vec<f32>>`. Write a function that takes in two Matrixes by reference and returns the product mat1 * mat2. The function signature is provided below.

You should make sure that the two input matrices are actually compatible. Remember that you can't multiply two matrices if the number of columns in the first matrix is not equal to the number of rows in the second matrix. Use assert! or assert_eq! to panic! if this condition is not met.


```rust

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // TODO
    unimplemented!();
}
```
