use leetcode::root_equals_sum_of_children::{check_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn leaf(val: i32) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn root(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[test]
fn example_1() {
    // root = [10, 4, 6] -> 10 == 4 + 6
    let tree = root(10, leaf(4), leaf(6));
    assert!(check_tree(tree));
}

#[test]
fn example_2() {
    // root = [5, 3, 1] -> 5 != 3 + 1
    let tree = root(5, leaf(3), leaf(1));
    assert!(!check_tree(tree));
}

#[test]
fn negative_children_sum() {
    // root = [-100, -50, -50] -> -100 == -50 + -50
    let tree = root(-100, leaf(-50), leaf(-50));
    assert!(check_tree(tree));
}

#[test]
fn mixed_signs() {
    // root = [0, -7, 7] -> 0 == -7 + 7
    let tree = root(0, leaf(-7), leaf(7));
    assert!(check_tree(tree));
}

#[test]
fn off_by_one() {
    // root = [9, 4, 6] -> 9 != 4 + 6
    let tree = root(9, leaf(4), leaf(6));
    assert!(!check_tree(tree));
}
