//! LeetCode 2114. Maximum Number of Words Found in Sentences
//! https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
//!
//! A sentence is a list of words separated by single spaces with no leading or
//! trailing spaces. Given an array of sentences, return the maximum number of
//! words that appear in a single sentence.

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count() as i32)
        .max()
        .unwrap_or(0)
}
