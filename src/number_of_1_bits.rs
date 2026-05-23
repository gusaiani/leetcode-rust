//! LeetCode 191. Number of 1 Bits
//! https://leetcode.com/problems/number-of-1-bits/
//!
//! Given a positive integer `n`, write a function that returns the number of
//! set bits in its binary representation (also known as the Hamming weight).

pub fn hamming_weight(n: i32) -> i32 {
    let mut bits = n as u32;
    let mut count = 0;
    while bits != 0 {
        bits &= bits - 1;
        count += 1;
    }
    count
}
