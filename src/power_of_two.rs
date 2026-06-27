//! LeetCode 231. Power of Two
//! https://leetcode.com/problems/power-of-two/
//!
//! Given an integer `n`, return `true` if it is a power of two. Otherwise,
//! return `false`.
//!
//! An integer `n` is a power of two if there exists an integer `x` such that
//! `n == 2^x`.
//!
//! Constraints: `-2^31 <= n <= 2^31 - 1`.

pub fn is_power_of_two(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut n = n;

    while n > 1 {
        if n % 2 != 0 {
            return false;
        }

        n /= 2;
    }

    true
}
