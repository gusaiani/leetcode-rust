//! LeetCode 2652. Sum Multiples
//! https://leetcode.com/problems/sum-multiples/
//!
//! Given a positive integer `n`, return the sum of all integers in the range
//! `[1, n]` inclusive that are divisible by 3, 5, or 7.

pub fn sum_of_multiples(n: i32) -> i32 {
    let mut sum = 0;

    for i in 3..=n {
        if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            sum += i;
        }
    }

    sum
}
