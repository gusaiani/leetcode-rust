//! LeetCode 1470. Shuffle the Array
//! https://leetcode.com/problems/shuffle-the-array/
//!
//! Given the array `nums` consisting of `2n` elements in the form
//! `[x1, x2, ..., xn, y1, y2, ..., yn]`, return the array in the form
//! `[x1, y1, x2, y2, ..., xn, yn]`.

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = Vec::with_capacity(nums.len());
    for i in 0..n {
        result.push(nums[i]); // x_i
        result.push(nums[i + n]); // y_i
    }
    result
}

// works but the one above takes half the length of nums
#[allow(clippy::needless_range_loop)]
pub fn shuffle_naive(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = vec![0; n * 2];

    for i in 0..n * 2 {
        let are_we_over_half_of_nums = i >= n;
        let mut index_to_place = i * 2;

        if are_we_over_half_of_nums {
            index_to_place = i * 2 - n * 2 + 1;
        }

        result[index_to_place] = nums[i];
    }

    result
}
