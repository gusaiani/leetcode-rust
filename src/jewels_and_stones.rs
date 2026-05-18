//! LeetCode 771. Jewels and Stones
//! https://leetcode.com/problems/jewels-and-stones/
//!
//! You're given strings `jewels` representing the types of stones that are
//! jewels, and `stones` representing the stones you have. Each character in
//! `stones` is a type of stone you have. You want to know how many of the
//! stones you have are also jewels.
//!
//! Letters are case sensitive, so `"a"` is considered a different type of
//! stone from `"A"`.

use std::collections::HashSet;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewel_set: HashSet<u8> = jewels.bytes().collect();
    stones.bytes().filter(|b| jewel_set.contains(b)).count() as i32
}
