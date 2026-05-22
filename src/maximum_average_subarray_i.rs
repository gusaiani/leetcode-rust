//! LeetCode 643. Maximum Average Subarray I
//! https://leetcode.com/problems/maximum-average-subarray-i/
//!
//! You are given an integer array `nums` consisting of `n` elements, and an
//! integer `k`.
//!
//! Find a contiguous subarray whose length is equal to `k` that has the
//! maximum average value and return this value. Any answer with a calculation
//! error less than 10^-5 will be accepted.
//!
//! Example 1:
//!   Input: nums = [1,12,-5,-6,50,3], k = 4
//!   Output: 12.75
//!   Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
//!
//! Example 2:
//!   Input: nums = [5], k = 1
//!   Output: 5.00000
//!
//! Constraints:
//!   n == nums.length
//!   1 <= k <= n <= 10^5
//!   -10^4 <= nums[i] <= 10^4

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;

    // Sum of the first window. Widen to i64 so a long window of extreme
    // values can never overflow during accumulation.
    let mut window: i64 = nums[..k].iter().copied().map(i64::from).sum();
    let mut best = window;

    // Slide the fixed-size window: add the entering element, drop the
    // leaving one — O(1) per step instead of re-summing k elements.
    for i in k..nums.len() {
        window += i64::from(nums[i]) - i64::from(nums[i - k]);
        best = best.max(window);
    }

    best as f64 / k as f64
}
