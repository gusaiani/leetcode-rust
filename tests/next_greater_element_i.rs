use leetcode::next_greater_element_i::next_greater_element;

#[test]
fn example_1() {
    assert_eq!(
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}

#[test]
fn single_element_no_greater() {
    assert_eq!(next_greater_element(vec![1], vec![1]), vec![-1]);
}

#[test]
fn last_element_is_largest() {
    assert_eq!(next_greater_element(vec![9], vec![5, 9]), vec![-1]);
}

#[test]
fn strictly_increasing() {
    assert_eq!(
        next_greater_element(vec![1, 2, 3], vec![1, 2, 3, 4]),
        vec![2, 3, 4]
    );
}

#[test]
fn strictly_decreasing_all_minus_one() {
    assert_eq!(
        next_greater_element(vec![4, 3, 2, 1], vec![4, 3, 2, 1]),
        vec![-1, -1, -1, -1]
    );
}

#[test]
fn subset_preserves_query_order() {
    assert_eq!(
        next_greater_element(vec![2, 1, 3], vec![3, 1, 2, 4]),
        vec![4, 2, 4]
    );
}
