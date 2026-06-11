//! LeetCode 1822. Sign of the Product of an Array
//! https://leetcode.com/problems/sign-of-the-product-of-an-array/
//!
//! There is a function `signFunc(x)` that returns:
//!   - `1` if `x` is positive.
//!   - `-1` if `x` is negative.
//!   - `0` if `x` is equal to `0`.
//!
//! You are given an integer array `nums`. Let `product` be the product of all
//! values in the array `nums`.
//!
//! Return `signFunc(product)`.

use std::cmp::Ordering;

pub fn array_sign(nums: Vec<i32>) -> i32 {
    nums.iter().fold(1, |sign, &x| match x.cmp(&0) {
        Ordering::Less => -sign,
        Ordering::Equal => 0,
        Ordering::Greater => sign,
    })
}
