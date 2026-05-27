//! LeetCode 1929. Concatenation of Array
//! https://leetcode.com/problems/concatenation-of-array/
//!
//! Given an integer array `nums` of length `n`, you want to create an array
//! `ans` of length `2n` where `ans[i] == nums[i]` and `ans[i + n] == nums[i]`
//! for `0 <= i < n` (0-indexed). Specifically, `ans` is the concatenation of
//! two `nums` arrays. Return the array `ans`.

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len() * 2);
    result.extend_from_slice(&nums);
    result.extend_from_slice(&nums);
    result
}
