//! LeetCode 344. Reverse String
//! https://leetcode.com/problems/reverse-string/
//!
//! Write a function that reverses a string. The input string is given as an
//! array of characters `s`.
//!
//! You must do this by modifying the input array in-place with O(1) extra
//! memory.

pub fn reverse_string(s: &mut [char]) {
    let n = s.len();
    for i in 0..n / 2 {
        s.swap(i, n - 1 - i);
    }
}
