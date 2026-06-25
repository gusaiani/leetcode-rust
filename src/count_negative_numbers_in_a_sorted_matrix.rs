//! LeetCode 1351. Count Negative Numbers in a Sorted Matrix
//! https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
//!
//! Given a `m x n` matrix `grid` which is sorted in non-increasing order both
//! row-wise and column-wise, return the number of negative numbers in `grid`.

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut row = 0;
    let mut col = num_cols - 1;

    let mut count = 0;

    while row < num_rows {
        if grid[row][col] < 0 {
            count += num_rows - row;

            if col == 0 {
                break;
            }
            col -= 1;
        } else {
            row += 1;
        }
    }

    count as i32
}
