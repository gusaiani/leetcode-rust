//! LeetCode 1979. Find Greatest Common Divisor of Array
//! https://leetcode.com/problems/find-greatest-common-divisor-of-array/
//!
//! Given an integer array `nums`, return the greatest common divisor of the
//! smallest number and largest number in `nums`.
//!
//! The greatest common divisor of two numbers is the largest positive integer
//! that evenly divides both numbers.

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut smallest = i32::MAX;
    let mut largest = i32::MIN;

    for &n in &nums {
        smallest = smallest.min(n);
        largest = largest.max(n);
    }

    greatest_common_denominator(largest, smallest)
}

fn greatest_common_denominator(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        greatest_common_denominator(b, a % b)
    }
}
