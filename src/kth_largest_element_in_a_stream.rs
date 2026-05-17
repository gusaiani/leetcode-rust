//! LeetCode 703. Kth Largest Element in a Stream
//! https://leetcode.com/problems/kth-largest-element-in-a-stream/
//!
//! Design a class to find the kth largest element in a stream. Note that it is
//! the kth largest element in the sorted order, not the kth distinct element.
//!
//! Implement `KthLargest`:
//! - `KthLargest::new(k, nums)` initializes the object with the integer `k` and
//!   the stream of integers `nums`.
//! - `add(val)` appends `val` to the stream and returns the element representing
//!   the kth largest element in the stream.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth = KthLargest {
            k: k as usize,
            heap: BinaryHeap::with_capacity(k as usize + 1),
        };
        for n in nums {
            kth.add(n);
        }
        kth
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        // Guaranteed by the problem to have >= k elements when queried.
        self.heap.peek().map(|&Reverse(v)| v).unwrap()
    }
}
