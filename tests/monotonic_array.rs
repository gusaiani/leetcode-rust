use leetcode::monotonic_array::is_monotonic;

#[test]
fn example_1_increasing() {
    assert!(is_monotonic(vec![1, 2, 2, 3]));
}

#[test]
fn example_2_decreasing() {
    assert!(is_monotonic(vec![6, 5, 4, 4]));
}

#[test]
fn example_3_not_monotonic() {
    assert!(!is_monotonic(vec![1, 3, 2]));
}

#[test]
fn single_element() {
    assert!(is_monotonic(vec![1]));
}

#[test]
fn two_equal_elements() {
    assert!(is_monotonic(vec![7, 7]));
}

#[test]
fn all_equal() {
    assert!(is_monotonic(vec![3, 3, 3, 3]));
}

#[test]
fn strictly_increasing() {
    assert!(is_monotonic(vec![-5, -1, 0, 10, 11]));
}

#[test]
fn strictly_decreasing() {
    assert!(is_monotonic(vec![11, 10, 0, -1, -5]));
}

#[test]
fn up_then_down() {
    assert!(!is_monotonic(vec![1, 2, 1]));
}

#[test]
fn down_then_up() {
    assert!(!is_monotonic(vec![5, 4, 6]));
}

#[test]
fn negative_increasing() {
    assert!(is_monotonic(vec![-3, -2, -2, -1]));
}
