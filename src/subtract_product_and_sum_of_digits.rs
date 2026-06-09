//! LeetCode 1281. Subtract the Product and Sum of Digits of an Integer
//! https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
//!
//! Given an integer number `n`, return the difference between the product of
//! its digits and the sum of its digits.

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut remaining = n;
    let mut product = 1;
    let mut sum = 0;

    while remaining > 0 {
        let digit = remaining % 10;
        product *= digit;
        sum += digit;
        remaining /= 10;
    }

    product - sum
}
