use leetcode::find_target_indices_after_sorting_array::target_indices;

#[test]
fn example_1() {
    // sorted: [1,2,2,3,5], target 2 -> indices 1 and 2
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
}

#[test]
fn example_2() {
    // sorted: [1,2,2,3,5], target 3 -> index 3
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
}

#[test]
fn example_3() {
    // sorted: [1,2,2,3,5], target 5 -> index 4
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
}

#[test]
fn target_absent() {
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 4), Vec::<i32>::new());
}

#[test]
fn single_element_match() {
    assert_eq!(target_indices(vec![7], 7), vec![0]);
}

#[test]
fn single_element_no_match() {
    assert_eq!(target_indices(vec![7], 1), Vec::<i32>::new());
}

#[test]
fn all_equal_to_target() {
    assert_eq!(target_indices(vec![4, 4, 4], 4), vec![0, 1, 2]);
}

#[test]
fn target_is_smallest() {
    // sorted: [1,1,3,8], target 1 -> indices 0 and 1
    assert_eq!(target_indices(vec![8, 1, 3, 1], 1), vec![0, 1]);
}

#[test]
fn target_is_largest() {
    // sorted: [2,4,9,9], target 9 -> indices 2 and 3
    assert_eq!(target_indices(vec![9, 2, 9, 4], 9), vec![2, 3]);
}
