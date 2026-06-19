use leetcode::unique_number_of_occurrences::unique_occurrences;

#[test]
fn example_1() {
    // counts: 1->1, 2->2, 3->3 -> all unique
    assert!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
}

#[test]
fn example_2() {
    // counts: 1->2, 2->2 -> duplicate count
    assert!(!unique_occurrences(vec![1, 2]));
}

#[test]
fn example_3() {
    // counts: -3->2, 0->1, 1->2, 2->1 -> duplicate counts (two 2s, two 1s)
    assert!(unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]));
}

#[test]
fn single_element() {
    assert!(unique_occurrences(vec![5]));
}

#[test]
fn all_same_value() {
    // single value -> single count -> trivially unique
    assert!(unique_occurrences(vec![7, 7, 7, 7]));
}

#[test]
fn two_values_same_count() {
    // counts: 1->3, 2->3 -> not unique
    assert!(!unique_occurrences(vec![1, 1, 1, 2, 2, 2]));
}

#[test]
fn negatives_only() {
    // counts: -1->1, -2->2, -3->3 -> all unique
    assert!(unique_occurrences(vec![-1, -2, -2, -3, -3, -3]));
}
