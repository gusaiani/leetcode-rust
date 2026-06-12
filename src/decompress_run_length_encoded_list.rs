//! LeetCode 1313. Decompress Run-Length Encoded List
//! https://leetcode.com/problems/decompress-run-length-encoded-list/
//!
//! We are given a list `nums` of integers representing a list compressed with
//! run-length encoding. Consider each adjacent pair of elements
//! `[freq, val] = [nums[2*i], nums[2*i + 1]]` (with `i >= 0`). For each such
//! pair, there are `freq` elements with value `val` concatenated in a sublist.
//! Concatenate all the sublists from left to right to generate the
//! decompressed list. Return the decompressed list.

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for pair in nums.chunks_exact(2) {
        let freq = pair[0];
        let val = pair[1];

        result.extend(std::iter::repeat_n(val, freq as usize));
    }

    result
}
