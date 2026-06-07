//! LeetCode 228. Summary Ranges
//! https://leetcode.com/problems/summary-ranges/
//!
//! You are given a sorted unique integer array `nums`.
//!
//! A range `[a, b]` is the set of all integers from `a` to `b` (inclusive).
//!
//! Return the smallest sorted list of ranges that cover all the numbers in the
//! array exactly. That is, each element of `nums` is covered by exactly one of
//! the ranges, and there is no integer `x` such that `x` is in one of the
//! ranges but not in `nums`.
//!
//! Each range `[a, b]` in the list should be output as:
//! - `"a->b"` if `a != b`
//! - `"a"` if `a == b`

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < nums.len() {
        let start = nums[i];

        while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
            i += 1;
        }

        let end = nums[i];

        if start == end {
            ranges.push(start.to_string());
        } else {
            ranges.push(format!("{start}->{end}"));
        }

        i += 1;
    }

    ranges
}
