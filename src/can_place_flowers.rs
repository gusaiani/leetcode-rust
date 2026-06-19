//! LeetCode 605. Can Place Flowers
//! https://leetcode.com/problems/can-place-flowers/
//!
//! You have a long flowerbed in which some plots are planted and some are not.
//! Flowers cannot be planted in adjacent plots. Given an integer array
//! `flowerbed` containing `0`s (empty) and `1`s (planted), and an integer `n`,
//! return `true` if `n` new flowers can be planted without violating the
//! no-adjacent-flowers rule, and `false` otherwise.

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut planted = 0;

    for i in 0..flowerbed.len() {
        let left_empty = i == 0 || flowerbed[i - 1] == 0;
        let right_empty = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;

        if flowerbed[i] == 0 && left_empty && right_empty {
            flowerbed[i] = 1;
            planted += 1;
        }
    }

    planted >= n
}
