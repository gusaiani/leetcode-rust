//! LeetCode 412. Fizz Buzz
//! https://leetcode.com/problems/fizz-buzz/
//!
//! Given an integer `n`, return a string array `answer` (1-indexed) where:
//! - `answer[i] == "FizzBuzz"` if `i` is divisible by 3 and 5.
//! - `answer[i] == "Fizz"` if `i` is divisible by 3.
//! - `answer[i] == "Buzz"` if `i` is divisible by 5.
//! - `answer[i] == i` (as a string) if none of the above conditions are true.
//!
//! Constraints: 1 <= n <= 10^4

pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        })
        .collect()
}
