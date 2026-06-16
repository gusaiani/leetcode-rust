//! LeetCode 1512. Number of Good Pairs
//! https://leetcode.com/problems/number-of-good-pairs/
//!
//! Given an array of integers `nums`, return the number of good pairs.
//! A pair `(i, j)` is called good if `nums[i] == nums[j]` and `i < j`.

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut identical_pairs = 0;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                identical_pairs += 1;
            }
        }
    }

    identical_pairs
}
