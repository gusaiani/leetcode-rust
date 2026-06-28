//! LeetCode 2236. Root Equals Sum of Children
//! https://leetcode.com/problems/root-equals-sum-of-children/
//!
//! You are given the `root` of a binary tree that consists of exactly 3 nodes:
//! the root, its left child, and its right child.
//!
//! Return `true` if the value of the root is equal to the sum of the values of
//! its two children, or `false` otherwise.
//!
//! Constraints:
//! - The tree consists only of the root, its left child, and its right child.
//! - `-100 <= Node.val <= 100`.

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

pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let node = root.unwrap();
    let node = node.borrow();

    let left_val = node.left.as_ref().unwrap().borrow().val;
    let right_val = node.right.as_ref().unwrap().borrow().val;

    node.val == left_val + right_val
}
