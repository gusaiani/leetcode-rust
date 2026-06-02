//! LeetCode 1342. Number of Steps to Reduce a Number to Zero
//! https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
//!
//! Given an integer `num`, return the number of steps to reduce it to zero.
//!
//! In one step, if the current number is even, you have to divide it by 2,
//! otherwise, you have to subtract 1 from it.

pub fn number_of_steps(num: i32) -> i32 {
    let mut n = num;
    let mut steps = 0;

    while n != 0 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n -= 1;
        }
        steps += 1;
    }

    steps
}
