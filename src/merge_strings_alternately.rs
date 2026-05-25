//! LeetCode 1768. Merge Strings Alternately
//! https://leetcode.com/problems/merge-strings-alternately/
//!
//! You are given two strings `word1` and `word2`. Merge the strings by adding
//! letters in alternating order, starting with `word1`. If a string is longer
//! than the other, append the additional letters onto the end of the merged
//! string. Return the merged string.

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::with_capacity(word1.len() + word2.len());
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(a), Some(b)) => {
                result.push(a);
                result.push(b);
            }
            (Some(a), None) => result.push(a),
            (None, Some(b)) => result.push(b),
            (None, None) => break,
        }
    }

    result
}
