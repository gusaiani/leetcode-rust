//! LeetCode 1832. Check if the Sentence Is a Pangram
//! https://leetcode.com/problems/check-if-the-sentence-is-pangram/
//!
//! A pangram is a sentence where every letter of the English alphabet appears
//! at least once. Given a string `sentence` containing only lowercase English
//! letters, return `true` if `sentence` is a pangram, or `false` otherwise.

pub fn check_if_pangram(sentence: String) -> bool {
    let mut seen: u32 = 0;

    for b in sentence.bytes() {
        let bit = b - b'a';
        seen |= 1 << bit;
    }

    seen == (1 << 26) - 1
}
