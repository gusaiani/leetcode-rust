//! LeetCode 387. First Unique Character in a String
//! https://leetcode.com/problems/first-unique-character-in-a-string/
//!
//! Given a string `s`, find the first non-repeating character in it and return
//! its index. If it does not exist, return -1.
//!
//! Constraints:
//! - 1 <= s.length <= 10^5
//! - s consists of only lowercase English letters.

pub fn first_uniq_char(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut counts = [0i32; 26];
    for &b in bytes {
        counts[(b - b'a') as usize] += 1;
    }
    for (i, &b) in bytes.iter().enumerate() {
        if counts[(b - b'a') as usize] == 1 {
            return i as i32;
        }
    }
    -1
}
