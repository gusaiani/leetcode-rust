//! LeetCode 258. Add Digits
//! https://leetcode.com/problems/add-digits/
//!
//! Given an integer `num`, repeatedly add all its digits until the result has
//! only one digit, and return it.

/// Iterative digit-sum: sum the digits in a pass, recurse while the sum still
/// has more than one digit. O(log num) time, O(1) space.
pub fn add_digits(num: i32) -> i32 {
    let mut num = num;
    let mut result = 0;

    while num > 0 {
        result += num % 10;
        num /= 10;
    }

    if result < 10 {
        result
    } else {
        add_digits(result)
    }
}

/// Digital root in O(1): a number is congruent to its digit sum modulo 9, so
/// the repeated digit sum collapses to `1 + (num - 1) % 9` for `num > 0`
/// (with 0 mapping to 0). No loop, no recursion.
pub fn add_digits_digital_root(num: i32) -> i32 {
    if num == 0 {
        0
    } else {
        1 + (num - 1) % 9
    }
}
