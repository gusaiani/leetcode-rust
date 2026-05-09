use leetcode::remove_element::remove_element;

fn assert_removed(mut nums: Vec<i32>, val: i32, expected_k: i32, mut expected_remaining: Vec<i32>) {
    let k = remove_element(&mut nums, val);
    assert_eq!(k, expected_k, "returned k mismatch");

    let mut head: Vec<i32> = nums.iter().take(k as usize).copied().collect();
    head.sort();
    expected_remaining.sort();
    assert_eq!(
        head, expected_remaining,
        "first k elements (order-insensitive) mismatch"
    );
}

#[test]
fn example_1() {
    assert_removed(vec![3, 2, 2, 3], 3, 2, vec![2, 2]);
}

#[test]
fn example_2() {
    assert_removed(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 1, 3, 0, 4]);
}

#[test]
fn empty_array() {
    assert_removed(vec![], 1, 0, vec![]);
}

#[test]
fn all_elements_match() {
    assert_removed(vec![4, 4, 4, 4], 4, 0, vec![]);
}

#[test]
fn no_elements_match() {
    assert_removed(vec![1, 2, 3, 4, 5], 9, 5, vec![1, 2, 3, 4, 5]);
}

#[test]
fn single_element_match() {
    assert_removed(vec![7], 7, 0, vec![]);
}

#[test]
fn single_element_no_match() {
    assert_removed(vec![7], 3, 1, vec![7]);
}

#[test]
fn val_at_boundaries() {
    assert_removed(vec![5, 1, 2, 3, 5], 5, 3, vec![1, 2, 3]);
}

#[test]
fn alternating_values() {
    assert_removed(vec![1, 2, 1, 2, 1, 2], 1, 3, vec![2, 2, 2]);
}
