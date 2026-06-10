//! LeetCode 1920. Build Array from Permutation
//! https://leetcode.com/problems/build-array-from-permutation/
//!
//! Given a zero-based permutation `nums` (0-indexed), build an array `ans` of
//! the same length where `ans[i] = nums[nums[i]]` for each `0 <= i < nums.len()`,
//! and return it.
//!
//! A zero-based permutation `nums` is an array of distinct integers from
//! `0` to `nums.len() - 1` (inclusive).

pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|&v| nums[v as usize]).collect()
}
