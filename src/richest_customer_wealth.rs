//! LeetCode 1672. Richest Customer Wealth
//! https://leetcode.com/problems/richest-customer-wealth/
//!
//! You are given an `m x n` integer grid `accounts` where `accounts[i][j]` is
//! the amount of money the `i`-th customer has in the `j`-th bank. Return the
//! wealth that the richest customer has.
//!
//! A customer's wealth is the total amount of money they have across all their
//! bank accounts. The richest customer is the one with the maximum wealth.

/*
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max: i32 = 0;

    for account in &accounts {
        let mut curr = 0;
        for &amount in account {
            curr += amount;
        }
        if curr > max {
            max = curr
        }
    }

    max
}
 */

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum::<i32>())
        .max()
        .unwrap_or(0)
}
