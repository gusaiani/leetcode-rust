//! LeetCode 27. Remove Element
//! https://leetcode.com/problems/remove-element/
//!
//! Given an integer array `nums` and an integer `val`, remove all occurrences of
//! `val` in `nums` in-place. The order of the elements may be changed. Then return
//! the number of elements in `nums` which are not equal to `val`.
//!
//! Consider the number of elements in `nums` which are not equal to `val` be `k`.
//! To get accepted, you need to do the following:
//!
//! - Change the array `nums` such that the first `k` elements of `nums` contain
//!   the elements which are not equal to `val`. The remaining elements of `nums`
//!   are not important as well as the size of `nums`.
//! - Return `k`.

pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}
