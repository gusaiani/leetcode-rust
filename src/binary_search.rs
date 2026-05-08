//! LeetCode 704. Binary Search
//! https://leetcode.com/problems/binary-search/
//!
//! Given an array of integers `nums` which is sorted in ascending order,
//! and an integer `target`, return the index of `target` in `nums`.
//! If it does not exist, return -1.
//!
//! You must write an algorithm with O(log n) runtime complexity.

use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0usize;
    let mut right = nums.len(); // exclusive

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    -1
}
