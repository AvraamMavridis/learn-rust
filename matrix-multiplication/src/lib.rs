use std::iter::Sum;

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

fn get_column(mat: &Matrix, index: usize) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();

    for row in mat {
        vec.push(row[index]);
    }

    vec
}

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut new_matrix: Matrix = Matrix::new();
    let row_length = mat1[0].len();
    let column_length = get_column(&mat2, 0).len();
    let number_of_rows = mat1.len();
    let number_of_columns = mat2[0].len();

    if number_of_rows != number_of_columns {
        panic!("Matrix multiplication is not possible, number of rows of 1st Matrix should be equal with the number of columns of the 2nd");
    }

    for (_i, _row) in mat1.iter().enumerate() {
        let mut new_row: Vec<f32> = Vec::new();

        if _row.len() != row_length {
            panic!("The matrix is not symmetric");
        }

        for (_j, _col) in mat2.iter().enumerate() {
            let _column = get_column(&mat2, _j);
            println!("{} {}", column_length, _column.len());

            if _column.len() != column_length {
                panic!("The matrix is not symmetric");
            }

            let sum: Vec<f32> = _row.into_iter().enumerate().map(|(index,x)| _column[index] * x).collect();
            let sum: f32 = sum.iter().sum();
            new_row.push(sum);
        }

        new_matrix.push(new_row);
    }

    new_matrix
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix1: Matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![2.0, 0.0], vec![1.0, 2.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![10.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected, "it should multiply matrixes correctly");
    }

    #[test]
    fn it_works_with_negative() {
        let matrix1: Matrix = vec![vec![-1.0, 2.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![-2.0, 0.0], vec![1.0, 2.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![-2.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected, "it should multiply matrixes correctly");
    }

    #[test]
    #[should_panic(expected = "The matrix is not symmetric")]
    fn it_should_panic_for_non_matrix_row() {
        let matrix1: Matrix = vec![vec![1.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![2.0, 0.0], vec![1.0, 2.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![10.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected);
    }

    #[test]
    #[should_panic(expected = "The matrix is not symmetric")]
    fn it_should_panic_for_non_matrix_column() {
        let matrix1: Matrix = vec![vec![1.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![2.0, 0.0], vec![1.0,2.0,3.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![10.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_for_out_of_bounds() {
        let matrix1: Matrix = vec![vec![1.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![2.0, 0.0], vec![1.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![10.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected);
    }

    #[test]
    #[should_panic(expected = "Matrix multiplication is not possible, number of rows of 1st Matrix should be equal with the number of columns of the 2nd")]
    fn it_should_panic_for_non_possible() {
        let matrix1: Matrix = vec![vec![-1.0, 2.0], vec![3.0, 4.0], vec![3.0, 4.0], vec![3.0, 4.0]];
        let matrix2: Matrix = vec![vec![-2.0, 0.0], vec![1.0, 2.0]];
        let expected: Matrix = vec![vec![4.0, 4.0], vec![-2.0, 8.0]];

        assert_eq!(mat_mult(&matrix1, &matrix2), expected);
    }
}
