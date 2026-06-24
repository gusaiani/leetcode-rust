//! LeetCode 896. Monotonic Array
//! https://leetcode.com/problems/monotonic-array/
//!
//! An array is monotonic if it is either monotone increasing or monotone
//! decreasing. An array `nums` is monotone increasing if for all `i <= j`,
//! `nums[i] <= nums[j]`. An array `nums` is monotone decreasing if for all
//! `i <= j`, `nums[i] >= nums[j]`.
//!
//! Given an integer array `nums`, return `true` if the given array is
//! monotonic, or `false` otherwise.

pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut is_direction_decided = false;
    let mut is_growing = false;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            if is_direction_decided && !is_growing {
                return false;
            }
            if !is_direction_decided {
                is_direction_decided = true;
                is_growing = true;
            }
        }
        if nums[i] < nums[i - 1] {
            if is_direction_decided && is_growing {
                return false;
            }
            if !is_direction_decided {
                is_direction_decided = true;
            }
        }
    }

    true
}
