//! LeetCode 110. Balanced Binary Tree
//! https://leetcode.com/problems/balanced-binary-tree/
//!
//! Given a binary tree, determine if it is height-balanced: a binary tree in
//! which the left and right subtrees of every node differ in height by no
//! more than 1.
//!
//! Constraints:
//! - The number of nodes in the tree is in the range `[0, 5000]`.
//! - `-10^4 <= Node.val <= 10^4`.

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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    height(&root) != i32::MIN
}

fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let node = match node {
        None => return -1,
        Some(n) => n.borrow(),
    };

    let left_height = height(&node.left);
    if left_height == i32::MIN {
        return i32::MIN;
    }

    let right_height = height(&node.right);
    if right_height == i32::MIN {
        return i32::MIN;
    }

    if (left_height - right_height).abs() > 1 {
        return i32::MIN;
    }

    left_height.max(right_height) + 1
}
