//! LeetCode 938. Range Sum of BST
//! https://leetcode.com/problems/range-sum-of-bst/
//!
//! Given the `root` node of a binary search tree and two integers `low` and
//! `high`, return the sum of values of all nodes with a value in the inclusive
//! range `[low, high]`.
//!
//! Constraints:
//! - The number of nodes in the tree is in the range `[1, 2 * 10^4]`.
//! - `1 <= Node.val <= 10^5`.
//! - `1 <= low <= high <= 10^5`.
//! - All `Node.val` are unique.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let node = match root {
        Some(n) => n,
        None => return 0,
    };

    let node = node.borrow();

    let mut result = 0;
    if node.val >= low && node.val <= high {
        result += node.val;
    }

    result += range_sum_bst(node.left.clone(), low, high);
    result += range_sum_bst(node.right.clone(), low, high);

    result
}
