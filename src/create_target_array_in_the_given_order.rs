//! LeetCode 1389. Create Target Array in the Given Order
//! https://leetcode.com/problems/create-target-array-in-the-given-order/
//!
//! Given two arrays of integers `nums` and `index`, create a target array
//! under the following rules:
//!
//! - Initially the target array is empty.
//! - From left to right, read `nums[i]` and `index[i]`, then insert the value
//!   `nums[i]` at position `index[i]` in the target array.
//! - Repeat until there are no elements to read in `nums` and `index`.
//!
//! Return the target array. It is guaranteed that the insertion operations are
//! always valid.

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut target: Vec<i32> = Vec::new();

    for (value, position) in nums.iter().zip(index.iter()) {
        target.insert(*position as usize, *value);
    }

    target
}
