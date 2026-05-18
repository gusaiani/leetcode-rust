//! LeetCode 1480. Running Sum of 1d Array
//! https://leetcode.com/problems/running-sum-of-1d-array/
//!
//! Given an array `nums`, the running sum of an array is defined as
//! `runningSum[i] = sum(nums[0]..=nums[i])`.
//!
//! Return the running sum of `nums`.

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}
