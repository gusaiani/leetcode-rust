//! LeetCode 67. Add Binary
//! https://leetcode.com/problems/add-binary/
//!
//! Given two binary strings `a` and `b`, return their sum as a binary string.

pub fn add_binary(a: String, b: String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = 0u8;
    let mut digits = Vec::with_capacity(a.len().max(b.len()) + 1);

    while i > 0 || j > 0 || carry > 0 {
        let mut sum = carry;
        if i > 0 {
            i -= 1;
            sum += a[i] - b'0';
        }
        if j > 0 {
            j -= 1;
            sum += b[j] - b'0';
        }
        digits.push(b'0' + sum % 2);
        carry = sum / 2;
    }

    digits.reverse();
    String::from_utf8(digits).unwrap()
}
