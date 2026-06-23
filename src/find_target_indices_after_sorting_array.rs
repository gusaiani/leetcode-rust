//! LeetCode 2089. Find Target Indices After Sorting Array
//! https://leetcode.com/problems/find-target-indices-after-sorting-array/
//!
//! You are given a 0-indexed integer array `nums` and a target element `target`.
//!
//! A target index is an index `i` such that `nums[i] == target` after sorting
//! `nums` in non-decreasing order.
//!
//! Return a list of the target indices of `nums` in increasing order. If there
//! are no target indices, return an empty list. The returned list must be
//! sorted in increasing order.

pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort_unstable();

    let mut result: Vec<i32> = Vec::new();

    for (i, &v) in nums.iter().enumerate() {
        if v == target {
            result.push(i as i32);
        }
    }

    result
}
