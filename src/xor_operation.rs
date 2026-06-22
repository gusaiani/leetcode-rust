//! LeetCode 1486. XOR Operation in an Array
//! https://leetcode.com/problems/xor-operation-in-an-array/
//!
//! You are given an integer `n` and an integer `start`.
//!
//! Define an array `nums` where `nums[i] = start + 2 * i` (0-indexed) and
//! `n == nums.length`.
//!
//! Return the bitwise XOR of all elements of `nums`.

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let values = (0..n).map(|i| start + 2 * i);
    values.fold(0, |acc, value| acc ^ value)
}
