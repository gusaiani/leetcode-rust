//! LeetCode 1047. Remove All Adjacent Duplicates In String
//! https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
//!
//! You are given a string `s` consisting of lowercase English letters. A
//! duplicate removal consists of choosing two adjacent and equal letters and
//! removing them.
//!
//! We repeatedly make duplicate removals on `s` until we no longer can.
//!
//! Return the final string after all such duplicate removals have been made.
//! It can be proven that the answer is unique.

pub fn remove_duplicates(s: String) -> String {
    let mut stack: Vec<u8> = Vec::new();

    for byte in s.into_bytes() {
        if stack.last() == Some(&byte) {
            stack.pop();
        } else {
            stack.push(byte);
        }
    }

    String::from_utf8(stack).unwrap()
}
