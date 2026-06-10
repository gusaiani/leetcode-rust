//! LeetCode 977. Squares of a Sorted Array
//! https://leetcode.com/problems/squares-of-a-sorted-array/
//!
//! Given an integer array `nums` sorted in non-decreasing order, return an
//! array of the squares of each number sorted in non-decreasing order.

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];
    let mut lo = 0;
    let mut hi = n - 1;

    for write in (0..n).rev() {
        let left_sq = nums[lo] * nums[lo];
        let right_sq = nums[hi] * nums[hi];

        if left_sq > right_sq {
            result[write] = left_sq;
            lo += 1;
        } else {
            result[write] = right_sq;
            hi = hi.saturating_sub(1);
        }
    }

    result
}
