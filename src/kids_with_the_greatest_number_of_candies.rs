//! LeetCode 1431. Kids With the Greatest Number of Candies
//! https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
//!
//! There are `n` kids with candies. You are given an integer array `candies`,
//! where each `candies[i]` represents the number of candies the ith kid has,
//! and an integer `extra_candies`, denoting the number of extra candies that
//! you have.
//!
//! Return a boolean array `result` of length `n`, where `result[i]` is `true`
//! if, after giving the ith kid all the `extra_candies`, they will have the
//! greatest number of candies among all the kids, or `false` otherwise.
//!
//! Note that multiple kids can have the greatest number of candies.

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let most = *candies.iter().max().unwrap();

    candies
        .iter()
        .map(|&kid| kid + extra_candies >= most)
        .collect()
}
