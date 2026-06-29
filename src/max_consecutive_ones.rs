//! LeetCode 485. Max Consecutive Ones
//! https://leetcode.com/problems/max-consecutive-ones/
//!
//! Given a binary array `nums`, return the maximum number of consecutive `1`s
//! in the array.

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut current = 0;

    for num in nums {
        if num == 1 {
            current += 1;
            max = max.max(current);
        } else {
            current = 0;
        }
    }

    max
}
