use leetcode::binary_search::search;

#[test]
fn example_1_target_found_in_middle() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(search(nums, 9), 4);
}

#[test]
fn example_2_target_not_present() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(search(nums, 2), -1);
}

#[test]
fn single_element_match() {
    assert_eq!(search(vec![5], 5), 0);
}

#[test]
fn single_element_no_match() {
    assert_eq!(search(vec![5], -3), -1);
}

#[test]
fn target_at_first_index() {
    let nums = vec![-10, -5, 0, 3, 7, 11];
    assert_eq!(search(nums, -10), 0);
}

#[test]
fn target_at_last_index() {
    let nums = vec![-10, -5, 0, 3, 7, 11];
    assert_eq!(search(nums, 11), 5);
}

#[test]
fn target_smaller_than_all() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(search(nums, -100), -1);
}

#[test]
fn target_larger_than_all() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(search(nums, 100), -1);
}

#[test]
fn large_input_finds_target() {
    let nums: Vec<i32> = (0..10_000).collect();
    assert_eq!(search(nums, 7777), 7777);
}

#[test]
fn handles_i32_extremes() {
    let nums = vec![i32::MIN, -1, 0, 1, i32::MAX];
    assert_eq!(search(nums.clone(), i32::MIN), 0);
    assert_eq!(search(nums.clone(), i32::MAX), 4);
    assert_eq!(search(nums, 0), 2);
}
