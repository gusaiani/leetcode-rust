//! LeetCode 617. Merge Two Binary Trees
//! https://leetcode.com/problems/merge-two-binary-trees/
//!
//! You are given two binary trees `root1` and `root2`.
//!
//! Imagine that when you put one of them to cover the other, some nodes of the
//! two trees are overlapped while the others are not. You need to merge the two
//! trees into a new binary tree. The merge rule is that if two nodes overlap,
//! then sum node values up as the new value of the merged node. Otherwise, the
//! NOT null node will be used as the node of the new tree.
//!
//! Return the merged tree.
//!
//! Note: The merging process must start from the root nodes of both trees.
//!
//! Constraints:
//! - The number of nodes in both trees is in the range `[0, 2000]`.
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

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root1.is_none() {
        return root2;
    }

    if root2.is_none() {
        return root1;
    }

    let n1 = root1.unwrap();
    let n2 = root2.unwrap();

    let merged_val = n1.borrow().val + n2.borrow().val;
    let mut merged = TreeNode::new(merged_val);

    merged.left = merge_trees(n1.borrow_mut().left.take(), n2.borrow_mut().left.take());
    merged.right = merge_trees(n1.borrow_mut().right.take(), n2.borrow_mut().right.take());

    Some(Rc::new(RefCell::new(merged)))
}
