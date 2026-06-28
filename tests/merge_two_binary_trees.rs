use leetcode::merge_two_binary_trees::{merge_trees, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn node(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn leaf(val: i32) -> Tree {
    node(val, None, None)
}

#[test]
fn example_1() {
    // root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
    // merged = [3,4,5,5,4,null,7]
    let root1 = node(1, node(3, leaf(5), None), leaf(2));
    let root2 = node(2, node(1, None, leaf(4)), node(3, None, leaf(7)));

    let expected = node(3, node(4, leaf(5), leaf(4)), node(5, None, leaf(7)));

    assert_eq!(merge_trees(root1, root2), expected);
}

#[test]
fn example_2() {
    // root1 = [1], root2 = [1,2] (2 is the left child)
    // merged = [2,2]
    let root1 = leaf(1);
    let root2 = node(1, leaf(2), None);

    let expected = node(2, leaf(2), None);

    assert_eq!(merge_trees(root1, root2), expected);
}

#[test]
fn both_empty() {
    assert_eq!(merge_trees(None, None), None);
}

#[test]
fn first_empty_returns_second() {
    let root2 = node(7, leaf(4), leaf(9));
    let expected = node(7, leaf(4), leaf(9));

    assert_eq!(merge_trees(None, root2), expected);
}

#[test]
fn second_empty_returns_first() {
    let root1 = node(7, leaf(4), leaf(9));
    let expected = node(7, leaf(4), leaf(9));

    assert_eq!(merge_trees(root1, None), expected);
}

#[test]
fn single_nodes_sum() {
    assert_eq!(merge_trees(leaf(5), leaf(-3)), leaf(2));
}

#[test]
fn differing_shapes_keep_non_null_child() {
    // root1 has only a left child, root2 has only a right child.
    let root1 = node(1, leaf(2), None);
    let root2 = node(1, None, leaf(3));

    let expected = node(2, leaf(2), leaf(3));

    assert_eq!(merge_trees(root1, root2), expected);
}
