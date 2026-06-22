//! LeetCode 1816. Truncate Sentence
//! https://leetcode.com/problems/truncate-sentence/
//!
//! A sentence is a list of words separated by single spaces with no leading or
//! trailing spaces. Each word consists of only uppercase and lowercase English
//! letters (no punctuation).
//!
//! You are given a sentence `s` and an integer `k`. Return the truncation of
//! `s` after the first `k` words (the sentence formed by its first `k` words).

pub fn truncate_sentence(s: String, k: i32) -> String {
    s.split_whitespace()
        .take(k as usize)
        .collect::<Vec<_>>()
        .join(" ")
}
