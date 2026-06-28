use leetcode::range_sum_of_bst::{range_sum_bst, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn leaf(val: i32) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn node(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[test]
fn example_1() {
    // root = [10,5,15,3,7,null,18], low = 7, high = 15 -> 7 + 10 + 15 = 32
    let tree = node(10, node(5, leaf(3), leaf(7)), node(15, None, leaf(18)));
    assert_eq!(range_sum_bst(tree, 7, 15), 32);
}

#[test]
fn example_2() {
    // root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10 -> 6 + 7 + 10 = 23
    let tree = node(
        10,
        node(5, node(3, leaf(1), None), node(7, leaf(6), None)),
        node(15, leaf(13), leaf(18)),
    );
    assert_eq!(range_sum_bst(tree, 6, 10), 23);
}

#[test]
fn single_node_in_range() {
    assert_eq!(range_sum_bst(leaf(5), 1, 10), 5);
}

#[test]
fn single_node_out_of_range() {
    assert_eq!(range_sum_bst(leaf(100), 1, 10), 0);
}

#[test]
fn boundaries_are_inclusive() {
    // low and high themselves count.
    let tree = node(5, leaf(3), leaf(8));
    assert_eq!(range_sum_bst(tree, 3, 8), 16);
}

#[test]
fn empty_tree() {
    assert_eq!(range_sum_bst(None, 1, 10), 0);
}
