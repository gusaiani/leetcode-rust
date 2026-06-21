//! LeetCode 496. Next Greater Element I
//! https://leetcode.com/problems/next-greater-element-i/
//!
//! The next greater element of some element `x` in an array is the first greater
//! element that is to the right of `x` in the same array.
//!
//! You are given two distinct 0-indexed integer arrays `nums1` and `nums2`,
//! where `nums1` is a subset of `nums2`.
//!
//! For each `0 <= i < nums1.length`, find the index `j` such that
//! `nums1[i] == nums2[j]` and determine the next greater element of `nums2[j]`
//! in `nums2`. If there is no next greater element, the answer for this query
//! is `-1`.
//!
//! Return an array `ans` of length `nums1.length` such that `ans[i]` is the
//! next greater element as described above.
use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut next_greater: HashMap<i32, i32> = HashMap::new();
    let mut stack: Vec<i32> = Vec::new();

    for num in nums2 {
        while let Some(&top) = stack.last() {
            if top < num {
                stack.pop();
                next_greater.insert(top, num);
            } else {
                break;
            }
        }
        stack.push(num);
    }

    nums1
        .iter()
        .map(|num| *next_greater.get(num).unwrap_or(&-1))
        .collect()
}
