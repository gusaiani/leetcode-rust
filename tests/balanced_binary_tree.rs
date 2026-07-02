use leetcode::balanced_binary_tree::{is_balanced, TreeNode};
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
    // root = [3,9,20,null,null,15,7] -> balanced
    let tree = node(3, leaf(9), node(20, leaf(15), leaf(7)));
    assert!(is_balanced(tree));
}

#[test]
fn example_2() {
    // root = [1,2,2,3,3,null,null,4,4] -> not balanced
    let tree = node(
        1,
        node(2, node(3, leaf(4), leaf(4)), node(3, None, None)),
        leaf(2),
    );
    assert!(!is_balanced(tree));
}

#[test]
fn example_3() {
    // root = [] -> balanced
    assert!(is_balanced(None));
}

#[test]
fn single_node() {
    assert!(is_balanced(leaf(1)));
}

#[test]
fn left_skewed_chain() {
    // A straight line of left children is never balanced beyond 2 levels.
    let tree = node(1, node(2, leaf(3), None), None);
    assert!(!is_balanced(tree));
}

#[test]
fn balanced_with_one_missing_leaf() {
    // root has two children, one of which has a single child of its own —
    // still within the +/-1 height tolerance.
    let tree = node(1, node(2, leaf(3), None), leaf(4));
    assert!(is_balanced(tree));
}
