//! LeetCode 1207. Unique Number of Occurrences
//! https://leetcode.com/problems/unique-number-of-occurrences/
//!
//! Given an array of integers `arr`, return `true` if the number of occurrences
//! of each value in the array is unique, or `false` otherwise.

use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut occurrences = HashMap::new();

    for item in arr {
        *occurrences.entry(item).or_insert(0) += 1;
    }

    let distinct_counts: HashSet<i32> = occurrences.values().copied().collect();
    distinct_counts.len() == occurrences.len()
}
