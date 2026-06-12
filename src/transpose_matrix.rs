//! LeetCode 867. Transpose Matrix
//! https://leetcode.com/problems/transpose-matrix/
//!
//! Given a 2D integer array `matrix`, return the transpose of `matrix`.
//!
//! The transpose of a matrix is the matrix flipped over its main diagonal,
//! switching the matrix's row and column indices. A matrix with dimensions
//! `m x n` becomes `n x m`.

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }

    result
}
