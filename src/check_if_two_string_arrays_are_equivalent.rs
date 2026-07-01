//! LeetCode 1662. Check If Two String Arrays Are Equivalent
//! https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
//!
//! Given two string arrays `word1` and `word2`, return `true` if the two arrays
//! represent the same string, and `false` otherwise. A string is represented by
//! an array if the array elements concatenated in order form the string.

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let left = word1.iter().flat_map(|s| s.chars());
    let right = word2.iter().flat_map(|s| s.chars());

    left.eq(right)
}
