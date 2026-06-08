//! LeetCode 118. Pascal's Triangle
//! https://leetcode.com/problems/pascals-triangle/
//!
//! Given an integer `num_rows`, return the first `num_rows` of Pascal's
//! triangle. In Pascal's triangle, each number is the sum of the two numbers
//! directly above it:
//!
//! ```text
//!     1
//!    1 1
//!   1 2 1
//!  1 3 3 1
//! 1 4 6 4 1
//! ```
//!
//! The first and last entry of every row is 1; every interior entry `row[j]`
//! equals `prev[j - 1] + prev[j]` from the row above.

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut row = vec![1; i + 1];

        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }

        triangle.push(row);
    }

    triangle
}
