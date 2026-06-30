//! LeetCode 709. To Lower Case
//! https://leetcode.com/problems/to-lower-case/
//!
//! Given a string `s`, return the string after replacing every uppercase
//! letter with the same lowercase letter.

pub fn to_lower_case(s: String) -> String {
    let mut bytes = s.into_bytes();
    bytes.make_ascii_lowercase();
    String::from_utf8(bytes).unwrap()
}
